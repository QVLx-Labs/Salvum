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
 * hashingtools.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;

/*********************************** Hashing tools ***********************************/
pub struct Hashing { event: Event }
impl Eventable for Hashing {
  fn on_init(&self) -> Vec<String> {
    print::println("Hashing tools");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn hashers(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Hashing {
    event: Event {
      name:   name,
      desc:   "Hashing tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** SHA tools ***********************************/
pub struct SHA { event: Event }
impl Eventable for SHA {
  fn on_init(&self) -> Vec<String> {
    //prompt the user for the hash size
    let size: u16 = match prompt_in_event!("SHA>", "Hash size '256' '512': ").parse() {
      Ok(sz) => sz,
      Err(err) => {
        println!("Unable to parse size : {}",err);
        return Vec::new();
      }
    };
    
    //prompt the user for a path to the input file
    let path_input = prompt_in_event!("SHA>","File to hash: ");
    if !Path::new(&path_input).exists() {
      println!("Input file path does not exist.");
      return Vec::new();
    }
    
    //prompt the user for a path to the input file
    //let path_output = prompt_in_event!("SHA>","Output file: ");
    let path_output = "out/sha/output_sha.txt";
    let path_output_clone = path_output.to_string();
    match run_command(Command::new("ext/sha/sha2")
    .args(vec![size.to_string(),path_input,path_output.to_string()])) {
      Ok(out) => out,
      Err(err) => {
        println!("Error: {}",err);
        return Vec::new();
      }
    };

    if Path::new(&path_output_clone).exists() {
      print::print_custom("Hash successful. Output written to out/sha/output_sha.txt\n","bluegreen");
      return Vec::new();
    }
    else {
      print::print_custom("Output file path does not exist. Hash unsuccessful.\n","orange");
      return Vec::new();
    }
  }
  fn on_test(&self) -> TestStatus {
    let mut tests = vec![
      vec!["256","tst/sha/inp/foo1.cpp","tst/sha/out/foo1.sha256","tst/sha/atf/sha_256_foo1"],
      vec!["512","tst/sha/inp/foo1.cpp","tst/sha/out/foo1.sha512","tst/sha/atf/sha_512_foo1"]
    ];
    let command = "ext/sha/sha2";
    for test in &mut tests {
      //get artifact path from the test vector
      let artifact_path_str = match test.pop() {
        Some(string) => string,
        None => {
          debug::print_debug("sha::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };

      //execute sha command
      let _output = match Command::new(command).args(test.clone()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("sha::on_test: failed to execute sha command. {}",err));
          return TestStatus::Failed;
        }
      };

      //read output file to a string
      let output_str = match fs::read_to_string(test[2]) {
        Ok(string) => string,
        Err(err) => {
          debug::print_debug(format!("sha::on_test: failed to read output file {}",err));
          return TestStatus::Failed;
        }
      };

      //read artifact to a string
      let artifact_str = match fs::read_to_string(artifact_path_str) {
        Ok(string) => string,
        Err(err) => {
          debug::print_debug(format!("sha::on_test: failed to read artifact file {}",err));
          return TestStatus::Failed;
        }
      };

      //compare the hash output to the artifact file
      if output_str != artifact_str {
        debug::print_debug("sha::on_test: output does not match artifact");
        return TestStatus::Failed;
      }
    }
    //output always matched artifact, therefore the test passes
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn sha(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SHA {
    event: Event {
      name:   name,
      desc:   "Hash a file with the sha256 or sha512 algorithm".to_string(),
      usage:  "Prompts you for: \
              \n\tHashsize 256 or 512 \
              \n\tInput payload file path \n".to_string(),
      author: "n3wm4n & RustCrypto (github.com/RustCrypto)".to_string(),
      easyrun: false,
      secure: true,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** hash ***********************************/
pub struct Hash { event: Event }
impl Eventable for Hash {
	fn on_init(&self) -> Vec<String> {
    return Vec::new();
	}
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() < 2 { return self.event.usage.clone(); }
		if args.len() > 3 { return self.event.usage.clone(); }
    let mut bitlength = "256";
    let algorithm;
    // Algorithm routing
    match args[0].as_str() {
      "md5" => algorithm = "--md5",
      "blake2" => algorithm = "--b2sum",
      "sha1" => algorithm = "--sha1",
      "sha224" => algorithm = "--sha224",
      "sha3" => {
                  if args.len() == 2 { algorithm = "--sha3-256"; }
                  else {
                    match args[1].as_str() {
                      "224" => algorithm = "--sha3-224",
                      "256" => algorithm = "--sha3-256",
                      "384" => algorithm = "--sha3-384",
                      "512" => algorithm = "--sha3-512",
                           _=> return self.event.usage.clone(), 
                    }
                  }
                }
       "sha256" => algorithm = "--sha256",
       "sha384" => algorithm = "--sha384",
       "sha512" => algorithm = "--sha512",
       "shake128" => {
                  if args.len() == 2 { algorithm = "--shake128"; bitlength = "128"; }
                  else { algorithm = "--shake128"; bitlength = &args[1]; }
                } 
       "shake256" => {
                  if args.len() == 2 { algorithm = "--shake256"; bitlength = "256"; }
                  else { algorithm = "--shake128"; bitlength = &args[1]; }
                } 
             _=> return self.event.usage.clone(),
    }
    let arguments;
    if args.len() == 2 {
      arguments = vec!["hashsum", algorithm, "--bits", bitlength, "--tag", &args[1]];
    }
    else {
      arguments = vec!["hashsum", algorithm, "--bits", bitlength, "--tag", &args[2]];
    }
    if main_info::get_file_redirect() {
			let output = match Command::new("ext/util/coreutils").args(arguments).output() {
				Ok(out) => out,
				Err(_e) => { return String::from(""); }
			};
			log::log("srmdir", &String::from_utf8_lossy(&output.stderr));
			return String::from_utf8_lossy(&output.stdout).to_string(); 
    }
    else {
			let output = match Command::new("ext/util/coreutils").args(arguments).output() {
				Ok(out) => out,
				Err(e) => {
					print::print_custom(&format!("{}\n",e),"orange");
					return String::from("");
				}  
			};
			// Print output
			let out = String::from_utf8_lossy(&output.stdout); 
			print::print_custom(&out,"purple");
			log::log("srmdir", &String::from_utf8_lossy(&output.stderr));
    }
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["--b2sum", "tst/hash/atf/b2sum.txt"],
                     vec!["--md5", "tst/hash/atf/md5.txt"],
                     vec!["--sha1", "tst/hash/atf/sha1.txt"],
                     vec!["--sha224", "tst/hash/atf/sha224.txt"],
                     vec!["--sha3-256", "tst/hash/atf/sha3-256.txt"],
                     vec!["--sha3-384", "tst/hash/atf/sha3-384.txt"],
                     vec!["--sha3-512", "tst/hash/atf/sha3-512.txt"],
                     vec!["--sha384", "tst/hash/atf/sha384.txt"],
                     vec!["--sha512", "tst/hash/atf/sha512.txt"],
                     vec!["--shake128", "tst/hash/atf/shake128.txt"],
                     vec!["--shake256", "tst/hash/atf/shake256.txt"]];

    // For each test in tests
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("ext/util/coreutils").args(vec!["hashsum", test[0], "--bits", "256", "--tag", "tst/hash/inp/hello.txt"]).output());

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[1]));

      // Compare
      if String::from_utf8_lossy(&output.stdout).to_string() != file_str {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn hash(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Hash {
			event: Event {
			name:   name,
			desc:   "Hashing algorithms for all to use.".to_string(),
			usage:  "hash [blake2|md5|sha{1|224|256|3|384|512}|shake{128|256}] <path_to_file>\nhash sha3 [256|384|512] <path_to_file>\n".to_string(),
			parent: parent,
			author: "Rust coreutils community and friends".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** rhash ***********************************/
pub struct Rhash { event: Event }
impl Eventable for Rhash {
	fn on_init(&self) -> Vec<String> {
    return Vec::new();
	}
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() != 2 { return self.event.usage.clone(); }
    let output = match Command::new("ext/rhash/rhash_drvr").args(vec![args[0].clone(),args[1].clone()]).output() {
			Ok(out) => out,
			Err(e) => {
        print::print_custom_uncapped("","orange");
        return format!("{}\n",e);
      }  
		};
		// Print output
    let out = String::from_utf8_lossy(&output.stdout).replace("   "," ");
		print::print_custom(&out,"grey");
    log::log("rhash", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let input_path = "tst/listfiles/inp";
		let artifact_path = "tst/listfiles/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/rhash/rhash_drvr").arg(input_path).output() {
			Ok(o) => o,
				Err(err) => {
					debug::print_debug(format!("Error: {}", err));
					return TestStatus::Failed;
				}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
				Err(err) => {
					debug::print_debug(format!("rhash::on_test: Failed to open the test file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str {
			return TestStatus::Failed;
		}
		return TestStatus::Failed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn rhash(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Rhash {
			event: Event {
			name:   name,
			desc:   "Like the standard mv command but with improvements.".to_string(),
			usage:  "Input a source and destination path and the file or directory will be moved.\n".to_string(),
			parent: parent,
			author: "Aleksey Kravchenko (rhash.admin@gmail.com)".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}
