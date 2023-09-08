/*
 * (c) 2021 QVLX LLC. All rights reserved.
 *
 * !! WARNING !!
 * THIS DOCUMENT CONTAINS CONFIDENTIAL AND PROPRIETARY DATA ORIGINATED BY QVLX LLC.
 * ALL DESIGN, MANUFACTURING PRODUCTION, USE, SALE AND PATENT RIGHTS ARE EXPRESSLY RESERVED.
 *
 * THE DATA CONTAINED IN THIS DOCUMENT IS SUBJECT TO ITAR/EAR RESTRICTIONS.
 *
 * THE RECIPIENT AGREES BY VIEWING THIS DOCUMENT NOT TO SUPPLY OR DISCLOSE ANY INFORMATION
 * REGARDING IT TO UNAUTHORIZED PERSONS OR INCORPORATE INTO ANY OTHER DESIGN OR USE THEREOF.
 *
 * VIOLATION OF THESE TERMS WILL BE SUBJECT TO PROSECUTION AT THE FULL EXTENT OF THE LAW.
 */

/*
 * QVLx Salvum 
 *
 * packetsniffers.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;

/*********************************** StegDetection ***********************************/
pub struct StegDetection { event: Event }
impl Eventable for StegDetection {
  fn on_init(&self) -> Vec<String> {
    print::println("Steganography Detection");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn stegdetection(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(StegDetection {
    event: Event {
      name:   name,
      desc:   "Steganography".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** UnicodeSecurity ***********************************/
pub struct UnicodeSecurity { event: Event }
impl Eventable for UnicodeSecurity {
  fn on_init(&self) -> Vec<String> {
    print::println("Unicode Security");
    let file_name = prompt_in_event!("unicode_sec>", "Path to file to scan: ");
    if file_name.trim().eq("") {
      println!("Need a valid file path.");  
      return Vec::new();
    }
    print::print_custom(&file_name, "brightgreen");
    println!();
    return vec![file_name.clone()];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 { return self.event.usage.clone(); }
    let file_name = args[0].clone();
    let split_prd: Vec<&str> = file_name.split(".").collect();
    let first_token_prd = split_prd[0];
    let split_slsh: Vec<&str> = first_token_prd.split("/").collect();
    let first_token_slsh = split_slsh[split_slsh.len() - 1];
    let mut out_filename = String::from("out/unicodesec/");
    out_filename.push_str(first_token_slsh);
    out_filename.push_str("_out.txt");
    
    let cfg_filename = String::from("cfg/unicodesec.cfg");
    let arguments = vec![file_name.trim().to_string(), out_filename, cfg_filename];
    
    // Run command
    let out = match run_console_command(Command::new("ext/unicodesec/unicode_sec").args(&arguments)) {
      Ok(o) => o,
      Err(err) => {
        return format!("failed to execute unicode sec. {}\n", err); 
      }   
    };
    return String::from_utf8_lossy(&out.stdout).to_string();
    //log::log("unicodesecurity", &String::from_utf8_lossy(&output.stderr));
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/unicodesec/atf/atf.txt";
    let input_path = "tst/unicodesec/inp/test.txt";
    let config_path = "tst/unicodesec/inp/unicodesec.cfg";
    let staging_path = "tst/unicodesec/out.txt";
    util::misc::cleanup(staging_path);

    let sec_args = vec![input_path, staging_path, config_path];

    // Run command
    match Command::new("ext/unicodesec/unicode_sec").args(sec_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Read output file
    let out_str = match fs::read_to_string(staging_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("unicodesec::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read artifact file
    let atf_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("unicodesec::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if out_str != atf_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn unicodesecurity(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(UnicodeSecurity {
    event: Event {
      name:   name,
      desc:   "Check for suspicious unicode characters in a file.".to_string(),
      usage:  "Requires a:\n\
                \tFile path (string)\n".to_string(),
      parent: parent,
      author: "m0nZSt3r & github.com/unicode-rs".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** PicSec ***********************************/
pub struct PicSec { event: Event }
impl Eventable for PicSec {
  fn on_init(&self) -> Vec<String> {
    print::println("Pic Security");
   
    // Prompt for original file path
    let source_name = prompt_in_event!("pic_sec>", "Path to original photo to scan: ");
 
    if source_name.trim().eq("") {
      println!("Need a valid file path.");  
      return Vec::new();
    }
    print::print_custom(&source_name, "brightgreen");
    println!();
    
    // Prompt for suspect file path
    let suspect_name = prompt_in_event!("pic_sec>", "Path to suspect photo to scan: ");
 
    if suspect_name.trim().eq("") {
      println!("Need a valid file path.");  
      return Vec::new();
    }
    print::print_custom(&suspect_name, "brightgreen");
    println!();
    return vec![source_name.clone(), suspect_name.clone()];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 2 || args.len() > 2 { 
      return self.event.usage.clone(); 
    }
    let arguments = vec![args[0].clone(), args[1].clone()];
    
    // Run command
    let output = match run_console_command(Command::new("ext/picsec/pic_sec").args(&arguments)) {
      Ok(out) => {
        log::log("picsec", &String::from_utf8_lossy(&out.stderr));
        out
      },
      Err(err) => { 
        return format!("failed to execute pic sec. {}\n", err);
      }   
    };
    return String::from_utf8_lossy(&output.stdout).to_string();
    //log::log("picsec", &String::from_utf8_lossy(&output.stderr));
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/picsec/atf/atf.txt";
    let input1_path = "tst/picsec/inp/source.png";
    let input2_path = "tst/picsec/inp/secrets.png";

    let sec_args = vec![input1_path, input2_path];

    // Run command
    let check = match Command::new("ext/picsec/pic_sec").args(sec_args).output() {
      Ok(o) => o,
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Read artifact file
    let atf_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("picsec::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != atf_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn picsec (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PicSec {
    event: Event {
      name:   name,
      desc:   "Diff a suspicious photo and an original to detect tampering.".to_string(),
      usage:  "picsec <photo1> <photo2>\n".to_string(),
      parent: parent,
      author: "m0nZSt3r".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
