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
 * cryptography.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;
use std::fs::create_dir;

/*********************************** Cryptography tools ***********************************/
pub struct Cryptography { event: Event }
impl Eventable for Cryptography {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Cryptography");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn cryptography(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Cryptography {
    event: Event {
      name:   name,
      desc:   "Cryptography tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** AES ***********************************/
pub struct AES { event: Event }
impl Eventable for AES {
  fn on_init(&self) -> Vec<String> {
    // Prompt for option
    let option = prompt_in_event!("AES>", "Option: ");
    let mut option_str = "".to_string();
    match &option[..] {
      "d" => {
        option_str.push_str(".dec");
      },
      "e" => {
        option_str.push_str(".enc");
      },
      _ => {
        println!("error invalid input... valid options are : 'd' or 'e'");
        return Vec::new();
      }
    }

    // Prompt for keysize
    let keysize_str = prompt_in_event!("AES>", "Key size: ");
    let keysize: u32 = match keysize_str.parse() {
      Ok(ks) => ks,
      Err(err) => {
        println!("error invalid input... valid key sizes are : 128, 192, 256 : {}", err);
        return Vec::new();
      } 
    };
    if !(keysize == 128 || keysize == 192 || keysize == 256) {
      println!("error invalid input... valid key sizes are : 128, 192, 256"); 
      return Vec::new();
    }
    
    // Prompt for input path
    let path_input_str = prompt_in_event!("AES>", "Input path: ");
    
    // Prompt for key path
    let path_key = prompt_in_event!("AES>", "Key path: ");

    //get the input path stem    
    let path_input = Path::new(&path_input_str);
    let path_input_stem = match path_input.file_stem() {
      Some(stem) => stem,
      None       => {
        print::println("Error : unable to get input file stem.");
        return Vec::new();
      }
    };
    //convert osstr to regular string
    let path_input_stem = match path_input_stem.to_str() {
      Some(string) => string,
      None         => {
        print::println("Error : unable to get input file stem.");
        return Vec::new();
      }
    };
    
    let dt = chrono::offset::Local::now(); //create the DateTime struct
    let datetime: String = str::replace(&dt.to_string()," ","_"); //convert to a string and replace any ' ' with '_'
    let datetime: String = str::replace(&datetime,":","-");
    let path_output = format!("out/aes/{}_{}{}",path_input_stem,&datetime[..16],option_str);

    match run_command(Command::new("ext/aes/aes")
      .args(vec![option, keysize_str, path_input_str, path_key, path_output])) {
        Ok(out) => out,
        Err(err) => {
          println!("Failed to execute the aes process. Error : {}",err);
          return Vec::new();
        }
    };
    //println!("{:?}",output.stdout);
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let mut tests = vec![
      vec!["e","128","tst/aes/myfile","tst/aes/key128","tst/aes/aes_e_key128_myfile.out","tst/aes/aes_e_key128_myfile"],
      vec!["e","192","tst/aes/myfile","tst/aes/key192","tst/aes/aes_e_key192_myfile.out","tst/aes/aes_e_key192_myfile"],
      vec!["e","256","tst/aes/myfile","tst/aes/key256","tst/aes/aes_e_key256_myfile.out","tst/aes/aes_e_key256_myfile"],
      vec!["d","128","tst/aes/myfile128e","tst/aes/key128","tst/aes/aes_d_key128_myfile.out","tst/aes/aes_d_key128_myfile128e"],
      vec!["d","192","tst/aes/myfile192e","tst/aes/key192","tst/aes/aes_d_key192_myfile.out","tst/aes/aes_d_key192_myfile192e"],
      vec!["d","256","tst/aes/myfile256e","tst/aes/key256","tst/aes/aes_d_key256_myfile.out","tst/aes/aes_d_key256_myfile256e"]
    ];

    let tests = &mut tests[..];
    for test in tests {
      let file_path_str = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("aes::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };

      //run the command
      let _output = match Command::new("ext/aes/aes").args(test[..].to_vec()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("aes::on_test: failed to execute aes. {}",err));
          return TestStatus::Failed;
        }
      };

      //pop the output path str from the test vector
      let output_path_str = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("aes::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };

      //read output file
      let output_path = Path::new(output_path_str);
      let output_str = match fs::read(&output_path) {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("aes::on_test: Failed to open the test file. \"{}\" {}",output_path_str, err));
          return TestStatus::Failed;
        }
      };

      //delete the output file
      
      //read test file
      let file_path = Path::new(file_path_str);
      let file_str = match fs::read(file_path) {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("aes::on_test: Failed to open the test file. \"{}\" {}",file_path_str, err));
          return TestStatus::Failed;
        }
      };

