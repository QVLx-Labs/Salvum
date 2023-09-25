/*
 * QVLx Salvum 
 *
 * reverseshell.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;

/*********************************** Reverse Shell ***********************************/
pub struct Reverseshell { event: Event }
impl Eventable for Reverseshell {
  fn on_init(&self) -> Vec<String> {
    print::println("Reverse Shell");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn reverseshell(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Reverseshell {
    event: Event {
      name:   name,
      desc:   "Reverse shell tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** rustcat ***********************************/
pub struct Rustcat { event: Event }
impl Eventable for Rustcat {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/rustcat/rcat").args(&args)));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let atf_path = "tst/rustcat/atf/atf.txt";

    // Run command
    let output = simple_test_match!(Command::new("ext/rustcat/rcat").arg("--help").output());

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
pub fn rustcat(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Rustcat {
    event: Event {
      name:   name,
      desc:   "Performs DHCP-based queries, attacks, and interceptions.".to_string(),
      usage:  "-l, --listen   Listen mode\n\
               -L, --local-history   Local history\n\
               -u, --udp   UDP mode\n\
               -e, --exec <command>   Execute command when connection recieved\n\
               -p, --port <port>   Local port\n\
               -r, --rshell <shell>   Reverse shell\n".to_string(),
      parent: parent,
      author: "Copyright (c) 2021 Robiot".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
