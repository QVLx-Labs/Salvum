/*
 * QVLx Salvum 
 *
 * main.rs -> orchestrating program for Salvum
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;

/*********************************** CRC tools ***********************************/
pub struct CRCTools { event: Event }
impl Eventable for CRCTools {
  fn on_init(&self) -> Vec<String> {
    print::println("CRC Tools.");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn cyclicredundancycheckers(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(CRCTools {
    event: Event {
      name:   name,
      desc:   "CRC tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** CRC_Gen ***********************************/
pub struct CRCGen { event: Event }
impl Eventable for CRCGen {
  fn on_init(&self) -> Vec<String> {
    // Prompt for input
    let input_file = prompt_in_event!("CyclicRedundancyCheck>", "Path to input file: ");

    // Prompt for bit length ('16', '32', or '64'):
    let bit_length = prompt_in_event!("CyclicRedundancyCheck>", "Bit length ('16' | '32' | '64'): ");
    return vec![input_file,bit_length];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 2 {
      return self.event.usage.clone();
    }
    // Run command
    let output = match run_command(Command::new("ext/crcgen/crc_tool").args(vec![args[0].clone(),args[1].clone()])) {
      Ok(out) => out,
      Err(err) => {
        print::print_custom(&format!("Error: {}", err),"orange");
        return String::from("");
      }
    };

    // Print output
    print::print_custom(&format!("{}", String::from_utf8_lossy(&output.stdout)),"lightbluegreen");
    log::log("CRC", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/crcgen/atf/atf.txt";

    // Run command
    let check = match Command::new("ext/crcgen/crc_tool").output() {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("crcgen::on_test: Failed to open the test file. {}", err));
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

pub fn crcgen(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync>{
  Box::new(CRCGen {
    event: Event {
      name:   name,
      desc:   "CRC Generator".to_string(),
      usage:  "Requires a:\n\
                \tFile path (string)\n\
                \tBit length (16, 32, 64)\n".to_string(),
      author: "$t@$h".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