      if file_str != output_str {
        debug::print_debug(format!("file\n{:?}\noutput\n{:?}",file_str,output_str));
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn aes(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(AES {
    event: Event {
      name:   name,
      desc:   "AES tool".to_string(),
      usage:  "Prompts you for: \
              \n\tOption. d for decrypt. e for encrypt \
              \n\tKey size. 128, 192, 256 \
              \n\tInput path \
              \n\tKey path\n".to_string(),
      parent: parent,
      author: "n3wm4n & Rust Crypto".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
/*********************************** AesGcm ***********************************/
pub struct AesGcm { event: Event }
impl Eventable for AesGcm {
  fn on_init(&self) -> Vec<String> {
    let option = prompt_in_event!("AES_GCM>", "Option (e)ncrypt or (d)ecrypt: ");
    let mut args: Vec<String> = Vec::new();
    match &option[..] {
      "d" => {
        let input_file = prompt_in_event!("AES_GCM>", "Path to ciphertext file: ");
        let key_file = prompt_in_event!("AES_GCM>", "Path to key file: ");
        let nonce_file = prompt_in_event!("AES_GCM>", "Path to nonce file: ");
        // args.push("-d".to_string());
        args = vec!["d".to_string(),input_file,key_file,nonce_file];
      },
      "e" => {
        let input_file = prompt_in_event!("AES_GCM>", "Path to plaintext file: ");
        args = vec!["e".to_string(),input_file];
      },
      _ => {
        println!("Invalid option. Valid options are : 'd' or 'e'");
      }
    }
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    let command = "ext/aesgcm/aes_gcm";
    if args.len() == 0 {
      return self.event.usage.clone();
    }
    else if args.len() == 4 && (args[0] == "d" || args[0] == "D" || args[0] == "decrypt") {
      //decrypt args len
      let filename = args[1].clone().replace(".","");
      let filename: String = match Path::new(&filename).file_name() {
        Some(string) => {
          string.to_string_lossy().to_string()
        },
        None => {
          return format!("Error creating path from argument.\n");
        }
      };
      let payload_name = format!("out/aesgcm/decrypted/{}.{}",filename,"dec");
      //let mut newargs = args.clone();
      let newargs = vec!["-d".clone(),&args[1],&args[2],&args[3],&payload_name];
      match run_command(Command::new(command).args(newargs)) {
        Ok(_out) => {
          //return format!("Decryption was successful.\n");
        },
        Err(err) => {
          
          return format!("Failed to execute aes_gcm successfully.\n{}",err);
        }
      }
			if Path::new(&payload_name).exists() {
        print::print_custom(&format!("Decrypted to: {}\n",payload_name),"bluegreen");
			  return String::from("");
      }
			else {
				print::print_custom("Key generation was unsuccessful.\n","orange");
			  return String::from("");
			}
    }
    else if args.len() == 2 && (args[0] == "e" || args[0] == "E" || args[0] == "encrypt") {
      // Usage: ./aes_gcm_256 -e <data file> <output key path> <output nonce path> <output payload path>
      let filename = args[1].clone().replace(".","");
      let filename: String = match Path::new(&filename).file_name() {
        Some(string) => {
          string.to_string_lossy().to_string()
        },
        None => {
          return format!("error");
        }
      };
      //create output key path
      let out_key_path = format!("out/aesgcm/keys/{}.key", filename);
      
      //create output nonce path
      let out_nonce_path = format!("out/aesgcm/nonce/{}.nonce", filename);

      //create output payload path
      let out_payload_path = format!("out/aesgcm/encrypted/{}.encrypted", filename);

      let newargs = vec!["-e".to_string(),args[1].clone(),out_key_path.clone(),out_nonce_path.clone(),out_payload_path.clone()];
      match run_command(Command::new(command).args(newargs)) {
        Ok(_/*out*/) => {
          //return format!("Successful encryption\n");
        }
        Err(err) => {
          return format!("{}\n",err);
        }
      }
			if Path::new(&out_payload_path).exists() && Path::new(&out_key_path).exists() && Path::new(&out_nonce_path).exists() {
				print::print_custom(&format!("Key written to: {}\n",out_key_path),"lightbluegreen");
				print::print_custom(&format!("Nonce written to: {}\n",out_nonce_path),"neongreen");
				print::print_custom(&format!("Encrypting to: {}\n",out_payload_path),"bluegreen");
			  return String::from("");
      }
			else {
				print::print_custom("Key generation was unsuccessful.\n","orange");
			  return String::from("");
			}
    }
    else {
      return self.event.usage.clone();
    }
  }
  fn on_test(&self) -> TestStatus {
    let command = "ext/aesgcm/aes_gcm";
    //test encrypt
    let outpath = "tst/aesgcm/out/foo1.enc";
    let test = ["-e","tst/aesgcm/inp/foo1.cpp","tst/aesgcm/out/foo1.key","tst/aesgcm/inp/foo1.nonce",outpath];
    let atfpath = "tst/aesgcm/atf/foo1.enc";
    /*let output = */match Command::new(command).args(test).output() {
      Ok(_) => {},
      Err(err) => {
        debug::print_debug(format!("Encrypt failed to exit aes gcm command. Err: {}",err));
        return TestStatus::Failed;
      }
    };
    let output_bytes = match fs::read(outpath) {
      Ok(bytes) => bytes,
      Err(err) => {
        debug::print_debug(format!("Encrypt ailed to read output path. Err: {}",err));
        return TestStatus::Failed;
      }
    };
    let atf_bytes = match fs::read(atfpath) {
      Ok(bytes) => bytes,
      Err(err) => {
        debug::print_debug(format!("Encrypt failed to read artifact file. Err: {}",err));
        return TestStatus::Failed;
      }
    };
    if output_bytes.len() != atf_bytes.len() {
      debug::print_debug(format!("{:?}\n{:?}",output_bytes,atf_bytes));
      return TestStatus::Failed;
    }

    //test decrypt
    let outpath = "tst/aesgcm/out/foo2.dec";
    let test = ["-d","tst/aesgcm/inp/foo2.enc","tst/aesgcm/inp/foo2.key","tst/aesgcm/inp/foo2.nonce",outpath];
    let atfpath = "tst/aesgcm/atf/foo2.dec";
    /*let output = */match Command::new(command).args(test).output() {
      Ok(_) => {},
      Err(err) => {
        debug::print_debug(format!("Decrypt failed to exit aes gcm command. Err: {}",err));
        return TestStatus::Failed;
      }
    };
    let output_bytes = match fs::read(outpath) {
      Ok(bytes) => bytes,
      Err(err) => {
        debug::print_debug(format!("Decrypt failed to read output path. Err: {}",err));
        return TestStatus::Failed;
      }
    };
    let atf_bytes = match fs::read(atfpath) {
      Ok(bytes) => bytes,
      Err(err) => {
        debug::print_debug(format!("Decrypt failed to read artifact file. Err: {}",err));
        return TestStatus::Failed;
      }
    };
    if output_bytes.len() != atf_bytes.len() {
      debug::print_debug(format!("{:?}\n{:?}",output_bytes,atf_bytes));
      return TestStatus::Failed;
    }

    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn aesgcm(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(AesGcm {
    event: Event {
      name:   name,
      desc:   "AES-GCM encryption tool.".to_string(),
      usage:  "Prompts you for: \
              \n\tOption. d for decrypt. e for encrypt \
              \n\tFile to encrypt or decrypt \
              \n\tKey file if decrypting \
              \n\tNonce file if decrypting\n".to_string(),
      parent: parent,
      author: "Rust Crypto (github.com/RustCrypto)".to_string(),
      easyrun: false,
      secure: true,
      links:  links
    }
  })
}

/*********************************** PGP tools ***********************************/
pub struct PGP { event: Event }
impl Eventable for PGP {
  fn on_init(&self) -> Vec<String> {
    print::println("PGP Tools");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn pgp(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PGP {
    event: Event {
      name:   name,
      desc:   "PGP tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** PGP Key Generation ***********************************/
pub struct PGPKey { event: Event }
impl Eventable for PGPKey {
  fn on_init(&self) -> Vec<String> {
    //prompt the user for their id
    let userid: String = prompt_in_event!("PGP Key Generation>", "Enter a user id: ");

    let path_output = "out/pgp/".to_string();
    let dt = chrono::offset::Local::now(); //create the DateTime struct
    let datetime: String = str::replace(&dt.to_string()," ","_"); //convert to a string and replace any ' ' with '_'
    let datetime: String = str::replace(&datetime,":","-");

    //create a key path string "files/pgp/date-time.key"
    let path_key: String = format!("{}{}.key",path_output.clone(),&datetime[..16]);
    
    //create a cert path string "files/pgp/date-time.cert"
    let path_crt: String = format!("{}{}.cert",path_output.clone(),&datetime[..16]);
    
    //prompt the user for the cipher algorithm option
    let opt_cipher: String = prompt_in_event!("PGP Key Generation>", "Enter a cipher option 'rsa3k' 'rsa4k' 'cv25519': ");
    
    //fork a child process to generate the private key using the sequoia tool
    match run_command(Command::new("ext/util/sq")
    .args(vec!["key","generate","--userid",&userid[..],"--cipher-suite",&opt_cipher[..],"--export",&path_key[..]])) {
      Ok(out) => out,
      Err(err) => {
        println!("Failed to generate the private key\nError: {}",err);
        return Vec::new();
      }
    };

    //fork a child process to generate the public cert using the sequoia tool
    //note that the public cert is extracted from the private key
    match run_command(Command::new("ext/util/sq")
    .args(vec!["key","extract-cert","--output",&path_crt[..],&path_key[..]])) {
      Ok(out) => out,
      Err(err) => {
        println!("Failed to extract the public crt from the private key\nError: {}",err);
        return Vec::new();
      }
    };

    //validate that the key and cert files were generated
    if Path::new(&path_key).exists() && Path::new(&path_crt).exists() {
      println!("Private key generated at: {}\nPublic crt generated at: {}",path_key,path_crt);
    }
    else {
      println!("Key generation was unsuccessful...");
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let atf_path = "tst/pgpkey/atf/atf.txt";
    // Run command
    let output = simple_test_match!(Command::new("ext/util/sq").arg("-V").output());

    // Read file
    let file_str = simple_test_match!(fs::read_to_string(atf_path));

    // Compare
    if String::from_utf8_lossy(&output.stdout) != file_str {
      debug::print_debug(format!("pgpkey::on_test: {}\n{}",file_str, String::from_utf8_lossy(&output.stdout)));
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn pgpkey(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync>{
  Box::new(PGPKey {
    event: Event {
      name:   name,
      desc:   "PGP key generation.".to_string(),
      usage:  "Prompts you for: \
              \n\tUser identifier \
              \n\tCipher option rsa3k, rsa4k, cv25519\n".to_string(),
      parent: parent,
      author: "Sequoia-PGP".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** PGP Encryption ***********************************/
pub struct PGPEncryption { event: Event }
impl Eventable for PGPEncryption {
  fn on_init(&self) -> Vec<String> {
    //prompt the user for a path to their public cert
    let path_cert: String = prompt_in_event!("PGP file encryption>", "Enter a path to your public cert: ");
    
    //prompt the user for a path to the file they wish to encrypt
    let path_input_str: String = prompt_in_event!("PGP file encryption>", "Enter a path to the file you wish to encrypt: ");

    //get the input path stem    
    let path_input = Path::new(&path_input_str);
    let path_input_stem = match path_input.file_stem() {
      Some(stem) => stem,
      None       => {
        print::println("Error : unable to get input file stem.");
        return Vec::new();
      }
    };
    //convert osstr to regular string
    let path_input_stem = match path_input_stem.to_str() {
      Some(string) => string,
      None         => {
        print::println("Error : unable to get input file stem.");
        return Vec::new();
      }
    };

    //get current date time
    let dt = chrono::offset::Local::now(); //create the DateTime struct
    let datetime: String = str::replace(&dt.to_string()," ","_"); //convert to a string and replace any ' ' with '_'
    let datetime: String = str::replace(&datetime,":","-");
    
    //create output path string
    let path_output = format!("out/pgp/{}_{}.enc",path_input_stem,&datetime[..16]);

    //ensure that the user provided valid paths
    if !Path::new(&path_cert).exists() {
      println!("Cert path does not exist: {}",path_cert);
    }
    if !Path::new(&path_input_str).exists() {
      println!("Input path does not exist: {}",path_input_str);
    }

    //fork a child process to encrypt a file using the sequoia tool
    match run_console_command(Command::new("ext/util/sq")
    .args(vec!["encrypt","--recipient-cert",&path_cert[..],"--output",&path_output[..],&path_input_str[..]])) {
      Ok(out) => out,
      Err(err) => {
        println!("Failed to encrypt the input file: {}\nError: {}",path_input_str,err);
        return Vec::new();
      }
    };
    
    //ensure that the encrypted file was created
    if !Path::new(&path_output).exists() {
      println!("Didn't find encrypted file: {}\nRetry the encryption with proper arguements.",path_output);
    }
    else {
      println!("Encrypted file successfully written to: {}",path_output);
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![
      vec!["encrypt","--recipient-cert","tst/pgpencryption/pgpkey_rsa3k_cert",
           "--output","tst/pgpencryption/pgpencryption_rsa3k_myfile.out","tst/pgpencryption/myfile",
           "tst/pgpencryption/pgpencryption_rsa3k_myfile.out","tst/pgpencryption/pgpencryption_rsa3k_myfile"],
      vec!["encrypt","--recipient-cert","tst/pgpencryption/pgpkey_rsa4k_cert",
           "--output","tst/pgpencryption/pgpencryption_rsa4k_myfile.out","tst/pgpencryption/myfile",
           "tst/pgpencryption/pgpencryption_rsa4k_myfile.out","tst/pgpencryption/pgpencryption_rsa4k_myfile"],
      vec!["encrypt","--recipient-cert","tst/pgpencryption/pgpkey_cv25519_cert",
           "--output","tst/pgpencryption/pgpencryption_cv25519_myfile.out","tst/pgpencryption/myfile",
           "tst/pgpencryption/pgpencryption_cv25519_myfile.out","tst/pgpencryption/pgpencryption_cv25519_myfile"]
    ];
    
    for mut test in tests {
      let test_file_path_str = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("pgpencryption::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };
      let test_file_path = Path::new(test_file_path_str);
      
      //get the key path as a string from test vec
      let output_path_str = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("pgpencryption::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };
      let output_path = Path::new(output_path_str);

      //execute command
      let _output = match Command::new("ext/util/sq").args(test[..].to_vec()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("pgpencryption::on_test: failed to execute pgpencryption. {}",err));
          return TestStatus::Failed;
        }
      };

      //read output file into byte vec
      let output_vec = match fs::read(output_path) {
        Ok(vec) => vec,
        Err(err) => {
          debug::print_debug(format!("pgpencryption::on_test: failed to read output file \"{}\" {}",output_path_str,err));
          return TestStatus::Failed;
        }
      };

      //delete output file
      match fs::remove_file(&output_path) {
        Ok(()) => {},
        Err(err) => {
          debug::print_debug(format!("pgpencryption::on_test: unable to delete test file after use. {}", err));
          return TestStatus::Failed;
        }
      }

      //read test file into byte vec
      let test_file_vec = match fs::read(test_file_path) {
        Ok(vec) => vec,
        Err(err) => {
          debug::print_debug(format!("pgpencryption::on_test: failed to read test file \"{}\" {}",test_file_path_str,err));
          return TestStatus::Failed;
        }
      };

      //compare output and test bytes
      if output_vec.len() != test_file_vec.len() {
        //println!("\noutput\n{}\ntestfile\n{}",output_vec.len(),test_file_vec.len());
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn pgpencryption(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync>{
  Box::new(PGPEncryption {
    event: Event {
      name:   name,
      desc:   "PGP Encryption".to_string(),
      usage:  "Prompts you for: \
              \n\tPath to public certificate \
              \n\tPath to the plaintext file to encrypt\n".to_string(),
      parent: parent,
      author: "Sequoia-PGP".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** PGP Decryption ***********************************/
pub struct PGPDecryption { event: Event }
impl Eventable for PGPDecryption {
  fn on_init(&self) -> Vec<String> {
    //prompt the user for a path to their private key
    let path_key: String = prompt_in_event!("PGP file decryption>", "Enter a path to your secret private key: ");
    
    //prompt the user for a path to the file they wish to encrypt
    let path_input_str: String = prompt_in_event!("PGP file decryption>", "Enter a path to the file you wish to decrypt: ");

    //get the input path stem    
    let path_input = Path::new(&path_input_str);
    let path_input_stem = match path_input.file_stem() {
      Some(stem) => stem,
      None       => {
        print::println("Error : unable to get input file stem.");
        return Vec::new();
      }
    };
    //convert osstr to regular string
    let path_input_stem = match path_input_stem.to_str() {
      Some(string) => string,
      None         => {
        print::println("Error : unable to get input file stem.");
        return Vec::new();
      }
    };
    
    //get current date time
    let dt = chrono::offset::Local::now(); //create the DateTime struct
    let datetime: String = str::replace(&dt.to_string()," ","_"); //convert to a string and replace any ' ' with '_'
    let datetime: String = str::replace(&datetime,":","-");
    
    //create output path string
    let path_output = format!("out/pgp/{}_{}.dec",path_input_stem,&datetime[..16]);

    //ensure that the user provided valid paths
    if !Path::new(&path_key).exists() {
      println!("Key path does not exist: {}",path_key);
    }
    if !Path::new(&path_input_str).exists() {
      println!("Input path does not exist: {}",path_input_str);
    }

    //fork a child process to encrypt a file using the sequoia tool
    match run_command(Command::new("ext/util/sq")
    .args(vec!["decrypt","--recipient-key",&path_key[..],"--output",&path_output[..],&path_input_str[..]])) {
      Ok(out) => out,
      Err(err) => {
        println!("Failed to decrypt the input file: {}\nError: {}",path_input_str,err);
        return Vec::new();
      }
    };
    
    //ensure that the decrypted file was created
    if !Path::new(&path_output).exists() {
      println!("Didn't find decrypted file: {}\nRetry the decryption with proper arguements.",path_output);
    }
    else {
      println!("Decrypted file successfully written to: {}",path_output);
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![
      vec!["decrypt","--recipient-key","tst/pgpdecryption/pgpkey_rsa3k_key",
           "--output","tst/pgpdecryption/pgpdecryption_rsa3k_myfile.out","tst/pgpdecryption/myfile_rsa3k",
           "tst/pgpdecryption/pgpdecryption_rsa3k_myfile.out","tst/pgpdecryption/pgpdecryption_rsa3k_myfile"],
      vec!["decrypt","--recipient-key","tst/pgpdecryption/pgpkey_rsa4k_key",
           "--output","tst/pgpdecryption/pgpdecryption_rsa4k_myfile.out","tst/pgpdecryption/myfile_rsa4k",
           "tst/pgpdecryption/pgpdecryption_rsa4k_myfile.out","tst/pgpdecryption/pgpdecryption_rsa4k_myfile"],
      vec!["decrypt","--recipient-key","tst/pgpdecryption/pgpkey_cv25519_key",
           "--output","tst/pgpdecryption/pgpdecryption_cv25519_myfile.out","tst/pgpdecryption/myfile_cv25519",
           "tst/pgpdecryption/pgpdecryption_cv25519_myfile.out","tst/pgpdecryption/pgpdecryption_cv25519_myfile"]
    ];

    for mut test in tests {
      let test_file_path_str = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("pgpdecryption::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };
      let test_file_path = Path::new(test_file_path_str);
      
      //get the key path as a string from test vec
      let output_path_str = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("pgpdecryption::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };
      let output_path = Path::new(output_path_str);

      //execute command
      let _output = match Command::new("ext/util/sq").args(test[..].to_vec()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("pgpdecryption::on_test: failed to execute pgpdecryption. {}",err));
          return TestStatus::Failed;
        }
      };

      //read output file into byte vec
      let output_vec = match fs::read(output_path) {
        Ok(vec) => vec,
        Err(err) => {
          debug::print_debug(format!("pgpdecryption::on_test: failed to read output file \"{}\" {}",output_path_str,err));
          return TestStatus::Failed;
        }
      };

      //delete output file
      match fs::remove_file(&output_path) {
        Ok(()) => {},
        Err(err) => {
          debug::print_debug(format!("pgpdecryption::on_test: unable to delete test file after use. {}", err));
          return TestStatus::Failed;
        }
      }

      //read test file into byte vec
      let test_file_vec = match fs::read(test_file_path) {
        Ok(vec) => vec,
        Err(err) => {
          debug::print_debug(format!("pgpdecryption::on_test: failed to read test file \"{}\" {}",test_file_path_str,err));
          return TestStatus::Failed;
        }
      };

      //compare output and test bytes
      if output_vec.len() != test_file_vec.len() {
        //println!("\noutput\n{}\ntestfile\n{}",output_vec.len(),test_file_vec.len());
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn pgpdecryption(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PGPDecryption {
    event: Event {
      name:   name,
      desc:   "PGP Decryption".to_string(),
      usage:  "Prompts you for: \
              \n\tPath to private key \
              \n\tPath to the ciphertext file to decrypt\n".to_string(),
      parent: parent,
      author: "Sequoia-PGP".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** ChaCha20Poly1305 ***********************************/
pub struct ChaCha20Poly1305 { event: Event }
impl Eventable for ChaCha20Poly1305 {
  fn on_init(&self) -> Vec<String> {
    // Prompt for flag
    let mut flag = prompt_in_event!("ChaCha20Poly1305>", "Flag ((e)ncrypt/(d)ecrypt): ");
    flag.make_ascii_lowercase();

    // Encrypt a file
    if flag.len() >= 1 {
			if &flag[0..1] == "e" || &flag[0..1] == "E" || &flag[0..1] == "encrypt" {
				let input_file = prompt_in_event!("ChaCha20Poly1305>", "File to encrypt: ");

				return vec!["-e".to_string(), input_file];
			// Decrypt a file
			} else if &flag[0..1] == "d" || &flag[0..1] == "D" || &flag[0..1] == "decrypt" {
				let input_file = prompt_in_event!("ChaCha20Poly1305>", "File to decrypt: ");
				let key = prompt_in_event!("ChaCha20Poly1305>", "Key: ");
				let nonce = prompt_in_event!("ChaCha20Poly1305>", "Nonce: ");

				return vec!["-d".to_string(), input_file, key, nonce];
			// Bad
			} else {
				return Vec::new();
			}
    }
    else {
      return Vec::new();
    }
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() != 2 && args.len() != 4 {
      return self.event.usage.to_string();
    }
    if args[0] == "e" || args[0] == "E" || args[0] == "encrypt" { args[0] = "-e".to_string(); }
    if args[0] == "d" || args[0] == "D" || args[0] == "decrypt" { args[0] = "-d".to_string(); }

    // Run command
    let output = simple_match!(run_command(Command::new("ext/chacha20poly1305/chacha20poly1305").args(args)));
    log::log("ChaCha20Poly1305", &String::from_utf8_lossy(&output.stderr));
    print::print_custom("Operation complete. Results are in the out/chacha20poly1305/ directory.\n","bluegreen");
    print::print_custom("Please move your results to the usr/ directory to avoid overwriting them.\n","neongreen");
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let runs = vec![vec!["-e", "tst/chacha20poly1305/inp/test.txt"],
                    vec!["-d", "out/chacha20poly1305/encrypted/encrypted.txt", "out/chacha20poly1305/key/key.txt", "out/chacha20poly1305/nonce/nonce.txt"]];

    // Encrypt the file
    simple_test_match!(Command::new("ext/chacha20poly1305/chacha20poly1305").args(runs[0].to_vec()).output());

    // Decrypt the file
    simple_test_match!(Command::new("ext/chacha20poly1305/chacha20poly1305").args(runs[1].to_vec()).output());

    let original = simple_test_match!(fs::read_to_string("tst/chacha20poly1305/inp/test.txt"));
    let decrypted = simple_test_match!(fs::read_to_string("out/chacha20poly1305/decrypted/decrypted.txt"));

    // Get rid of leftover files
    util::misc::cleanup("out/chacha20poly1305/encrypted/encrypted.txt");
    util::misc::cleanup("out/chacha20poly1305/key/key.txt");
    util::misc::cleanup("out/chacha20poly1305/nonce/nonce.txt");
    util::misc::cleanup("out/chacha20poly1305/decrypted/decrypted.txt");

    if original != decrypted {
      return TestStatus::Failed;
    }

    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn chacha20poly1305(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ChaCha20Poly1305 {
    event: Event {
      name:   name,
      desc:   "Encrypts a file with the ChaCha20Poly1305 cipher.".to_string(),
      usage:  "Prompts you for:\n\
                \tEncrypt: (-e)\n\
                \t-Input file (string)\n\
                \tDecrypt: (-d)\n\
                \t-Input file (string)\n\
                \t-Key file (string)\n\
                \t-Nonce file (string)\n".to_string(),
      author: "Matzr3lla & Rust Crypto (github.com/RustCrypto)".to_string(),
      parent: parent,
      easyrun: false,
      secure: true,
      links:  links
    }
  })
}

/*********************************** CryptoDetector ***********************************/
pub struct CryptoDetector { event: Event }
impl Eventable for CryptoDetector {
  fn on_init(&self) -> Vec<String> {
    //prompt user for path/link to source files
    let mut path_src = prompt_in_event!("CryptoDetector>", "Enter a path/link to the file(s) you wish to scan: ");

    //fetch the current path
    let current_path = match env::current_dir() {
      Ok(dir) => dir,
      Err(err) => {
        println!("Something bad happened. Err: {}",err);
        return Vec::new();
      }
    };
    //convert the current pathbuf to a &str
    let current_path_str = match current_path.to_str() {
      Some(path_str) => path_str,
      None => {
        print::println("Error unable to get current path string.");
        return Vec::new();
      }
    };

    //append the current path with the crypto-detector string
    let working_path = format!("{}/ext/crypto-detector",current_path_str);

    //remove this
    //println!("working_path : {}", working_path);
    
    //append the current path with the new report date path
    let mut out_path_str: String = match current_path.to_str() {
      Some(s) => s.to_string(),
      None => return Vec::new()
    };
    let dt = chrono::offset::Local::now(); //create the DateTime struct
    let datetime: String = str::replace(&dt.to_string()," ","_"); //convert to a string and replace any ' ' with '_'
    let datetime: String = str::replace(&datetime,":","-");
    //out_path_str.push_str("/files/cryptodetector");
    out_path_str.push_str("/out/cryptodetector/report_");
    out_path_str.push_str(&datetime[..10]);

    //create the report path dir if it doesnt already exist
    let out_path = Path::new(&out_path_str);
    if !out_path.exists() {
      let report_dir_res = create_dir(&out_path);
      match report_dir_res {
        Ok(_) => {},
        Err(err) => {
          print::println(&format!("Error creating report directory : {}",err));
        }
      }
    }
    util::misc::reltoabs(&mut path_src); 
    //create args vec
    let args = vec!["scan-for-crypto.py".to_string(), "--output".to_string(), out_path_str.clone(), path_src];

    //execute crypto-detector
    match run_console_command(Command::new("python3").args(args).current_dir(working_path.clone())) {
      Ok(_) => {},
      Err(err) => {
        print::println(&format!("Unable to execute crypto-detector. Error : {}",err));
        return Vec::new();
      }
    }
    
    //check that the output file exists
    if out_path.exists() {
      print::println(&format!("Reports written to: {}",out_path_str));
    }
    else {
      print::println(&format!("ERROR! Report file does not exist : {}",out_path_str));
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let mut test = vec!["python3","-m","unittest","discover","tests","-v","tst/cryptodetector/crypto-detector_python3_-m_unittest_discover_tests_-v"];
    let test_file_path = match test.pop() {
      Some(string) => string,
      None => {
        debug::print_debug("crypto-detector::on_test: invalid test vector");
        return TestStatus::Failed;
      }
    };
    
    //change directory to crypto detector
    let crypto_path_str = "ext/crypto-detector";
    let crypto_path = Path::new(crypto_path_str);

    //execute test command: python3 -m unittest discover tests -v
    //get output from executing command
    let output = match Command::new(&test[0]).args(&test[1..]).current_dir(&crypto_path).output() {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("crypto-detector::on_test: failed to execute test script. {}",err));
        return TestStatus::Failed;
      }
    };
    
    //read output struct into byte vector
    let mut output_vec = output.stderr.clone();
    output_vec.append(&mut output.stdout.clone());

    //convert output bytes to a string
    let output_str: String = match String::from_utf8(output_vec) {
      Ok(out_str) => out_str,
      Err(err) => {
        debug::print_debug(format!("crypto-detector::on_test: failed to read output bytes. {}",err));
        return TestStatus::Failed;
      }
    };
    
    //read in the test file artifact
    let test_file_str = match fs::read_to_string(test_file_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("crypto-detector::on_test: Failed to open the test file. \"{}\" {}", test_file_path, err));
        return TestStatus::Failed;
      }
    };

    //compare test output to test artifact file
      //filter out Ran * tests in *s
      //want the first 26 lines 
    let output_str_filtered = &output_str.split('\n').collect::<Vec<&str>>()[..26];
    let test_file_str_filtered = &test_file_str.split('\n').collect::<Vec<&str>>()[..26];
    
    if output_str_filtered != test_file_str_filtered {
      return TestStatus::Failed;
    }

    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn cryptodetector(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(CryptoDetector {
    event: Event {
      name:   name,
      desc:   "Efficient code parser that detect encryption algorithm use in code.".to_string(),
      usage:  "Prompts you for: \
              \n\tPath to file(s) to scan\n".to_string(),
      parent: parent,
      author: "Wind River Systems".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** Crypto-URI ***********************************/
pub struct Cryptouri { event: Event }
impl Eventable for Cryptouri {
  fn on_init(&self) -> Vec<String> {
    // Prompt for file name
    let s_or_p = prompt_in_event!("URI>", "Generate single key or key pair? (S or P): ");
    if s_or_p.trim().eq("") {
      println!("Need a valid option.");
      return Vec::new();
    }

    // Run command
    let output = match run_console_command(Command::new("ext/cryptouri/crypto_uri").args(vec![s_or_p.trim(),"y"])) {
      Ok(out) => out,
      Err(_) => { return Vec::new(); }
    };

    println!("{}", String::from_utf8_lossy(&output.stdout));
    log::log("uri", &String::from_utf8_lossy(&output.stderr));
    print::print_custom("Keys written to $(SALVUM_ROOT)/out/uri folder","green");
    println!();
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/cryptouri/atf/atf.txt";

    // Run check
    let check = match Command::new("ext/cryptouri/crypto_uri").args(vec!["s","n"]).output() {
	    Ok(out) => out,
	    Err(err) => {
		    debug::print_debug(format!("uri::on_test: failed to execute test script. {}",err));
		    return TestStatus::Failed;
	    }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("uri::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if !String::from_utf8_lossy(&check.stdout).trim().contains(&file_str.trim()) {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn cryptouri(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Cryptouri {
    event: Event {
      name:   name,
      desc:   "Generate URI format Aes256Gcm key or Ed25519 key pair.".to_string(),
      usage:  "Pass an s for single Aes256Gcm key or p for Ed25519 key pair.\n".to_string(),
      parent: parent,
      author: "CryptoURI (cryptouri.org)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** Cocoon ***********************************/
pub struct Cocoon { event: Event }
impl Eventable for Cocoon
 {
  fn on_init(&self) -> Vec<String> {
    // Run command
    match run_console_command(Command::new("ext/cocoon/slm_cocoon").arg("Y")) {
      Ok(_) => (),
      Err(_) => { return Vec::new(); }
    };
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/cocoon/atf/atf.txt";

    // Run check
    let check = match run_bounded_command(Command::new("ext/cocoon/slm_cocoon").arg("N"),false,2) {
	    Ok(out) => out,
	    Err(err) => {
		    debug::print_debug(format!("cocoon::on_test: failed to execute test script. {}",err));
		    return TestStatus::Failed;
	    }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("cocoon::on_test: Failed to open the test file. {}", err));
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
pub fn cocoon(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Cocoon {
    event: Event {
      name:   name,
      desc:   "Simple encryption using Chacha20Poly1305 with PBKDF2.".to_string(),
      usage:  "Pass a key file of your choosing and select prompted options.\n".to_string(),
      parent: parent,
      author: "Alexander Fadeev".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** RSA ***********************************/
pub struct RSA { event: Event }
impl Eventable for RSA {
  fn on_init(&self) -> Vec<String> {
    // Prompt for flag
    let mut flag = prompt_in_event!("RSA>", "Flag ((e)ncrypt/(d)ecrypt): ");
    flag.make_ascii_lowercase();

    // Encrypt a file
    if flag.len() >= 1 {
			if &flag[0..1] == "e" || &flag[0..1] == "E" || &flag[0..1] == "encrypt" {
				let input_file = prompt_in_event!("RSA>", "File to encrypt: ");

				return vec!["e".to_string(), input_file];
			// Decrypt a file
			} else if &flag[0..1] == "d" || &flag[0..1] == "D" || &flag[0..1] == "decrypt" {
				let input_file = prompt_in_event!("RSA>", "File to decrypt: ");
				let key = prompt_in_event!("RSA>", "Private Key: ");

				return vec!["d".to_string(), input_file, key];
			// Bad
			} else {
				return Vec::new();
			}
    }
    else {
      return Vec::new();
    }
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() < 2 || args.len() > 3 {
      return self.event.usage.to_string();
    }
    if args[0] == "e" || args[0] == "E" || args[0] == "encrypt" { args[0] = "-e".to_string(); }
    if args[0] == "d" || args[0] == "D" || args[0] == "decrypt" { args[0] = "-d".to_string(); }

    // Run command
    let output = simple_match!(run_command(Command::new("ext/rsa/rsa").args(args)));
    log::log("RSA", &String::from_utf8_lossy(&output.stderr));
    print::print_custom("Operation complete. Results are in the out/rsa/ directory.\n","bluegreen");
    print::print_custom("Please move your results to the usr/ directory to avoid overwriting them.\n","neongreen");
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let runs = vec![vec!["-e", "tst/rsa/inp/test.txt"],
                    vec!["-d", "out/rsa/encrypted/encrypted.txt", "out/rsa/keys/pem_private_key.txt"]];

    // Encrypt the file
    simple_test_match!(Command::new("ext/rsa/rsa").args(runs[0].to_vec()).output());

    // Decrypt the file
    simple_test_match!(Command::new("ext/rsa/rsa").args(runs[1].to_vec()).output());

    let original = simple_test_match!(fs::read_to_string("tst/rsa/inp/test.txt"));
    let decrypted = simple_test_match!(fs::read_to_string("out/rsa/decrypted/decrypted.txt"));

    // Get rid of leftover files
    util::misc::cleanup("out/rsa/encrypted/encrypted.txt");
    util::misc::cleanup("out/rsa/keys/pem_private_key.txt");
    util::misc::cleanup("out/rsa/keys/pem_public_key.txt");
    util::misc::cleanup("out/rsa/decrypted/decrypted.txt");

    if original != decrypted {
      return TestStatus::Failed;
    }

    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn rsa(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(RSA {
    event: Event {
      name:   name,
      desc:   "Generates a private and public key on the spot for a file that you want encrypted with the RSA cipher".to_string(),
      usage:  "Prompts you for:\n\
                \tEncrypt: e\n\
                \t-Input file (string)\n\
                \tDecrypt: d\n\
                \t-Input file (string)\n\
                \t-Private pem key file (string)\n".to_string(),
      author: "Matzr3lla".to_string(),
      parent: parent,
      easyrun: false,
      secure: true,
      links:  links
    }
  })
}

/*********************************** Svanill ***********************************/
pub struct Svanill { event: Event }
impl Eventable for Svanill {
  fn on_init(&self) -> Vec<String> {
    print::println("Svanill toolsuite");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn svanill(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Svanill {
    event: Event {
      name:   name,
      desc:   "Svanill encryption mechanism".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }   
  })
}

/*********************************** svanill_encrypt ***********************************/
pub struct SvanillEncrypt { event: Event }
impl Eventable for SvanillEncrypt {
	fn on_init(&self) -> Vec<String> {
    print::println("Svanill encryption mechanism");
    let length = prompt_in_event!("Svanill>", "Path to file to encrypt: ");
		return vec![length];
	}
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() != 1 { 
      return self.event.usage.clone(); 
    }
    let mut output_name = String::from("out/svanill/enc_");
    output_name.push_str(&args[0]);
    match Command::new("ext/svanill/svanill").args(vec!["-i".to_string(),args[0].clone(),"-o".to_string(),output_name]).output() {
			Ok(_) => (),
			Err(e) => {
        print::print_custom_uncapped("","orange");
        return format!("{}\n",e);
      }  
		};
		// Print output
		print::print_custom("Directory and contents deleted successfully.\n","brightgreen");
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let input_path = "tst/listfiles/inp";
		let artifact_path = "tst/listfiles/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/listfiles/natls").arg(input_path).output() {
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
					debug::print_debug(format!("natls::on_test: Failed to open the test file. {}", err));
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
pub fn svanill_encrypt(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(SvanillEncrypt {
			event: Event {
			name:   name,
			desc:   "Like the standard mv command but with improvements.".to_string(),
			usage:  "Input a source and destination path and the file or directory will be moved.\n".to_string(),
			parent: parent,
			author: "Riccardo Attilio Galli".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** svanill_decrypt ***********************************/
pub struct SvanillDecrypt { event: Event }
impl Eventable for SvanillDecrypt {
	fn on_init(&self) -> Vec<String> {
    print::println("Svanill encryption mechanism");
    let length = prompt_in_event!("Svanill>", "Path to file to encrypt: ");
		return vec![length];
	}
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() != 1 { 
      return self.event.usage.clone(); 
    }
    let mut output_name = String::from("out/svanill/enc_");
    output_name.push_str(&args[0]);
    match Command::new("ext/svanill/svanill").args(vec!["-i".to_string(),args[0].clone(),"-o".to_string(),output_name]).output() {
			Ok(_) => (),
			Err(e) => {
        print::print_custom_uncapped("","orange");
        return format!("{}\n",e);
      }  
		};
		// Print output
		print::print_custom("Directory and contents deleted successfully.\n","brightgreen");
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let input_path = "tst/listfiles/inp";
		let artifact_path = "tst/listfiles/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/listfiles/natls").arg(input_path).output() {
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
					debug::print_debug(format!("natls::on_test: Failed to open the test file. {}", err));
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
pub fn svanill_decrypt(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(SvanillDecrypt {
			event: Event {
			name:   name,
			desc:   "Like the standard mv command but with improvements.".to_string(),
			usage:  "Input a source and destination path and the file or directory will be moved.\n".to_string(),
			parent: parent,
			author: "Riccardo Attilio Galli".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** svanill_edit ***********************************/
pub struct SvanillEdit { event: Event }
impl Eventable for SvanillEdit {
	fn on_init(&self) -> Vec<String> {
    print::println("Svanill encryption mechanism");
    let length = prompt_in_event!("Svanill>", "Path to file to encrypt: ");
		return vec![length];
	}
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() != 1 { 
      return self.event.usage.clone(); 
    }
    let mut output_name = String::from("out/svanill/enc_");
    output_name.push_str(&args[0]);
    match Command::new("ext/svanill/svanill").args(vec!["-i".to_string(),args[0].clone(),"-o".to_string(),output_name]).output() {
			Ok(_) => (),
			Err(e) => {
        print::print_custom_uncapped("","orange");
        return format!("{}\n",e);
      }  
		};
		// Print output
		print::print_custom("Directory and contents deleted successfully.\n","brightgreen");
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let input_path = "tst/listfiles/inp";
		let artifact_path = "tst/listfiles/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/listfiles/natls").arg(input_path).output() {
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
					debug::print_debug(format!("natls::on_test: Failed to open the test file. {}", err));
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
pub fn svanill_edit(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(SvanillEdit {
			event: Event {
			name:   name,
			desc:   "Like the standard mv command but with improvements.".to_string(),
			usage:  "Input a source and destination path and the file or directory will be moved.\n".to_string(),
			parent: parent,
			author: "Riccardo Attilio Galli".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}
