/*
 * QVLx Salvum 
 *
 * dostools.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;
use std::str;
use crate::alerts;

/*********************************** DOS ***********************************/
pub struct DoS { event: Event }
impl Eventable for DoS {
  fn on_init(&self) -> Vec<String> {
    print::println("DoS");
    alerts::print_warning(); 
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn dos(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(DoS {
    event: Event {
      name:   name,
      desc:   "Denial of Service attack tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** GENERATORS ***********************************/
pub struct Generators { event: Event }
impl Eventable for Generators {
  fn on_init(&self) -> Vec<String> {
    print::println("Generators");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn generators(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Generators {
    event: Event {
      name:   name,
      desc:   "generator tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** CQHACK ***********************************/
pub struct CQHack { event: Event }
impl Eventable for CQHack {
  fn on_init(&self) -> Vec<String> {
    // prompt for arguments: IP_address, packet size, bandwith, time
    let address = prompt_in_event!("cqHack>", "Address: ");
    let packet_size = prompt_in_event!("cqHack>", "Packet size: ");
    let bandwidth = prompt_in_event!("cqHack>", "Bandwidth: ");
    let time = prompt_in_event!("cqHack>", "Time: ");
    
    return vec![address, packet_size, bandwidth, time];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 4 {
      return self.event.usage.to_string();
    }
    
    let mut packet_size = String::from("--size=");
    packet_size.push_str(&args[1]);

    let mut bandwidth = String::from("--bandwidth=");
    bandwidth.push_str(&args[2]);

    let mut time = String::from("--time=");
    time.push_str(&args[3]);
    
    let arguments = vec![args[0].clone(),packet_size,bandwidth,time];

    // Run command
    let output = simple_match!(run_console_command(&mut Command::new("ext/cqhack/cqHack.pl").args(arguments)));

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["127.0.0.1", "--time=1", "tst/cqhack/atf/atf.txt"]];
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("ext/cqhack/cqHack.pl").args(&test[..2]).output());
      let out_str = String::from_utf8_lossy(&output.stdout);

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[2]));

      // Compare
      if out_str != file_str {
        debug::print_debug(format!("\n-----\n{}\n-----\n{}\n-----\n", out_str, file_str));
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn cqhack(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(CQHack {
    event: Event {
      name:   name,
      desc:   "Performs a DoS attack on open ports".to_string(),
      usage:  "Prompts you for:\n\
                \tAddress (String)\n\
                \tPacket Size (decimal integer; min 65, max 1500)\n\
                \tBandwidth (decimal integer)\n\
                \tTime (decimal integer)\n".to_string(),
      parent: parent,
      author: "SamY from cqHack".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** CONNECTION KILLER ***********************************/
pub struct ConnectionKillers { event: Event }
impl Eventable for ConnectionKillers {
  fn on_init(&self) -> Vec<String> {
    print::println("Connection Killers");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn connectionkillers(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ConnectionKillers {
    event: Event {
      name:   name,
      desc:   "connectionkillers tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** TCPKILL ***********************************/
pub struct TcpKill { event: Event }
impl Eventable for TcpKill {
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

    //prompt user for network interface selection
    let prompt = String::from("Please select network interface to use");
    print::println(&format!("{} ", prompt));

    //get interface option from user
    let interface = match terminal::get_selection(selections.clone()) {
      Some(opt) => selections[opt],
      _ => {
        print::print_custom("Bad selection.\n","orange");
        return Vec::new();
      }
    };

    //prompt for connection address to kill
    let connection = prompt_in_event!("TcpKill>", "Connection address or name: ");
    let mut connection_arg = "host ".to_string();
    connection_arg.push_str(&connection);

    // Prompt for brute mode
    let mode_str = prompt_in_event!("TcpKill>", "Brute force mode number (1 - 9): ");
    //check for valid mode
    let mode_num: u8 = match mode_str.parse() {
      Ok(mode) => mode,
      Err(_) => {
        print::println(&format!("Invalid mode. Must be 1 - 9: {}",mode_str));
        return Vec::new();
      }
    };
    if mode_num < 1 || mode_num > 9 {
      print::println(&format!("Invalid mode. Must be 1 - 9: {}",mode_num));
      return Vec::new();
    }
    //create the mode arg being passed to tcpkill
    let mut mode_arg = "-".to_string();
    mode_arg.push_str(&mode_str);
    let args = vec!["-i", interface, &mode_arg,&connection_arg];
    match run_console_command(Command::new("ext/tcpkill/tcpkill").args(args)) {
      Ok(_) => {},
      Err(err) => {
        print::println(&format!("Failed to run tcpkill. Error: {}",err));
        return Vec::new();
      }
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/tcpkill/atf/atf.txt";

    let check = match run_bounded_command_err(Command::new("ext/tcpkill/tcpkill").args(vec!["-i","lo","port","9999"]),false,1) {
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
    if String::from_utf8_lossy(&check.stderr) != file_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn tcpkill(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(TcpKill {
    event: Event {
      name:   name,
      desc:   "Terminates connections to or from a host, network, port, or combination of all.".to_string(),
      usage:  "Prompts you for: \
              \n\tNetwork interface device \
              \n\tConnection address or name \
              \n\tBrute force mode (1-9)\n".to_string(),
      parent: parent,
      author: "Dug Song".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** aSYNcrone ***********************************/
pub struct Asyncrone { event: Event }
impl Eventable for Asyncrone {
  fn on_init(&self) -> Vec<String> {
    // Prompt for source port
    let src_port = prompt_in_event!("asyncrone>", "Enter Source Port: ");

    // Prompt for target ip address
    let tar_ip = prompt_in_event!("asyncrone>", "Enter Target IP: ");

    // Prompt for target port
    let tar_port = prompt_in_event!("ayncrone>", "Enter Target port: ");
    
    return vec![src_port, tar_ip, tar_port, "1".to_string()];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/asyncrone/aSYNcrone").args(args)));

    // Print output
    log::log("asyncrone", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
    
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/asyncrone/atf/atf.txt";

    // Run check
    let check = match Command::new("ext/asyncrone/aSYNcrone").arg("-h").output() {  
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("asyncrone::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("asyncrone::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    let check_str = String::from_utf8_lossy(&check.stdout);
    if check_str != file_str {
      debug::print_debug(format!("asyncrone::on_test: artifact does not match test file"));
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn asyncrone(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Asyncrone {
    event: Event {
      name:   name,
      desc:   "Performs a SYN flood to disable a target".to_string(),
      usage:  "asyncrone <source_port> <target_IP> <target_port> <number_of_threads>\n".to_string(),
      parent: parent,
      author: "Fatih Sensoy".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

