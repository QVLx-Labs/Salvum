/*
 * QVLx Salvum 
 *
 * packetsniffers.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;

/*********************************** Detection Evasion ***********************************/
pub struct DetectionEvasion { event: Event }
impl Eventable for DetectionEvasion {
  fn on_init(&self) -> Vec<String> {
    print::println("Detection Evasion");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn detectionevasion(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(DetectionEvasion {
    event: Event {
      name:   name,
      desc:   "Detection evasion tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** Fragrouter ***********************************/
pub struct Fragrouter { event: Event }
impl Eventable for Fragrouter {
  fn on_init(&self) -> Vec<String> {
    print::println("Fragrouter");
    
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

    //prompt user
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

    //prompt for -p option
    let p_option = prompt_in_event!("FragRouter>","Would you to preserve the entire protocol header in the first fragment? (y/n) : ");

    //attack selection options
    let selections = vec![
      "baseline-1: Normal IP forwarding.",
      "frag-1: Send ordered 8-byte IP fragments.",
      "frag-2: Send ordered 24-byte IP.",
      "frag-3: Send ordered 8-byte IP, one fragment sent out of order.",
      "frag-4: Send ordered 8-byte IP, duplicate penultimate fragment.",
      "frag-5: Send out of order 8-byte IP, duplicatethe penultimate fragment.",
      "frag-6: Send ordered 8-byte IP, send marked last fragment first.",
      "frag-7: Send ordered 16-byte IP, precede each with 8-byte null that overlaps latter half.",
      "tcp-1: TCP handshake, send fake FIN and RST (bad checksum), send ordered 1-byte.",
      "tcp-3: TCP handshake, send ordered 1-byte, duplicate penultimates of original TCP packets.",
      "tcp-4: TCP handshake, send ordered 1-byte, send additional 1-byte that overlaps penultimate with null.",
      "tcp-5: TCP handshake, send ordered 2-byte, precede each segment with 1-byte null that overlaps latter half.",
      "tcp-7: TCP handshake, send ordered 1-byte mix with 1-byte nulls, same connection different sequence numbers.",
      "tcp-8: TCP handshake, send ordered 1-byte segments with one segment sent out of order.",
      "tcp-9: TCP handshake, send out of order 1-byte segments.",
      "tcbc-2: TCP handshake, send ordered 1-byte segments mix with SYN packets, same connection params.",
      "tcbc-3: Do not complete TCP handshake, send null ordered 1-byte segments as if one had occured.",
      "tcbt-1: TCP handshake, RST stop connection, reconnect with different sequence numbers, send ordered 1-byte.",
      "ins-2: TCP handshake, send ordered 1-byte segments with bad TCP checksums.",
      "ins-3: TCP handshake, send ordered 1-byte segments with no ACK flag set."];
    //flags vector coupled to the selection strings
    let flags = vec!["-B1","-F1","-F2","-F3","-F4","-F5","-F6","-F7","-T1","-T3","-T4","-T5","-T7","-T8","-T9","-C2","-C3","-R1","-I2","-I3"];
    //get attack option from user
    let index = match terminal::get_selection(selections) {
      Some(opt) => opt,
      _ => {
        print::println("");
        return Vec::new();
      }
    };
    //create fragrouter command and execute
    let mut args = vec!["-i", interface];
    if p_option.eq("y") || p_option.eq("Y") {
      args.push("-p");
    }
    args.push(flags[index]);
    match run_console_command(Command::new("ext/fragrouter/fragrouter").args(args)) {
      Ok(_) => {},
      Err(_) => {}
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/fragrouter/atf/atf.txt";

    let check = match run_bounded_command_err(Command::new("ext/fragrouter/fragrouter").arg("-M2"), false, 1) {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("fragrouter::on_test: Failed to run command. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("fragrouter::on_test: Failed to open the atf file. {}", err));
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
pub fn fragrouter(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Fragrouter {
    event: Event {
      name:   name,
      desc:   "A network intrusion detection evasion toolkit.".to_string(),
      usage:  "Prompts you for: \
              \n\tNetwork interface device \
              \n\tHeader preservation \
              \n\tAttack option\n".to_string(),
      author: "nidsbench@anzen.com (potentially outdated)".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
/*********************************** PWNCAT ***********************************/
pub struct Pwncat { event: Event }
impl Eventable for Pwncat {
  fn on_init(&self) -> Vec<String> {
    print::println("pwncat");
    return vec!["--help".to_string()];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    //let mut pwncat_child = match Command::new("pwncat").args(args).spawn() {
    //  Ok(child) => child,
    //  Err(err) => {
    //    return format!("failed to spawn pwncat child proccess. {}\n",err);
    //  }
    //};
    //let _exit_status = match pwncat_child.wait() {
    //  Ok(status) => status,
    //  Err(err) => {
    //    return format!("failed to wait on pwncat child proccess. {}\n",err);
    //  }
    //};
    let pwncat_output = match run_console_command(Command::new("pwncat").args(args)) {
      Ok(output) => output,
      Err(err) => {
        return format!("failed to execute pwncat {}",err);
      }
    };
    return String::from_utf8_lossy(&pwncat_output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let test = vec!["-z","localhost","1-1000"];
    let command = "pwncat";
    let atf_path_str = "tst/pwncat/atf/pwncat_-z_localhost_1-1000";
    let output = match Command::new(command).args(test).output() {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(&format!("pwncat::on_test: failed to run pwncat command. {}", err));
        return TestStatus::Failed;
      }
    };
    if !output.status.success() {
      debug::print_debug(&format!("pwncat::on_test: pwncat did not execute successfully."));
      return TestStatus::Failed;
    }
    let output_str = String::from_utf8_lossy(&output.stdout);
    let atf_str = match fs::read_to_string(atf_path_str) {
      Ok(string) => string,
      Err(err) => {
        debug::print_debug(&format!("pwncat::on_test: failed to read artifact file to a string. {}", err));
        return TestStatus::Failed;
      }
    };
    let atf_str = atf_str.trim();
    if !output_str.contains(atf_str) {
      debug::print_debug(&format!("pwncat::on_test: output doesnt match artifact"));
      return TestStatus::Failed;
    }
    //let check = match run_bounded_command_err(Command::new("ext/fragrouter").arg("-M2"), false, 1)
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn pwncat(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Pwncat {
    event: Event {
      name:   name,
      desc:   "Netcat on steroids with Firewall, IDS/IPS evasion, bind/reverse shell, port forwarding.".to_string(),
      usage:  "pwncat [options] <hostname> <port>\n".to_string(),
      author: "github.com/cytopia".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
