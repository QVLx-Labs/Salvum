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
 * binaryanalysis.rs 
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;  
  
/*********************************** BINARY ANALYSIS ***********************************/
pub struct BinaryAnalysis { event: Event }
impl Eventable for BinaryAnalysis {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Binary Analysis");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn binaryanalysis(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(BinaryAnalysis {
    event: Event {
      name:   name,
      desc:   "Binary analysis tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}  
  
/*********************************** Static Bin ***********************************/
pub struct StaticBin { event: Event }
impl Eventable for StaticBin {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Static Binary Analysis");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn staticbin(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(StaticBin {
    event: Event {
      name:   name,
      desc:   "Static Binary analysis tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
 
/*********************************** BINARYANALYSISPLATFORM ***********************************/
pub struct BAP { event: Event }
impl Eventable for BAP {
  fn on_init(&self) -> Vec<String> {
    // Prompt for input file
    let option = prompt_in_event!("BAP>", "Dissasemble or CFG (D or C)? ");
    let mut opt_arg = "";
    if option.eq("D") || option.eq("d") { opt_arg = "dasm"; }
    if option.eq("C") || option.eq("c") { opt_arg = "cfg"; }

    // Prompt for input file
    let input_path = prompt_in_event!("BAP>", "Binary input file: ");

    // Prompt for output file
    let output_path = prompt_in_event!("BAP>", "Output file: ");

    return vec![opt_arg.to_string(), input_path, output_path];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 3 { return self.event.usage.to_string(); }
    // Run command
    if args[0].eq("dasm") {
			let output = simple_match!(run_command(Command::new("bap").args(vec![args[1].clone(),"-d".to_string()])));
			
      // Create a file
			let mut file = simple_match!(File::create(args[2].clone()));

			// Write the address to it
			simple_match!(file.write_all(String::from_utf8_lossy(&output.stdout).as_bytes()));

			return String::from_utf8_lossy(&output.stdout).to_string();
    }
    else if args[0].eq("cfg") || args[0].eq("CFG") {
			let output = simple_match!(run_command(Command::new("bap").args(vec!["objdump",&args[1],"--show-bil"])));
    
			// Create a file
			let mut file = simple_match!(File::create(args[2].clone()));

			// Write the address to it
			simple_match!(file.write_all(String::from_utf8_lossy(&output.stdout).as_bytes()));

			return String::from_utf8_lossy(&output.stdout).to_string();
    }
    else { return self.event.usage.to_string(); }
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["tst/bap/inp/hello-gcc", "-d", "tst/bap/atf/atf.txt"]];
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("bap").args(&test[..2]).output());
      let out_str = String::from_utf8_lossy(&output.stdout);

      // Read file
      let atf_str = simple_test_match!(fs::read_to_string(test[2]));

      let out_lines: Vec<&str> = out_str.split("\n").collect();
      let atf_lines: Vec<&str> = atf_str.split("\n").collect();

      // Compare
      if out_lines.len() != atf_lines.len() {
        debug::print_debug(format!("-- Output --\n{}\n-- Artifact --\n{}", String::from_utf8_lossy(&output.stdout), atf_str));
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn bap(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(BAP {
    event: Event {
      name:   name,
      desc:   "Provides a standard and microexecution interpreter and a symbolic executor".to_string(),
      usage:  "Requires:\n\
                \tOption: dasm or CFG\n\
                \tBinary file path (string)\n\
                \tOutput file path (string)\n".to_string(),
      author: "Carnegie Mellon University Cylab".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
 
/*********************************** BINWALK ***********************************/
pub struct Binwalk { event: Event }
impl Eventable for Binwalk {
  fn on_init(&self) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();

    // Prompt for options
    let opt = match terminal::get_selection(vec!["None", "Extract", "Scan CPU Arch", "Scan Signatures"]) {
      Some(opt) => opt,
      None => 0,
    };
    match opt {
      0 => { }
      1 => { args.push("-e".to_string()); }
      2 => { args.push("-Y".to_string()); }
      _ => { args.push("-B".to_string()); }
    };

    // Prompt for file
    let path = prompt_in_event!("Binwalk>", "Binary File Path: ");
    args.push(path);
    
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 2 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_command(Command::new("binwalk").args(args)));

    log::log("Binwalk", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/binwalk/atf/atf.txt";
    let input_path = "tst/binwalk/inp/test.elf";

    let check = simple_test_match!(Command::new("binwalk").args(vec!["-S",input_path]).output());

    // Read file
    let file_str = simple_test_match!(fs::read_to_string(artifact_path));

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn binwalk(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Binwalk {
    event: Event {
      name:   name,
      desc:   "Module for analyzing, reverse engineering, and extracting firmware images".to_string(),
      usage:  "Prompts you for:\n\
                \tOptions\n\
                \tFile path (string)\n".to_string(),
      author: "ReFirmLabs".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
 
/*********************************** BINARY SECURITY CHECK ***********************************/
pub struct BinarySecurityCheck { event: Event }
impl Eventable for BinarySecurityCheck {
  fn on_init(&self) -> Vec<String> {
    // Prompt for file
    let bin_path = prompt_in_event!("BinarySecurityCheck>", "Binary File Path: ");
    
    // Prompt for lib
    let lib_path = prompt_in_event!("BinarySecurityCheck>", "Library File Path (blank for default): ");
    
    return vec![bin_path, lib_path];
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 2 {
      return self.event.usage.to_string();
    }

    // No library given
    if args.len() == 1 {
      args.push(String::from("--libc=/usr/lib/gcc/x86_64-linux-gnu/9/libcc1.so"));
    // On init, no library given
    } else if args[1] == "" {
      args[1] = String::from("--libc=/usr/lib/gcc/x86_64-linux-gnu/9/libcc1.so");
    // Library given
    } else {
      args[1] = format!("--libc={}", args[1]);
    }

    // Run command
    let output = simple_match!(run_command(Command::new("ext/binseccheck/binary-security-check").args(args)));

    log::log("BinarySecurityCheck", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["--libc=/usr/lib/gcc/x86_64-linux-gnu/9/libcc1.so", "tst/binseccheck/inp/hello-gcc", "tst/binseccheck/atf/output"]];
    for test in tests {
      let output = simple_test_match!(Command::new("ext/binseccheck/binary-security-check").args(test[..2].to_vec()).output());
      let actual = &output.stdout;

      // Load the file into a string
      let expected = simple_test_match!(fs::read(test[2]));

      if !actual.eq(&expected) {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn binseccheck(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(BinarySecurityCheck {
    event: Event {
      name:   name,
      desc:   "Analyzes ELF, Archive, and PE32 binaries for security implementations. Where it displays the security features in colors as so: green is present, red is missing, and yellow is uncertain".to_string(),
      usage:  "Requires a:\n\
                \tBinary file path (string)\n\
                \tLibrary file path (optional string)\n".to_string(),
      author: "Koutheir Attouchi".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
  
/*********************************** RUNTIME ANALYSIS ***********************************/
pub struct RuntimeBin { event: Event }
impl Eventable for RuntimeBin {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Runtime Binary Analysis");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn runtimebin(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(RuntimeBin {
    event: Event {
      name:   name,
      desc:   "Runtime Binary analysis tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
 
/*********************************** QILING ***********************************/
pub struct Qiling { event: Event }
impl Eventable for Qiling {
  fn on_init(&self) -> Vec<String> {
    // Prompt for file
    let bin_path = prompt_in_event!("Qiling>", "Binary File Path: ");
    
    // Prompt for rootfs
    let rootfs_path = prompt_in_event!("Qiling>", "Rootfs Path: ");
    
    return vec![bin_path, rootfs_path];
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() != 2 {
      return self.event.usage.to_string();
    }
    args = vec!["run".to_string(), "-f".to_string(), args[0].clone(), "--rootfs".to_string(), args[1].clone()];

    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/qiling/qltool").args(args)));

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/qiling/atf/atf.txt";
    let input_path = "tst/qiling/inp/test.asm";

    let ql_args = vec!["code", "--os", "linux", "--arch", "x86", "--format", "asm", "-f", input_path];

    let check = simple_test_match!(Command::new("ext/qiling/qltool").args(ql_args).output());

    // Read file
    let file_str = simple_test_match!(fs::read_to_string(artifact_path));

    // Compare
    let check_str =  String::from_utf8_lossy(&check.stderr);
    if check_str != file_str {
      debug::print_debug(&format!("qiling::on_test: artifact doesnt match output\ncheck: {}\n file: {}\n",check_str,file_str));
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn qiling(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Qiling {
    event: Event {
      name:   name,
      desc:   "Binary emulation for a wide range of architectures. x86, x86-64, 8086, ARM, ARM64, MIPS".to_string(),
      usage:  "Prompts you for:\n\
                \tBinary file path (string)\n\
                \tRootfs path (string)\n".to_string(),
      author: "Qiling Framework".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
 
/*********************************** USERCORN ***********************************/
pub struct Usercorn { event: Event }
impl Eventable for Usercorn {
  fn on_init(&self) -> Vec<String> {
    // Prompt for file
    let bin_path = prompt_in_event!("Usercorn>", "Binary File Path: ");
    
    return vec![bin_path];
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() != 1 {
      return self.event.usage.to_string();
    }
    util::misc::reltoabs(&mut args[0]);
    args.insert(0, "run".to_string());
    
    // Run command
    let output = simple_match!(run_console_command(Command::new("./usercorn").args(args).current_dir("ext/usercorn")));
    
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/usercorn/atf/atf.txt";
    let input_path = "tst/usercorn/inp/x86.linux.elf";

    let output = simple_test_match!(Command::new("ext/usercorn/usercorn").args(vec!["run", input_path]).output());
    util::misc::cleanup("test.file");
    let out_str = String::from_utf8_lossy(&output.stdout);

    // Read file
    let file_str = simple_test_match!(fs::read_to_string(artifact_path));

    let mut out_vec: Vec<&str> = out_str.split('\n').collect();
    out_vec.remove(2);
    let mut str_vec: Vec<&str> = file_str.split('\n').collect();
    str_vec.remove(2);
    
    if !out_vec.eq(&str_vec) {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn usercorn(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Usercorn {
    event: Event {
      name:   name,
      desc:   "Dynamic binary emulator supporting x86, ARM, MIPS, and SPARC.".to_string(),
      usage:  "Requires a:\n\
                \tFile path (string)\n".to_string(),
      author: "Ryan Hileman".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
