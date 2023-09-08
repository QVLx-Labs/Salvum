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
 * ecctools.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
 use crate::events::*;

 /********************************** SPOOFERS **********************************/
pub struct Spoofers { event: Event }
impl Eventable for Spoofers {
  fn on_init(&self) -> Vec<String> {
    print::println("Spoofers");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn spoofers(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Spoofers {
    event: Event {
      name:   name,
      desc:   "Spoofing attack tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** ARPSpoof ***********************************/
pub struct ARPSpoof { event: Event }
impl Eventable for ARPSpoof {
  fn on_init(&self) -> Vec<String> {
    // Prompt for IP Address
    let ip_addr = prompt_in_event!("ARPSpoof>", "Enter target IP address to poison or hit enter for default (All hosts on LAN): ");

    let gateway_ip = prompt_in_event!("ARPSpoof>", "Enter router IP address to spoof or leave blank for default gateway: ");

    //let prompt = create_prompt("ARPSpoof>", "Enter network interface to use or just hit enter. (Default is loopback): ");
    //let interface = prompt_in_event!(prompt);
    
    //get network interface options
    //run netstat and pipe into tail then pipe into choose first token of each line
    let interfaces = match util::pipes::Pipe::new("netstat -i")
      .then("tail -n +3").then("ext/choose 0").finally() { // Turn the Pipe into a Result<Child>
        Ok(sel) => sel,
        Err(err) => {
          print::println(&format!("Unable to retrieve network devices. Error : {}",err));
          return Vec::new();
        }
    };
    let interfaces = match interfaces.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        print::println(&format!("Unable to retrieve network devices. Error : {}",err));
        return Vec::new();
      }
    };

    let interfaces_str = String::from_utf8_lossy(&interfaces.stdout);
    print::print("Potential interfaces: ");
    for (i, item) in interfaces_str.lines().enumerate(){
      if i == 4 { break; }
      else{ let temp = String::from(item.to_string());
             print::print_custom(&temp, "orange");
             print!("  ");
      }
    }
    println!();

    let args: Vec<String>;
    let localhost;

    let time = prompt_in_event!("ARPSpoof>", "Enter the max time to run this tool in seconds. (Default is 10): ");

    // no IPs given
    if ip_addr.trim().is_empty() && gateway_ip.trim().is_empty(){
      localhost = String::from("127.0.0.1").trim().to_string();
      args = vec![localhost, time];
    }
    // only gateway given
    else if ip_addr.trim().is_empty() && !gateway_ip.trim().is_empty(){
      localhost = String::from("127.0.0.1");
      args = vec!["-t".to_string(), localhost.trim().to_string(), gateway_ip.trim().to_string(), time];
    }
    // only attacker given
    else if !ip_addr.trim().is_empty() && gateway_ip.trim().is_empty(){
      args = vec![gateway_ip.trim().to_string(), time];
    }
    // both given
    else{
      args = vec!["-t".to_string(), ip_addr.trim().to_string(), gateway_ip.trim().to_string(), time];
    }

    return args;
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    let time: u64 = match args.pop() {
      Some(t) => match t.parse() {
        Ok(p) => p,
        Err(_) => 10,
      },
      None => 10
    };

    // Run main command
    let output = simple_match!(run_bounded_command(Command::new("ext/arpspoof/arpspoof_apt").args(args), true, time));

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/arpspoof/atf/atf.txt";
    let git_args = vec!["-t", "127.0.0.1", "127.0.0.0"];

    let check = match Command::new("ext/arpspoof/arpspoof_apt").args(git_args).output() {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("arpspoof::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("arpspoof::on_test: Failed to open the test file. {}", err));
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
pub fn arpspoof(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ARPSpoof {
    event: Event {
      name:   name,
      desc:   "ARPSpoof".to_string(),
      usage:  "Will prompt you for: [poison_ip] [spoof_ip] [interface] [time]\n".to_string(),
      parent: parent,
      author: "Alex Landau (github.com/alandau)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** Claim-ip ***********************************/
pub struct ClaimIP { event: Event }
impl Eventable for ClaimIP {
  fn on_init(&self) -> Vec<String> {
    print::println("ClaimIP");

    //get network interface options
    //run netstat and pipe into tail then pipe into choose first token of each line
    let selections = match util::pipes::Pipe::new("netstat -i")
      .then("tail -n +3").then("ext/choose 0").finally() { // Turn the Pipe into a Result<Child>
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

    //prompt user
    let prompt = String::from("Please select network interface to spoof on");
    print::println(&format!("{} ", prompt));

    //get interface option from user
    let interface = match terminal::get_selection(selections.clone()) {
      Some(opt) => selections[opt],
      _ => {
        print::print_custom("Bad selection.\n","orange");
        return Vec::new();
      }
    };    

    // Prompt for IP Address
    let ip_addr = prompt_in_event!("ClaimIP>", "Please enter IP address to pose as (Default 127.0.0.1): ");
    let args: Vec<String>;
    let localhost;

    // no IPs given
    if ip_addr.trim().is_empty() {
      localhost = String::from("127.0.0.1").trim().to_string();
      args = vec![interface.trim().to_string(), localhost];
    }
    else {
      args = vec![interface.trim().to_string(), ip_addr.trim().to_string()];
    }

    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run main command
    let output = simple_match!(run_command(&mut Command::new("ext/claimip/claim-ip").args(args)));

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/claimip/atf/atf.txt";
    let claim_args = vec!["lo", "127.0.0.1"];

    let check = match run_bounded_command(Command::new("ext/claimip/claim-ip").args(claim_args), false, 3) {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("claimIP::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("claimIP::on_test: Failed to open the test file. {}", err));
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
pub fn claimip(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ClaimIP {
    event: Event {
      name:   name,
      desc:   "ClaimIP".to_string(),
      usage:  "Will prompt you for: [poison_ip] [spoof_ip] [interface] [time]\n".to_string(),
      parent: parent,
      author: "Justin Ossevoort (github.com/internetionals)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** space_packet_spoof ***********************************/
pub struct SpacePacketSpoof { event: Event }
impl Eventable for SpacePacketSpoof {
  fn on_init(&self) -> Vec<String> {
    print::println("CCSDS Space Packet Spoofer");
    return Vec::new();
   }
   fn on_run(&self, args: Vec<String>) -> String {
     // Run command
     let output = simple_match!(run_command(Command::new("ext/spacepacketspoof/space_packet_spoof").args(args)));

     log::log("space_packet_spoof", &String::from_utf8_lossy(&output.stderr));
     return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/ananas/atf/atf.txt";

    // Run command
    let output = match run_bounded_command(Command::new("ext/ananas/ananas").arg("N"), false, 2) {
      Ok(o) => o,
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Read artifact file
    let atf_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("ananas::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&output.stdout) != atf_str {
      return TestStatus::Failed;
    }
    return TestStatus::Failed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn spacepacketspoof(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SpacePacketSpoof {
    event: Event {
      name:   name,
      desc:   "Ananas NaN-based Obfuscator.".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "Matzr3lla & KubOS Preservation Group".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** fits_spoof ***********************************/
pub struct FitsSpoof { event: Event }
impl Eventable for FitsSpoof {
  fn on_init(&self) -> Vec<String> {
    print::println("NASA FITS Packet Spoofer");
    return Vec::new();
   }
   fn on_run(&self, args: Vec<String>) -> String {
     // Run command
     let output = simple_match!(run_command(Command::new("ext/fitsspoof/FITSspoof").args(args)));

     log::log("fitsspoof", &String::from_utf8_lossy(&output.stderr));
     return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/ananas/atf/atf.txt";

    // Run command
    let output = match run_bounded_command(Command::new("ext/ananas/ananas").arg("N"), false, 2) {
      Ok(o) => o,
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Read artifact file
    let atf_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("ananas::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&output.stdout) != atf_str {
      return TestStatus::Failed;
    }
    return TestStatus::Failed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn fitsspoof(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(FitsSpoof {
    event: Event {
      name:   name,
      desc:   "Ananas NaN-based Obfuscator.".to_string(),
      usage:  "Matzr3lla & Matthew Stadelman (github.com/stadelmanma)".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
