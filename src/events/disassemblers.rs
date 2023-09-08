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
 * disassembling.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;  
  
/*********************************** DISASSEMBLING ***********************************/
pub struct Disassemblers { event: Event }
impl Eventable for Disassemblers {
  fn on_init(&self) -> Vec<String> {
    print::println("Disassembling");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn disassemblers(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Disassemblers {
    event: Event {
      name:   name,
      desc:   "Disassembling tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
 
/*********************************** UDCLI ***********************************/
pub struct UDCLI { event: Event }
impl Eventable for UDCLI {
  fn on_init(&self) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    // Prompt for file
    let file = prompt_in_event!("UDCLI>", "Binary File: ");
    args.push(file);

    // Prompt for output file
    let output_file = prompt_in_event!("UDCLI>", "Output file (blank for stdout): ");
    if output_file != "" {
      args.push(output_file);
    }
    
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_command(Command::new("ext/udcli/udcli").args(args.clone())));
    log::log("UDCLI", &String::from_utf8_lossy(&output.stderr));

    if args.len() == 1 {
      // Print output
      return String::from_utf8_lossy(&output.stdout).to_string();
    } else {
      // Create a file
      let mut file = simple_match!(File::create(args[1].clone()));

      // Write the address to it
      simple_match!(file.write_all(&output.stdout));
      return String::from("");
    }
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["tst/udcli/inp/hello-gcc", "tst/udcli/atf/output"]];
    for test in tests {
      let output = simple_test_match!(Command::new("ext/udcli/udcli").args(test[..1].to_vec()).output());
      let actual = String::from_utf8_lossy(&output.stdout);

      // Load the file into a string
      let expected = simple_test_match!(fs::read_to_string(test[1]));

      if actual != expected {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn udcli(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(UDCLI {
    event: Event {
      name:   name,
      desc:   "Dissasembler for x86 and x86_64".to_string(),
      usage:  "Prompts you for:\n\
                \tBinary file path (string)\n\
                \tOutput file (optional string)\n".to_string(),
      author: "Copyright (c) 2002-2013 Vivek Thampi".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
/*********************************** RADARE2 ***********************************/
pub struct Radare2 { event: Event }
impl Eventable for Radare2 {
  fn on_init(&self) -> Vec<String> {
    //prompt for file to dissasemble
    return Vec::new();
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() > 2 { 
      return self.event.usage.clone(); 
    }
    if args.len() == 0 { args = vec!["--".to_string()]; }
    else if args.len() == 1 { args.insert(0,"-cV".to_string()); }
    //execute radare command
    let mut radare_child = match Command::new("ext/radare/radare2").args(args).env("LD_LIBRARY_PATH", "ext/radare/").spawn() {
      Ok(child) => child,
      Err(err) => {
        return format!("failed to spawn radare child. {}\n", err);
      }
    };
    let exit_status = match radare_child.wait() {
      Ok(status) => status,
      Err(err) => {
        return format!("failed to wait on radare child. {}\n", err);
      }
    };
    // check for successful exit
    if !exit_status.success() {
      return format!("radare did not exit successfully\n");
    }
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    //./ext/radare2 -cpi -q files/WndRvr_vx5
    let test = vec!["-h"];
    let atf_path_str = "tst/radare/atf/atf.txt";
    let atf_string = match fs::read_to_string(atf_path_str) {
      Ok(string) => string,
      Err(err) => {
        debug::print_debug(&format!("radare::on_test: failed to read artifact to a string. {}",err));
        return TestStatus::Failed;
      }
    };
    let radare_output = match Command::new("ext/radare/radare2").args(test).env("LD_LIBRARY_PATH", "ext/radare/").output() {
      Ok(output) => output,
      Err(err) => {
        debug::print_debug(&format!("radare::on_test: failed to execute radare. {}",err));
        return TestStatus::Failed;
      }
    };
    let radare_output_str = String::from_utf8_lossy(&radare_output.stdout);
    if radare_output_str != atf_string {
      debug::print_debug(&format!("\noutput: {}\n", radare_output_str));
      debug::print_debug(&format!("\nartifact: {}\n", atf_string));
      debug::print_debug("radare::on_test: output does not match artifact.");
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn radare2(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Radare2 {
    event: Event {
      name:   name,
      desc:   "Outputs the assembly instructions from a binary file".to_string(),
      usage:  "radare <binary_file_path>\n".to_string(),
      author: "radare.org".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
