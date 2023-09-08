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
 * rawtools.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;

/*********************************** RAW TOOLS ***********************************/
pub struct RawTools { event: Event }
impl Eventable for RawTools {
  fn on_init(&self) -> Vec<String> {
    print::println("Raw Tools.");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn raw(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(RawTools {
    event: Event {
      name:   name,
      desc:   "Raw tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** ROP ***********************************/
pub struct Rop { event: Event }
impl Eventable for Rop {
  fn on_init(&self) -> Vec<String> {
    print::println("Return Oriented Programming Tools");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn rop(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Rop {
    event: Event {
      name:   name,
      desc:   "Return Oriented Programming Tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** ropr ***********************************/
pub struct Ropr { event: Event }
impl Eventable for Ropr {
  fn on_init(&self) -> Vec<String> {
    // Prompt for seed
    let input_file = prompt_in_event!("Ropr>", "Path to binary: ");

    return vec![input_file];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 { return self.event.usage.to_string(); }

    // Run command
    if main_info::is_bg() || main_info::get_file_redirect() {
			let output = simple_match!(run_command(Command::new("ext/ropr/ropr").args(args)));
			log::log("Ropr", &String::from_utf8_lossy(&output.stderr));
			return String::from_utf8_lossy(&output.stdout).to_string();
    }
    else {
      simple_match!(Command::new("ext/ropr/ropr").args(args).status());
			return String::from("");
    }
  }
  fn on_test(&self) -> TestStatus {
    let atf_file = "tst/ropr/atf/atf.txt";
 
    let output = simple_test_match!(Command::new("ext/ropr/ropr").arg("--help").output());
    let file_str = simple_test_match!(fs::read_to_string(atf_file));
    let output_str = String::from_utf8_lossy(&output.stdout);
    if output_str != file_str {
      debug::print_debug(&format!("output: {}\natf: {}\n",output_str, file_str));
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ropr(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Ropr {
    event: Event {
      name:   name,
      desc:   "Detects and prints ROP gadgets for a given binary.".to_string(),
      usage:  "Requires a:\n\
                \tInput file path (string)\n".to_string(),
      parent: parent,
      author: "Ben Lichtman (B3NNY)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** MANIPULATION ***********************************/
pub struct Manipulation { event: Event }
impl Eventable for Manipulation {
  fn on_init(&self) -> Vec<String> {
    print::println("Binary Manipulation");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn manipulation(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Manipulation {
    event: Event {
      name:   name,
      desc:   "Binary Manipulation".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** symtool ***********************************/
pub struct Symtool { event: Event }
impl Eventable for Symtool {
  fn on_init(&self) -> Vec<String> {
    // Prompt for seed
    let subcommand = prompt_in_event!("Symtool>", "Subcommand (hide | rename): ");
    if subcommand == "hide" {
      let input_file = prompt_in_event!("Symtool>","Path to input binary: ");
      let output_file = prompt_in_event!("Symtool>","Path to output binary: ");
      let symbol_to_hide = prompt_in_event!("Symtool>", "Symbol to hide: ");
      return vec!["hide".to_string(),input_file,output_file,symbol_to_hide];
    }
    else if subcommand == "rename" {
      let input_file = prompt_in_event!("Symtool>","Path to input binary: ");
      let output_file = prompt_in_event!("Symtool>","Path to output binary: ");
      let old_name = prompt_in_event!("Symtool>","Symbol to rename: ");
      let new_name = prompt_in_event!("Symtool>","New name: ");
      return vec!["rename".to_string(),input_file,output_file,old_name,new_name];
    }
    else { return Vec::new(); }
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 5 {
      return self.event.usage.to_string();
    }
    if args[0] == "hide" {
      if args.len() != 4 { return self.event.usage.to_string(); }
      let output = simple_match!(run_command(Command::new("ext/symtool/symtool").args(vec!["--verbose".to_string(), "--hidden".to_string(), args[1].clone(), args[2].clone(),args[3].clone()])));
      log::log("Symtool", &String::from_utf8_lossy(&output.stderr));
      return String::from_utf8_lossy(&output.stdout).to_string();
    }
    else if args[0] == "rename" {
      if args.len() != 5 { return self.event.usage.to_string(); }
      let output = simple_match!(run_command(Command::new("ext/symtool/symtool").args(vec!["--verbose".to_string(), "--rename".to_string(), args[1].clone(), args[2].clone(), args[3].clone(), args[4].clone()])));
      log::log("Symtool", &String::from_utf8_lossy(&output.stderr));
      return String::from_utf8_lossy(&output.stdout).to_string();
    }
    else {
      return self.event.usage.to_string();
    }
  }
  fn on_test(&self) -> TestStatus {
    let atf_file = "tst/symtool/atf/atf.txt";
 
    let output = simple_test_match!(Command::new("ext/symtool/symtool").arg("--version").output());
    let file_str = simple_test_match!(fs::read_to_string(atf_file));
    let output_str = String::from_utf8_lossy(&output.stdout);
    if output_str != file_str {
      debug::print_debug(&format!("output: {}\natf: {}\n",output_str, file_str));
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn symtool(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Symtool {
    event: Event {
      name:   name,
      desc:   "Static symbol manipulation for ELF binaries.".to_string(),
      usage:  "Works like this:\n\tsymtool [hide|rename] [args...]\n\nExamples:\n\tsymtool hide <symbol_to_hide> <old_bin_path> <new_bin_path>\n\tsymtool rename <old_symbol_name> <new_symbol_name> <old_bin_path> <new_bin_path>\n".to_string(),
      parent: parent,
      author: "Caleb Zulawski".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** UNPACKING ***********************************/
pub struct Unpacking { event: Event }
impl Eventable for Unpacking {
  fn on_init(&self) -> Vec<String> {
    print::println("Unpacking");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn unpacking(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Unpacking {
    event: Event {
      name:   name,
      desc:   "Unpacking utilities".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** VMLINUX-TO-ELF ***********************************/
pub struct DecompressELF { event: Event }
impl Eventable for DecompressELF {
  fn on_init(&self) -> Vec<String> {
    // Prompt for seed
    let input_file = prompt_in_event!("DecompressELF>", "Input file: ");

    // Prompt for bits
    let output_file = prompt_in_event!("DecompressELF>", "Output file: ");

    return vec![input_file, output_file];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 2 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_command(Command::new("ext/decompresself/main.py").args(args)));
    log::log("DecompressELF", &String::from_utf8_lossy(&output.stderr));

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    util::misc::cleanup("tst/decompresself/vmlinux");
    let test = vec!["ext/decompresself/main.py", "tst/decompresself/inp/bzImage", "tst/decompresself/vmlinux"];

    // Run command
    simple_test_match!(Command::new("python3").args(test).output());

    // Run check
    let check_out = simple_test_match!(util::pipes::Pipe::new(&"file tst/decompresself/vmlinux".to_string())
                                        .then("grep ELF")
                                        .then("wc -l")
                                        .finally());

    let check = simple_test_match!(check_out.wait_with_output());

    // Read file
    let file_str = simple_test_match!(fs::read_to_string("tst/decompresself/atf/decelf_out.txt"));

    // Compare
    let output_str = String::from_utf8_lossy(&check.stdout);
    if output_str != file_str {
      debug::print_debug(&format!("output : {}\natf : {}\n",output_str, file_str));
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn decompresself(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(DecompressELF {
    event: Event {
      name:   name,
      desc:   "Used to unpack your compressed elf files".to_string(),
      usage:  "Requires a:\n\
                \tInput file path (string)\n\
                \tOutput file path (string)\n".to_string(),
      parent: parent,
      author: "Marin M".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** x7z ***********************************/
pub struct X7z { event: Event }
impl Eventable for X7z {
  fn on_init(&self) -> Vec<String> {
    // Prompt for seed
    let input_file = prompt_in_event!("7z>", "Path to target to unpack: ");
    return vec![input_file];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 { 
      return self.event.usage.clone(); 
    }
    let input = args[0].clone();
    if !std::path::Path::new(&input).exists() {
      return String::from("Package not found.\n");
    }
    let basename = input.split(".").collect::<Vec<&str>>()[0];
    let mut output = String::from("-o");
    output.push_str(basename);

    if std::path::Path::new(basename).exists() {
      return String::from("This path already exists, exiting early\n");
    }

    // Run command
    let output = simple_match!(run_command(Command::new("ext/x7z/7z").args(vec!["x",&args[0],&output])));

    // Print output
    let out = String::from_utf8_lossy(&output.stdout);
    let out_nl = out.split("\n").collect::<Vec<&str>>();
    for i in 4..out_nl.len() {
      print::print_custom(out_nl[i],"gold");
      if i != (out_nl.len() - 1) { println!(); }
    }
    let mut done = String::from("Extracted package to --> ");
    done.push_str(basename); 
    print::print_custom(&done,"brightgreen");
    print::println("");
    log::log("7z", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    // Run command
    let check = simple_test_match!(Command::new("ext/x7z/7z").arg("-h").output());
    // Read file
    let file_str = simple_test_match!(fs::read_to_string("tst/x7z/atf/atf.txt"));

    // Compare
    if String::from_utf8_lossy(&check.stdout).contains(&file_str) {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn x7z(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(X7z {
    event: Event {
      name:   name,
      desc:   "Used to unpack CAB, ARJ, CPIO, RPM and DEB packages".to_string(),
      usage:  "x7z <path_to_package>\n".to_string(),
      parent: parent,
      author: "7-Zip (Igor Pavlov)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
/*********************************** DECRYPTION ***********************************/
pub struct Decryption { event: Event }
impl Eventable for Decryption {
  fn on_init(&self) -> Vec<String> {
    print::println("Decryption");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn decryption(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Decryption {
    event: Event {
      name:   name,
      desc:   "Decryption tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** CIPHEY ***********************************/
pub struct Ciphey { event: Event }
impl Eventable for Ciphey {
  fn on_init(&self) -> Vec<String> {
    //Prompt for binary mode
    let is_bnry_in = &prompt_in_event!("Ciphey>", "Am I going to decrypt a binary? Y or N: ");
    let mut is_bnry: bool = false;
    if is_bnry_in.eq("Y") { is_bnry = true; }
    else{ println!("Ok I'll need non-binary input."); }

    let mut infile = String::new();
    let mut instring = String::new();
    
    // Prompt for input file
    let is_infile = &prompt_in_event!("Ciphey>", "Shall I read from a file? Y or N: ");

    if is_infile.eq("Y") {
      // Prompt for file path
      let filename = &prompt_in_event!("Ciphey>", "Input file path: ");
      infile.push_str(&filename.to_string());
    }
    else{
      // Prompt for string
      println!("Ok I'll need an input string.");
      let in_str = &prompt_in_event!("Ciphey>", "Input string: ");
      instring.push_str(&in_str.to_string());
    }

    if is_infile.eq("Y") && is_bnry == true {
      let output = match run_command(Command::new("ext/ciphey/ciphey").args(vec!["-q", "-f", &infile, "-b"])) {
        Ok(out) => out,
        Err(err) => { println!("Error: {}", err); return Vec::new(); }
      }; 
      print!("{}", String::from_utf8_lossy(&output.stdout));
      log::log("Ciphey", &String::from_utf8_lossy(&output.stderr));
    }
    else if is_infile.eq("Y") && is_bnry == false {
      let output = match run_command(Command::new("ext/ciphey/ciphey").args(vec!["-q", "-f", &infile])) {
        Ok(out) => out,
        Err(err) => { println!("Error: {}", err); return Vec::new(); }
      };
      print!("{}", String::from_utf8_lossy(&output.stdout));
      log::log("Ciphey", &String::from_utf8_lossy(&output.stderr));
    } 
    else if is_infile.eq("N") && is_bnry == true {
      let output = match run_command(Command::new("ext/ciphey/ciphey").args(vec!["-q", "-b", &instring])) {
        Ok(out) => out,
        Err(err) => { println!("Error: {}", err); return Vec::new(); }
      };
      print!("{}", String::from_utf8_lossy(&output.stdout));
      log::log("Ciphey", &String::from_utf8_lossy(&output.stderr));
    } 
    else{
      let output = match run_command(Command::new("ext/ciphey/ciphey").args(vec!["-q", &instring])) {
        Ok(out) => out,
        Err(err) => { println!("Error: {}", err); return Vec::new(); }
      };  
      print!("{}", String::from_utf8_lossy(&output.stdout));
      log::log("Ciphey", &String::from_utf8_lossy(&output.stderr));
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let mut tests = vec![
      vec!["-q","-f","tst/ciphey/inp/encrypted0","tst/ciphey/atf/ciphey_-q_-f_encrypted0"],
      vec!["-q","-f","tst/ciphey/inp/encrypted1","tst/ciphey/atf/ciphey_-q_-f_encrypted1"],
      vec!["-q","-f","tst/ciphey/inp/encrypted2","tst/ciphey/atf/ciphey_-q_-f_encrypted2"]
    ];
    let command = "ext/ciphey/ciphey";
    for test in &mut tests {
      //get artifact path
      let artifact_path_str = match test.pop() {
        Some(string) => string,
        None => {
          debug::print_debug("ciphey::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };

      //execute ciphey command
      let output = match Command::new(command).args(test).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("ciphey::on_test: failed to execute ciphey. {}", err));
          return TestStatus::Failed;
        }
      };

      //read output vector into a string
      let output_str = match String::from_utf8(output.stdout) {
        Ok(string) => string,
        Err(err) => {
          debug::print_debug(format!("ciphey::on_test: failed to read output to a string. {}", err));
          return TestStatus::Failed;
        }
      };

      //read artifact file to string
      let artifact_str = match fs::read_to_string(artifact_path_str) {
        Ok(string) => string,
        Err(err) => {
          debug::print_debug(format!("ciphey::on_test: failed to read artifact file to a string. {}", err));
          return TestStatus::Failed;
        }
      };

      //compare output to artifact
      if output_str != artifact_str {
        debug::print_debug("ciphey::on_test: artifact does not match output");
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ciphey(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Ciphey {
    event: Event {
      name:   name,
      desc:   "Decrypts your encrypted files".to_string(),
      usage:  "Just type ciphey to invoke\n".to_string(),
      parent: parent,
      author: "Ciphey (github.com/ciphey)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
