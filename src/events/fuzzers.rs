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
 * fuzzingtools.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;
  
/*********************************** FUZZING ***********************************/
pub struct Fuzzing { event: Event }
impl Eventable for Fuzzing {
  fn on_init(&self) -> Vec<String> {
    print::println("Fuzzing");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn fuzzers(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Fuzzing {
    event: Event {
      name:   name,
      desc:   "Fuzzing tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
 
/*********************************** HONGGFUZZ ***********************************/
pub struct Honggfuzz { event: Event }
impl Eventable for Honggfuzz {
  fn on_init(&self) -> Vec<String> {
    // Prompt for file
    let bin_file = prompt_in_event!("Honggfuzz>", "Binary File Path: ");

    // Prompt for input directory
    let input_dir = prompt_in_event!("Honggfuzz>", "Input Directory Path: ");
    
    return vec![bin_file, input_dir];
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() != 2 {
      return self.event.usage.to_string();
    }
    util::misc::reltoabs(&mut args[0]);
    util::misc::reltoabs(&mut args[1]);
    
    let arguments = vec!["-x".to_string(), "-i".to_string(), args[1].trim().to_string().clone(),
      "-W".to_string(), "files/honggfuzz/workspace".to_string(), "--run_time".to_string(), "10".to_string(), "--".to_string(), args[0].trim().to_string().clone(), "___FILE___".to_string()];

    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/honggfuzz/honggfuzz").args(arguments)));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["-x", "-i", "tst/honggfuzz/inp/inputfiles",
                          "-W", "tst/honggfuzz/inp/workspace", "-n", "1", "-N", "100",
                          "--", "tst/honggfuzz/inp/badcode1", "___FILE___", "tst/honggfuzz/atf/atf.txt"]];
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("ext/honggfuzz/honggfuzz").args(&test[..12]).output());
      let out_str = String::from_utf8_lossy(&output.stderr);

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[12]));

      let out_vec: Vec<&str> = out_str.split('\n').collect();
      let str_vec: Vec<&str> = file_str.split('\n').collect();

      // Compare
      if out_vec[1] != str_vec[1] {
        debug::print_debug(format!("\n-----\n{}\n-----\n{}\n-----\n", out_vec[1], str_vec[1]));
        util::misc::cleanup("tst/honggfuzz/inp/workspace/*");
        return TestStatus::Failed;
      }
    }
    util::misc::cleanup("tst/honggfuzz/inp/workspace/*");
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn honggfuzz(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Honggfuzz {
    event: Event {
      name:   name,
      desc:   "Fuzzes a binary using files given from an input directory".to_string(),
      usage:  "Prompts you for:\n\
                \tBinary file path (string)\n\
                \tInput directory (string)\n".to_string(),
      author: "Google".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
