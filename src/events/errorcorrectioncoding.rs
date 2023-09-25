/*
 * QVLx Salvum 
 *
 * ecctools.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;
use std::io::Read;

/************************************* ECC Tools *************************************/
pub struct ECCTools { event: Event }
impl Eventable for ECCTools {
  fn on_init(&self) -> Vec<String> {
    print::println("ECC Tools");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn errorcorrectioncoding(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync>{
  Box::new(ECCTools {
    event: Event {
      name:   name,
      desc:   "Error Correction Code tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/************************************* BCH Tools **************************************/
pub struct BCHTools { event: Event }
impl Eventable for BCHTools {
  fn on_init(&self) -> Vec<String> {
    print::println("BCH Code tools");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn bchtools(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(BCHTools {
    event: Event {
      name:   name,
      desc:   "BCH Code tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/************************************* REEDSOLOMON **************************************/
pub struct ReedSolomon { event: Event }
impl Eventable for ReedSolomon {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn on_run(&self, _args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/reedsolomon/reed_solomon").args(vec![""])));

    log::log("ReedSolomon", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["3", "0", "12", "tst/reedsolomon/inp/hello"],
                     vec!["3", "0", "12", "tst/reedsolomon/inp/hello"],
                     vec!["3", "0", "12", "tst/reedsolomon/inp/hello"],
                     vec!["3", "0", "12", "tst/reedsolomon/inp/hello"],
                     vec!["3", "0", "12", "tst/reedsolomon/inp/hello"]];
    
    for test in tests {
      util::misc::cleanup("tst/reedsolomon/atf/*");

      // Encode the file
      let mut child = simple_test_match!(Command::new("ext/reedsolomon/reed_solomon").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn());
      let child_stdin = match child.stdin.as_mut() {
        Some(s) => s,
        None => {
          debug::print_debug("No stdin");
          return TestStatus::Failed;
        }
      };
      simple_test_match!(child_stdin.write_all(format!("1\n{}\n{}\n", "tst/reedsolomon/inp/hello.txt", "tst/reedsolomon/atf/hello_enc.txt").as_bytes()));
      drop(child_stdin);
      simple_test_match!(child.wait());

      // Corrupt the encoded file
      simple_test_match!(Command::new("ext/filecorrupter/file_corrupter").args(vec!["-i", &(test[3].to_string() + "_enc.txt"), "-o", &(test[3].to_string() + "_cor.txt"), "-c", test[0], "-l", test[1], "-u", test[2]]).output());

      // Decode the corruption
      let mut child = simple_test_match!(Command::new("ext/reedsolomon/reed_solomon").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn());
      let child_stdin = match child.stdin.as_mut() {
        Some(s) => s,
        None => {
          debug::print_debug("No stdin");
          return TestStatus::Failed;
        }
      };
      simple_test_match!(child_stdin.write_all(format!("2\n{}\n{}\n0\n", test[3].to_string() + "_cor.txt", test[3].to_string() + "_dec.txt").as_bytes()));
      drop(child_stdin);
      simple_test_match!(child.wait());

      // Check diff
      let output = simple_test_match!(Command::new("diff").args(vec![test[3].to_string() + ".txt", test[3].to_string() + "_dec.txt"]).output());

      // Confirm no differences between the files
      if String::from_utf8_lossy(&output.stdout) != "" {
        //println!("{}", String::from_utf8_lossy(&output.stdout));
        return TestStatus::Failed;
      }
    }

    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn reedsolomon(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ReedSolomon {
    event: Event {
      name:   name,
      desc:   "Reed Solomon allows for encoding and decoding with ECC capabilities. Can encode your files, and if it becomes corrupted, the decoder will attempt to recover the file".to_string(),
      usage:  "Requires a:\n\
                \tFile path (string)\n\
                \tByte number (optional decimal integer)\n".to_string(),
      parent: parent,
      author: "r00r00 & Mike Lubinets".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/************************************* Hamming **************************************/
pub struct Hamming { event: Event }
impl Eventable for Hamming {
  fn on_init(&self) -> Vec<String> {
    print::println("Hamming Code tools");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}

pub fn hamming(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Hamming {
    event: Event {
      name:   name,
      desc:   "Hamming Code tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/************************************* RustyHam *************************************/
pub struct RustyHam { event: Event }
impl Eventable for RustyHam {
  fn on_init(&self) -> Vec<String> {
    // Run command
    let output = match run_console_command(&mut Command::new("ext/rustyham/rustyham")) {
      Ok(out) => out,
      Err(err) => {
        println!("Error: {}", err);
        return Vec::new();
      }
    };

    // Print output
    print!("{}", String::from_utf8_lossy(&output.stdout));
    log::log("RustyHam", &String::from_utf8_lossy(&output.stderr));
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/rustyham/atf/atf.txt";

    // Run command
    let check = match run_bounded_command(&mut Command::new("ext/rustyham/rustyham"), false, 3) {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("rustyham::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str {
      debug::print_debug("rustyham::on_test: artifact doesnt match output");
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn rustyham(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(RustyHam {
    event: Event {
      name:   name,
      desc:   "RustyHam tools".to_string(),
      usage:  "Requires a:\n\
                \tOption to select (number)\n\
                \tText to encode/decode (string)\n".to_string(),
      parent: parent,
      author: "$t@$h & github.com/tckmn".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/************************************* Labrador *************************************/
pub struct Labrador { event: Event }
impl Eventable for Labrador {
  fn on_init(&self) -> Vec<String> {
    // Run command
    let output = match run_console_command(&mut Command::new("ext/labrador/lbdr_ecc")) {
      Ok(out) => out,
      Err(err) => {
        println!("Error: {}", err);
        return Vec::new();
      }
    };

    // Print output
    print!("{}", String::from_utf8_lossy(&output.stdout));
    log::log("Ldbr_ecc", &String::from_utf8_lossy(&output.stderr));
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/labrador/atf/atf.txt";

    // Run command
    let check = match run_bounded_command(&mut Command::new("ext/labrador/lbdr_ecc"), false, 1) {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("lbrdr::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn labrador(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Labrador {
    event: Event {
      name:   name,
      desc:   "Labrador LDBC ECC tool".to_string(),
      usage:  "Requires a:\n\
                \tOption to select ('e' or 'd')\n\
                \tCode selection (interactive)\n\
                \tFile path (string)\n".to_string(),
      parent: parent,
      author: "$t@$h & Adam Greig".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/************************************* SecDed *************************************/
pub struct SecDed { event: Event }
impl Eventable for SecDed {
  fn on_init(&self) -> Vec<String> {
    //prompt the user for encode or decode option
    let option = prompt_in_event!("SecDed>","Enter decode/encode option 'd' or 'e' : ");
    if !(option.eq("e") || option.eq("d")) { //handle invalid input
      println!("Invalid option : {}",option);
      return Vec::new();
    }

    //prompt the user for codec block size
    let size = prompt_in_event!("SecDed>","Enter codec block size '64' or '128' : ");
    if !(size.eq("64") || size.eq("128")) { //handle invalid input
      println!("Invalid size : {}",size);
      return Vec::new();
    }

    //prompt the user for the input path
    let path_input = prompt_in_event!("SecDed>","Enter an input file path : ");
    if !Path::new(&path_input).exists() { //make sure that the file path exists
      println!("Input file does not exist : {}",path_input);
      return Vec::new();
    } 

    //prompt the user for the output path
    //let path_output = prompt_in_event!("SecDed>","Enter an output file path : ");
    //dont need to check for existence... secded will create this file
    let path_output = "out/secded/output_codec";

    //execute the encode or decode operation
    let _output = match run_command(Command::new("ext/secded/salvum_secded")
      .args(vec![option,size,path_input,path_output.to_string()])) {
      Ok(out) => out,
      Err(err) => {
        println!("Error : {}",err);
        return Vec::new();
      }
    };

    //verify that the output file exists
    if Path::new(&path_output).exists() {
      println!("Operation successful. Output at : {}",path_output);
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["e","64","tst/secded/dbase.c","tst/secded/dbase.e","tst/secded/secded_e_64_dbase-c_dbase-e"],
                     vec!["e","128","tst/secded/drive.c","tst/secded/drive.e","tst/secded/secded_e_128_drive-c_drive-e"],
                     vec!["d","64","tst/secded/dbase.e","tst/secded/dbase.d","tst/secded/secded_e_128_dbase-e_dbase-d"],
                     vec!["d","128","tst/secded/drive.e","tst/secded/drive.d","tst/secded/secded_e_128_drive-e_drive-d"]];
    
    for mut test in tests {
      let test_file_path = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("secded::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };
      let _output = match Command::new("ext/secded/salvum_secded").args(test.clone()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("salvum_secded::on_test: failed to execute salvum_secded. {}",err));
          return TestStatus::Failed;
        }
      };
  
      //read test file path
      let mut test_file = match File::open(test_file_path) {
        Ok(file) => file,
        Err(err) => {
          debug::print_debug(format!("salvum_secded::on_test: failed to open test file artifact. \"{}\" {}", test_file_path, err));
          return TestStatus::Failed;
        }
      };
      let mut test_file_buffer = Vec::<u8>::new();
      let _num_bytes = match test_file.read(&mut test_file_buffer) {
        Ok(num) => num,
        Err(err) => {
          debug::print_debug(format!("salvum_secded::on_test: failed to read test file artifact. \"{}\" {}", test_file_path, err));
          return TestStatus::Failed;
        }
      };
      //read output file path
      let mut output_file = match File::open(test[3]) {
        Ok(file) => file,
        Err(err) => {
          debug::print_debug(format!("salvum_secded::on_test: failed to open output file. \"{}\" {}", test[3], err));
          return TestStatus::Failed;
        }
      };
      let mut output_file_buffer = Vec::<u8>::new();
      let _num_bytes = match output_file.read(&mut output_file_buffer) {
        Ok(num) => num,
        Err(err) => {
          debug::print_debug(format!("salvum_secded::on_test: failed to read test file artifact. \"{}\" {}", test[3], err));
          return TestStatus::Failed;
        }
      };
      if test_file_buffer != output_file_buffer {
        //let t = match String::from_utf8(test_file_buffer) {
        //  Ok(string) => string,
        //  Err(err) => {
        //    println!("err. {}", err);
        //    return TestStatus::Failed;
        //  }
        //};
        //let o = match String::from_utf8(output_file_buffer) {
        //  Ok(string) => string,
        //  Err(err) => {
        //    println!("err. {}", err);
        //    return TestStatus::Failed;
        //  }
        //};
        //println!("\ntest\n{}\noutput\n{}",t,o);

        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn secded(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SecDed {
    event: Event {
      name:   name,
      desc:   "SecDed - will encode or decode files to prevent or detect bit errors. If two bits are altered, the double error is detected but not corrected. If more than two bits are altered, the results are ambiguous.".to_string(),
      usage:  "Prompts you for: \
              \n\tOption e for encode, d for decode \
              \n\tCodec block size 64, 128 \
              \n\tInput payload file path \n".to_string(),
      parent: parent,
      author: "n3wm4n & Pierre Avital".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/************************************* TESTING TOOLS **************************************/
pub struct ECCTestingTools { event: Event }
impl Eventable for ECCTestingTools {
  fn on_init(&self) -> Vec<String> {
    print::println("ECC Testing tools");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ecctests(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ECCTestingTools {
    event: Event {
      name:   name,
      desc:   "ECC Tests".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/************************************* FILECORRUPTER **************************************/
pub struct FileCorrupter { event: Event }
impl Eventable for FileCorrupter {
  fn on_init(&self) -> Vec<String> {
    // Prompt for input file
    let filein = prompt_in_event!("FileCorrupter>", "Input file: ");
    
    // Prompt for output file
    let fileout = prompt_in_event!("FileCorrupter>", "Output file: ");

    // Prompt for bytes to corrupt
    let corruption = prompt_in_event!("FileCorrupter>", "Number of bytes to corrupt: ");

    // Prompt for lower bound
    let lower_bound = prompt_in_event!("FileCorrupter>", "Lower bound of the corruption: ");

    // Prompt for upper bound
    let upper_bound = prompt_in_event!("FileCorrupter>", "Upper bound of the corruption: ");
    
    return vec!["-i".to_string(), filein, "-o".to_string(), fileout, "-c".to_string(), corruption, "-l".to_string(), lower_bound, "-u".to_string(), upper_bound];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 5 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/filecorrupter/file_corrupter").args(args)));

    log::log("FileCorrupter", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["0", "3", "0", "100", "hello"],
                     vec!["1", "3", "0", "100", "hello"],
                     vec!["2", "3", "0", "100", "hello"],
                     vec!["3", "3", "0", "100", "hello"],
                     vec!["4", "3", "0", "100", "hello"]];
    
    for test in tests {
      let input = &format!("tst/filecorrupter/inp/{}.txt", test[4]);
      let orig = &format!("tst/filecorrupter/inp/{}{}.txt", test[4], test[0]);
      let output = &format!("tst/filecorrupter/atf/{}_corrupted{}.txt", test[4], test[0]);
      util::misc::cleanup(output);

      // Run command
      simple_test_match!(Command::new("ext/filecorrupter/file_corrupter").args(vec!["-s", test[0], "-c", test[1], "-l", test[2], "-u", test[3], "-i", input, "-o", output]).output());

      // Confirm no differences between the files
      let output = simple_test_match!(Command::new("diff").args(vec![orig, output]).output());
      if String::from_utf8_lossy(&output.stdout) != "" {
        debug::print_debug(format!("{}", String::from_utf8_lossy(&output.stdout)));
        return TestStatus::Failed;
      }
    }

    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn filecorrupter(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(FileCorrupter {
    event: Event {
      name:   name,
      desc:   "Overwrites random bytes within a given file".to_string(),
      usage:  filter::parse_help_menu(Command::new("ext/filecorrupter/file_corrupter").args(vec!["-h"]),vec!["!ALL!".to_string()], "\n", 0, ""),
      parent: parent,
      author: "r00r00".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
