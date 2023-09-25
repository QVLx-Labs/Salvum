/*
 * QVLx Salvum 
 *
 * networkanalyzers.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */

// Imports
use crate::events::*;

/*********************************** NETWORK ANALYZER ***********************************/
pub struct NetworkAnalyzer { event: Event }
impl Eventable for NetworkAnalyzer {
  fn on_init(&self) -> Vec<String> {
    print::println("Network analyzer");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn networkanalyzer(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(NetworkAnalyzer {
    event: Event {
      name:   name,
      desc:   "Network analyzing tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** DoS Detection ***********************************/
pub struct DosDetection { event: Event }
impl Eventable for DosDetection {
  fn on_init(&self) -> Vec<String> {
    print::println("DoS Detection");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn dosdetection(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(DosDetection {
    event: Event {
      name:   name,
      desc:   "DoS Detection".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }    
  })
}

/*********************************** RIM ***********************************/
pub struct Rim { event: Event }
impl Eventable for Rim {
  fn on_init(&self) -> Vec<String> {
    // Prompt for IP address
    let address = prompt_in_event!("Rim>", "IP Address: ") + "\n";

    // Prompt for username
    let username = prompt_in_event!("Rim>", "Username: ");

    // Prompt for password
    let password = prompt_passwd_in_event("Rim>", "Password: ");

    return vec![address, username, password];
  }
  
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 3 {
      return self.event.usage.clone();
    }

    let mut address = args[0].to_string();
    let username = args[1].to_string();
    let mut password = args[2].to_string();

    // Create a file
    let mut file = simple_match!(File::create("usr/host.txt"));

    // Write the address to it
    simple_match!(file.write_all(address.as_bytes()));

    let output = simple_match!(run_command(Command::new("ext/rim/rim").args(vec!["-f", "usr/host.txt", "-u", &username, "-p", &password])));

    // Delete the file once done
    simple_match!(fs::remove_file("usr/host.txt"));

    address.zeroize();
    password.zeroize();

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let input_path = "tst/rim/inp/test.txt";
    let artifact_path = "tst/rim/atf/atf.txt";

    let rim_args = vec!["-f", input_path];

    // Run command
    let check = simple_test_match!(Command::new("ext/rim/rim").args(rim_args).output());


    // Read file
    let file_str = simple_test_match!(fs::read_to_string(artifact_path));

    // Compare
    if String::from_utf8_lossy(&check.stderr) != file_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn rim(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Rim {
    event: Event {
      name:   name,
      desc:   "Grabs the remote network statuses of an address".to_string(),
      usage:  "Requires a:\n\
                \tIP Address (string)\n\
                \tUsername for the IP(string)\n\
                \tPassword for the user(string)\n".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** IDS ***********************************/
pub struct Ids { event: Event }
impl Eventable for Ids {
  fn on_init(&self) -> Vec<String> {
    print::println("ids tools");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ids(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Ids {
    event: Event {
      name:   name,
      desc:   "ids tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }    
  })
}

/*********************************** ArpDefense ***********************************/
pub struct ArpDefense { event: Event }
impl Eventable for ArpDefense {
  fn on_init(&self) -> Vec<String> {
    // Prompt for ip addr
    let ipaddr = prompt_in_event!("ArpDefense>", "Ip address to monitor: ");
    //prompt for network interface
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
    //get interface option from user
    let interface = match terminal::get_selection(selections.clone()) {
      Some(opt) => selections[opt],
      _ => {
        print::print_custom("Bad selection.\n","orange");
        return Vec::new();
      }
    };

    //create args vec
    let args = vec!["-a".to_string(),ipaddr,"-f".to_string(),interface.to_string()];
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    let mut args_new = args.clone();
    args_new.insert(0, "ext/arpdefense/defendARP.py".to_string());
    //println!("{:?}",args_new);
    //spawn up arpdefense child process
    let arpdefense_output = match run_console_command(Command::new("python3").args(args_new)) {
      Ok(output) => output,
      Err(err) => {
        return format!("failed to execute arpdefense child. {}\n", err);
      }
    };
    /*let mut child_proc = match Command::new("python3").args(args_new).spawn() {
      Ok(child) => child,
      Err(err) => {
        return format!("failed to spawn arpdefense child. {}\n", err);
      }
    };
    match child_proc.wait() {
      Ok(_exitstatus) => {
        return format!("\n");
      },
      Err(err) => {
        match child_proc.kill() {
          Ok(()) => {
            return format!("failed to wait on arpdefense. {}\n", err);
          },
          Err(err) => {
            return format!("failed to wait and kill arpdefense. {}\n", err);
          }
        };
      }
    };*/
    return String::from_utf8_lossy(&arpdefense_output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let test = vec!["ext/arpdefense/defendARP.py","-a","localhost","-t"];
    //read artifact file to a i32 in mem
    let atf_path_str = "tst/arpdefense/atf_exitcode";
    let atf_path = Path::new(atf_path_str);
    let atf_code = match fs::read_to_string(atf_path) {
      Ok(string) => {
        match string.trim().parse::<i32>() {
          Ok(num) => num,
          Err(err) => {
            //TODO: handle error
            debug::print_debug(&format!("arpdefense::on_test: failed to parse atf to integer. {}", err));
            return TestStatus::Failed;
          }
        }
      }
      Err(err) => {
        debug::print_debug(&format!("arpdefense::on_test: failed to parse atf to integer. {}", err));
        return TestStatus::Failed;
      }
    };
    //run arpdefend command
    let output = match Command::new("python3").args(test).output() {
      Ok(out) => out,
      Err(err) => {
        //TODO: handle error
        debug::print_debug(&format!("arpdefense::on_test: failed to run arpdefense. {}", err));
        return TestStatus::Failed;
      }
    };
    //get exit code from command output
    let code = match output.status.code() {
      Some(num) => num,
      None => {
        //TODO: handle error
        debug::print_debug("arpdefense::on_test: failed to get exit status from arpdefense.");
        return TestStatus::Failed;
      }
    };
    //compare comand output against atf file
    if code != atf_code {
      //TODO: handle error
      debug::print_debug("arpdefense::on_test: arpdefense exit status doesnt match artifact");
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn arpdefense(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ArpDefense {
    event: Event {
      name:   name,
      desc:   "arpdefense - a script that can be run on a single device to protect against ARP Poisoning Attacks and to identify which device on the network is executing the attack.".to_string(),
      usage:  "Prompts you for: \
      \n\tIp address to monitor\
      \n\tNetwork interface device to monitor \n".to_string(),
      author: "Alan Reed (github.com/aarreedd)".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
