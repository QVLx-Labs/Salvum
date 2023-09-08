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
use regex::Regex;

/*********************************** MITM ***********************************/
pub struct ManInTheMiddle { event: Event }
impl Eventable for ManInTheMiddle {
  fn on_init(&self) -> Vec<String> {
    print::println("MITM");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn maninthemiddle(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ManInTheMiddle {
    event: Event {
      name:   name,
      desc:   "Man in the Middle attack tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** KEY DATABASE ***********************************/
pub struct MITMKeyDatabase { event: Event }
impl Eventable for MITMKeyDatabase {
  fn on_init(&self) -> Vec<String> {
    print::println("MITM Key Databases");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn mitmkeydb(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(MITMKeyDatabase {
    event: Event {
      name:   name,
      desc:   "Key database attack tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** LITTLEBLACKBOX ***********************************/
pub struct LittleBlackBox { event: Event }
impl Eventable for LittleBlackBox {
  fn on_init(&self) -> Vec<String> {
    // Prompt for args
    let infile = prompt_in_event!("LittleBlackBox>", "Path to cert or key .pem: ");
    let args = vec![infile.trim().to_string()];
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 {
      return self.event.usage.clone();
    }

    let dt = chrono::offset::Local::now(); //create the DateTime struct
    let datetime: String = str::replace(&dt.to_string()," ","_"); //convert to a string and replace any ' ' with '_'
    let datetime: String = str::replace(&datetime,":","-");
    let mut outpath = String::from("out/lbb/out_lbb_");
    outpath.push_str(&datetime[..16]); 
    outpath.push_str(".txt");
    let mut note = String::from("Report will be written to ");
    note.push_str(&outpath);
    note.push_str("\n");
    print::print_custom(&note[..],"orange");
    let arguments = vec!["ext/lbb/lbb.py",&args[0],&outpath[..]];
    let output = simple_match!(run_console_command(Command::new("python3").args(arguments)));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/lbb/atf/atf.txt";
    let input_path = "tst/lbb/inp/testkey.pem";
    let lbb_args = vec!["ext/lbb/lbb.py", input_path, "/dev/null"];

    let check = simple_test_match!(Command::new("python3").args(lbb_args).output());

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
pub fn littleblackbox(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(LittleBlackBox {
    event: Event {
      name:   name,
      desc:   "LittleBlackBox".to_string(),
      usage:  "Input a .pem file and we'll search the database\n".to_string(),
      author: "github.com/devttys0".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** PROXIES ***********************************/
pub struct MITMProxies { event: Event }
impl Eventable for MITMProxies {
  fn on_init(&self) -> Vec<String> {
    print::println("MITM Proxies");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn mitmproxies(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(MITMProxies {
    event: Event {
      name:   name,
      desc:   "Proxy attack tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** SSLSPLIT ***********************************/
pub struct SSLSplit { event: Event }
impl Eventable for SSLSplit {
  fn on_init(&self) -> Vec<String> {
    // Prompt for key
    let key = prompt_in_event!("SSLSplit>", "Private Key: ");
    
    // Prompt for pem
    let pem = prompt_in_event!("SSLSplit>", "PEM: ");
    
    // Prompt for address
    let address = prompt_in_event!("SSLSplit>", "IP Address: ");
    
    // Prompt for port
    let port = prompt_in_event!("SSLSplit>", "Port: ");
    
    return vec![key, pem, address, port];
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() != 4 {
      return self.event.usage.to_string();
    }
    args = vec!["-k".to_string(), args[0].clone(), "-c".to_string(), args[1].clone(), "-P".to_string(), "https".to_string(), args[2].clone(), args[3].clone()];

    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/sslsplit/sslsplit").args(args)));
    
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["tst/sslsplit/atf/atf.txt"]];
    simple_test_match!(Command::new("cp").args(vec!["ext/sslsplit/sslsplit", "tst/sslsplit/inp/"]).output());

    for test in tests {
      // Run command
      let output = match Command::new("./sslsplit.test").current_dir("tst/sslsplit/inp").output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("sslsplit::on_test: Failed to execute. {}", err));
          util::misc::cleanup("tst/sslsplit/inp/sslsplit");
          return TestStatus::Failed;
        }
      };
      let out_str = String::from_utf8_lossy(&output.stdout);
      let output_lines = out_str.split("\n");

      // Read file
      let atf_str = match fs::read_to_string(test[0]) {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("sslsplit::on_test: Failed to open the file: {}. {}", test[2], err));
          util::misc::cleanup("tst/sslsplit/inp/sslsplit");
          return TestStatus::Failed;
        }
      };

      let atf_lines: Vec<&str> = atf_str.split("\n").collect();
      let filter_regex = match Regex::new(r"(opts.t.c|([0-9]){1,3}%: Checks:)") {
        Ok(regex) => regex,
        Err(err) => {
          debug::print_debug(&format!("fsstat::on_test: failed to create filter regex. {}",err));
          return TestStatus::Failed;
        }
      };

      for (index, line) in output_lines.enumerate() {
        if index > (atf_lines.len() - 1) {
          break;
        }
        // if filter_regex.is_match(line) {
          // println!("{}\nline matches regex",line);
        // }
        if (!filter_regex.is_match(line)) && (line != atf_lines[index]) {
          debug::print_debug(format!("sslsplit::on_test: output doesnt match atf\noutput:{}\natf:{}",line,atf_lines[index]));
          util::misc::cleanup("tst/sslsplit/inp/sslsplit");
          return TestStatus::Failed;
        }
      }

      // Compare
      //if out_str != file_str {
      //  debug::print_debug(format!("\n-----\n{}\n-----\n{}\n-----\n", out_str, file_str));
      //  util::misc::cleanup("tst/sslsplit/inp/sslsplit");
      //  return TestStatus::Failed;
      //}

      
      ////compare atf to output
      //let output_lines = fiwalk_output_str.split("\n");
      //let atf_lines = atf_str.split("\n");
      //let atf_lines: Vec<&str> = atf_lines.collect();
    }

    util::misc::cleanup("tst/sslsplit/inp/sslsplit");
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn sslsplit(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SSLSplit {
    event: Event {
      name:   name,
      desc:   "Performs SSL/TLS interception.".to_string(),
      usage:  filter::parse_help_menu(Command::new("ext/sslsplit/sslsplit").args(vec!["-h"]),
                                      filter::load_rules_cfg("sslsplit"), "\n", 0,
                                      "Usage: sslsplit [-D] [-f conffile] [-o opt=val] [options...] [proxyspecs...]\n\nExamples:\n\tsslsplit -k ca.key -c ca.pem -P  https 127.0.0.1 8443\n\tsslsplit -k ca.key -c ca.pem -P  https ::1 8443\n\nOptions:\n").to_string(),
      author: "Daniel Roethlisberger (github.com/droe)".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** bettercap ***********************************/
pub struct Bettercap { event: Event }
impl Eventable for Bettercap {
  fn on_init(&self) -> Vec<String> {
    match run_console_command(&mut Command::new("ext/bettercap/bettercap")) {
      Ok(_) => {}
      Err(err) => {
        println!("Error: {}", err);
      }
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/bettercap/atf/atf.txt";

    let check = match Command::new("ext/bettercap/bettercap").arg("-h").output() {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("bettercap::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("bettercap::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stderr) != file_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn bettercap(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Bettercap {
    event: Event {
      name:   name,
      desc:   "Ettercap, but better; Performs various attacks on different interfaces.".to_string(),
      usage:  "Simply invoke 'bettercap' to start using.\n".to_string(),
      author: "bettercap.org".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** dhcplayer ***********************************/
pub struct DHCPlayer { event: Event }
impl Eventable for DHCPlayer {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/dhcplayer/DHCPlayer").args(&args)));

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let atf_path = "tst/dhcplayer/atf/atf.txt";

    // Run command
    let output = simple_test_match!(Command::new("ext/dhcplayer/DHCPlayer").arg("--help").output());

    // Read file
    let file_str = simple_test_match!(fs::read_to_string(atf_path));

    // Compare
    if String::from_utf8_lossy(&output.stdout).to_string() != file_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn dhcplayer(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(DHCPlayer {
    event: Event {
      name:   name,
      desc:   "Performs DHCP-based queries, attacks, and interceptions.".to_string(),
      usage:  "Invoke dhcplayer with one of these subcommands:\n\
                \tdiscover\n\
                \trelease\n\
                \tserver\n\
                \tstarv\n".to_string(),
      parent: parent,
      author: "$t@$h & Eloy (github.com/zer1t0)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** ARPlayer ***********************************/
pub struct ARPlayer { event: Event }
impl Eventable for ARPlayer {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/arplayer/ARPlayer").args(&args)));

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let atf_path = "tst/arplayer/atf/atf.txt";

    // Run command
    let output = simple_test_match!(Command::new("ext/arplayer/ARPlayer").arg("--help").output());

    // Read file
    let file_str = simple_test_match!(fs::read_to_string(atf_path));

    // Compare
    if String::from_utf8_lossy(&output.stdout).to_string() != file_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn arplayer(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ARPlayer {
    event: Event {
      name:   name,
      desc:   "Performs DHCP-based queries, attacks, and interceptions.".to_string(),
      usage:  "Invoke arplayer with one of these subcommands:\n\
                \tforward\n\
                \treply\n\
                \tscan\n\
                \tspoof\n".to_string(),
      parent: parent,
      author: "$t@$h & Eloy (github.com/zer1t0)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
