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
 * rawtools.rs
 *
 * authors: $t@$h, r00r00, n3wm4n
 */
use crate::events::*;
use crate::alerts::print_notice;

/******************************* BASIC TOOLS *********************************/
pub struct BasicTools { event: Event }
impl Eventable for BasicTools {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Basic Tools");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
} 
pub fn basic(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(BasicTools {
    event: Event {
      name:   name,
      desc:   "Basic tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/***************************** Password Tools ******************************/
pub struct PasswordTools { event: Event }
impl Eventable for PasswordTools {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Password Tools");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn passwordtools(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PasswordTools {
    event: Event {
      name:   name,
      desc:   "Password tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/******************************* PassRS *********************************/
pub struct PassRS { event: Event }
impl Eventable for PassRS {
  fn on_init(&self) -> Vec<String> {
    print_notice();
    
    let mut args: Vec<String> = Vec::new();
    
    //prompt for lowercase
    let lowercase = prompt_in_event!("Pass-RS>","Include lower case characters? 'y' or 'n' : ");
    if lowercase.eq("n")  {
      args.push("-nl".to_string()); //add flag to args if no
    }
    else if lowercase.eq("y") {}
    else {
      println!("Enter 'y' or 'n'");
      return Vec::new();
    }

    //prompt for uppercase
    let uppercase = prompt_in_event!("Pass-RS>","Include upper case characters? 'y' or 'n' : ");
    if uppercase.eq("n") {
      args.push("-nu".to_string());
    }
    else if uppercase.eq("y") {}
    else {
      println!("Enter 'y' or 'n'");
      return Vec::new();
    }

    //prompt for numeric
    let numeric = prompt_in_event!("Pass-RS>","Include numeric characters? 'y' or 'n' : ");
    if numeric.eq("n") {
      args.push("-nd".to_string());
    }
    else if numeric.eq("y") {}
    else {
      println!("Enter 'y' or 'n'");
      return Vec::new();
    }
    
    //prompt for special characters
    let special = prompt_in_event!("Pass-RS>","Include special characters? 'y' or 'n' : ");
    if special.eq("y") {
      args.push("-s".to_string());
    }
    else if special.eq("n") {}
    else {
      println!("Enter 'y' or 'n'");
      return Vec::new();
    }
    if lowercase.eq("n") && uppercase.eq("n") && numeric.eq("n") && special.eq("n") {
      print::print_custom("Can't disable all character options.\n","orange");
      return Vec::new();
    }

    //prompt for password length
    let length: u8 = match prompt_in_event!("Pass-RS>","Enter a character length (max=38) : ").parse() {
      Ok(val) => val,
      Err(err) => {
        println!("Please enter a number. Unable to parse : {}",err);
        return Vec::new();
      }
    };
    if length > 38 {
      print::print_custom("Max character length of 38.\n","orange");
      print::print_custom(&format!("Your length was {}\n",length),"lightorange");
      return Vec::new();
    } 
    args.push(length.to_string());

    let output = match run_command(Command::new("ext/passrs/pass-rs").args(args)) {
      Ok(out) => out,
      Err(err) => {
        println!("Unable to generate a password.\nError : {}",err);
        return Vec::new();
      }
    };

    print::print_custom(&format!("Password generated : {}",String::from_utf8_lossy(&output.stdout).to_string()),"bluegreen");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["-s", "38", "tst/passrs/passrs_-s_38"],
                     vec!["-nl", "20", "tst/passrs/passrs_-nl_20"],
                     vec!["-nu", "15", "tst/passrs/passrs_-nu_15"],
                     vec!["-nd", "10", "tst/passrs/passrs_-nd_10"],
                     vec!["-nl", "-nd", "5", "tst/passrs/passrs_-nl_-nd_5"]];
    for mut test in tests {

      let file_path = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("pass-rs::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };

      //run the command
      let output = match Command::new("ext/passrs/pass-rs").args(test[..].to_vec()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("pass-rs::on_test: failed to execute pass-rs. {}",err));
          return TestStatus::Failed;
        }
      };

      //read file
      let file_str = match fs::read_to_string(file_path) {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("pass-rs::on_test: Failed to open the test file. {}", err));
          return TestStatus::Failed;
        }
      };

      //compare the length of the command output and file_str length
      let output_len = output.stdout.len();
      let file_str_len = file_str.len();
      
      if output_len != file_str_len {
        //debug::print_debug(format!("out_len : {}\nfile_len : {}\n",output_len,file_str_len));
        debug::print_debug("pass-rs::on_test: artifact length doesn't match output length.");
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
}
pub fn passrs(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PassRS {
    event: Event {
      name:   name,
      desc:   "A cryptographically secure password generator.".to_string(),
      usage:  "Prompts you for: \
               \n\tLower case character inclusion \
               \n\tUppercase character inclusion \
               \n\tNumeric character inclusion \
               \n\tSpecial character inclusion \
               \n\tPassword length\n".to_string(),
      author: "Matt Yaraskavitch".to_string(),
      easyrun: false,
      secure: true,
      parent: parent,
      links:  links
    }
  })
}

/******************************* pswd *********************************/
pub struct Pswd { event: Event }
impl Eventable for Pswd {
  fn on_init(&self) -> Vec<String> {
    print_notice();
    
    let mut args: Vec<String> = Vec::new();

    //prompt for password length
    let length: u32 = match prompt_in_event!("Pswd>","Enter a character length : ").parse() {
      Ok(val) => val,
      Err(err) => {
        println!("Please enter a number. Unable to parse : {}",err);
        return Vec::new();
      }
    };
    args.push(length.to_string());
    
    //prompt for lowercase
    let lowercase = prompt_in_event!("Pswd>","Include lower case characters? 'y' or 'n' : ");
    if lowercase.eq("y")  {
      args.push("-l".to_string()); //add flag to args if no
    }
    else if lowercase.eq("n") {}
    else {
      println!("Enter 'y' or 'n'");
      return Vec::new();
    }

    //prompt for uppercase
    let uppercase = prompt_in_event!("Pswd>","Include upper case characters? 'y' or 'n' : ");
    if uppercase.eq("y") {
      args.push("-u".to_string());
    }
    else if uppercase.eq("n") {}
    else {
      println!("Enter 'y' or 'n'");
      return Vec::new();
    }

    //prompt for numeric
    let numeric = prompt_in_event!("Pswd>","Include numeric characters? 'y' or 'n' : ");
    if numeric.eq("y") {
      args.push("-n".to_string());
    }
    else if numeric.eq("n") {}
    else {
      println!("Enter 'y' or 'n'");
      return Vec::new();
    }
    
    //prompt for special characters
    let special = prompt_in_event!("Pswd>","Include special characters? 'y' or 'n' : ");
    if special.eq("y") {
      args.push("-s".to_string());
    }
    else if special.eq("n") {}
    else {
      println!("Enter 'y' or 'n'");
      return Vec::new();
    }

    if args.len() < 2 {
      println!("You must include atleast one character type.");
      return Vec::new();
    }

    let output = match run_command(Command::new("ext/pswd/salvum_passwd").args(args)) {
      Ok(out) => out,
      Err(err) => {
        println!("Unable to generate a password.\nError : {}",err);
        return Vec::new();
      }
    };

    println!("Password generated: {}",String::from_utf8_lossy(&output.stdout).to_string().replace("\n",""));
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["50", "-n", "-l", "-u", "-s", "tst/pswd/passwd_50_-n_-l_-u_-s"],
                   vec!["40", "-n", "-l", "-u", "tst/pswd/passwd_40_-n_-l_-u"],
                   vec!["30", "-n", "-l", "tst/pswd/passwd_30_-n_-l"],
                   vec!["20", "-n", "tst/pswd/passwd_20_-n"],
                   vec!["10", "-n", "-s", "tst/pswd/passwd_10_-n_-s"]];
    for mut test in tests {
      //pop the file path off of the test vec
      let file_path = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("pswd::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };

      //run the command
      let output = match Command::new("ext/pswd/salvum_passwd").args(test[..].to_vec()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("pswd::on_test: failed to execute passwd. {}",err));
          return TestStatus::Failed;
        }
      };

      //read file
      let file_str = match fs::read_to_string(file_path) {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("pswd::on_test: Failed to open the test file. {}", err));
          return TestStatus::Failed;
        }
      };

      //compare the length of the command output and file_str length
      let output_len = output.stdout.len();
      let file_str_len = file_str.len();
      //debug::print_debug(format!("out_len : {}\nfile_len : {}\n",output_len,file_str_len));
      if output_len != file_str_len {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
}
pub fn pswd(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Pswd {
    event: Event {
      name:   name,
      desc:   "A simple password generator".to_string(),
      usage:  "Prompts you for:\
               \n\tPassword length\
               \n\tLower case character inclusion \
               \n\tUppercase character inclusion \
               \n\tNumeric character inclusion \
               \n\tSpecial character inclusion\n".to_string(),
      author: "n3wm4n".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
 
/*********************************** passscore ***********************************/
pub struct PassScore { event: Event }
impl Eventable for PassScore {
  fn on_init(&self) -> Vec<String> {
    //prompt for password
    let password = prompt_in_event!("PassScore>","Enter a password to score : ");

    let output = match run_command(Command::new("ext/passscore/salvum_pass_score").args(vec![password])) {
      Ok(out) => out,
      Err(err) => {
        println!("Unable to score password.\nError : {}",err);
        return Vec::new();
      }
    };
    println!("Password score : {}",String::from_utf8_lossy(&output.stdout).to_string());
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["43X9j7gaExKER2VYK1rnIvHIl9iH8sA7kOiLU0LsuKtgP5YHtk", "tst/passscore/passscore_43X9j7gaExKER2VYK1rnIvHIl9iH8sA7kOiLU0LsuKtgP5YHtk"],
                     vec!["hGD7RtBYfkwqFCgQteJXc05UAfRUHK", "tst/passscore/passscore_hGD7RtBYfkwqFCgQteJXc05UAfRUHK"],
                     vec!["0af2hjdvvg2p75ilhw5y", "tst/passscore/passscore_0af2hjdvvg2p75ilhw5y"]];
    for mut test in tests {
      let file_path = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("pass-rs::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };

      //run the command
      let output = match Command::new("ext/passscore/salvum_pass_score").args(test[..].to_vec()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("passscore::on_test: failed to execute passscore. {}",err));
          return TestStatus::Failed;
        }
      };
      
      //convert stdout to a String
      let output_str: String = match String::from_utf8(output.stdout) {
        Ok(out_str) => out_str,
        Err(err) => {
          debug::print_debug(format!("passscore::on_test: failed to read stdout. {}",err));
          return TestStatus::Failed;
        }
      };
      
      //read file
      let file_str: String = match fs::read_to_string(file_path) {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("passscore::on_test: Failed to open the test file. {}", err));
          return TestStatus::Failed;
        }
      };
      
      //check that the command produced the correct score
      if output_str != file_str {
        return TestStatus::Failed;
      }
      
    }
    //all commands returned correct score
    return TestStatus::Passed;
  }
}
pub fn passscore(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PassScore {
    event: Event {
      name:   name,
      desc:   "A password strength scorer.".to_string(),
      usage:  "Prompts you for:\n\tPassword to score\n".to_string(),
      parent: parent,
      author: "n3wm4n".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** saltyhash ***********************************/
pub struct SaltyHash { event: Event }
impl Eventable for SaltyHash {
  fn on_init(&self) -> Vec<String> {
    let mut args = Vec::<String>::new();

    //prompt for password
    let password = prompt_in_event!("SaltyHash>","Enter a password to hash: ");
    args.push(password);

    //prompt for salt
    let salt = prompt_in_event!("SaltyHash>","Enter a 16 character salt or leave blank for default: ");
    if salt.len() != 0 && salt.len() != 16 {
      println!("A custom salt must be 16 characters not: {}",salt.len());
      return Vec::new();
    }
    else if salt.len() == 16 {
      args.push("-s".to_string());
      args.push(salt);
    }

    //prompt for pepper
    let pepper = prompt_in_event!("SaltyHash>","Enter a 16 character pepper or leave blank for none: ");
    if pepper.len() != 0 && pepper.len() != 16 {
      println!("A pepper must be 16 characters not: {}",pepper.len());
      return Vec::new();
    }
    else if pepper.len() == 16 {
      args.push("-p".to_string());
      args.push(pepper);
    }

    let output = match run_command(Command::new("ext/saltyhash/salvum_saltyhash").args(args)) {
      Ok(out) => out,
      Err(err) => {
        println!("Unable to hash password.\nError: {}",err);
        return Vec::new();
      }
    };
    print::print_custom(&format!("{}",String::from_utf8_lossy(&output.stdout).to_string()),"purple");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["mypassword", "-s", "0123456789abcdef", "tst/saltyhash/saltyhash_mypassword_-s_0123456789abcdef"],
                     vec!["myotherpassword", "-s", "0123456789abcdef", "-p", "fedcba9876543210", "tst/saltyhash/saltyhash_myotherpassword_-s_0123456789abcdef_-p_fedcba9876543210"]];
    
    //
    for mut test in tests {
      //get the test file path
      let file_path = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("saltyhash::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };

      //run the command
      let output = match Command::new("ext/saltyhash/salvum_saltyhash").args(test[..].to_vec()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("saltyhash::on_test: failed to execute saltyhash. {}",err));
          return TestStatus::Failed;
        }
      };

      //convert stdout to a String
      let output_str: String = match String::from_utf8(output.stdout) {
        Ok(out_str) => out_str,
        Err(err) => {
          debug::print_debug(format!("saltyhash::on_test: failed to read stdout. {}",err));
          return TestStatus::Failed;
        }
      };

      //read test file
      let file_str = match fs::read_to_string(file_path) {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("saltyhash::on_test: Failed to open the test file. {}", err));
          return TestStatus::Failed;
        }
      };

      //check that the command produced the correct score
      //println!("output_str\n{}\nfile_str\n{}\n",output_str,file_str);
      if output_str != file_str {
        return TestStatus::Failed;
      }
    }
    //all the outputs of running saltyhash matches test files
    return TestStatus::Passed;
  }
}
pub fn saltyhash(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SaltyHash {
    event: Event {
      name:   name,
      desc:   "A tool that will hash and salt strings.".to_string(),
      usage:  "Prompts you for:\
               \n\tPassword to hash\
               \n\t16 character salt\
               \n\t16 character pepper (optional)\n".to_string(),
      author: "n3wm4n".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** zpass ***********************************/
pub struct ZPass { event: Event }
impl Eventable for ZPass {
  fn on_init(&self) -> Vec<String> {
    let password = prompt_in_event!("zpass>", "Enter a password to score: ");
    let min_length = 3;
    let max_length = 100;
    if !((password.len() > min_length) && (password.len() < max_length)) {
      print::println(&format!("Invalid password length. {}", password.len()));
      return Vec::new();
    }
    return vec![password];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    //check for proper args
    //println!("args: {:?}\nlen: {}",args, args.len());
    if args.len() != 1 {
      return self.event.usage.clone();
    }

    //get password arg
    let password = args[0].clone();
    
    //set command path
    let command = "ext/zpass/zpass";
    //execute zpass command
    let output = match run_command(Command::new(command).arg(password)) {
      Ok(out) => out,
      Err(err) => {
        return format!("Failed to execute zpass. {}\n", err);
      }
    };
    //read output to a string
    let output_str = match String::from_utf8(output.stdout) {
      Ok(string) => string,
      Err(err) => {
        return format!("Failed to execute read stdout to string. {}\n", err);
      }
    };
    //get the exit code of the command
    match output.status.code() {
      Some(status) => {
        //check that the status code is zero
        if status != 0 {
          return String::from("Failed to execute zpass. Returned non-zero.\n");
        }
      },
      None => {
        return String::from("Failed to execute zpass. Returned non-zero.\n");
      }
    };
    
    //print the output of the command
    return output_str;
  }
  fn get_event(&self) -> &Event { return &self.event; }
  fn on_test(&self) -> TestStatus {
    let mut tests = vec![
      vec!["password","tst/zpass/atf/zpass_password"],
      vec!["helloworld","tst/zpass/atf/zpass_helloworld"],
      vec!["helloworld12","tst/zpass/atf/zpass_helloworld12"],
      vec!["verysecurepassword123","tst/zpass/atf/zpass_verysecurepassword123"],
      vec!["verysecurepassword123456","tst/zpass/atf/zpass_verysecurepassword123456"],
    ];
    for test in &mut tests {
      let artifact_path_str = match test.pop() {
        Some(string) => string,
        None => {
          debug::print_debug("zpass::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };
      
      //execute command
      let command = "ext/zpass/zpass";
      let output = match Command::new(command).args(test).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("zpass:on_test: failed to execute zpass. {}", err));
          return TestStatus::Failed;
        }
      };
      
      //check for zero exit code
      //get the exit code of the command
      match output.status.code() {
        Some(status) => {
          //check that the status code is zero
          if status != 0 {
            debug::print_debug("zpass:on_test: zpass returned with a nonzero exit code.");
            return TestStatus::Failed;
          }
        },
        None => {
          debug::print_debug("zpass:on_test: zpass returned with a nonzero exit code.");
          return TestStatus::Failed;
        }
      };
      
      //read output to a string
      let output_str = match String::from_utf8(output.stdout) {
        Ok(string) => string,
        Err(err) => {
          debug::print_debug(format!("zpass:on_test: failed to read output to a string. {}", err));
          return TestStatus::Failed;
        }
      };

      //read artifact to string
      let artifact_str = match fs::read_to_string(artifact_path_str) {
        Ok(string) => string,
        Err(err) => {
          debug::print_debug(format!("zpass:on_test: failed to read output to a string. {}", err));
          return TestStatus::Failed;
        }
      };
      
      //compare output to artifact
      if output_str != artifact_str {
        debug::print_debug("zpass:on_test: output does not match artifact.");
        return TestStatus::Failed;
      }
    }

    return TestStatus::Passed;
  }
}
pub fn zpass(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ZPass {
    event: Event {
      name:   name,
      desc:   "A tool that classifies passwords by their strength.".to_string(),
      usage:  "Prompts you for:\n\tPassword to score\n".to_string(),
      author: "zawwz".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** PRNG ***********************************/
pub struct PRNG { event: Event }
impl Eventable for PRNG {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("PRNG");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn prng(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(PRNG {
    event: Event {
      name:   name,
      desc:   "PRNG tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
 
/*********************************** MERSENNE TWISTER ***********************************/
pub struct MersenneTwister { event: Event }
impl Eventable for MersenneTwister {
  fn on_init(&self) -> Vec<String> {
    // Prompt for seed
    let seed = prompt_in_event!("MersenneTwister>", "Seed: ");

    // Prompt for bits
    let bits = prompt_in_event!("MersenneTwister>", "Number of bits: ");
    
    return vec![seed, bits];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 2 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_command(Command::new("ext/mersennetwister/mersenne_twister").args(args)));

    log::log("MersenneTwister", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["1", "64", "tst/mersennetwister/inp/mt_1_64"],
                     vec!["2", "64", "tst/mersennetwister/inp/mt_2_64"],
                     vec!["3", "64", "tst/mersennetwister/inp/mt_3_64"],
                     vec!["4", "64", "tst/mersennetwister/inp/mt_4_64"],
                     vec!["5", "64", "tst/mersennetwister/inp/mt_5_64"]];

    // For each test in tests
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("ext/mersennetwister/mersenne_twister").args(test[..2].to_vec()).output());

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[2]));

      // Compare
      if String::from_utf8_lossy(&output.stdout).to_string() != file_str {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn mersennetwister(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(MersenneTwister {
    event: Event {
      name:   name,
      desc:   "Generates a pseudorandom number using the mersenne twister algorithm".to_string(),
      usage:  "Requires a:\n\
                \tSeed (decimal integer)\n\
                \tNumber of bits (decimal integer)\n".to_string(),
      parent: parent,
      author: "r00r00".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
 
/************************************** rand_hexstr *****************************************/
pub struct RandHexStr { event: Event }
  impl Eventable for RandHexStr {
    fn on_init(&self) -> Vec<String> {
    // Prompt for input for length of string in bits
    let bit_length = prompt_in_event!("rand_hexstr>", "Length of string in number of bytes: ");
    return vec![bit_length];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let output = match run_command(Command::new("ext/randhexstr/rand_hexstr").args(vec![&args[0], &"N".to_string(), &"".to_string()])) {
      Ok(out) => out,
      Err(err) => {
        println!("Error: {}", err);
        return String::from("");
      }
    };
    // Print output
    print::print_custom(&format!("{}", String::from_utf8_lossy(&output.stdout).to_string()),"purple");
    log::log("RandHexStr", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
   fn on_test(&self) -> TestStatus {
    let test = vec!["10", "tst/randhexstr/out.txt", "N"];

    // Run command
    let hex = match Command::new("ext/randhexstr/rand_hexstr").args(test).output() {
      Ok(h) => (h),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string("tst/randhexstr/atf/atf.txt") {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("randhexstr::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let hex_str = String::from_utf8_lossy(&hex.stdout);

    // Compare
    if hex_str.len() != file_str.len() { return TestStatus::Failed; }
    return TestStatus::Passed;
  }  
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn randhexstr(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(RandHexStr {
    event: Event {
      name:   name,
      desc:   "Random Hex String generator tool".to_string(),
      usage:  "Prompts for:\n\targ1: length in bytes\n".to_string(),
      parent: parent,
      author: "$t@$h".to_string(),
      easyrun: false,
      secure: true,
      links:  links
    }
  })
}

/*********************************** CPRNG ***********************************/
pub struct CPRNG { event: Event }
impl Eventable for CPRNG {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("CPRNG");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn cprng(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(CPRNG {
    event: Event {
      name:   name,
      desc:   "CPRNG tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
 
/*********************************** CHACHA ***********************************/
pub struct ChaCha { event: Event }
impl Eventable for ChaCha {
  fn on_init(&self) -> Vec<String> {
    // Prompt for seed
    let seed = prompt_in_event!("ChaCha>", "Seed: ");

    // Prompt for bits
    let bits = prompt_in_event!("ChaCha>", "Number of bits: ");
    
    return vec![seed, bits];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 2 {
      return self.event.usage.to_string();
    }

    if args[0].len() > 20 {
      print::print_custom("Seed can be at most 20 digits long.\n","orange");
      print::print_custom(&format!("Your value was {} digits long.\n",args[0].len()),"lightorange");
      return String::from("");
    } 
   
    let bits: i32 = match args[1].parse() {
      Ok(o) => o,
     Err(e) => { println!("parsing int failed in chacha: {}", e); return (-1).to_string(); }
    }; 
    if bits > 64 {
      print::print_custom("Bit length can be at most 64.\n","orange");
      print::print_custom(&format!("Your value was {} bits.\n", bits.to_string()),"lightorange");
      return String::from("");
    } 

    // Run command
    let output = simple_match!(run_command(Command::new("ext/chacha/chacharng").args(args)));

    log::log("ChaCha", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["1", "64", "tst/chacha/inp/cc_1_64"],
                     vec!["2", "64", "tst/chacha/inp/cc_2_64"],
                     vec!["3", "64", "tst/chacha/inp/cc_3_64"],
                     vec!["4", "64", "tst/chacha/inp/cc_4_64"],
                     vec!["5", "64", "tst/chacha/inp/cc_5_64"]];

    // For each test in tests
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("ext/chacha/chacharng").args(test[..2].to_vec()).output());

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[2]));

      // Compare
      if String::from_utf8_lossy(&output.stdout).to_string() != file_str {
        return TestStatus::Failed;
      }
    }

    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn chacha(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ChaCha {
    event: Event {
      name:   name,
      desc:   "Generates a cryptographically pseudorandom number using the chacha algorithm".to_string(),
      usage:  "Requires a:\n\
                \tSeed (decimal integer)\n\
                \tNumber of output bits (decimal integer)\n".to_string(),
      parent: parent,
      author: "r00r00".to_string(),
      easyrun: false,
      secure: true,
      links:  links
    }
  })
}
 
/*********************************** HC128 ***********************************/
pub struct HC128 { event: Event }
impl Eventable for HC128 {
  fn on_init(&self) -> Vec<String> {
    // Prompt for seed
    let seed64 = prompt_in_event!("HC128>", "Seed bits 0-63: ");

    // Prompt for seed
    let seed128 = prompt_in_event!("HC128>", "Seed bits 64-127: ");
    
    // Prompt for seed
    let seed192 = prompt_in_event!("HC128>", "Seed bits 128-191: ");
    
    // Prompt for seed
    let seed256 = prompt_in_event!("HC128>", "Seed bits 192-255: ");

    // Prompt for bits
    let bits = prompt_in_event!("HC128>", "Number of bits: ");
    
    return vec![seed64, seed128, seed192, seed256, bits];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 5 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_command(Command::new("ext/hc128/hc128rng").args(args)));

    log::log("HC128", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["1", "1", "1", "1", "64", "tst/hc128/inp/hc_1_64"],
                     vec!["2", "2", "2", "2", "64", "tst/hc128/inp/hc_2_64"],
                     vec!["3", "3", "3", "3", "64", "tst/hc128/inp/hc_3_64"],
                     vec!["4", "4", "4", "4", "64", "tst/hc128/inp/hc_4_64"],
                     vec!["5", "5", "5", "5", "64", "tst/hc128/inp/hc_5_64"]];

    // For each test in tests
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("ext/hc128/hc128rng").args(test[..5].to_vec()).output());

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[5]));

      // Compare
      if String::from_utf8_lossy(&output.stdout).to_string() != file_str { return TestStatus::Failed; }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn hc128(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(HC128 {
    event: Event {
      name:   name,
      desc:   "Generates a cryptographically pseudorandom number using the hc128 algorithm".to_string(),
      usage:  "Requires a:\n\
                \tSeed 1: bits 0-63 (decimal integer)\n\
                \tSeed 2: bits 64-127 (decimal integer)\n\
                \tSeed 3: bits 128-191 (decimal integer)\n\
                \tSeed 4: bits 192-255 (decimal integer)\n\
                \tNumber of output bits (decimal integer)\n".to_string(),
      parent: parent,
      author: "r00r00".to_string(),
      easyrun: false,
      secure: true,
      links:  links
    }
  })
}

/*********************************** RNG ***********************************/
pub struct RNG { event: Event }
impl Eventable for RNG {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("RNG");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn rng(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(RNG {
    event: Event {
      name:   name,
      desc:   "RNG tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
 
/*********************************** OSRand ***********************************/
pub struct OSRand { event: Event }
impl Eventable for OSRand {
  fn on_init(&self) -> Vec<String> {
    // Prompt for bit length
    let bitlength = prompt_in_event!("OS_Rand>", "Number of digits (8|16|32|64|128|256|512): ");
    let mut arguments = Vec::new();
    arguments.push(bitlength);
    
    // Prompt for array or int
    let array = prompt_in_event!("OS_Rand>", "Array or single integer (A/I)?: ");
    let array_size;
    
    // If array, prompt for array size and push args
    if array.eq("A") || array.eq("a") { 
      arguments.push("array".to_string());
      array_size = prompt_in_event!("OS_Rand>", "Size of array: ");
      arguments.push(array_size);
    }
    else if array.eq("I") || array.eq("i") { arguments.push("int".to_string()); }
    else { return Vec::new(); }
    return arguments;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() > 3 || args.len() < 2 { return self.event.usage.clone() }
    
    let mut arguments = Vec::new();
    arguments.push(args[0].clone());
    if args[1].eq("array") || args[1].eq("a") {
      if args.len() < 3 { return self.event.usage.clone() }
      arguments.push("-a".to_string());
      arguments.push(args[2].clone());
    }
    else if args[1].eq("int") || args[1].eq("i") {
      if !args[0].eq("8") &&
         !args[0].eq("16") &&
         !args[0].eq("32") &&
         !args[0].eq("64") &&
         !args[0].eq("128") &&
         !args[0].eq("256") &&
         !args[0].eq("512") {
        print::print_custom("Bad bit size\n","orange");
        return self.event.usage.clone();
      }
    }
    else {
      print::print_custom("Invalid argument\n","orange");
      return self.event.usage.clone();
    }

    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/osrand/os_rand").args(arguments)));
    print::print_custom(&format!("{}",&String::from_utf8_lossy(&output.stdout)),"gold");

    // Print output
    log::log("osrand", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let atf_path = "tst/osrand/atf/atf.txt";

    // Run command
    let output = simple_test_match!(Command::new("ext/osrand/os_rand").arg("-h").output());

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
pub fn osrand(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(OSRand {
    event: Event {
      name:   name,
      desc:   "Generates a cryptosecure random number using kernel sources.".to_string(),
      usage:  "osrand [bitlength: 8|16|32|64|128|256|512] [array|int] [size_of_array_if_array]\n\nExamples:\n\tosrand 8 i\n\tosrand 16 a 3\n".to_string(),
      parent: parent,
      author: "$t@$h".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
/*********************************** ENTROPY ***********************************/
pub struct EntropyAnalysis { event: Event }
impl Eventable for EntropyAnalysis {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Entropy analysis suite");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn entropyanalysis(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(EntropyAnalysis {
    event: Event {
      name:   name,
      desc:   "Entropy determination tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
 
/*********************************** slm_entropy ***********************************/
pub struct SlmEntropy { event: Event }
impl Eventable for SlmEntropy {
  fn on_init(&self) -> Vec<String> {
    // Prompt for seed
    let option = prompt_in_event!("Slm_entropy>", "Draw plot? (Y/N)(Default is N): ");
    let argument;
    if option.eq("Y") || option.eq("y") { argument = "plot"; }
    else { argument = "quick"; }
    let path = prompt_in_event!("Slm_entropy>", "File path: ");
    return vec![argument.to_string(),path];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() == 0 { return self.event.usage.clone(); }
    let output;
    if args.len() == 1 {
      // Run command
      output = simple_match!(run_console_command(Command::new("ext/slm_entropy/entropy_printer").arg(args[0].clone())));
    }
    else if args.len() == 2 {
      if args[0].eq("plot") {
        // Run command
        output = simple_match!(run_console_command(Command::new("ext/slm_entropy/slm_plot").arg(args[1].clone()).stderr(Stdio::null())));
      }
      else if args[0].eq("quick") {
        // Run command
        output = simple_match!(run_console_command(Command::new("ext/slm_entropy/entropy_printer").arg(args[1].clone()).stderr(Stdio::null())));
      }
      else { return self.event.usage.clone(); }
    }
    else { return self.event.usage.clone(); }
    
    log::log("Ent", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/ent/atf/atf.txt";
    let input_path = "tst/ent/inp/test.txt";

    let check = simple_test_match!(Command::new("ext/ent/ent").arg(input_path).output());

    // Read file
    let file_str = simple_test_match!(fs::read_to_string(artifact_path));

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn slm_entropy(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SlmEntropy {
    event: Event {
      name:   name,
      desc:   "ent tool".to_string(),
      usage:  "Prompts you for:\n\
                \tFile (string)\n".to_string(),
      parent: parent,
      author: "$t@$h".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** ENT ***********************************/
pub struct Ent { event: Event }
impl Eventable for Ent {
  fn on_init(&self) -> Vec<String> {
    // Prompt for seed
    let path = prompt_in_event!("Ent>", "File path: ");
    
    return vec![path];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_command(Command::new("ext/ent/ent").args(args)));

    log::log("Ent", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/ent/atf/atf.txt";
    let input_path = "tst/ent/inp/test.txt";

    let check = simple_test_match!(Command::new("ext/ent/ent").arg(input_path).output());

    // Read file
    let file_str = simple_test_match!(fs::read_to_string(artifact_path));

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ent(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Ent {
    event: Event {
      name:   name,
      desc:   "Quickly calculates entropy based statistics on files".to_string(),
      usage:  "Requires a:\n\
                \tFile (string)\n".to_string(),
      parent: parent,
      author: "John Walker\nfourmilab.ch".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** NIST SP 800-90B ***********************************/
pub struct SP80090B { event: Event }
impl Eventable for SP80090B {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("SP80090B");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn sp80090b(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SP80090B {
    event: Event {
      name:   name,
      desc:   "sp80090b tools".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
 
/*********************************** SP IID ***********************************/
pub struct SPIID { event: Event }
impl Eventable for SPIID {
  fn on_init(&self) -> Vec<String> {
    // Prompt for file path
    let path = prompt_in_event!("SPIID>", "File path: ");

    // Prompt for bits
    let bits = prompt_in_event!("SPIID>", "Bits per symbol: ");
    
    return vec![path, bits];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 2 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/spiid/ea_iid").args(args)));

    log::log("Ent", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["tst/spiid/inp/normal.bin", "tst/spiid/atf/atf.txt"]];

    // For each test in tests
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("ext/spiid/ea_iid").args(test[..1].to_vec()).output());

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
pub fn spiid(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SPIID {
    event: Event {
      name:   name,
      desc:   "Runs independent and identically distributed entropy tests on a file".to_string(),
      usage:  filter::parse_help_menu(Command::new("ext/spiid/ea_iid").args(vec!["-h"]),
                                      filter::load_rules_cfg("sp80090b/ea_iid"), "\n", 1,
                                      "Usage is: ea_iid [-v] <file_name> [bits_per_symbol]\n").to_string(),
      parent: parent,
      author: "NIST".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
 
/*********************************** SP NON IID ***********************************/
pub struct SPNonIID { event: Event }
impl Eventable for SPNonIID {
  fn on_init(&self) -> Vec<String> {
    // Prompt for file path
    let path = prompt_in_event!("SPNonIID>", "File path: ");

    // Prompt for bits
    let bits = prompt_in_event!("SPNonIID>", "Bits per symbol: ");
    
    return vec![path, bits];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 2 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/spnoniid/ea_non_iid").args(args)));

    log::log("Ent", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["tst/spnoniid/inp/normal.bin", "tst/spnoniid/atf/atf.txt"]];

    // For each test in tests
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("ext/spnoniid/ea_non_iid").args(test[..1].to_vec()).output());

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[1]));

      // Compare
      if String::from_utf8_lossy(&output.stdout).to_string() != file_str { return TestStatus::Failed; }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn spnoniid(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SPNonIID {
    event: Event {
      name:   name,
      desc:   "Runs entropys tests that are not independent and identically distributed on a file".to_string(),
      usage:  filter::parse_help_menu(Command::new("ext/spnoniid/ea_non_iid").args(vec!["-h"]),
                                      filter::load_rules_cfg("sp80090b/ea_non_iid"), "\n", 1,
                                      "Usage is: ea_non_iid [-v] <file_name> [bits_per_symbol]\n").to_string(),
      parent: parent,
      author: "NIST".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}
 
/*********************************** SP RESTART ***********************************/
pub struct SPRestart { event: Event }
impl Eventable for SPRestart {
  fn on_init(&self) -> Vec<String> {
    // Prompt for file path
    let path = prompt_in_event!("SPRestart>", "File path: ");

    // Prompt for bits
    let bits = prompt_in_event!("SPRestart>", "Bits per symbol: ");

    // Prompt for bits
    let init_ent = prompt_in_event!("SPRestart>", "Initial entropy estimate: ");

    // Prompt for extra flags
    let flags = prompt_in_event!("SPRestart>", "Extra flags (Enter for none): ");

    let mut args: Vec<String> = vec![path, bits, init_ent];
    if flags != "" {
      args.insert(1, flags);
    }
    
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 3 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/sprestart/ea_restart").args(args)));

    log::log("Ent", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["tst/sprestart/inp/normal.bin", "5", "tst/sprestart/atf/atf.txt"]];

    // For each test in tests
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("ext/sprestart/ea_restart").args(test[..2].to_vec()).output());

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[2]));

      // Compare
      if String::from_utf8_lossy(&output.stdout).to_string() != file_str { return TestStatus::Failed; }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn sprestart(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SPRestart {
    event: Event {
      name:   name,
      desc:   "Runs restart tests using your previously calculated entropy tests as input".to_string(),
      usage:  filter::parse_help_menu(Command::new("ext/sprestart/ea_restart").args(vec!["-h"]),
                                      filter::load_rules_cfg("sp80090b/ea_restart"), "\n", 1,
                                      "Usage is: ea_restart [-i|-n] [-v] <file_name> [bits_per_symbol] <H_I>\n").to_string(),
      parent: parent,
      author: "NIST".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** Tropy ************************************/
pub struct Tropy { event: Event }
impl Eventable for Tropy {
  fn on_init(&self) -> Vec<String> {
    print::println("Tropy entropy analyzer");
    let path = prompt_in_event!("Tropy>", "File path: ");
    let mut block_size = "1024".to_string();
    let size = prompt_in_event!("Tropy>", "Granularity size in bytes (default 1024): ");
    if size.len() > 0 { block_size = size; }
    let arguments = vec![block_size, path];
    return arguments;
   }   
   fn on_run(&self, args: Vec<String>) -> String {
     if args.len() < 1 || args.len() > 2 { return self.event.usage.clone(); }
     let mut block_size = "1024".to_string();
     let path;
     if args.len() == 1 {
       path = args[0].clone();
     }
     else {
       block_size = args[0].clone();
       path = args[1].clone();
       if !args[0].chars().all(char::is_numeric) {
         print::print_custom("Non-numeric value for block size given.\n","orange");
         return self.event.usage.clone();
       }
     }
     let mut arguments = Vec::new();
     arguments.push(path);
     arguments.push("--bytes".to_string());
     arguments.push(block_size.to_string());
     // Run command 
     let output = match run_console_command(Command::new("ext/tropy/tropy").args(arguments)) {
       Ok(out) => out,
       Err(err) => { return format!("Error: {}\n", err); }
     };

     log::log("tropy", &String::from_utf8_lossy(&output.stderr));
     return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let input_path = "tst/tropy/inp/test.elf";
    let artifact_path = "tst/tropy/atf/atf.txt";

    // Run command
    let check = match Command::new("ext/tropy/tropy").arg(input_path).output() {
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
        debug::print_debug(format!("tropy::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }   
    };  

    // Compare
    if String::from_utf8_lossy(&check.stdout) != atf_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn tropy(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Tropy {
    event: Event {
      name:   name,
      desc:   "Tropy entropy analyzer.".to_string(),
      usage:  "Requires a:\n\
                \tGranularity (number of bytes)\n\
                \tFile (string)\n".to_string(),
      parent: parent,
      author: "Felix Girke\nfelix.girke@tu-dortmund.de".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }   
  })  
}  
 
/*********************************** GithubKeygen ************************************/
pub struct GithubKeygen { event: Event }
impl Eventable for GithubKeygen {
  fn on_init(&self) -> Vec<String> {
    let cmd = prompt_in_event!("GitHubKeygen>", "");
    let cmd_args: Vec<&str> = cmd.split(" ").collect();

    if cmd_args[0] == "" {
      println!("Need args. Please see usage.");
      return Vec::new();
    }

    if filter::if_command_contains_rule(&cmd_args, filter::load_rules_cfg("githubkeygen")) {
      let output = match run_console_command(Command::new("ext/githubkeygen/github-keygen").args(cmd_args)) {
        Ok(out) => out,
        Err(err) => {
          println!("Error: {}", err);
          return Vec::new();
        }
      };
      print!("{}", String::from_utf8_lossy(&output.stdout).to_string());
      print!("{}", String::from_utf8_lossy(&output.stderr));
    }
    else { println!("Not supported."); }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/githubkeygen/atf/atf.txt";
    let git_args = vec!["config", "user.name", "test"];

    match Command::new("git").args(git_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("githubkeygen::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    }

    let keygen_arg = "-h";
    let check = match Command::new("ext/githubkeygen/github-keygen").arg(keygen_arg).output() {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("githubkeygen::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("githubkeygen::on_test: Failed to open the test file. {}", err));
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
pub fn githubkeygen(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(GithubKeygen {
    event: Event {
      name:   name,
      desc:   "A more secure way to generate and manage your GitHub keys.".to_string(),
      usage:  filter::parse_help_menu(Command::new("ext/githubkeygen/github-keygen").args(vec!["-h"]),
                                      filter::load_rules_cfg("githubkeygen"), "\x0A\x0A", 0, 
                                      "GitHub-Keygen\n\
                                      Usage: [-R] [-t <key-type>] [-b <key-bits>].\n\
                                    <user> [-t <key-type>] [-b <key-bits>] [-d] [-r]\n").to_string(),
      parent: parent,
      author: "Olivier Mengue\n".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    } 
  })
}

/*********************************** GlassPumpkin ***********************************/
pub struct GlassPumpkin { event: Event }
impl Eventable for GlassPumpkin {
  fn on_init(&self) -> Vec<String> {
    // Prompt for options
    let g_or_c = prompt_in_event!("GlassPumpkin>", "Generate prime or check a prime (g or c): ");
    let mut arguments = Vec::new();
    if g_or_c.trim().eq("g") || g_or_c.trim().eq("G") {
      arguments.push("g".to_string());
      let digits = prompt_in_event!("GlassPumpkin>", "Bit length of generated prime (Minimum of 128): ");
      arguments.push(digits.trim().to_string());
      return arguments;
    }
    else if g_or_c.trim().eq("c") || g_or_c.trim().eq("C") {
      arguments.push("c".to_string());
      let num = prompt_in_event!("GlassPumpkin>", "Number to check primality: ");
      arguments.push(num.trim().to_string());
      return arguments;
    }
    else {
      print::print_custom("Invalid argument.","orange");
      println!();
      return Vec::new();
    }
  }
  fn on_run(&self, mut args: Vec<String>) -> String {
    // Run command
    if args.len() != 2 { return self.event.usage.clone(); }
    if args[0].eq("g") { args.push("0".to_string()); }
    if args[0].eq("c") { args.insert(1,"0".to_string()); }
    if !args[2].chars().all(char::is_numeric) {
      print::print_custom("Invalid string passed, non-numerics found.\n","lightorange");
      return String::from(""); 
    }
    let output = simple_match!(run_command(Command::new("ext/glasspumpkin/glspkn").args(args)));

    log::log("Ent", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/glasspumpkin/atf/atf.txt";

    let check = match Command::new("ext/glasspumpkin/glspkn").args(vec!["g","128","0"]).output() {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("glasspumpkin::on_test: Failed to run command. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("glasspumpkin::on_test: Failed to open the atf file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    let out_str_utf8 = String::from_utf8_lossy(&check.stdout);
    let output_tkns_nl = out_str_utf8.split("\n").collect::<Vec<&str>>();
    let artifact: usize = match file_str.parse() {
      Ok(o) => o,
      Err(e) => {
        debug::print_debug(format!("glasspumpkin::on_test: Failed to parse artifact. {}", e));
        return TestStatus::Failed;
      }
    }; 
    let output_tkns_ws = output_tkns_nl[2].split(" ").collect::<Vec<&str>>();
    if !output_tkns_ws[0].trim().eq("Output:") { return TestStatus::Failed; }
    if output_tkns_ws[1].trim().len() != artifact
    && output_tkns_ws[1].trim().len() != (artifact - 1)
    && output_tkns_ws[1].trim().len() != (artifact + 1) {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn glasspumpkin(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(GlassPumpkin {
    event: Event {
      name:   name,
      desc:   "GlassPumpkin cybersecure prime number generator".to_string(),
      usage:  "glasspumpkin g <length_of_prime>\nglasspumpkin c <prime_to_check>\n\nNeed a minimum length of 128 when generating\n".to_string(),
      author: "$t@$h & Michael Lodder".to_string(),
      easyrun: false,
      secure: true,
      parent: parent,
      links:  links
    }
  })
}  

/*********************************** Nanoid ***********************************/
pub struct Nanoid { event: Event }
impl Eventable for Nanoid {
  fn on_init(&self) -> Vec<String> {
    // Prompt for options
    let length = prompt_in_event!("Nanoid>", "Length of cryptosecure string to generate: ");
    if length.trim().eq("0") {
      print::print_custom("Invalid argument.","orange");
      println!();
      return Vec::new();
    }
    let arguments = vec![length];
    return arguments;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 { return self.event.usage.clone(); }
    // Run command
    let output = simple_match!(run_command(Command::new("ext/nanoid/nanoid").args(args)));

    log::log("Ent", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/nanoid/atf/atf.txt";

    let check = match Command::new("ext/nanoid/nanoid").arg("10").output() {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("nanoid::on_test: Failed to run command. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("nanoid::on_test: Failed to open the atf file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    let out_str_utf8 = String::from_utf8_lossy(&check.stdout);

    let output_tkns_nl = out_str_utf8.split("\n").collect::<Vec<&str>>();
    let artifact: usize = match file_str.parse() {
      Ok(o) => o,
      Err(e) => {
        debug::print_debug(format!("nanoid::on_test: Failed to parse artifact. {}", e));
        return TestStatus::Failed;
      }
    }; 

    if output_tkns_nl[2].trim().len() != artifact {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn nanoid(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Nanoid {
    event: Event {
      name:   name,
      desc:   "Nanoid pseudo-random string generator".to_string(),
      usage:  "nanoid <length_of_string>\n".to_string(),
      author: "Andrey Sitnik".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** Obfuscation ***********************************/
pub struct Obfuscation { event: Event }
impl Eventable for Obfuscation {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Obfuscation");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn obfuscation(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Obfuscation {
    event: Event {
      name:   name,
      desc:   "Obfuscation apps".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }   
  })  
}

/*********************************** ananas ***********************************/
pub struct Ananas { event: Event }
impl Eventable for Ananas {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 { return self.event.usage.clone(); }
    let output = simple_match!(run_console_command(Command::new("ext/ananas/ananas").args(vec!["y".to_string(),args[0].clone()]))); 
    log::log("ananas", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/ananas/atf/atf.txt";

    // Run command
    let output = match Command::new("ext/ananas/ananas").arg("__TEST__").output() {
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
    if String::from_utf8_lossy(&output.stdout).to_string() != atf_str {
      return TestStatus::Failed;
    }   
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ananas(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Ananas {
    event: Event {
      name:   name,
      desc:   "Ananas NaN-based Obfuscator.".to_string(),
      usage:  "Prompts you for:\n\
                \tFile path (String)\n".to_string(),
      parent: parent,
      author: "matzr3lla & Cyrille Lavigne".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }   
  })  
}

/*********************************** zw ***********************************/
pub struct Zw { event: Event }
impl Eventable for Zw {
  fn on_init(&self) -> Vec<String> {
    print::println("ZW Obfuscator");
   
    // Prompt for file name
    let input_path = prompt_in_event!("zw>", "Path of file to copy and obfuscate: ");
 
    if input_path.trim().eq("") {
      print::print_custom("No filepath given.\n","orange");  
      return Vec::new();
    }   
    print::print_custom(&input_path, "brightgreen");
    println!();  
    return vec![input_path];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 { return self.event.usage.clone(); }
    let mut cmd = String::from("cat ");
    if !Path::new(&args[0]).exists() {
      print::print_custom("File not found at that path.\n","orange");
      return String::from("");
    }
    let split_filename = args[0].split(".").collect::<Vec<&str>>();
    let no_extension = split_filename[0];
    let split_no_extension = no_extension.split("/").collect::<Vec<&str>>();
    let basename = split_no_extension[split_no_extension.len() - 1];
    let extension = split_filename[1];
    cmd.push_str(&args[0]);
    let mut cmd2 = String::new();
    if extension.eq("zw") { // decode
      cmd2.push_str("ext/zw/zw -d");
      let output = match util::pipes::Pipe::new(&cmd.to_string())
                                           .then(&cmd2.to_string()) 
                                           .finally() {
        Ok(sel) => sel, 
        Err(err) => { return format!("Zw -d command failed. Error: {}\n", err); }
      };  
      let out = match output.wait_with_output() {
        Ok(sel) => sel, 
        Err(err) => { return format!("Zw -d waiting on output failed. Error: {}\n", err); }
      };
      let decoded = String::from_utf8_lossy(&out.stdout).to_string();
      let mut outname = String::from("out/zw/");
      outname.push_str(basename);
      outname.push_str(".txt");
      util::misc::write_file(decoded,outname.clone());  
      print::print_custom(&format!("zw: File decoded to --> {}\n", outname),"neongreen");
    }   
    else { // encode
      cmd2.push_str("ext/zw/zw -e");
      let output = match util::pipes::Pipe::new(&cmd.to_string())
                                           .then(&cmd2.to_string()) 
                                           .finally() {
        Ok(sel) => sel, 
        Err(err) => { return format!("Zw -e command failed. Error: {}\n", err); }
      };  
      let out = match output.wait_with_output() {
        Ok(sel) => sel, 
        Err(err) => { return format!("Zw -e waiting on output failed. Error: {}\n", err); }
      };  
      let encoded = String::from_utf8_lossy(&out.stdout).to_string();
      let mut outname = String::from("out/zw/");
      outname.push_str(basename);
      outname.push_str(".zw");
      util::misc::write_file(encoded,outname.clone());  
      print::print_custom(&format!("zw: File encoded to --> {}\n", outname),"neongreen");
    }   
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let inp_path = "tst/zw/inp/hello.txt";
    let atf_path = "tst/zw/atf/atf.txt";

    // Run command
    let output = simple_test_match!(Command::new("ext/zw/zw").args(vec!["-e", inp_path]).output());

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
pub fn zw(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Zw {
    event: Event {
      name:   name,
      desc:   "Obfuscates files using zero-width characters".to_string(),
      usage:  "zw <path_to_file>\n".to_string(),
      easyrun: false,
      secure: false, 
      parent: parent,
      author: "$t@$h & Tyler (tje)".to_string(),
      links:  links
    }   
  })  
}
