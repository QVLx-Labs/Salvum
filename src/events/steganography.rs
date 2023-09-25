/*
 * QVLx Salvum 
 *
 * steganography.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
 use crate::events::*;

 
/*********************************** Steganography ***********************************/
pub struct Steganography { event: Event }
impl Eventable for Steganography {
  fn on_init(&self) -> Vec<String> {
    print::println("Steganography Detection");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn steganography(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Steganography {
    event: Event {
      name:   name,
      desc:   "Steganography".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** STEGSNOW ***********************************/
pub struct StegSnow { event: Event }
impl Eventable for StegSnow {
  fn on_init(&self) -> Vec<String> {
    let option: String = prompt_in_event!("StegSnow>", "Conceal or extract? (c/e): ");
    if option == "c" {
      //prompt user for files to conceal
      let path_input_str: String = prompt_in_event!("StegSnow>", "File to conceal a message inside: ");
      let path_inject_str: String = prompt_in_event!("StegSnow>", "File with message to inject: ");
      
      //format the output path string
      let path_input = Path::new(&path_input_str);
      let file_name = match path_input.file_name() {
        Some(osstr) => osstr,
        None => {
          print::println(&format!("Invalid file path"));
          return Vec::new();
        }
      };
      let file_name = match file_name.to_str() {
        Some(string) => string,
        None => {
          print::println("Unable to read file name to a string");
          return Vec::new();
        }
      };
      let path_output_str = format!("out/stegsnow/{}.injected",file_name);
      print::println(&format!("Concealed file will be written to: {}", path_output_str));
      //execute stegsnow
      let args = vec!["-f".to_string(),path_inject_str,path_input_str,path_output_str];
      return args;
    }
    else if option == "e" {
      let path_input_str: String = prompt_in_event!("StegSnow>", "File to extract a concealed message from: ");

      //format the output path string
      let path_input = Path::new(&path_input_str);
      let file_name = match path_input.file_name() {
        Some(osstr) => osstr,
        None => {
          print::println(&format!("Invalid file path"));
          return Vec::new();
        }
      };
      let file_name = match file_name.to_str() {
        Some(string) => string,
        None => {
          print::println("Unable to read file name to a string");
          return Vec::new();
        }
      };
      let path_output_str = format!("out/stegsnow/{}.extracted",file_name);
      print::println(&format!("Extracted message will be written to: {}", path_output_str));
      //execute stegsnow
      let args: Vec<String> = vec![path_input_str,path_output_str];
      return args;
    }
    else {
      print::println("Invalid option");
      return Vec::new();
    }
  }
  fn on_run(&self, args: Vec<String>) -> String {
    let command = "ext/stegsnow/stegsnow";
    let stegsnow_output = match run_command(Command::new(command).args(args)) {
      Ok(output) => output,
      Err(err) => {
        return format!("failed to execute stegsnow. {}\n", err);
      }
    };

    let mut output_str: String = String::from_utf8_lossy(&stegsnow_output.stdout).to_string();
    output_str.push_str(&String::from_utf8_lossy(&stegsnow_output.stderr).to_string());
    return output_str;
  }
  fn on_test(&self) -> TestStatus {
    let injected_atf_path = "tst/stegsnow/atf/poem.injected";
    let injected_output_path = "tst/stegsnow/out/poem.injected";

    let extracted_atf_path = "tst/stegsnow/atf/poem.extracted";
    let extracted_output_path = "tst/stegsnow/out/poem.extracted";
    let message_path = "tst/stegsnow/inp/message.txt";
    let payload_path = "tst/stegsnow/inp/poem.txt";
  
    let command = "ext/stegsnow/stegsnow";

    /*********** inject test ***********/

    //conceal a message into a file
    let args = vec!["-f",message_path,payload_path,injected_output_path];
    let _stegsnow_output = match Command::new(command).args(args).output() {
      Ok(output) => output,
      Err(err) => {
        debug::print_debug(&format!("stegsnow::on_test: failed to execute stegsnow. {}",err));
        return TestStatus::Failed;
      }
    };
    
    //read artifact file into byte vec
    let injected_atf_bytes = match fs::read(injected_atf_path) {
      Ok(bytes) => bytes,
      Err(err) => {
        debug::print_debug(&format!("stegsnow::on_test: failed to read atf. {}", err));
        return TestStatus::Failed;
      }
    };

    //read output file into byte vec
    let injected_output_bytes = match fs::read(injected_output_path) {
      Ok(bytes) => bytes,
      Err(err) => {
        debug::print_debug(&format!("stegsnow::on_test: failed to read output file. {}", err));
        return TestStatus::Failed;
      }
    };

    //check the output file against the artifact file
    if !injected_atf_bytes.eq(&injected_output_bytes) {
      debug::print_debug("stegsnow::on_test: output file doesnt match artifact.");
      return TestStatus::Failed;
    }

    /*********** extract test ***********/

    //extract a message from a file
    let args = vec![injected_output_path,extracted_output_path];
    let _stegsnow_output = match Command::new(command).args(args).output() {
      Ok(output) => output,
      Err(err) => {
        debug::print_debug(&format!("stegsnow::on_test: failed to execute stegsnow. {}",err));
        return TestStatus::Failed;
      }
    };

    //read artifact file into byte vec
    let extracted_atf_bytes = match fs::read(extracted_atf_path) {
      Ok(bytes) => bytes,
      Err(err) => {
        debug::print_debug(&format!("stegsnow::on_test: failed to read atf. {}", err));
        return TestStatus::Failed;
      }
    };

    //read output file into byte vec
    let extracted_output_bytes = match fs::read(extracted_output_path) {
      Ok(bytes) => bytes,
      Err(err) => {
        debug::print_debug(&format!("stegsnow::on_test: failed to read output file. {}", err));
        return TestStatus::Failed;
      }
    };

    //check the output file against the artifact file
    if !extracted_atf_bytes.eq(&extracted_output_bytes) {
      debug::print_debug("stegsnow::on_test: output file doesnt match artifact.");
      return TestStatus::Failed;
    }

    //inject and extract were successful
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn stegsnow(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(StegSnow {
    event: Event {
      name:   name,
      desc:   "Conceals messages in text files by appending tabs and spaces to lines".to_string(),
      usage:  "Prompts you for: \
                \n\tConceal or extract option (c/e) \
                \n\tPath to a file in which a message will be concealed inside \
                \n\tPath to a file with a secrete message\n".to_string(),
      parent: parent,
      author: "darkside.com.au/snow/".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** StegPDF ***********************************/
pub struct StegPDF { event: Event }
impl Eventable for StegPDF {
  fn on_init(&self) -> Vec<String> {
    print::println("Steganography pdf tools");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn stegpdf(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(StegPDF {
    event: Event {
      name:   name,
      desc:   "StegPDF".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** Pdfcop ***********************************/
pub struct Pdfcop { event: Event }
impl Eventable for Pdfcop {
  fn on_init(&self) -> Vec<String> {
    let path_input_str: String = prompt_in_event!("Pdfcop>", "PDF to scan: ");
    let path_input = Path::new(&path_input_str);
    let file_name = match path_input.file_name() {
      Some(osstr) => osstr,
      None => {
        print::println(&format!("Invalid file path"));
        return Vec::new();
      }
    };
    //let file_name = file_name.to_string_lossy();
    let file_name = match file_name.to_str() {
      Some(string) => string,
      None => {
        print::println("unable to read file name to a string");
        return Vec::new();
      }
    };
    let output_path_str = format!("out/origami/{}.check",file_name);
    let args = vec!["-o".to_string(),output_path_str, path_input_str];
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    let command = "ext/origami/pdfcop";
    let output_path = args[1].clone();
    let pdfcop_output = match run_command(Command::new(command).args(args)) {
      Ok(output) => {
        output
      },
      Err(err) => {
        return format!("failed to execute pdfcop. {}\n",err);
      }
    };
    print::println(&format!("Output written to {}", output_path));
    return String::from_utf8_lossy(&pdfcop_output.stdout).to_string();
    //print::println(&String::from_utf8_lossy(&pdfcop_output.stderr));
    
  }
  fn on_test(&self) -> TestStatus {
    let atf_path_str = "tst/pdfcop/atf/pdfcop_manual";
    let input_path_str = "tst/pdfcop/inp/manual.pdf";
    let command = "ext/origami/pdfcop";
    let args = vec![input_path_str,"-n"];
    //execute pdfcop
    let pdfcop_output = match Command::new(command).args(args).output() {
      Ok(output) => output,
      Err(err) => {
        debug::print_debug(&format!("pdfcop::on_test: failed to execute pdfcop. {}", err));
        return TestStatus::Failed;
      }
    };

    //read output to a string
    let pdfcop_output_str = String::from_utf8_lossy(&pdfcop_output.stdout);

    //read atf file to a string
    let atf_file_str = match fs::read_to_string(atf_path_str) {
      Ok(string) => string,
      Err(err) => {
        debug::print_debug(&format!("pdfcop::on_test: failed to read atf to a string. {}", err));
        return TestStatus::Failed;
      }
    };
    let atf_file_str = atf_file_str.trim();
  
    if !pdfcop_output_str.contains(&atf_file_str) {
      debug::print_debug("test output doesn't contain the artifact string");
      return TestStatus::Failed;
    }

    //test passed
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn pdfcop(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Pdfcop {
    event: Event {
      name:   name,
      desc:   "Pdfcop - a tool that scans PDF documents for malicious structures.".to_string(),
      usage:  "Prompts you for: \
                \n\tPath to a pdf file to scan\n".to_string(),
      parent: parent,
      author: "Guillaume (github.com/gdelugre)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** PDFDECOMPRESS ***********************************/
pub struct PdfDecompress { event: Event }
impl Eventable for PdfDecompress {
  fn on_init(&self) -> Vec<String> {
    let path_input_str: String = prompt_in_event!("PdfDecompress>", "PDF to decompress: ");
    let path_input = Path::new(&path_input_str);
    let file_name = match path_input.file_name() {
      Some(osstr) => osstr,
      None => {
        print::println(&format!("Invalid file path"));
        return Vec::new();
      }
    };
    let file_name = match file_name.to_str() {
      Some(string) => string,
      None => {
        print::println("unable to read file name to a string");
        return Vec::new();
      }
    };
    let output_path_str = format!("out/origami/{}.decom",file_name);
    let args: Vec<String> = vec![path_input_str,"-o".to_string(),output_path_str];
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    let command = "ext/origami/pdfdecompress";
    let output_path = args[2].clone();
    let pdfcop_output = match run_command(Command::new(command).args(args)) {
      Ok(output) => {
        output
      },
      Err(err) => {
        return format!("failed to execute pdfdecompress. {}\n",err);
      }
    };
    print::println(&format!("Output written to {}", output_path));
    return String::from_utf8_lossy(&pdfcop_output.stdout).to_string();
    //print::println(&String::from_utf8_lossy(&pdfcop_output.stderr));
  }
  fn on_test(&self) -> TestStatus {
    let atf_path_str = "tst/pdfdecompress/atf/pdfdecompress_manual";
    let input_path_str = "tst/pdfdecompress/inp/manual.pdf";
    let command = "ext/origami/pdfdecompress";
    let args = vec![input_path_str];

    //execute pdfdecompress
    let pdfdecompress_output = match Command::new(command).args(args).output() {
      Ok(output) => output,
      Err(err) => {
        debug::print_debug(&format!("pdfdecompress::on_test: failed to execute pdfdecompress. {}", err));
        return TestStatus::Failed;
      }
    };

    let pdfdecompress_output = pdfdecompress_output.stdout.clone();

    //read atf file to a string
    let atf_bytes = match fs::read(atf_path_str) {
      Ok(bytes) => bytes,
      Err(err) => {
        debug::print_debug(&format!("pdfdecompress::on_test: failed to read atf. {}", err));
        return TestStatus::Failed;
      }
    };
  
    if !pdfdecompress_output.eq(&atf_bytes) {
      debug::print_debug("pdfdecompress::on_test: test output doesn't match the artifact");
      return TestStatus::Failed;
    }

    //test passed
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn pdfdecompress(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PdfDecompress {
    event: Event {
      name:   name,
      desc:   "PdfDecompress - uncompresses all of the binary streams of a pdf document".to_string(),
      usage:  "Prompts you for: \
                \n\tPath to a pdf file to decompress\n".to_string(),
      parent: parent,
      author: "Guillaume (github.com/gdelugre)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** PDFDECRYPT ***********************************/
pub struct PdfDecrypt { event: Event }
impl Eventable for PdfDecrypt {
  fn on_init(&self) -> Vec<String> {
    let path_input_str: String = prompt_in_event!("PdfDecrypt>", "PDF to decrypt: ");
    let password_str: String = prompt_in_event!("PdfDecrypt>", "Password of the document: ");
    let path_input = Path::new(&path_input_str);
    let file_name = match path_input.file_name() {
      Some(osstr) => osstr,
      None => {
        print::println(&format!("Invalid file path"));
        return Vec::new();
      }
    };
    let file_name = match file_name.to_str() {
      Some(string) => string,
      None => {
        print::println("unable to read file name to a string");
        return Vec::new();
      }
    };
    let output_path_str = format!("out/origami/{}.decrypt",file_name);
    //println!("{}",file_name);
    let args: Vec<String> = vec![path_input_str,"-p".to_string(),password_str,"-o".to_string(),output_path_str];
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    let command = "ext/origami/pdfdecrypt";
    let output_path = args[4].clone();
    let pdfcop_output = match run_command(Command::new(command).args(args)) {
      Ok(output) => {
        output
      },
      Err(err) => {
        return format!("failed to execute pdfdecrypt. {}\n",err);
      }
    };
    print::println(&format!("Output written to {}", output_path));
    return String::from_utf8_lossy(&pdfcop_output.stdout).to_string();
    //print::println(&String::from_utf8_lossy(&pdfcop_output.stderr));
  }
  fn on_test(&self) -> TestStatus {
    let atf_path_str = "tst/pdfdecrypt/atf/pdfdecrypt_manual";
    let input_path_str = "tst/pdfdecrypt/inp/manual.enc";
    let command = "ext/origami/pdfdecrypt";
    let password = "password";
    let args = vec![input_path_str,"-p",password];

    //execute pdfdecompress
    let pdfdecrypt_output = match Command::new(command).args(args).output() {
      Ok(output) => output,
      Err(err) => {
        debug::print_debug(&format!("pdfdecrypt::on_test: failed to execute pdfdecrypt. {}", err));
        return TestStatus::Failed;
      }
    };

    let pdfdecrypt_output = pdfdecrypt_output.stdout.clone();

    //read atf file to a string
    let atf_bytes = match fs::read(atf_path_str) {
      Ok(bytes) => bytes,
      Err(err) => {
        debug::print_debug(&format!("pdfdecrypt::on_test: failed to read atf. {}", err));
        return TestStatus::Failed;
      }
    };
  
    if !pdfdecrypt_output.eq(&atf_bytes) {
      debug::print_debug("pdfdecrypt::on_test: test output doesn't match the artifact");
      return TestStatus::Failed;
    }

    //test passed
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn pdfdecrypt(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PdfDecrypt {
    event: Event {
      name:   name,
      desc:   "PdfDecrypt - decrypts an encrypted pdf file".to_string(),
      usage:  "Prompts you for: \
                \n\tPath to a pdf file to decrypt \
                \n\tPassword used to unlock the pdf file \n".to_string(),
      parent: parent,
      author: "Guillaume (github.com/gdelugre)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** PDFENCRYPT ***********************************/
pub struct PdfEncrypt { event: Event }
impl Eventable for PdfEncrypt {
  fn on_init(&self) -> Vec<String> {
    let path_input_str: String = prompt_in_event!("PdfEncrypt>", "PDF to encrypt: ");
    let password_str: String = prompt_in_event!("PdfEncrypt>", "Password of the document: ");
    let path_input = Path::new(&path_input_str);
    let file_name = match path_input.file_name() {
      Some(osstr) => osstr,
      None => {
        print::println(&format!("Invalid file path"));
        return Vec::new();
      }
    };
    let file_name = match file_name.to_str() {
      Some(string) => string,
      None => {
        print::println("unable to read file name to a string");
        return Vec::new();
      }
    };
    let output_path_str = format!("out/origami/{}.enc",file_name);
    let args: Vec<String> = vec![path_input_str,"-p".to_string(),password_str,"--hardened".to_string(),"-o".to_string(),output_path_str];
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    let command = "ext/origami/pdfencrypt";
    let output_path = args[5].clone();
    let pdfcop_output = match run_command(Command::new(command).args(args)) {
      Ok(output) => {
        output
      },
      Err(err) => {
        return format!("failed to execute pdfencrypt. {}\n",err);
      }
    };
    print::println(&format!("Output written to {}", output_path));
    return String::from_utf8_lossy(&pdfcop_output.stdout).to_string();
    //print::println(&String::from_utf8_lossy(&pdfcop_output.stderr));
  }
  fn on_test(&self) -> TestStatus {
    let atf_path_str = "tst/pdfencrypt/atf/pdfencrypt_manual";
    let input_path_str = "tst/pdfencrypt/inp/manual.pdf";
    let command = "ext/origami/pdfencrypt";
    let password = "password";
    let args = vec![input_path_str,"-p",password];

    //execute pdfdecompress
    let pdfencrypt_output = match Command::new(command).args(args).output() {
      Ok(output) => output,
      Err(err) => {
        debug::print_debug(&format!("pdfencrypt::on_test: failed to execute pdfencrypt. {}", err));
        return TestStatus::Failed;
      }
    };

    if !pdfencrypt_output.status.success() {
      debug::print_debug("pdfencrypt::on_test: failed to execute pdfencrypt non zero exit code.");
      return TestStatus::Failed;
    }

    //read atf file to a string
    let _atf_bytes = match fs::read(atf_path_str) {
      Ok(bytes) => bytes,
      Err(err) => {
        debug::print_debug(&format!("pdfencrypt::on_test: failed to read atf. {}", err));
        return TestStatus::Failed;
      }
    };

    //test passed
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn pdfencrypt(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PdfEncrypt {
    event: Event {
      name:   name,
      desc:   "PdfEncrypt - encrypts a pdf file".to_string(),
      usage:  "Prompts you for: \
                \n\tPath to a pdf file to encrypt \
                \n\tPassword used to lock/unlock the pdf file \n".to_string(),
      parent: parent,
      author: "Guillaume (github.com/gdelugre)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** PDFEXTRACT ***********************************/
pub struct PdfExtract { event: Event }
impl Eventable for PdfExtract {
  fn on_init(&self) -> Vec<String> {
    let path_input_str: String = prompt_in_event!("PdfExtract>", "PDF to extract: ");
    let path_input = Path::new(&path_input_str);
    let file_name = match path_input.file_name() {
      Some(osstr) => osstr,
      None => {
        print::println(&format!("Invalid file path"));
        return Vec::new();
      }
    };
    let file_name = match file_name.to_str() {
      Some(string) => string,
      None => {
        print::println("unable to read file name to a string");
        return Vec::new();
      }
    };
    let output_path_str = format!("out/origami/{}.extract",file_name);
    let args: Vec<String> = vec![path_input_str,"-d".to_string(),output_path_str,"-s".to_string(),"-a".to_string(),"-f".to_string(),"-f".to_string(),"-j".to_string(),"-m".to_string(),"-i".to_string()];
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    let command = "ext/origami/pdfextract";
    let output_path = args[2].clone();
    let pdfcop_output = match run_command(Command::new(command).args(args)) {
      Ok(output) => {
        output
      },
      Err(err) => {
        return format!("failed to execute PdfExtract. {}\n",err);
      }
    };
    print::println(&format!("Output written to {}", output_path));
    return String::from_utf8_lossy(&pdfcop_output.stdout).to_string();
    //print::println(&String::from_utf8_lossy(&pdfcop_output.stderr));
    
  }
  fn on_test(&self) -> TestStatus {
    let atf_path_str = "tst/pdfextract/atf/pdfextract_manual";
    let input_path_str = "tst/pdfextract/inp/manual.pdf";
    let command = "ext/origami/pdfextract";
    //let output_path = "manual.dump";
    //let password = "password";
    let args = vec![input_path_str];

    //execute pdfdecompress
    let pdfextract_output = match Command::new(command).args(args).output() {
      Ok(output) => output,
      Err(err) => {
        debug::print_debug(&format!("pdfextract::on_test: failed to execute pdfextract. {}", err));
        return TestStatus::Failed;
      }
    };

    if !pdfextract_output.status.success() {
      debug::print_debug("pdfextract::on_test: failed to execute pdfextract non zero exit code.");
      return TestStatus::Failed;
    }
    
    let pdfextract_output = pdfextract_output.stdout.clone();

    //read atf file to a string
    let atf_bytes = match fs::read(atf_path_str) {
      Ok(bytes) => bytes,
      Err(err) => {
        debug::print_debug(&format!("pdfextract::on_test: failed to read atf. {}", err));
        return TestStatus::Failed;
      }
    };
  
    //
    if !pdfextract_output.eq(&atf_bytes) {
      debug::print_debug("pdfextract::on_test: test output doesn't match the artifact");
      return TestStatus::Failed;
    }

    //clean up manual.dump
    match fs::remove_dir_all("manual.dump") {
      Ok(()) => {},
      Err(_err) => {
        debug::print_debug("pdfextract::on_test: failed to remove .dump dir");
        return TestStatus::Failed;
      }
    };

    //test passed
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn pdfextract(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PdfExtract {
    event: Event {
      name:   name,
      desc:   "PdfExtract - extracts various data out of a document such as: streams, scripts, images, fonts, metadata, and attachments.".to_string(),
      usage:  "Prompts you for: \
                \n\tPath to a pdf to extract\n".to_string(),
      parent: parent,
      author: "Guillaume (github.com/gdelugre)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** PDFMETADATA ***********************************/
pub struct PdfMetadata { event: Event }
impl Eventable for PdfMetadata {
  fn on_init(&self) -> Vec<String> {
    let path_input_str: String = prompt_in_event!("PdfMetadata>", "PDF to obtain metadata about: ");
    let args: Vec<String> = vec![path_input_str,"-i".to_string(),"-x".to_string()];
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    let command = "ext/origami/pdfmetadata";
    //let output_path = args[2];
    let pdfcop_output = match run_command(Command::new(command).args(args)) {
      Ok(output) => {
        output
      },
      Err(err) => {
        return format!("failed to execute PdfMetadata. {}\n",err);
      }
    };
    return String::from_utf8_lossy(&pdfcop_output.stdout).to_string();
    //print::println(&String::from_utf8_lossy(&pdfcop_output.stderr));
    //print::println(&format!("Output written to {}", output_path));
  }
  fn on_test(&self) -> TestStatus {
    let atf_path_str = "tst/pdfmetadata/atf/pdfmetadata_manual";
    let input_path_str = "tst/pdfmetadata/inp/manual.pdf";
    let command = "ext/origami/pdfmetadata";
    let args = vec![input_path_str,"-n"];

    //execute pdfdecompress
    let pdfmetadata_output = match Command::new(command).args(args).output() {
      Ok(output) => output,
      Err(err) => {
        debug::print_debug(&format!("pdfmetadata::on_test: failed to execute pdfmetadata. {}", err));
        return TestStatus::Failed;
      }
    };

    if !pdfmetadata_output.status.success() {
      debug::print_debug("pdfmetadata::on_test: failed to execute pdfmetadata non zero exit code.");
      return TestStatus::Failed;
    }
    
    let pdfmetadata_output = pdfmetadata_output.stdout.clone();

    //read atf file to a string
    let atf_bytes = match fs::read(atf_path_str) {
      Ok(bytes) => bytes,
      Err(err) => {
        debug::print_debug(&format!("pdfmetadata::on_test: failed to read atf. {}", err));
        return TestStatus::Failed;
      }
    };
  
    //
    if !pdfmetadata_output.eq(&atf_bytes) {
      debug::print_debug("pdfmetadata::on_test: test output doesn't match the artifact");
      return TestStatus::Failed;
    }

    //test passed
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn pdfmetadata(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PdfMetadata {
    event: Event {
      name:   name,
      desc:   "PdfMetadata - Prints out the metadata contained in a PDF document.".to_string(),
      usage:  "Prompts you for: \
                \n\tPath to a pdf to obtain metadata about\n".to_string(),
      parent: parent,
      author: "Guillaume (github.com/gdelugre)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** PDFATTACH ***********************************/
pub struct PdfAttach { event: Event }
impl Eventable for PdfAttach {
  fn on_init(&self) -> Vec<String> {
    let path_input_str: String = prompt_in_event!("PdfAttach>", "PDF to modify: ");
    let path_attach_str: String = prompt_in_event!("PdfAttach>", "file to attach: ");
    let path_input = Path::new(&path_input_str);
    let file_name = match path_input.file_name() {
      Some(osstr) => osstr,
      None => {
        print::println(&format!("Invalid file path"));
        return Vec::new();
      }
    };
    let file_name = match file_name.to_str() {
      Some(string) => string,
      None => {
        print::println("unable to read file name to a string");
        return Vec::new();
      }
    };
    let output_path_str = format!("out/origami/{}.attach",file_name);
    let args: Vec<String> = vec![path_input_str,output_path_str,path_attach_str];
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    let command = "ext/origami/pdfattach";
    //let output_path = args[1];
    let pdfcop_output = match run_command(Command::new(command).args(args)) {
      Ok(output) => {
        output
      },
      Err(err) => {
        return format!("failed to execute PdfMetadata. {}\n",err);
      }
    };
    return String::from_utf8_lossy(&pdfcop_output.stdout).to_string();
    //print::println(&String::from_utf8_lossy(&pdfcop_output.stderr));
    //print::println(&format!("Output written to {}", output_path));
  }
  fn on_test(&self) -> TestStatus {
    let atf_path_str = "tst/pdfattach/atf/pdfattach_helloworld";
    let input_path_str = "tst/pdfattach/inp/manual.pdf";
    let output_path_str = "tst/pdfattach/out/pdfattach_helloworld";
    let attach_path_str = "tst/pdfattach/inp/helloworld";
    let command = "ext/origami/pdfattach";
    let args = vec![input_path_str,output_path_str,attach_path_str];

    //execute pdfdecompress
    let pdfattach_output = match Command::new(command).args(args).output() {
      Ok(output) => output,
      Err(err) => {
        debug::print_debug(&format!("pdfattach::on_test: failed to execute pdfattach. {}", err));
        return TestStatus::Failed;
      }
    };

    if !pdfattach_output.status.success() {
      debug::print_debug("pdfattach::on_test: failed to execute pdfattach non zero exit code.");
      return TestStatus::Failed;
    }

    //read atf file to a string
    let output_bytes = match fs::read(output_path_str) {
      Ok(bytes) => bytes,
      Err(err) => {
        debug::print_debug(&format!("pdfattach::on_test: failed to read output file. {}", err));
        return TestStatus::Failed;
      }
    };

    //read atf file to a string
    let atf_bytes = match fs::read(atf_path_str) {
      Ok(bytes) => bytes,
      Err(err) => {
        debug::print_debug(&format!("pdfattach::on_test: failed to read atf. {}", err));
        return TestStatus::Failed;
      }
    };
  
    if !output_bytes.eq(&atf_bytes) {
      debug::print_debug("pdfattach::on_test: output file doesnt match artifact file.");
        return TestStatus::Failed;
    }
    //test passed
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn pdfattach(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PdfAttach {
    event: Event {
      name:   name,
      desc:   "PdfAttach - a tool used to attach files to a pdf".to_string(),
      usage:  "Prompts you for: \
                \n\tPath to a pdf \
                \n\tPath to a file to be attached \n".to_string(),
      parent: parent,
      author: "Guillaume (github.com/gdelugre)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
