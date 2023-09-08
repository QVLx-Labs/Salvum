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
 * cracking.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;

/*********************************** CRACKING ***********************************/
pub struct Cracking { event: Event }
impl Eventable for Cracking {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Cracking");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn cracking(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Cracking {
    event: Event {
      name:   name,
      desc:   "Cracking tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** PASSWORDHASH CRACKING ***********************************/
pub struct PasswordHash { event: Event }
impl Eventable for PasswordHash {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Password & Hash Cracking");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn passwordhash(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PasswordHash {
    event: Event {
      name:   name,
      desc:   "Password and hack cracking tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** JOHNTHERIPPER ************************************/
pub struct JohnTheRipper { event: Event }
impl Eventable for JohnTheRipper {
  fn on_init(&self) -> Vec<String> {
    let cmd = prompt_in_event!("JohnTheRipper>", "");
    let cmd_args = cmd.split(" ").collect();
    if filter::if_command_contains_rule(&cmd_args, filter::load_rules_cfg("johntheripper")) {
      let output = match run_command(Command::new("ext/johntheripper/john/run/john").args(cmd_args)) {
        Ok(out) => out,
        Err(err) => {
          println!("Error: {}", err);
          return Vec::new();
        }
      };
      print!("{}",String::from_utf8_lossy(&output.stderr));
      print!("{}",String::from_utf8_lossy(&output.stdout));
    } else {
      println!("Not supported.");
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/johntheripper/atf/atf.txt";

    // Run check
    let check = match run_bounded_command(Command::new("ext/johntheripper/john/run/john").arg("--test=0"),false,7) {
        Ok(out) => out,
        Err(err) => {
          println!("\nError: {}",err);
          return TestStatus::Failed;
        }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("jtr::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Compare
    let out_str_utf8 = String::from_utf8_lossy(&check.stdout);
    let output_tkns_nl = out_str_utf8.split("\n").collect::<Vec<&str>>();
    let file_tkns_nl = file_str.split("\n").collect::<Vec<&str>>();
    if (output_tkns_nl.len() < 5) || (file_tkns_nl.len() < 5) {
      debug::print_debug("jtr::on_test: Output has less than expected");
      return TestStatus::Failed;
    }
    for i in 1..5 {
      if output_tkns_nl[i] != file_tkns_nl[i] {
        debug::print_debug(format!("jtr::on_test: Difference on line {}\n-----\n{}\n-----\n{}\n", i, output_tkns_nl[i], file_tkns_nl[i]));
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn johntheripper(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(JohnTheRipper {
    event: Event {
      name:   name,
      desc:   "The famous John password and hash cracking tool.".to_string(),
      usage:  filter::parse_help_menu(Command::new("ext/johntheripper/john/run/john").args(vec!["-h"]),
                                      filter::load_rules_cfg("johntheripper"), "\n--", 0, 
                                      "John the Ripper\n\
                                       Copyright (c) 1996-2021 by Solar Designer and others\n\
                                       Usage: [OPTIONS] [PASSWORD-FILES]").to_string(),
      author: "Openwall Project".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** HASHCAT ************************************/
pub struct HashCat { event: Event }
impl Eventable for HashCat {
  fn on_init(&self) -> Vec<String> {
    alerts::print_generic_warning("HashCat is only supported on a subset of systems.");
    let cmd = prompt_in_event!("HashCat>", "");
    let cmd_args = cmd.split(" ").collect();
    if filter::if_command_contains_rule(&cmd_args, filter::load_rules_cfg("hashcat")) {
      let output = match run_command(Command::new("hashcat").args(cmd_args)) {
        Ok(out) => out,
        Err(err) => {
          println!("Error: {}", err);
          return Vec::new();
        }
      };
      print!("{}", String::from_utf8_lossy(&output.stderr));
      print!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
      println!("Not supported.");
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/hashcat/atf/atf.txt";
    let input_path = "tst/hashcat/inp/testhash.txt";
    let staging_path = "tst/hashcat/crack.txt";
    let blank_path = "tst/hashcat/inp/blankfile.txt";
    //let wordlist_path = "res/KaliLists/rockyou.txt";
    let wordlist_path = "tst/hashcat/inp/wordlist.txt";
    
    util::misc::cleanup(staging_path);
    
    // Copy blank file to staging location
    let copy_args = vec![blank_path, staging_path];
    match Command::new("cp").args(copy_args).output() {
        Ok(_) => (),
        Err(err) => {
          println!("\nError: {}",err);
          return TestStatus::Failed;
        }
    };
    
    // Change permissions of staging file to write to it
    let chmod_args = vec!["777", staging_path];
    match Command::new("chmod").args(chmod_args).output() {
        Ok(_) => (),
        Err(err) => {
          println!("\nError: {}",err);
          return TestStatus::Failed;
        }
    };
    
    // Run check
    let hashcat_args = vec!["--force","--potfile-disable","-m","0","-a","0","-o",staging_path,input_path,wordlist_path];
    match Command::new("hashcat").args(hashcat_args).output() {
        Ok(_) => (),
        Err(err) => {
          println!("\nError: {}",err);
          return TestStatus::Failed;
        }
    };

    // Read crack file
    let crack_str = match fs::read_to_string(staging_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("hashcat::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read artifact file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("hashcat::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    let crack_str_split_colon = crack_str.split(":").collect::<Vec<&str>>();
    
    if (crack_str_split_colon.len() < 2) || (crack_str_split_colon[1] != file_str) { 
      debug::print_debug(&format!("crack_str: {:?}\nfile_str{}",crack_str_split_colon,file_str));
      debug::print_debug("output doesnt match artifact");
      return TestStatus::Failed; 
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn hashcat(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(HashCat {
    event: Event {
      name:   name,
      desc:   "The well-known HashCat password and hash cracking tool.".to_string(),
      usage:  filter::parse_help_menu(Command::new("hashcat").args(vec!["-h"]),
                                      filter::load_rules_cfg("hashcat"), "\n", 0, 
                                      "HashCat\n\
                                      Usage: [options] [hash|hashfile|hccapxfile] [dictionary|mask|directory]").to_string(),
      author: "hashcat (github.com/hashcat)".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** RAINBOWCRACK ************************************/
pub struct RainbowCrack { event: Event }
impl Eventable for RainbowCrack {
  fn on_init(&self) -> Vec<String> {
    print::println("RainbowCrack");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn rainbowcrack(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(RainbowCrack {
    event: Event {
      name:   name,
      desc:   "".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

fn setup_rainbowcrack_workspace(cmnd: &mut Command, cmnd_name: &str, workspace: &str) -> std::io::Result<std::process::Output> {
  // Copy everything over
  for file in vec!["ext/rainbowcrack/alglib0.so", "ext/rainbowcrack/charset.txt", &format!("ext/rainbowcrack/{}", cmnd_name)] {
    let _ = match run_console_command(Command::new("cp").args(vec![file, &workspace])) {
      Ok(out) => out,
      Err(err) => {
        println!("Error copying the file. {}", err);
        return Err(err);
      }
    };
  }

  // Run command
  let output = match run_console_command(cmnd.current_dir(workspace.clone())) {
    Ok(out) => out,
    Err(err) => {
      println!("Error executing {}. {}", cmnd_name, err);
      return Err(err);
    }
  };

  // Clear workspace
  for file in vec!["alglib0.so", "charset.txt", cmnd_name] {
    let fname = format!("{}/{}", workspace, file);
    util::misc::cleanup(&fname);
  }

  return Ok(output);
}

/*********************************** RTGEN ************************************/
pub struct RTGen { event: Event }
impl Eventable for RTGen {
  fn on_init(&self) -> Vec<String> {
    print::print("Hash Algorithm: ");
    let hash_alg_vec = vec!["lm",
                            "ntlm",
                            "md5",
                            "sha1",
                            "mysqlsha1",
                            "halflmchall",
                            "ntlmchall",
                            "oracle-SYSTEM",
                            "md5-half"];
    let hash_alg = match terminal::get_selection(hash_alg_vec.clone()) {
      Some(sel) => hash_alg_vec[sel],
      None => {
        println!("No selection made, exiting...");
        return Vec::new();
      }
    };
    println!("{}", hash_alg);
    
    print::print("Charset: ");
    let charset_vec = vec!["numeric",
                           "alpha",
                           "alpha-numeric",
                           "loweralpha",
                           "loweralpha-numeric",
                           "mixalpha",
                           "mixalpha-numeric",
                           "ascii-32-95",
                           "ascii-32-65-123-4",
                           "alpha-numeric-symbol32-space"];
    let charset = match terminal::get_selection(charset_vec.clone()) {
      Some(sel) => charset_vec[sel],
      None => {
        println!("No selection made, exiting...");
        return Vec::new();
      }
    };
    println!("{}", charset);

    let pl_min = prompt_in_event!("rtgen>", "Plaintext length minimum: ");

    let pl_max = prompt_in_event!("rtgen>", "Plaintext length maximum: ");

    let table_index = prompt_in_event!("rtgen>", "Table index: ");

    let chain_len = prompt_in_event!("rtgen>", "Chain length: ");

    let chain_num = prompt_in_event!("rtgen>", "Number of rainbow chains to generate: ");

    let part_index = prompt_in_event!("rtgen>", "Part index: ");
    
    let workspace = prompt_in_event!("rtgen>", "Workspace directory: ");

    return vec![hash_alg.to_string(), charset.to_string(), pl_min, pl_max, table_index, chain_len, chain_num, part_index, workspace];
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() != 9 {
      return self.event.usage.to_string();
    }

    // Get their workspace
    let workspace = match args.pop() {
      Some(w) => w,
      None => {
        return self.event.usage.clone();
      }
    };

    let output = simple_match!(setup_rainbowcrack_workspace(Command::new("./rtgen").args(args), "rtgen", &workspace));

    // Print output
    log::log("rtgen", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
    
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["md5", "loweralpha-numeric", "1", "7", "0", "3800", "1000", "0", "tst/rtgen/atf/atf.txt"]];
    for test in tests {
      util::misc::cleanup("tst/rtgen/inp/md5_loweralpha-numeric#1-7_0_3800x1000_0.rt");

      // Run command
      simple_test_match!(setup_rainbowcrack_workspace(Command::new("./rtgen").args(&test[..8]).stdout(Stdio::null()), "rtgen", "tst/rtgen"));
      
      let out_str = simple_test_match!(fs::read("tst/rtgen/md5_loweralpha-numeric#1-7_0_3800x1000_0.rt"));

      // Read file
      let file_str = simple_test_match!(fs::read(test[8]));

      // Compare
      if out_str != file_str {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn rtgen(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(RTGen {
    event: Event {
      name:   name,
      desc:   "Generates a rainbow table(s), it will take a long time...".to_string(),
      usage:  "Requires a:\n\
                \tHash algorithm (string)\n\
                \tCharset (string)\n\
                \tPlaintext min length (decimal integer)\n\
                \tPlaintext max length (decimal integer)\n\
                \tTable index (decimal integer)\n\
                \tChain length (decimal integer)\n\
                \tChain number (decimal integer)\n\
                \tPart index (decimal integer)\n\
                \tWorkspace dir (string)\n".to_string(),
      author: "RainbowCrack Project".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** RTSORT ************************************/
pub struct RTSort { event: Event }
impl Eventable for RTSort {
  fn on_init(&self) -> Vec<String> {
    let workspace = prompt_in_event!("rtsort>", "Workspace directory: ");

    return vec![workspace];
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() != 1 {
      return self.event.usage.to_string();
    }

    // Get their workspace
    let workspace = match args.pop() {
      Some(w) => w,
      None => {
        return self.event.usage.clone();
      }
    };
    
    alerts::print_generic_warning("Do not stop this early or your table may become damaged.");
    let output = simple_match!(setup_rainbowcrack_workspace(Command::new("./rtsort").args(vec!["."]), "rtsort", &workspace));

    // Print output
    log::log("rtsort", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
    
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec![".", "tst/rtsort/atf/atf.txt"]];
    for test in tests {
      simple_test_match!(run_console_command(Command::new("cp").args(vec!["tst/rtgen/md5_loweralpha-numeric#1-7_0_3800x1000_0.rt", "tst/rtsort/inp/"])));

      // Run command
      let output = simple_test_match!(setup_rainbowcrack_workspace(Command::new("./rtsort").args(&test[..1]).stdout(Stdio::piped()), "rtsort", "tst/rtsort/inp"));
      let out_str = String::from_utf8_lossy(&output.stdout);

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[1]));

      let mut out_vec: Vec<&str> = out_str.split('\n').collect();
      out_vec.remove(1);
      let mut str_vec: Vec<&str> = file_str.split('\n').collect();
      str_vec.remove(1);

      // Compare
      if !out_vec.eq(&str_vec) {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn rtsort(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(RTSort {
    event: Event {
      name:   name,
      desc:   "Sorts all rainbow tables inside of a given directory".to_string(),
      usage:  "Requires a:\n\
                \tWorkspace dir (string)\n".to_string(),
      author: "RainbowCrack Project".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** RCRACK ************************************/
pub struct RCrack { event: Event }
impl Eventable for RCrack {
  fn on_init(&self) -> Vec<String> {
    let path_vec = vec!["Single Hash", "Hash File", "LM pwdump", "NTLM pwdump"];
    let path_arg_vec = vec!["-h", "-l", "-lm", "-ntlm"];
    print::print("rcrack> Path Type: ");
    let path_arg = match terminal::get_selection(path_vec) {
      Some(sel) => path_arg_vec[sel],
      None => {
        return Vec::new();
      }
    };
    println!("{}", path_arg);

    let param = prompt_in_event!("rcrack>", "Hash or File Path: ");

    let workspace = prompt_in_event!("rcrack>", "Workspace directory: ");

    return vec![path_arg.to_string(), param, workspace];
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() != 3 {
      return self.event.usage.to_string();
    }

    // Get their workspace
    let workspace = match args.pop() {
      Some(w) => w,
      None => {
        return self.event.usage.clone();
      }
    };
    args.insert(0, ".".to_string());
    
    let output = simple_match!(setup_rainbowcrack_workspace(Command::new("./rcrack").args(args), "rcrack", &workspace));

    // Print output
    log::log("rcrack", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
    
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec![".", "-h", "fcea920f7412b5da7be0cf42b8c93759", "tst/rcrack/atf/atf.txt"]];
    for test in tests {
      simple_test_match!(run_console_command(Command::new("cp").args(vec!["tst/rtsort/inp/md5_loweralpha-numeric#1-7_0_3800x1000_0.rt", "tst/rcrack/inp/"])));

      // Run command
      let output = simple_test_match!(setup_rainbowcrack_workspace(Command::new("./rcrack").args(&test[..3]).stdout(Stdio::piped()), "rcrack", "tst/rcrack/inp"));
      let out_str = String::from_utf8_lossy(&output.stdout);

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[3]));

      // Remove performance specific lines
      let mut out_vec: Vec<&str> = out_str.split('\n').collect();
      out_vec.drain(7..20);
      out_vec.remove(1);
      let mut str_vec: Vec<&str> = file_str.split('\n').collect();
      str_vec.drain(7..20);
      str_vec.remove(1);

      // Compare
      if !out_vec.eq(&str_vec) {
        debug::print_debug(format!("\n-----\n{:?}\n-----\n{:?}\n-----\n", out_vec, str_vec));
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn rcrack(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(RCrack {
    event: Event {
      name:   name,
      desc:   "Will crack a hash(s) from a given rainbow table(s)".to_string(),
      usage:  "Requires a:\n\
                \tPath type (option string)\n\
                \tFile path (string)\n\
                \tWorkspace dir (string)\n".to_string(),
      author: "RainbowCrack Project".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/************************************ CRC Reversing ***********************************/
pub struct CrcReversing { event: Event }
impl Eventable for CrcReversing {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("CRC Reversing");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn crcreversing(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(CrcReversing {
    event: Event {
      name:   name,
      desc:   "CRC reversing tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/************************************** CrcHack *****************************************/
pub struct CRCHack { event: Event }
impl Eventable for CRCHack {
  fn on_init(&self) -> Vec<String> {
    // Prompt for input
    let input_file = prompt_in_event!("crchack>", "Path to input file: ");

    // Prompt for bit length ('8', '16', or '32')
    let bit_length = prompt_in_event!("crchack>", "Bit length ('8' or '16' or '32'): ");
    let mut bitlenflag = String::new();
    bitlenflag.push_str(&bit_length);
  
    // Prompt for generator polynomial
    let poly = prompt_in_event!("crchack>", "Generator polynomial: ");
    let mut polyflag = String::new();
    polyflag.push_str(&poly);

    // Run command
    let output = match run_command(Command::new("ext/crchack/crchack").args(vec!["-w", &bit_length[..], "-p", &polyflag[..], &input_file[..]])) {
      Ok(out) => out,
      Err(err) => {
        println!("Error: {}", err);
        return Vec::new();
      }
    };
    // Print output
    print!("{}", String::from_utf8_lossy(&output.stdout));
    log::log("CRCHack", &String::from_utf8_lossy(&output.stderr));
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/crchack/atf/atf.txt";
    let input_path = "tst/crchack/inp/test.txt";

    let check = match Command::new("ext/crchack/crchack").arg(input_path).output() {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("tcpkill::on_test: Failed to run bounded command. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("tcpkill::on_test: Failed to open the atf file. {}", err));
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
pub fn crchack(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(CRCHack {
    event: Event {
      name:   name,
      desc:   "CRC reversing tool".to_string(),
      usage:  "arg1: polynomial, arg2: bit length, arg3: input file path\n".to_string(),
      author: "github.com/resilar".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** Reveng ************************************/
pub struct Reveng { event: Event }
  impl Eventable for Reveng {
    fn on_init(&self) -> Vec<String> {
    let cmd = prompt_in_event!("Reveng>", "");
    let cmd_args = cmd.split(" ").collect();
    if filter::if_command_contains_rule(&cmd_args, filter::load_rules_cfg("reveng")) {
      let output = match run_command(Command::new("ext/reveng/reveng").args(cmd_args)) {
        Ok(out) => out,
        Err(err) => {
          println!("Error: {}", err);
          return Vec::new();
        }
      };
      print!("{}", String::from_utf8_lossy(&output.stderr));
      print!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
      println!("Not supported.");
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/reveng/atf/atf.txt";

    let check = match Command::new("ext/reveng/reveng").args(vec!["-e","deadbeef123"]).output() {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("reveng::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("reveng::on_test: Failed to open the test file. {}", err));
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
pub fn reveng(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Reveng {
    event: Event {
      name:   name,
      desc:   "Arbitrary-precision CRC calculator and algorithm finder.".to_string(),
      usage:  filter::parse_help_menu(Command::new("ext/reveng/reveng").args(vec!["-h"]),
                                      filter::load_rules_cfg("reveng"), "\x0A\x09-", 0, 
                                      "Reveng\n\
                                      Copyright (C) 2021 Gregory Cook.").to_string(),
      author: "Gregory Cook".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
