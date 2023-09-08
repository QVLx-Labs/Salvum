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
 * systemscanning.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;


/*********************************** SYSTEM SCANNING ***********************************/
pub struct Scanners { event: Event }
impl Eventable for Scanners {
  fn on_init(&self) -> Vec<String> {
    print::println("System scanning");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn scanners(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Scanners {
    event: Event {
      name:   name,
      desc:   "Scanning tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** AUDITORS ***********************************/
pub struct Auditors { event: Event }
impl Eventable for Auditors {
  fn on_init(&self) -> Vec<String> {
    print::println("Auditors");
    return Vec::new();
  }
  fn get_event(&self) -> &Event {
    return &self.event;
  }
}
pub fn auditors(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Auditors {
    event: Event {
      name:   name,
      desc:   "Auditing tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** LYNIS ***********************************/
pub struct Lynis { event: Event }
impl Eventable for Lynis {
  fn on_init(&self) -> Vec<String> {
    //sudo chown -R root:root ext/lynis
    match Command::new("chown").args(vec!["-R","root:root","ext/lynis/lynis"]).output() {
      Ok(_) => {},
      Err(err) => {
        debug::print_debug(format!("lynis::on_init: lynis file ownership/permissions is invalid {}",err));
        return Vec::new();
      }
    };

    //prompt the user for a report path
    let path_report = "out/lynis/lynis_report";
    //get the absolute path of the current dir
    let current_dir = match env::current_dir() {
      Ok(dir) => dir,
      Err(err) => {
        println!("Something bad happened {}", err);
        return Vec::new();
      }
    };
    
    //append the cur dir path with the lynis path
    let mut lynis_path_str: String = match current_dir.to_str() {
      Some(s) => s.to_string(),
      None => return Vec::new()
    };
    lynis_path_str.push_str("/ext/lynis/lynis");
    let _lynis_path = Path::new(&lynis_path_str);
    let lynis_dir = Path::new("ext/lynis");

    println!("Starting system audit this may take a moment..");
    let output = match run_console_command(Command::new("./lynis")
    .args(vec!["audit","system","--report-file",&path_report[..]])
    .current_dir(&lynis_dir)) {
      Ok(out) => out,
      Err(err) => {
        println!("failed to execute lynis : {}",err);
        return Vec::new();
      }
    };
    println!("{}",String::from_utf8_lossy(&output.stdout));
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let command = "./lynis";
    let working_dir = "ext/lynis";
    let mut tests = vec![
      vec!["audit","dockerfile","../../tst/lynis/inp/dockerfile_gcc","--no-colors","--no-log","tst/lynis/atf/lynis_audit_system_-Q_-q"]/*,
      vec!["audit","dockerfile","../../tst/lynis/inp/dockerfile_lynx","--no-colors","--no-log","tst/lynis/atf/lynis_audit_system_-Q_-q"]*/
    ];
    for test in &mut tests {
      let artifact_path_str = match test.pop() {
        Some(string) => string,
        None => {
          debug::print_debug("lynis::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };
  
      //sudo chown -R root:root ext/lynis
      match Command::new("chown").args(vec!["-R","root:root","ext/lynis/lynis"]).output() {
        Ok(_) => {},
        Err(err) => {
          debug::print_debug(format!("lynis::on_test: lynis file ownership/permissions is invalid {}",err));
          return TestStatus::Failed;
        }
      };
  
      let output = match Command::new(command).current_dir(working_dir).args(test).output() {
        Ok(output) => output,
        Err(err) => {
          debug::print_debug(format!("lynis::on_test: failed to execute lynis. {}",err));
          return TestStatus::Failed;
        }
      };
  
      let exit_status = match output.status.code() {
        Some(status) => status,
        None => {
          debug::print_debug("lynis::on_test: failed to get lynis exit code.");
          return TestStatus::Failed;
        }
      };
  
      let artifact_path = Path::new(artifact_path_str);
      let artifact_str = match fs::read_to_string(artifact_path) {
        Ok(string) => string,
        Err(err) => {
          debug::print_debug(format!("lynis::on_test: failed to read lynis artifact file. {}", err));
          return TestStatus::Failed;
        }
      };
      //println!("artifact_str {}",artifact_str);
      let artifact_str: String = artifact_str.split_whitespace().collect();
      let artifact_exit_status = match artifact_str.parse::<i32>() {
        Ok(num) => num,
        Err(err) => {
          debug::print_debug(format!("lynis::on_test: failed to parse lynis artifact file to i32. {}", err));
          return TestStatus::Failed;
        }
      };
  
      if exit_status != artifact_exit_status {
        debug::print_debug("lynis::on_test: lynis returned with a non-zero exit code");
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event {return &self.event;}
}
pub fn lynis(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Lynis {
    event: Event {
      name:   name,
      desc:   "Lynis - a battle-tested security tool for systems running Linux, macOS, or Unix-based operating system. It performs an extensive health scan of your systems to support system hardening and compliance testing.".to_string(),
      usage:  "Prompts you for: \
              \n\tPath in which report is written to\n".to_string(),
      parent: parent,
      author: "cisofy.com".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** LINUX MALWARE DETECT ***********************************/
pub struct LMD { event: Event }
impl Eventable for LMD {
  fn on_init(&self) -> Vec<String> {
    //prompt the user for a path in which they want to scan
    let path_input: String = prompt_in_event!("LMD>", "Path to scan: ");

    let output = match run_command(Command::new("ext/maldetect/maldet")
      .args(vec!["-a",&path_input[..]])) {
        Ok(out) => out,
        Err(err) => {
          println!("\nError: {}",err);
          return Vec::new();
        }
    };
    println!("{}",String::from_utf8_lossy(&output.stdout));
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus { 
    return TestStatus::Failed;
  }
  fn get_event(&self) -> &Event {return &self.event;}
}
pub fn lmd(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(LMD {
    event: Event {
      name:   name,
      desc:   "LMD".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "Ryan MacDonald (github.com/rfxn)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** CLAM AV ***********************************/
pub struct ClamAV { event: Event }
impl Eventable for ClamAV {
  fn on_init(&self) -> Vec<String> {
    //println!("clam av status: {:?}", get_clamav_status());
    //println!("stopping clamav...");
    //match stop_clamav() {
    //  Ok(()) => {},
    //  Err(err) => {
    //    println!("{}",err);
    //    return;
    //  }
    //};
    //println!("clam av status: {:?}", get_clamav_status());

    //prompt the user for a path in which they want to scan
    let status = match Command::new("freshclam").stdout(Stdio::null()).stderr(Stdio::null()).status() {
      Ok(status) => status,
      Err(err) =>{
        print::println(&format!("Failed to execute freshclam. Err: {}", err));
        return Vec::new();
      }
    };
    if !status.success() {
      print::println("Freshclam returned with a non zero exit code");
      return Vec::new();
    }


    let path_input: String = prompt_in_event!("Clam AV>", "Path to scan: ");

    //prompt the user for a path in which the log will be written
    //let path_log: String = prompt_in_event!("Clam AV>", "Path to write the log: ");
    let path_log = "out/clamav/clamav_log";

    let mut arg_log: String = String::from("--log=");
    arg_log.push_str(&path_log[..]);

    //create a new child process to run the clamscan
    let output = match run_command(Command::new("ext/clamav/clamscan")
      .args(vec![arg_log,path_input])) {
        Ok(out) => out,
        Err(err) => {
          println!("\nError: {}",err);
          return Vec::new();
        }
    };
    //print the scan's output
    println!("{}",String::from_utf8_lossy(&output.stdout));

    //ensure that the log file was generated
    if Path::new(&path_log).exists() {
      println!("Scan successful log file found: {}",path_log);
    }
    else {
      println!("Unable to find the log file: {}",path_log);
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/clamav/atf/atf.txt";
    let input_path = "tst/clamav/inp/test.elf";
   
    let scan_args = vec!["--scan-elf=yes", "--verbose", input_path];

    // Run check
    let check = match Command::new("ext/clamav/clamscan").args(scan_args).output() {
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
        debug::print_debug(format!("clamav::on_test: Failed to open artifact file. {}", err));
        return TestStatus::Failed;
      }
    };
   
    // Compare
    let out_str_utf8 = String::from_utf8_lossy(&check.stdout);
    let output_tkns_nl = out_str_utf8.split("\n").collect::<Vec<&str>>();
    let file_tkns_nl = file_str.split("\n").collect::<Vec<&str>>();
    
    if output_tkns_nl.len() < 10 { return TestStatus::Failed; }
    if output_tkns_nl[3] != file_tkns_nl[0] { return TestStatus::Failed; }
    if output_tkns_nl[6] != file_tkns_nl[3] { return TestStatus::Failed; }
    if output_tkns_nl[7] != file_tkns_nl[4] { return TestStatus::Failed; }
    if output_tkns_nl[8] != file_tkns_nl[5] { return TestStatus::Failed; }
    if output_tkns_nl[9] != file_tkns_nl[6] { return TestStatus::Failed; }

    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event {return &self.event;}
}
pub fn clamav(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ClamAV {
    event: Event {
      name:   name,
      desc:   "Versatile anti-virus toolkit.".to_string(),
      usage:  "Prompts you for: \
              \n\tPath to scan \n".to_string(),
      parent: parent,
      author: "clamav.net".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
pub fn get_clamav_status() -> bool {
  let pid_output = match Command::new("pidof").arg("freshclam").output() {
    Ok(output) => output,
    Err(_) => {
      return false;
    }
  };
  let exit_success = pid_output.status.success();
  if exit_success {
    match stop_clamav() {
      Ok(_) => {},
      Err(_) => {}
    };
    return true;
  }
  else {
    return false;
  }
}
//pub fn stop_clamav(verbose: bool) -> Result<(), String> {
//  //get proccess id of freshclam
//  //let cur_mode = terminal::get_color_mode();
//  //terminal::set_color_mode(terminal::Mode::Magenta);
//  ////is the service running already or not?
//  //if servf_tftp_status() {
//  //  print::println("An TFTP service was found still running. Sudo permissions will be needed to stop the service.");
//  //  if !servf_stop_prompt("TFTP") {
//  //    terminal::set_color_mode(cur_mode);
//  //    return;
//  //  }
//  
//  if verbose {
//    let cur_color_mode = terminal::get_color_mode();
//    terminal::set_color_mode(terminal::Mode::Magenta);
//    
//    //let option: String = prompt_in_event!("Clam Av>","Would you like to kill the background process? (yes/no) : ");
//    //let event_name = "Clam Av>";
//    //let option: = match terminal::get_input(format!("{} {} {}", constants::CONSOLE_PROMPT, $event_name, $question)) {
//    //  Ok(out) => out,
//    //  Err(_) => { return; }
//    //};
//    //let option = option.trim().to_lowercase();
//    print::println("Clam Av was found running in the background.");
//    let msg: &str = "Clam Av> Would you like to kill the background process?";
//
//    if alerts::confirm_task(msg) == constants::CONFIRMED {
//      match _stop_clamav() {
//        Ok(()) => {},
//        Err(err) => return Err(err)
//      };
//    }
//    terminal::set_color_mode(cur_color_mode);
//  }
//  else {
//    match _stop_clamav() {
//      Ok(()) => {},
//      Err(err) => return Err(err)
//    };
//  }
//  return Ok(());
//}
pub fn stop_clamav() -> Result<(), String> {
  let pid_output = match Command::new("pidof").arg("freshclam").output() {
    Ok(output) => output,
    Err(err) => {
      return Err(err.to_string());
    }
  };
  let pid_str = String::from_utf8_lossy(&pid_output.stdout);
  let pid_str = pid_str.trim();
  
  let pid_i32: i32 = match pid_str.parse() {
    Ok(pid) => pid,
    Err(_) => { return Err("Unable to parse process id string into an integer.".to_string()) }
  };
  //kill proccess id
  if pid_i32 > 0 {
    let clam_pid = Pid::from_raw(pid_i32);
    let sig_kill = Signal::SIGKILL;
    match kill(clam_pid, sig_kill) {
      Ok(()) => {},
      Err(err) => {
        return Err(err.to_string());
      }
    };
  }
  return Ok(());
}
//let cur_mode = terminal::get_color_mode();
//  terminal::set_color_mode(terminal::Mode::Magenta);
//  //is the service running already or not?
//  if servf_tftp_status() {
//    print::println("An TFTP service was found still running. Sudo permissions will be needed to stop the service.");
//    if !servf_stop_prompt("TFTP") {
//      terminal::set_color_mode(cur_mode);

/*********************************** ROOTKIT DETECTION ***********************************/
pub struct RootkitDetection { event: Event }
impl Eventable for RootkitDetection {
  fn on_init(&self) -> Vec<String> {
    print::println("Rootkit detection");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn rootkitdetection(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(RootkitDetection {
    event: Event {
      name:   name,
      desc:   "Rootkit - is a tool to locally check for signs of a rootkit.  It checks if the network interface is in promiscuous, checks for lastlog deletions, checks for wtmp deletions, checks for wtmpx deletions, checks for signs of LKM trojans, checks for signs of LKM trojans, checks for quick and dirty strings replacement, and checks for utmp deletions.
     ".to_string(),
      usage:  "No args required. Simple execution event.".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** CHKROOTKIT ***********************************/
pub struct Chkrootkit { event: Event }
impl Eventable for Chkrootkit {
  fn on_init(&self) -> Vec<String> {
    //get the absolute path of the current dir
    let current_dir = match env::current_dir() {
      Ok(dir) => dir,
      Err(err) => {
        println!("{}", err);
        return Vec::new();
      }
    };
    
    //append the cur dir path with the chkrootkit path
    let mut chkrootkit_path_str: String = match current_dir.to_str() {
      Some(s) => s.to_string(),
      None => return Vec::new()
    };
    chkrootkit_path_str.push_str("/ext/chkrootkit");
    let chkrootkit_path = Path::new(&chkrootkit_path_str);

    //create a new child process to run the root kit check
    let _output = match run_console_command(Command::new("sh")
    .args(vec!["-c","./chkrootkit"])
    .current_dir(&chkrootkit_path)) {
      Ok(out) => out,
      Err(err) => {
        println!("\nError: {}",err);
        return Vec::new();
      }
    };
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let command = "./ext/chkrootkit/chkrootkit";
    let mut test = vec!["-q","tst/chkrootkit/atf/chkrootkit_-q"];
    let artifact_path_str = match test.pop() {
      Some(string) => string,
      None => {
        debug::print_debug("chkrootkit::on_test: invalid test vector");
        return TestStatus::Failed;
      }
    };

    let output = match Command::new(command).args(test).output() {
      Ok(output) => output,
      Err(err) => {
        debug::print_debug(format!("chkrootkit::on_test: failed to execute chkrootkit. {}",err));
        return TestStatus::Failed;
      }
    };

    let exit_status = match output.status.code() {
      Some(status) => status,
      None => {
        debug::print_debug("chkrootkit::on_test: failed to get chkrootkit exit code.");
        return TestStatus::Failed;
      }
    };

    let artifact_path = Path::new(artifact_path_str);
    let artifact_str = match fs::read_to_string(artifact_path) {
      Ok(string) => string,
      Err(err) => {
        debug::print_debug(format!("chkrootkit::on_test: failed to read chkrootkit artifact file. {}", err));
        return TestStatus::Failed;
      }
    };
    //println!("artifact_str {}",artifact_str);
    let artifact_str: String = artifact_str.split_whitespace().collect();
    let artifact_exit_status = match artifact_str.parse::<i32>() {
      Ok(num) => num,
      Err(err) => {
        debug::print_debug(format!("chkrootkit::on_test: failed to parse chkrootkit artifact file to i32. {}", err));
        return TestStatus::Failed;
      }
    };

    if exit_status != artifact_exit_status {
      debug::print_debug("chkrootkit::on_test: chkrootkit returned with a non-zero exit code");
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn chkrootkit(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Chkrootkit {
    event: Event {
      name:   name,
      desc:   "Chkrootkit tools".to_string(),
      usage:  "No user input required. Tool automatically begins root kit scan.`".to_string(),
      parent: parent,
      author: "chkrootkit.org".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** ArmorLib ***********************************/
pub struct ArmorLib { event: Event }
impl Eventable for ArmorLib {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); }
    let output = simple_match!(run_console_command(Command::new("ext/armorlib/armorlib")
      .args(vec!["scan", &args[0].trim()])));

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/armorlib/atf/atf.txt";
    let input_path = "tst/armorlib/inp/test.bin";
    
    // Run check
    let check = match Command::new("ext/armorlib/armorlib").args(vec!["scan", input_path]).output() {
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
        debug::print_debug(format!("armorlib::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };
   
    // Compare
    let out_str_utf8 = String::from_utf8_lossy(&check.stdout);
    let output_tkns_nl = out_str_utf8.split("\n").collect::<Vec<&str>>();
    let file_tkns_nl = file_str.split("\n").collect::<Vec<&str>>();

    if output_tkns_nl.len() != file_tkns_nl.len() { return TestStatus::Failed; }

    for i in 0..output_tkns_nl.len() {
      let output_tkns_info = output_tkns_nl[i].split("[INFO]").collect::<Vec<&str>>();
      let file_tkns_info = file_tkns_nl[i].split("[INFO]").collect::<Vec<&str>>();
      if output_tkns_info.len() > 1 && output_tkns_info.len() > 1 {
        if output_tkns_info[1] != file_tkns_info[1] { return TestStatus::Failed; }
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event {return &self.event;}
}
pub fn armorlib(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ArmorLib {
    event: Event {
      name:   name,
      desc:   "Scans a binary file for security and privacy threats".to_string(),
      usage:  "armorlib <file_path>\n".to_string(),
      parent: parent,
      author: "Matzr3lla, m0nZSt3r, & R. Miles McCain (github@sendmiles.email)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** faf ***********************************/
pub struct Faf { event: Event }
impl Eventable for Faf {
  fn on_init(&self) -> Vec<String> {
    //prompt the user for a path in which they want to scan
    let path_input: String = prompt_in_event!("faf>", "Enter path to a directory to scan: ");

    let output = match run_console_command(Command::new("ext/faf/slm_faf.sh")
      .arg(&path_input.trim())) {
        Ok(out) => out,
        Err(err) => {
          println!("\nError: {}",err);
          return Vec::new();
        }
    };
    print!("{}",String::from_utf8_lossy(&output.stdout));
    let outfile_name = String::from("out/faf/faf_out.txt");
    let mut out_file = match File::create(&outfile_name) {
      Ok(o) => o,
     Err(e) => { println!("Error writing file: {}", e); return Vec::new(); }
    };
    match out_file.write_all(String::from_utf8_lossy(&output.stdout).as_bytes()) {
      Ok(_) => (),
     Err(e) => println!("Error with writing file: {}", e)
    };
    let mut closing = String::from("Results written to $(SAVLUM_ROOT)/");
    closing.push_str(&outfile_name);
    closing.push_str("\n");
    print::print_custom(&closing, "orange");
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/faf/atf/atf.txt";
    let input_path = "tst/faf/inp";
    
    // Run check
    let check = match Command::new("ext/faf/slm_faf.sh").arg(input_path).output() {
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
        debug::print_debug(format!("faf::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };
   
    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event {return &self.event;}
}
pub fn faf(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Faf {
    event: Event {
      name:   name,
      desc:   "File Anomaly Finder".to_string(),
      usage:  "faf <directory_path>".to_string(),
      parent: parent,
      author: "$t@$h".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** lemmeknow ***********************************/
pub struct Lemmeknow { event: Event }
impl Eventable for Lemmeknow {
  fn on_init(&self) -> Vec<String> {
    //prompt the user for a path in which they want to scan
    let path_input: String = prompt_in_event!("lemmeknow>", "Enter path to a file to scan: ");
    return vec![path_input.clone()];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { 
      return self.event.usage.clone(); 
    }

    let output = simple_match!(run_console_command(Command::new("ext/lemmeknow/lemmeknow").arg(&args[0].trim())));

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/lemmeknow/atf/atf.txt";
    let input_path = "tst/lemmeknow/inp/test.txt";
    
    // Run check
    let check = match Command::new("ext/lemmeknow/lemmeknow").arg(input_path).output() {
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
        debug::print_debug(format!("lemmeknow::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };
   
    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event {return &self.event;}
}
pub fn lemmeknow(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Lemmeknow {
    event: Event {
      name:   name,
      desc:   "File analyzer".to_string(),
      usage:  "lemmeknow <path_to_file>\n".to_string(),
      parent: parent,
      author: "Swanand Mulay (github.com/swanandx)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** detcve ***********************************/
pub struct Detcve { event: Event }
impl Eventable for Detcve {
  fn on_init(&self) -> Vec<String> {
    //prompt the user for a path in which they want to scan
    let path_input: String = prompt_in_event!("detcve>", "Enter path to file or directory to scan : ");
    return vec![path_input.clone()];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if main_info::is_bg() {
      print!("This app doesn't have background run support yet.");
      return String::from("Detcve app doesn't have background run support yet.\n");
    }
    use walkdir::WalkDir;
    if args.len() < 1 || args.len() > 1 { 
      return self.event.usage.clone();
    }

    let dir = args[0].trim();
    let path = Path::new(&dir);
    let mut once: bool = false;
    let md = match std::fs::metadata(path.clone()) {
      Ok(o) => o,
      Err(_) => { return String::from("Failed to grab the metadata of your path"); }
    };
    
    if md.is_dir() {
      for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if md.is_dir() && !once { once = true; continue; }
        let name = match file.path().to_str() {
          Some(n) => n,
          None => { return String::from("Failure"); }
        };
        print::println(&format!("File: {}", name));
        
        match Command::new("python3").args(vec!["-m","cve_bin_tool.cli",&name.trim()]).status() {
          Ok(_) => {}
          Err(e) => { return format!("{}", e); }
        }
      } 
    }
    else {
      print::println(&format!("File: {}", dir));
      
      match Command::new("python3").args(vec!["-m","cve_bin_tool.cli",&dir]).status() {
        Ok(_) => {}
        Err(e) => { return format!("{}", e); }
      }
    }
    
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/detcve/atf/atf.txt";
    let input_path = "tst/detcve/inp/test.txt";

		match Command::new("python3").args(vec!["setup.py","install"]).current_dir("ext/detcve") {
			    _ => {},
		}
		match Command::new("cp").args(vec!["ext/detcve/cve.db","/root/.cache/cve-bin-tool/"]) {
			    _ => {},
		}
    
    // Run check
    let check = match Command::new("python3").args(vec!["-m","cve_bin_tool.cli",input_path]).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("detcve::on_test: Failed to run test command. {}", err));
          return TestStatus::Failed;
        }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("detcve::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };
   
    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event {return &self.event;}
}
pub fn detcve(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Detcve {
    event: Event {
      name:   name,
      desc:   "Detect CVE-known vulnerabilities in file or directory.".to_string(),
      usage:  "detcve <file_or_directory_path>\ndetcve update\n\nupdate subcommand grabs latest CVE entries off the net.\n".to_string(),
      parent: parent,
      author: "Intel Corporation".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** yara ***********************************/
pub struct Yara { event: Event }
impl Eventable for Yara {
  fn on_init(&self) -> Vec<String> {
    //prompt the user for a path in which they want to scan
    let path_input: String = prompt_in_event!("yara>", "Enter path to file or directory to scan : ");
    return vec![path_input.clone()];
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    // Convert relative path to absolute
    match std::env::current_dir() {
      Ok(mut d) => {
        d.push(args[0].clone());
        args[0] = match d.into_os_string().into_string() {
          Ok(s) => s,
          Err(_) => args[0].clone(),
        };
      }
      Err(_) => {}
    };

    let output = simple_match!(run_console_command(Command::new("./slm_yara").args(vec!["rules/",&args[0].trim()]).current_dir("ext/yara/yara_orig")));
    
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/yara/atf/atf.txt";
    
    // Run check
    let check = match Command::new("./slm_yara").args(vec!["rules/","../../../tst/yara/inp/test.txt"]).current_dir("ext/yara/yara_orig").output() {
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
        debug::print_debug(format!("yara::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };
   
    // Compare
    if String::from_utf8_lossy(&check.stdout).split("\n").collect::<Vec<&str>>().len() !=
       file_str.split("\n").collect::<Vec<&str>>().len() {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event {return &self.event;}
}
pub fn yara(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Yara {
    event: Event {
      name:   name,
      desc:   "Scan a directory or file against curated Yara rule sets.".to_string(),
      usage:  "yara <path_to_file_or_dir>\n".to_string(),
      parent: parent,
      author: "Virus Total\tcontact@virustotal.com\n\nMatzr3lla and M0nZSt3r".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
