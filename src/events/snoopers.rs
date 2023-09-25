/*
 * QVLx Salvum 
 *
 * snoopingtools.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;
  
/*********************************** SNOOPING ***********************************/
pub struct Snooping { event: Event }
impl Eventable for Snooping {
  fn on_init(&self) -> Vec<String> {
    print::println("Snooping");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn snoopers(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Snooping {
    event: Event {
      name:   name,
      desc:   "Snooping tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** JTAG ***********************************/
pub struct JTAG { event: Event }
impl Eventable for JTAG {
  fn on_init(&self) -> Vec<String> {
    print::println("JTAG");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn jtag(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(JTAG {
    event: Event {
      name:   name,
      desc:   "JTAG tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** JLINKEXE ***********************************/
pub struct JLinkExe { event: Event }
impl Eventable for JLinkExe {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn on_run(&self, _args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_console_command(&mut Command::new("JLinkExe")));
    
    log::log("JLinkExe", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["tst/jlinkexe/atf/atf.txt"]];
    for test in tests {
      // Run command
      let output = simple_test_match!(run_bounded_command(&mut Command::new("JLinkExe"), false, 1));
      let out_str = String::from_utf8_lossy(&output.stdout);

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[0]));

      let mut out_vec: Vec<&str> = out_str.split('\n').collect();
      out_vec.resize(3, "");
      let str_vec: Vec<&str> = file_str.split('\n').collect();

      // Compare
      if !out_vec.eq(&str_vec) {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn jlinkexe(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(JLinkExe {
    event: Event {
      name:   name,
      desc:   "Allows you to access the JTAG port on a board".to_string(),
      usage:  "Simply invoke jlinkexe to open the console".to_string(),
      parent: parent,
      author: "SEGGER".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** OPENOCD ***********************************/
pub struct OpenOCD { event: Event }
impl Eventable for OpenOCD {
  fn on_init(&self) -> Vec<String> {
    // Prompt for config
    let config = prompt_in_event!("OpenOCD>", "Config file: ");

    return vec![config];
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() < 1 { return self.event.usage.clone(); }
    args.insert(0, "-f".to_string());

    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/openocd/riscv/vega/openocd").args(args)));
    
    log::log("OpenOCD", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["-f", "tst/openocd/inp/openocd.cfg", "tst/openocd/atf/atf.txt"]];
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("ext/openocd/riscv/vega/openocd").args(&test[..2]).output());
      let out_str = String::from_utf8_lossy(&output.stderr);

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[2]));

      // Compare
      if out_str != file_str {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn openocd(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(OpenOCD {
    event: Event {
      name:   name,
      desc:   "Allows you to access the JTAG port on a RISC-V board".to_string(),
      usage:  "Prompts you for:\n\
                \tConfig File (string)\n".to_string(),
      author: "SiFive".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** UART ***********************************/
pub struct UART { event: Event }
impl Eventable for UART {
  fn on_init(&self) -> Vec<String> {
    print::println("UART");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn uart(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(UART {
    event: Event {
      name:   name,
      desc:   "UART tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** SBRUTE.PY ***********************************/
pub struct SBrute { event: Event }
impl Eventable for SBrute {
  fn on_init(&self) -> Vec<String> {
    // Prompt for serial device
    let sd = prompt_in_event!("SBrute>", "Serial device to connect to: ");
    
    // Prompt for baud rate
    let baud = prompt_in_event!("SBrute>", "Baud rate: ");
    
    // Prompt for username
    let username = prompt_in_event!("SBrute>", "Username to force: ");

    return vec![sd, baud, username];
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() < 3 { return self.event.usage.clone(); }
    args = vec!["sbrute.py".to_string(), "-d".to_string(), args[0].clone(), "-b".to_string(), args[1].clone(), "-u".to_string(), args[2].clone()];

    // Run command
    let output = simple_match!(run_command(Command::new("python3")
                                .args(args)
                                .current_dir("ext/sbrute")));

    // Print output
    log::log("SBrute", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let test = vec!["sbrute.py","-d","/dev/null","-b","115200","-u","admin"];
    let atf_path = "tst/sbrute/atf/sbrute_-d_null_-b_115200_-u_admin";
    let sbrute_path = Path::new("ext/sbrute");

    //execute command
    let mut output = simple_test_match!(Command::new("python3").args(test).current_dir(sbrute_path).output());

    //capture output
    let mut output_vec: Vec<u8> = Vec::new();
    output_vec.append(&mut output.stdout);
    output_vec.append(&mut output.stderr);
    let output_str = simple_test_match!(String::from_utf8(output_vec));

    //capture exit code
    //get the exit code of the test script
    let _exit_status = match output.status.code() {
      Some(status) => status,
      None => {
        debug::print_debug("sbrute::on_test: failed to get sbrute exit code.");
        return TestStatus::Failed;
      }
    };

    //readt atf to string
    let atf_str = simple_test_match!(fs::read_to_string(atf_path));

    //compare output to atf
    if output_str != atf_str {
      debug::print_debug("sbrute::on_test: artifact doesnt match output.");
      return TestStatus::Failed;
    }

    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn sbrute(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SBrute {
    event: Event {
      name:   name,
      desc:   "Brute forces a serial device".to_string(),
      usage:  "Prompts you for:\n\
                \tSerial device (string)\n\
                \tBaud rate (integer)\n\
                \tUsername (string)\n".to_string(),
      parent: parent,
      author: "Merimetso".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** MINITERM.PY ***********************************/
pub struct MiniTerm { event: Event }
impl Eventable for MiniTerm {
  fn on_init(&self) -> Vec<String> {
    // Prompt for serial device
    let sd = prompt_in_event!("MiniTerm>", "Serial device to connect to: ");
    
    // Prompt for baud rate
    let baud = prompt_in_event!("MiniTerm>", "Baud rate: ");
    if sd.len() == 0 || baud.len() == 0 { return Vec::new(); }
    return vec![sd, baud];
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() < 2 { return self.event.usage.clone(); }
    args.insert(0, "ext/miniterm/miniterm.py".to_string());
    
    // Run command
    let output = simple_match!(run_console_command(Command::new("python3").args(args)));

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/miniterm/atf/atf.txt";
    let term_args = vec!["ext/miniterm/miniterm.py", "-h"];

    let check = simple_test_match!(Command::new("python3").args(term_args).output());

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
pub fn miniterm(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(MiniTerm {
    event: Event {
      name:   name,
      desc:   "A simple terminal program for the serial port.".to_string(),
      usage:  "Prompts you for:\n\
                \tSerial device (string)\n\
                \tBaud rate (integer)\n".to_string(),
      parent: parent,
      author: "Merimetso".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** Baudrate.py ***********************************/
pub struct Baudrate { event: Event }
impl Eventable for Baudrate {
  fn on_init(&self) -> Vec<String> {
    // Prompt for serial port to scan
    let mut sp = &prompt_in_event!("Baudrate>", "Serial port to connect to [eg. /dev/ttyUSB0]: ");

    let temp = &String::from("/dev/ttyUSB0");
    if !Path::new(&sp).exists() || sp.is_empty() {
      sp = temp;
    }

    // Prompt for baud rate
    let mode = &prompt_in_event!("Baudrate>", "Write serial config to file and start minicom when done? (Y or N): ");
    
    let mut write_flag = String::from("");
    if mode.to_string().trim().eq("Y"){
      write_flag.push_str("-n");
    }

    // Run command
    match run_console_command(Command::new("python3").args(vec!["ext/baudrate/baudrate.py", "-p", sp.trim(), "-a", &write_flag])) {
      Ok(out) => out,
        Err(_) => { return Vec::new(); }
    };
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/baudrate/atf/atf.txt";
    let term_args = vec!["ext/baudrate/baudrate.py","-h"];

    let check = match Command::new("python3").args(term_args).output() {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("baudrate::on_test: Failed to execute command. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("baudrate::on_test: Failed to open the artifact file. {}", err));
        return TestStatus::Failed;
      }
    };

    if String::from_utf8_lossy(&check.stdout) != file_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn baudrate(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Baudrate {
    event: Event {
      name:   name,
      desc:   "Determines the baudrate of a serial port".to_string(),
      usage:  "Prompts you for:\n\
                \tSerial device (string)\n\
                \tBaud rate (integer)\n".to_string(),
      parent: parent,
      author: "devttys0".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** NetworkSniffers ***********************************/
pub struct NetworkSniffers { event: Event }
impl Eventable for NetworkSniffers {
  fn on_init(&self) -> Vec<String> {
    print::println("Network Sniffers");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn networksniffers(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(NetworkSniffers {
    event: Event {
      name:   name,
      desc:   "network sniffing tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** DSNIFF ***********************************/
pub struct DSniff { event: Event }
impl Eventable for DSniff {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 { return self.event.usage.clone(); }
    // Run command
    let output = simple_match!(run_console_command(Command::new("dsniff").args(args)));

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/dsniff/atf/atf.txt";
    let sniff_args = vec!["-i", "lo"];

    let check = match run_bounded_command_err(Command::new("dsniff").args(sniff_args),false,2) {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("dsniff::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("dsniff::on_test: Failed to open the test file. {}", err));
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
pub fn dsniff(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(DSniff {
    event: Event {
      name:   name,
      desc:   "Password sniffing and network traffic analysis tool.".to_string(),
      usage:  "Usage : [-cdmn] [-i interface | -p pcapfile] [-s snaplen] [-f services] [-t trigger] [-r | -w savefile] [expression]\n\t-c : Perform half-duplex TCP stream reassembly, to handle asymmetrically routed traffic.\n\t-d : Enable debugging mode.\n\t-m : enable automatic protocol detection.\n\t-n : Do not resolve IP addresses to hostnames.\n\t-i interface : specify the interface to listen on.\n\t-p pcapfile : Rather than processing the contents of packets observed upon the network process the given PCAP capture file.\n\t-s snaplen : Analyze at most the first snaplen bytes of each TCP connection, rather than the default of 1024.\n\t-f services : Load triggers from a services file.\n\t-t trigger Load triggers from a comma-separated list, specified as port/proto=service (e.g. 80/tcp=http).\n\t-r : Read sniffed sessions from a savefile created with the -w option.\n\t -w file : Write sniffed sessions to savefile rather than parsing and printing them out.\n\t expression : Specify a tcpdump(8) filter expression to select traffic to sniff. On a hangup signal dsniff will dump its current trigger table to dsniff.services.\n".to_string(),
      parent: parent,
      author: "Dug Song".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
/*********************************** TCPDUMP ***********************************/
pub struct TCPDump { event: Event }
fn dump_interface() -> Vec<String> {
  //get interface device selection
  //get network interface options
  //run netstat and pipe into tail then pipe into choose first token of each line
  let selections = match util::pipes::Pipe::new("netstat -i")
    .then("tail -n +3").then("ext/util/choose 0").finally() { // Turn the Pipe into a Result<Child>
      Ok(sel) => sel,
      Err(err) => {
        print::println(&format!("Unable to retrieve network devices. Error : {}",err));
        return Vec::new();
      }
  };
  let selections = match selections.wait_with_output() {
    Ok(sel) => sel,
    Err(err) => {
      print::println(&format!("Unable to retrieve network devices. Error : {}",err));
      return Vec::new();
    }
  };
  //convert stdout into string
  //split up string by white space
  let selections_str = String::from_utf8_lossy(&selections.stdout).to_string();
  let selections: Vec<&str> = selections_str.split_whitespace().collect();

  // Prompt for num packets
  let num_packets: u32 = match prompt_in_event!("TcpDump>", "Num of packets to capture: ").trim().parse() {
    Ok(num) => num,
    Err(err) => {
      println!("Failed to parse num packets. {}", err); 
      return Vec::new();
    }
  };

  //get interface option from user
  let interface = match terminal::get_selection(selections.clone()) {
    Some(opt) => selections[opt],
    _ => {
      print::print_custom("Bad selection.\n","orange");
      return Vec::new();
    }
  };
  
  let num_packets_str = num_packets.to_string();
  let args = vec!["-c".to_string(),num_packets_str,"-i".to_string(),interface.to_string()];
  return args;
}
impl Eventable for TCPDump {
  fn on_init(&self) -> Vec<String> {
    return dump_interface();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    let arguments;
    if args.len() == 0 {
      return self.event.usage.clone();
    }
    if args.len() == 2 {
      arguments = vec!["-c".to_string(),args[1].clone(),"-i".to_string(),args[0].clone()];
    }
    else {
      arguments = args.clone();
    }
    let command = "tcpdump";
    print::print_custom(&format!("Sniffing until {} packets are received.\n", args[1]),"purple");
    let tcpdump_output = match run_command(Command::new(command).args(arguments)) {
      Ok(out) => out,
      Err(err) => {
        return format!("failed to execute tcp dump. {}\n", err);
      }
    };
    print::print_custom(&format!("{}", String::from_utf8_lossy(&tcpdump_output.stdout).to_string()),"gold");
    return String::from(""); 
  }
  fn on_test(&self) -> TestStatus {
    let test = vec!["-c","5","-i","lo"];
    let atf_path = "tst/tcpdump/atf/tcpdump_exitcode";
    //start up tcpdump for 5 packets on interface device lo
    let command = "tcpdump";
    let child = match Command::new(command).args(test).stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
      Ok(ch) => ch,
      Err(err) => {
        debug::print_debug(&format!("tcpdump::on_test: failed to execute tcp dump. {}", err));
        return TestStatus::Failed;
      }
    };
    let args = vec!["localhost","-i","0.2","-c","5"];
    match Command::new("ping").args(args).output() {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(&format!("tcpdump::on_test: failed to ping localhost. {}", err));
        return TestStatus::Failed;
      }
    };
    //wait for tcpdump to return
    let tcpdump_output = match child.wait_with_output() {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(&format!("tcpdump::on_test: failed to wait on tcpdump. {}", err));
        return TestStatus::Failed;
      }
    };
    ////check that tcpdump returned with non zero code
    //if !tcpdump_output.status.success() {
    //  debug::print_debug("tcpdump::on_test: tcpdump returned with nonzero exit code.");
    //  return TestStatus::Failed;
    //}
    //read artifact file to an integer
    //first read atf file to string
    let atf_int: i32 = match fs::read_to_string(atf_path) {
      Ok(string) => {
        //remove whitespace from the string
        let string: String = string.split_whitespace().collect();
        //parse an int from the string
        match string.parse::<i32>() {
          Ok(num) => num,
          Err(err) => {
            debug::print_debug(format!("tcpdump::on_test: failed to read atf to integer. {}", err));
            return TestStatus::Failed;
          }
        }
      },
      Err(err) => {
        debug::print_debug(format!("tcpdump::on_test: failed to read atf to string. {}", err));
        return TestStatus::Failed;
      }
    };
    //compare exit status against the artifact
    match tcpdump_output.status.code() {
      Some(status) => {
        //check for non-zer exit code
        if status != atf_int {
          debug::print_debug("tcpdump::on_test: tcpdump returned with a non-zero exit code.");
          return TestStatus::Failed;
        }
      },
      None => {
        debug::print_debug("tcpdump::on_test: tcpdump returned with a non-zero exit code.");
        return TestStatus::Failed;
      }
    };
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn tcpdump(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(TCPDump {
    event: Event {
      name:   name,
      desc:   "Reports the contents of packets on a network interface".to_string(),
      usage:  "tcpdump <network_interface> <number_of_packets>\n".to_string(),
      author: "tcpdump.org".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
/*********************************** TSHARK ***********************************/
pub struct TShark { event: Event }
impl Eventable for TShark {
  fn on_init(&self) -> Vec<String> {
    print::println("TShark");
    return vec!["".to_string()];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 { return self.event.usage.clone(); }
    //TODO: ask for tshark assurance. are you sure you
    //TODO: execute tshark command
    let tshark_output = match run_console_command(Command::new("tshark").args(args)) {
      Ok(out) => out,
      Err(err) => {
        return format!("failed to wait on tshark output {}", err);
      }
    };
    return String::from_utf8_lossy(&tshark_output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let test = vec!["-c","5","-i","lo"];
    let atf_path = "tst/tshark/atf/tshark_exitcode";
    //start up tcpdump for 5 packets on interface device lo
    let command = "tshark";
    let child = match Command::new(command).args(test).stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
      Ok(ch) => ch,
      Err(err) => {
        debug::print_debug(&format!("tshark::on_test: failed to execute tshark. {}", err));
        return TestStatus::Failed;
      }
    };
    let args = vec!["localhost","-i","0.2","-c","5"];
    match Command::new("ping").args(args).output() {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(&format!("tshark::on_test: failed to ping localhost. {}", err));
        return TestStatus::Failed;
      }
    };
    //wait for tcpdump to return
    let tshark_output = match child.wait_with_output() {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(&format!("tshark::on_test: failed to wait on tshark. {}", err));
        return TestStatus::Failed;
      }
    };
    ////check that tcpdump returned with non zero code
    //if !tcpdump_output.status.success() {
    //  debug::print_debug("tcpdump::on_test: tcpdump returned with nonzero exit code.");
    //  return TestStatus::Failed;
    //}
    //read artifact file to an integer
    //first read atf file to string
    let atf_int: i32 = match fs::read_to_string(atf_path) {
      Ok(string) => {
        //remove whitespace from the string
        let string: String = string.split_whitespace().collect();
        //parse an int from the string
        match string.parse::<i32>() {
          Ok(num) => num,
          Err(err) => {
            debug::print_debug(format!("tshark::on_test: failed to read atf to integer. {}", err));
            return TestStatus::Failed;
          }
        }
      },
      Err(err) => {
        debug::print_debug(format!("tshark::on_test: failed to read atf to string. {}", err));
        return TestStatus::Failed;
      }
    };
    //compare exit status against the artifact
    match tshark_output.status.code() {
      Some(status) => {
        //check for non-zer exit code
        if status != atf_int {
          debug::print_debug("tshark::on_test: tshark returned with a non-zero exit code.");
          return TestStatus::Failed;
        }
      },
      None => {
        debug::print_debug("tshark::on_test: tshark returned with a non-zero exit code.");
        return TestStatus::Failed;
      }
    };
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn tshark(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(TShark {
    event: Event {
      name:   name,
      desc:   "Network analyzer that captures packets from a live network or reads from capture file.".to_string(),
      usage:  filter::parse_help_menu(Command::new("tshark").args(vec!["--help"]),
                                      vec!["!ALL!".to_string()], "\n", 0, ""),
      parent: parent,
      author: "wireshark.org".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** SNIFF ***********************************/
pub struct Sniff { event: Event }
impl Eventable for Sniff {
  fn on_init(&self) -> Vec<String> {
    //get network interface options
    //run netstat and pipe into tail then pipe into choose first token of each line
    let selections = match util::pipes::Pipe::new("netstat -i")
      .then("tail -n +3").then("ext/util/choose 0").finally() { // Turn the Pipe into a Result<Child>
        Ok(sel) => sel,
        Err(err) => {
          print::println(&format!("Unable to retrieve network devices. Error : {}",err));
          return Vec::new();
        }
    };
    let selections = match selections.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        print::println(&format!("Unable to retrieve network devices. Error : {}",err));
        return Vec::new();
      }
    };
    //convert stdout into string
    //split up string by white space
    let selections_str = String::from_utf8_lossy(&selections.stdout).to_string();
    let selections: Vec<&str> = selections_str.split_whitespace().collect();

    //prompt for interface option from user
    let interface = match terminal::get_selection(selections.clone()) {
      Some(opt) => selections[opt],
      _ => {
        print::print_custom("Bad selection.\n","orange");
        return Vec::new();
      }
    };
    // Prompt for num packets
    let num_packets: u32 = match prompt_in_event!("Sniff>", "num packets: ").trim().parse() {
      Ok(num) => num,
      Err(err) => {
        println!("Failed to parse num packets. {}", err); 
        return Vec::new();
      }
    };
    
    let num_packets_str = &num_packets.to_string();
    let args = vec![interface.to_string(),num_packets_str.to_string()];
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 2 { return self.event.usage.clone(); }
    //execute command
    let working_dir = "ext/sniff";
    let command = "./sniff_glue";
    print::print_custom(&format!("Sniffing until {} packets are captured.\n",&args[1]),"purple");
    let output = simple_match!(run_console_command(Command::new(command).current_dir(working_dir).args(args)));

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let working_dir = "ext/sniff";
    let command = "./sniff_glue";
    let test = vec!["lo","5"];
    let atf_path_str = "tst/sniff/atf/sniffglue_atf";

    let args = vec!["localhost","-i","0.2","-c","10"];
    let mut ping_child = match Command::new("ping").args(args).stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
      Ok(ping_child) => ping_child,
      Err(err) => {
        debug::print_debug(&format!("sniff::on_test: failed to ping localhost. {}", err));
        return TestStatus::Failed;
      }
    };

    let sg_output = match Command::new(command).current_dir(working_dir).args(test).output() {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(&format!("sniff::on_test: failed to spawn sniff child. {}", err));
        return TestStatus::Failed;
      }
    };
    match ping_child.wait() {
      Ok(_) => {},
      Err(err) => {
        debug::print_debug(&format!("sniff::on_test: failed to wait on ping command. {}", err));
        return TestStatus::Failed;
      }
    };

    let atf_str = match fs::read_to_string(atf_path_str) {
      Ok(string) => string,
      Err(err) => {
        debug::print_debug(&format!("sniff::on_test: failed to read artifact file. {}", err));
        return TestStatus::Failed;
      }
    };
    let atf_str = atf_str.trim();
    let output_string = String::from_utf8_lossy(&sg_output.stdout);
    //println!("{}",output_string);
    if !output_string.contains(atf_str) {
      debug::print_debug("sniff::on_test: output doesnt match artifact");
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn sniff(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Sniff {
    event: Event {
      name:   name,
      desc:   "Network sniffing tool that uses a thread pool to utilize all cpu cores.".to_string(),
      usage:  "sniff <network_interface> <number_of_packets>\n".to_string(),
      parent: parent,
      author: "Matzr3lla & github.com/kpcyrd".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** PortScanners ***********************************/
pub struct PortScanners { event: Event }
impl Eventable for PortScanners {
  fn on_init(&self) -> Vec<String> {
    print::println("Port Scanners");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn portscanners(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PortScanners {
    event: Event {
      name:   name,
      desc:   "port scanning tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** Sandmap ***********************************/
pub struct Sandmap { event: Event }
impl Eventable for Sandmap {
  fn on_init(&self) -> Vec<String> {
    // Run command
    println!("Loading Sandmap... please stand by");
    return Vec::new();
  }
  fn on_run(&self, _args: Vec<String>) -> String {
    let output = simple_match!(run_console_command(&mut Command::new("ext/sandmap/bin/sandmap")));

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/sandmap/atf/atf.txt";

    // Run check
    let check = match Command::new("ext/sandmap/bin/sandmap").arg("--help").output() {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("sandmap::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("sandmap::on_test: Failed to open the test file. {}", err));
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
pub fn sandmap(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Sandmap {
    event: Event {
      name:   name,
      desc:   "Performs network and system reconnaissance".to_string(),
      usage:  "Simply invoke 'sandmap' to start using\n".to_string(),
      parent: parent,
      author: "Michal Zy (github.com/trimstray)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** rang3r ***********************************/
pub struct Rang3r { event: Event }
impl Eventable for Rang3r {
  fn on_init(&self) -> Vec<String> {
    // Prompt for args
    let ip_addr = prompt_in_event!("rang3r> Please give IP address on network to scan: ", "");
    
    // Run command
    return vec![ip_addr];
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    args.insert(0, "ext/rang3r/qscan.py".to_string());
    args.insert(1, "-i".to_string());
    let output = simple_match!(run_console_command(Command::new("python3").args(args)));
    
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/rang3r/atf/atf.txt";

    // Run check
    let check = match util::pipes::Pipe::new(&"python3 ext/rang3r/qscan.py -i 127.0.0.1".to_string())
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("ccdiff::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("ccdiff::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("batcat::on_test: Failed to open the test file. {}", err));
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
pub fn rang3r(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Rang3r {
    event: Event {
      name:   name,
      desc:   "Finds existing hosts on a network and open ports.".to_string(),
      usage:  "rang3r <IP_address>\n".to_string(),
      parent: parent,
      author: "Florian Kunushevci (github.com/floriankunushevci)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
