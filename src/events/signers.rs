/*
 * QVLx Salvum 
 *
 * signingtools.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;

/*********************************** CODE SIGNING ***********************************/
pub struct Signing { event: Event }
impl Eventable for Signing {
  fn on_init(&self) -> Vec<String> {
    print::println("Signing tools");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn signers(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Signing {
    event: Event {
      name:   name,
      desc:   "Signing tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** PGP SIGNING ***********************************/
pub struct PGPSigning { event: Event }
impl Eventable for PGPSigning {
  fn on_init(&self) -> Vec<String> {
    //prompt the user for a path to the input file
    let path_input: String = prompt_in_event!("PGP Signing>", "Enter a path to a file to sign: ");
    if !Path::new(&path_input).exists() {
      println!("Input file path does not exist: {}", path_input);
      return Vec::new();
    }

    //prompt the user for a path in which the signed message will be written to
    let path_output = "out/pgp/pgp_output_signed";
    //prompt the user for a path to their private key used to sign the input
    let path_key: String = prompt_in_event!("PGP Signing>", "Enter a path to a private PGP key: ");
    if !Path::new(&path_key).exists() {
      println!("Key file path does not exist: {}", path_key);
      return Vec::new();
    }

    //fork a child process to sign the message with the private key using the sequoia tool
    match run_command(Command::new("ext/util/sq")
    .args(vec!["-f","sign","--output",&path_output[..],"--signer-key",&path_key[..],&path_input[..]])) {
      Ok(out) => out,
      Err(err) => {
        println!("Failed to sign the input file\nError: {}",err);
        return Vec::new();
      }
    };

    //validate that the signed file was generated
    if Path::new(&path_output).exists() {
      print::print_custom(&format!("Signed file was generated at: {}\n",path_output),"neongreen");
    }
    else {
      print::print_custom("Signing was unsuccessful...\n","orange");
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![
      vec!["sign","--output","tst/pgpsigning/foo1.signed","--signer-key",
           "tst/pgpsigning/pgpkey_rsa3k_key","tst/pgpsigning/foo1.cpp","tst/pgpsigning/pgpsigning_foo1_rsa3k"
      ],
      vec!["sign","--output","tst/pgpsigning/dbase.signed","--signer-key",
           "tst/pgpsigning/pgpkey_rsa3k_key","tst/pgpsigning/dbase.c","tst/pgpsigning/pgpsigning_dbase_rsa3k"
      ]
    ];
    for mut test in tests {
      //get artifact path
      let artifact_path_str = match test.pop() {
        Some(string) => string,
        None => {
          debug::print_debug("pgpsigning::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };

      //execute sq command
      match Command::new("ext/util/sq").args(test.clone()).output() {
        Ok(_) => {}
        Err(err) => {
          debug::print_debug(format!("pgpsigning::on_test: failed to execute sq command {}",err));
          return TestStatus::Failed;
        }
      }
      
      //read output file to a string
      let output_path_str = test[2];
      let output_str = match fs::read_to_string(output_path_str) {
        Ok(string) => string,
        Err(err) => {
          debug::print_debug(format!("pgpsigning::on_test: failed to read output file \"{}\" {}",output_path_str,err));
          return TestStatus::Failed;
        }
      };

      //read artifact file to a string
      let artifact_str = match fs::read_to_string(artifact_path_str) {
        Ok(string) => string,
        Err(err) => {
          debug::print_debug(format!("pgpsigning::on_test: failed to read artifact file \"{}\" {}",artifact_path_str,err));
          return TestStatus::Failed;
        }
      };

      //println!("artifact\n{}\noutput\n{}",artifact_str.len(),output_str.len());
      if artifact_str.len() != output_str.len() {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn pgpsigning(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PGPSigning {
    event: Event {
      name:   name,
      desc:   "PGP signing utility".to_string(),
      usage:  "Prompts you for: \
              \n\tPath to file to sign \
              \n\tPath to signed file output \
              \n\tPath to private pgp key\n".to_string(),
      parent: parent,
      author: "Sequoia-PGP".to_string(),
      easyrun: false,
      secure: true,
      links:  links
    }
  })
}

/*********************************** PGP VERIFYING ***********************************/
pub struct PGPVerifying { event: Event }
impl Eventable for PGPVerifying {
  fn on_init(&self) -> Vec<String> {
    //prompt the user for a path to the input file
    let path_input: String = prompt_in_event!("PGP Verifying>", "Enter a path to a file to verify: ");
    if !Path::new(&path_input).exists() {
      println!("Input file path does not exist: {}", path_input);
      return Vec::new();
    }

    //prompt the user for a path in which the verified message will be written to
    let path_output = "out/pgp/pgp_output_verified";
    //prompt the user for a path to their public cert will be used to verify the input
    let path_cert: String = prompt_in_event!("PGP Verifying>", "Enter a path to a public PGP cert: ");
    if !Path::new(&path_cert).exists() {
      println!("Key file path does not exist: {}", path_cert);
      return Vec::new();
    }

    //fork a child process to verify the message with the public cert using the sequoia tool
    let output = match run_command(Command::new("ext/util/sq")
    .args(vec!["-f","verify","--output","/dev/null","--signer-cert",&path_cert[..],&path_input[..]])) {
      Ok(out) => out,
      Err(err) => {
        println!("Failed to verify the input file\nError: {}",err);
        return Vec::new();
      }
    };

    util::misc::write_file(String::from_utf8_lossy(&output.stderr).to_string(),path_output.to_string());
    print::print_custom(&format!("{}",String::from_utf8_lossy(&output.stderr).to_string()),"neongreen");

    //validate that the verified file was generated
    if Path::new(&path_output).exists() {
      print::print_custom(&format!("Verified file was generated at: {}\n",path_output),"bluegreen");
    }
    else {
      print::print_custom("Verification was unsuccessful...\n","orange");
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![
      vec!["verify","--output","tst/pgpverifying/foo1.verified","--signer-cert",
           "tst/pgpverifying/pgpkey_rsa3k_cert","tst/pgpverifying/foo1.signed","tst/pgpverifying/pgpverifying_foo1_rsa3k"
      ],
      vec!["verify","--output","tst/pgpverifying/dbase.verified","--signer-cert",
           "tst/pgpverifying/pgpkey_rsa3k_cert","tst/pgpverifying/dbase.signed","tst/pgpverifying/pgpverifying_dbase_rsa3k"
      ]
    ];
    for mut test in tests {
      //get artifact path
      let artifact_path_str = match test.pop() {
        Some(string) => string,
        None => {
          debug::print_debug("pgpverifying::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };
      
      //execute sq command
      match Command::new("ext/util/sq").args(test.clone()).output() {
        Ok(_) => {}
        Err(err) => {
          debug::print_debug(format!("pgpverifying::on_test: failed to execute sq command {}",err));
          return TestStatus::Failed;
        }
      };
      
      //read output file to a string
      let output_path_str = test[2];
      let output_str = match fs::read_to_string(output_path_str) {
        Ok(string) => string,
        Err(err) => {
          debug::print_debug(format!("pgpverifying::on_test: failed to read output file \"{}\" {}",output_path_str,err));
          return TestStatus::Failed;
        }
      };

      //read artifact file to a string
      let artifact_str = match fs::read_to_string(artifact_path_str) {
        Ok(string) => string,
        Err(err) => {
          debug::print_debug(format!("pgpsigning::on_test: failed to read artifact file \"{}\" {}",artifact_path_str,err));
          return TestStatus::Failed;
        }
      };
      if artifact_str.len() != output_str.len() {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn pgpverifying(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PGPVerifying {
    event: Event {
      name:   name,
      desc:   "PGP verification utility".to_string(),
      usage:  "Prompts you for: \
              \n\tPath to file to verify \
              \n\tPath to verified file output \
              \n\tPath to public pgp certificate\n".to_string(),
      parent: parent,
      author: "Sequoia-PGP".to_string(),
      easyrun: false,
      secure: true,
      links:  links
    }
  })
}

/*********************************** ASCII_armor ***********************************/
pub struct ASCIIarmor { event: Event }
impl Eventable for ASCIIarmor {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() != 1 { return self.event.usage.to_string();}
    util::misc::reltoabs(&mut args[0]);
    let is_bin = match binaryornot::is_binary(&args[0]) {
      Ok(o) => o,
     Err(_) => return String::from(""),
    };
    if is_bin {
      print::print_custom("Binary detected. Applying armor.\n","purple");
      let output = simple_match!(run_command(Command::new("ext/util/sq").args(vec!["armor".to_string(),args[0].clone()])));
      util::misc::write_file(String::from_utf8_lossy(&output.stdout).to_string(),"out/asciiarmor/armored_file".to_string());
      print::print_custom("Output written to -> out/asciiarmor/armored_file\n","lightbluegreen");
      log::log("ascii_armor", &String::from_utf8_lossy(&output.stderr));
      return String::from("");
    }
    else {
      print::print_custom("ASCII detected. Removing armor.\n","purple");
      let output = simple_match!(run_command(Command::new("ext/util/sq").args(vec!["dearmor".to_string(),args[0].clone()])));
      util::misc::write_file(String::from_utf8_lossy(&output.stdout).to_string(),"out/asciiarmor/dearmored_file".to_string());
      print::print_custom("Output written to -> out/asciiarmor/dearmored_file\n","lightbluegreen");
      log::log("ascii_armor", &String::from_utf8_lossy(&output.stderr));
      return String::from("");
    }
    // Run command
  }
  fn on_test(&self) -> TestStatus {
    let atf_path = "tst/asciiarmor/atf/atf.txt";
		// Run command
		let output = simple_test_match!(Command::new("ext/util/sq").arg("-V").output());

		// Read file
		let file_str = simple_test_match!(fs::read_to_string(atf_path));

		// Compare
    debug::print_debug(format!("asciiarmor::on_test: {}\n{}",file_str, String::from_utf8_lossy(&output.stdout)));

		if String::from_utf8_lossy(&output.stdout) != file_str {
			return TestStatus::Failed;
		}
    return TestStatus::Passed;
  }
}
pub fn asciiarmor(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ASCIIarmor {
    event: Event {
      name:   name,
      desc:   "Obfuscates a binary with ASCII Armor or reverse.".to_string(),
      usage:  "Requires a:\n\
                \tPath to binary (string)\n".to_string(),
      parent: parent,
      author: "Sequoia-PGP".to_string(),
      easyrun: false,
      secure: true,
      links:  links
    }
  })
}

/*********************************** WATCHDOG ***********************************/
pub struct WatchDog { event: Event }
impl Eventable for WatchDog {
  fn on_init(&self) -> Vec<String> {
    // Prompt for key
    let key = prompt_in_event!("WatchDog>", "PEM Key: ");
    
    // Prompt for input file
    let input_file = prompt_in_event!("WatchDog>", "Input File: ");

    // Prompt for output file
    let output_file = prompt_in_event!("WatchDog>", "Output File: ");

    return vec![key, input_file, output_file];
  }
  fn get_event(&self) -> &Event { return &self.event; }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 3 {
      return self.event.usage.to_string();
    }

    let is_bin = match binaryornot::is_binary(&args[1]) {
      Ok(o) => o,
     Err(_) => return String::from(""),
    };
    if !is_bin {
      print::print_custom("File doesn't appear to be a binary. Operation probably won't work.\n","lightorange");
    }

    let arguments = vec!["sha256".to_string(),args[0].clone(),args[0].clone(),args[1].clone(),args[2].clone()];
    
    // Run command
    let output = simple_match!(run_command(Command::new("ext/watchdog/elf-sign").args(arguments)));

    log::log("WatchDog", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["sha256", "tst/watchdog/watchdog_key.pem", "tst/watchdog/watchdog_key.pem", "tst/watchdog/hello-gcc", "tst/watchdog/hello-gcc-new", "tst/watchdog/wd_hello_gcc"]];

    // For each test in tests
    for test in tests {
      util::misc::cleanup("tst/watchdog/hello-gcc-new");

      // Run command
      let output = simple_test_match!(Command::new("ext/watchdog/elf-sign").args(test[..5].to_vec()).output());

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[5]));

      // Compare
      if String::from_utf8_lossy(&output.stdout) != file_str {
        return TestStatus::Failed;
      }

      // Check signed diff
      let output = simple_test_match!(Command::new("diff").args(vec!["tst/watchdog/hello-gcc-signed", "tst/watchdog/hello-gcc-new"]).output());

      // Confirm no differences between the ELFs
      if String::from_utf8_lossy(&output.stdout) != "" {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
}
pub fn watchdog(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(WatchDog {
    event: Event {
      name:   name,
      desc:   "Signs ELF binaries with a pem key.".to_string(),
      usage:  "Requires a:\n\
                \tPem key (string)\n\
                \tInput file (string)\n\
                \tOutput file (string)\n".to_string(),
      parent: parent,
      author: "NUAA WatchDog".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
