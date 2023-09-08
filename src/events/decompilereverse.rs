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
 * decompilation.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;

/*********************************** DECOMPILER/REVERSING ***********************************/
pub struct DecompileReverse { event: Event }
impl Eventable for DecompileReverse {
  fn on_init(&self) -> Vec<String> {
    print::println("Decompile and reverse");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn decompilereverse(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(DecompileReverse {
    event: Event {
      name:   name,
      desc:   "Decompile and reversing tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/******************************* BOOMERANG *********************************/
pub struct Boomerang { event: Event }
impl Eventable for Boomerang {
  fn on_init(&self) -> Vec<String> {
    // Output directory
    let output = prompt_in_event!("Boomerang>", "Output directory: ");

    // Prompt for file path
    let path = prompt_in_event!("Boomerang>", "File path: ");
    
    return vec![output, path];
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() != 2 {
      return self.event.usage.to_string();
    }
    args.insert(0, "-o".to_string());

    // Run command
    simple_match!(Command::new("cp").args(vec!["ext/boomerang/lib/libboomerang.so", "."]).output());
    let output = simple_match!(run_command(Command::new("ext/boomerang/bin/boomerang-cli").args(args)));
    util::misc::cleanup("libboomerang.so");

    // Print output
    log::log("Boomerang", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["-o", "tst/boomerang/out", "tst/boomerang/inp/hello"]];
    for test in tests {
      util::misc::cleanup("tst/boomerang/out");

      simple_test_match!(Command::new("cp").args(vec!["ext/boomerang/lib/libboomerang.so", "."]).output());
      simple_test_match!(Command::new("ext/boomerang/bin/boomerang-cli").args(test).output());
      util::misc::cleanup("libboomerang.so");
      
      let atf: String = simple_test_match!(fs::read_to_string("tst/boomerang/atf/hello.c"));
      let out: String = simple_test_match!(fs::read_to_string("tst/boomerang/out/hello/hello.c"));

      if atf != out {
        util::misc::cleanup("tst/boomerang/out");
        return TestStatus::Failed;
      }
    }
    util::misc::cleanup("tst/boomerang/out");
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn boomerang(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Boomerang {
    event: Event {
      name:   name,
      desc:   "Boomerang decompiler works with 32-bit formats".to_string(),
      usage:  "Requires a:\n\
                \tOutput directory (string)\n\
                \tBinary file path (string)\n".to_string(),
      author: "The Boomerang Decompiler Project".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/******************************* RetDec *********************************/
pub struct RetDec { event: Event }
impl Eventable for RetDec {
  fn on_init(&self) -> Vec<String> {
    // Prompt for file path
    let path = prompt_in_event!("RetDec>", "File path: ");
    
    return vec![path];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_command(Command::new("ext/retdec/bin/retdec-decompiler.py").args(args)));

    // Print output
    log::log("RetDec", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
    
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["--cleanup", "tst/retdec/inp/add"]];
    for test in tests {
      util::misc::cleanup("tst/retdec/inp/add.*");

      simple_test_match!(Command::new("ext/retdec/bin/retdec-decompiler.py").args(test).output());
      
      let atf: String = simple_test_match!(fs::read_to_string("tst/retdec/atf/add.c"));
      let out: String = simple_test_match!(fs::read_to_string("tst/retdec/inp/add.c"));

      if atf != out {
        util::misc::cleanup("tst/retdec/inp/add.*");
        return TestStatus::Failed;
      }
    }
    util::misc::cleanup("tst/retdec/inp/add.*");
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn retdec(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(RetDec {
    event: Event {
      name:   name,
      desc:   "A retargetable machine-code decompiler based on LLVM".to_string(),
      usage:  "Requires a:\n\
                \tFile path (string)\n".to_string(),
      author: "Avast".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
