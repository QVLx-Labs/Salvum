/*
 * QVLx Salvum 
 *
 * kernelhardening.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;

/*********************************** KERNEL HARDENING ***********************************/
pub struct KernelHardeners { event: Event }
impl Eventable for KernelHardeners {
  fn on_init(&self) -> Vec<String> {
    print::println("Kernel Hardening");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn kernelhardeners(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(KernelHardeners {
    event: Event {
      name:   name,
      desc:   "Kernel hardening tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** SELINUX ***********************************/
pub struct SELinux { event: Event }
impl Eventable for SELinux {
  fn on_init(&self) -> Vec<String> {
    //// Prompt for seed
    //let prompt = create_prompt("MersenneTwister>", "Seed: ").blue();
    //let seed = prompt_in_event!(prompt);
    //self.on_run(vec![&seed, &bits]);

    //prompt for image target
    let mut image = prompt_in_event!("SELinux>","Image target (leave blank for default) : ");
    if image.eq("") {
      image = "core-image-selinux".to_string();
    }

    //prompt for machine target
    let mut machine = prompt_in_event!("SELinux>","Machine target (leave blank for default) : ");
    if machine.eq("") {
      machine = "qemux86-64".to_string();
    }
    
    //verify that the img directory exists
    if !Path::new("img/").exists() {
      println!("Unable to build SELinux directory environment not 
        initialized correctly slm/img/ does not exist");
      return Vec::new();
    }

    //fetch the current path
    let current_path = match env::current_dir() {
      Ok(dir) => dir,
      Err(err) => {
        println!("Something bad happened {}",err);
        return Vec::new();
      }
    };
    
    //append the current path with the new selinux dir
    let mut path_selinux: String = match current_path.to_str() {
      Some(s) => s.to_string(),
      None => return Vec::new()
    };
    let dt = chrono::offset::Local::now(); //create the DateTime pub struct
    let datetime: String = str::replace(&dt.to_string()," ","_"); //convert to a string and replace any ' ' with '_'
    let datetime: String = str::replace(&datetime,":","-");
    path_selinux.push_str("/img/selinux_");
    path_selinux.push_str(&datetime[..19]);
  
    //create the new selinux directory
    println!("Creating new selinux directory : {}",path_selinux);
    match run_command(Command::new("mkdir").args(vec![path_selinux.clone()])) {
      Ok(_) => {},
      Err(err) => {
        println!("Unable to create the yocto environment directory : {}",err);
      }
    };

    //ensure that the directory was created
    if !Path::new(&path_selinux).exists() {
      println!("Unable to create the yocto environment directory : {}",path_selinux);
    }

    //copy the bake-kernel script into the new directory
    match run_command(Command::new("cp").args(vec!["ext/selinux/bake-selinux-kernel.sh",&path_selinux[..]])) {
      Ok(_) => {},
      Err(err) => {
        println!("Unable to copy bake-kernel.sh to the environment directory : {}",err);
      }
    };

    //run the yocto environment initialization script
    match run_console_command(Command::new("bash").args(vec!["./bake-selinux-kernel.sh",&image[..],&machine[..]]).current_dir(path_selinux.clone())) {
      Ok(_) => {
        println!("Yocto SELinux build environment initialized");
      },
      Err(err) => {
        println!("Yocto SELinux build environment failed to initialize : {}",err);
      }
    };
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    //set artifact path
    let atf_path = "tst/selinux/atf/selinux_atf";

    //set test dir path
    let test_path = Path::new("tst/selinux/atf");

    //execute test script
    let mut output = match Command::new("./test-quick.sh").arg("").current_dir(&test_path).output() {
      Ok(out) => {out},
      Err(err) => {
        debug::print_debug(format!("selinux::on_test: failed to run test script. {}", err));
        return TestStatus::Failed;
      }
    };

    let mut output_vec:Vec::<u8> = Vec::new();
    output_vec.append(&mut output.stdout);
    output_vec.append(&mut output.stderr);
    let _output_str = match String::from_utf8(output_vec) {
      Ok(string) => string,
      Err(err) => {
        debug::print_debug(format!("selinux::on_test: failed to convert output to string. {}", err));
        return TestStatus::Failed;
      }
    };
  
    //get the exit code of the test script
    let exit_status = match output.status.code() {
      Some(status) => status,
      None => {
        debug::print_debug("selinux::on_test: failed to get selinx exit code.");
        return TestStatus::Failed;
      }
    };

    //read artifact file to an integer
    //first read atf file to string
    let atf_int: i32 = match fs::read_to_string(atf_path) {
      Ok(string) => {
        //remove whitespace from the string
        let string: String = string.split_whitespace().collect();
        //parse an int from the string
        match string.parse::<i32>() {
          Ok(num) => num,
          Err(err) => {
            debug::print_debug(format!("selinux::on_test: failed to read atf to integer. {}", err));
            return TestStatus::Failed;
          }
        }
      },
      Err(err) => {
        debug::print_debug(format!("selinux::on_test: failed to read atf to string. {}", err));
        return TestStatus::Failed;
      }
    };

    //check for non-zer exit code
    if exit_status != atf_int {
      debug::print_debug("selinux::on_test: selinux returned with a non-zero exit code.");
        return TestStatus::Failed;
    }

    //verify that configuration files were created
    let conf_paths = vec!["tst/wrlinux/atf/build/conf/bblayers.conf","tst/wrlinux/atf/build/conf/local.conf"];
    for path in conf_paths {
      let new_path = Path::new(path);
      if !new_path.exists() {
        debug::print_debug(format!("selinux::on_test: configuration file doesn't exist. {}", path));
        return TestStatus::Failed;
      }
    }

    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn selinux(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SELinux {
    event: Event {
      name:   name,
      desc:   "SELinux".to_string(),
      usage:  "Prompts you for: \
              \n\tImage target (optional) \
              \n\tMachine target (optional)\n".to_string(),
      author: "n3wm4n".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** WRLINUX ***********************************/

pub struct WRLinux { event: Event }
impl Eventable for WRLinux {
  fn on_init(&self) -> Vec<String> {
    //// Prompt for seed
    //let prompt = create_prompt("MersenneTwister>", "Seed: ").blue();
    //let seed = prompt_in_event!(prompt);
    //self.on_run(vec![&seed, &bits]);

    //prompt for image target
    let mut image = prompt_in_event!("WRLinux>","Image target (leave blank for default) : ");
    if image.eq("") {
      image = "".to_string();
    }

    //prompt for machine target
    let mut machine = prompt_in_event!("WRLinux>","Machine target (leave blank for default) : ");
    if machine.eq("") {
      machine = "qemux86-64".to_string();
    }
    
    //verify that the img directory exists
    if !Path::new("img/").exists() {
      println!("Unable to build WRLinux directory environment not 
        initialized correctly slm/img/ does not exist");
      return Vec::new();
    }

    //fetch the current path
    let current_path = match env::current_dir() {
      Ok(dir) => dir,
      Err(err) => {
        println!("Something bad happened {}",err);
        return Vec::new();
      }
    };
    
    //append the current path with the new selinux dir
    let mut path_wrlinux: String = match current_path.to_str() {
      Some(s) => s.to_string(),
      None => return Vec::new()
    };
    let dt = chrono::offset::Local::now(); //create the DateTime pub struct
    let datetime: String = str::replace(&dt.to_string()," ","_"); //convert to a string and replace any ' ' with '_'
    let datetime: String = str::replace(&datetime,":","-");
    path_wrlinux.push_str("/img/wrlinux_");
    path_wrlinux.push_str(&datetime[..19]);
  
    //create the new selinux directory
    println!("Creating new wrlinux directory : {}",path_wrlinux);
    match run_command(Command::new("mkdir").args(vec![path_wrlinux.clone()])) {
      Ok(_) => {},
      Err(err) => {
        println!("Unable to create the environment directory : {}",err);
      }
    };

    //ensure that the directory was created
    if !Path::new(&path_wrlinux).exists() {
      println!("Unable to create the environment directory : {}",path_wrlinux);
    }

    //copy the bake-kernel script into the new directory
    match run_command(Command::new("cp").args(vec!["ext/wrlinux/bake-wrlinux-kernel.sh",&path_wrlinux[..]])) {
      Ok(_) => {},
      Err(err) => {
        println!("Unable to copy bake-kernel.sh to the environment directory : {}",err);
      }
    };

    //run the yocto environment initialization script
    match run_console_command(Command::new("bash").args(vec!["./bake-wrlinux-kernel.sh",&image[..],&machine[..]]).current_dir(path_wrlinux.clone())) {
      Ok(_) => {
        println!("WRLinux build environment initialized");
      },
      Err(err) => {
        println!("WRLinux build environment failed to initialize : {}",err);
      }
    };
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    //set artifact path
    let atf_path = "tst/wrlinux/atf/wrlinux_atf";

    //set test dir path
    let test_path = Path::new("tst/wrlinux/atf");

    //execute test script
    let mut output = match Command::new("./test-quick.sh").arg("").current_dir(&test_path).output() {
      Ok(out) => {out},
      Err(err) => {
        debug::print_debug(format!("wrlinux::on_test: failed to run test script. {}", err));
        return TestStatus::Failed;
      }
    };

    let mut output_vec:Vec::<u8> = Vec::new();
    output_vec.append(&mut output.stdout);
    output_vec.append(&mut output.stderr);
    let _output_str = match String::from_utf8(output_vec) {
      Ok(string) => string,
      Err(err) => {
        debug::print_debug(format!("wrlinux::on_test: failed to convert output to string. {}", err));
        return TestStatus::Failed;
      }
    };
  
    //get the exit code of the test script
    let exit_status = match output.status.code() {
      Some(status) => status,
      None => {
        debug::print_debug("wrlinux::on_test: failed to get wrlinux exit code.");
        return TestStatus::Failed;
      }
    };

    //read artifact file to an integer
    //first read atf file to string
    let atf_int: i32 = match fs::read_to_string(atf_path) {
      Ok(string) => {
        //remove whitespace from the string
        let string: String = string.split_whitespace().collect();
        //parse an int from the string
        match string.parse::<i32>() {
          Ok(num) => num,
          Err(err) => {
            debug::print_debug(format!("wrlinux::on_test: failed to read atf to integer. {}", err));
            return TestStatus::Failed;
          }
        }
      },
      Err(err) => {
        debug::print_debug(format!("wrlinux::on_test: failed to read atf to string. {}", err));
        return TestStatus::Failed;
      }
    };

    //check for non-zer exit code
    if exit_status != atf_int {
      debug::print_debug("wrlinux::on_test: wrlinux returned with a non-zero exit code.");
      return TestStatus::Failed;
    }

    //verify that configuration files were created
    let conf_paths = vec!["tst/wrlinux/atf/build/conf/bblayers.conf","tst/wrlinux/atf/build/conf/local.conf"];
    for path in conf_paths {
      let new_path = Path::new(path);
      if !new_path.exists() {
        debug::print_debug(format!("wrlinux::on_test: configuration file doesn't exist. {}", path));
        return TestStatus::Failed;
      }
    }

    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn wrlinux(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(WRLinux {
    event: Event {
      name:   name,
      desc:   "WRLinux".to_string(),
      usage:  "Prompts you for: \
              \n\tImage target (optional) \
              \n\tMachine target (optional)\n".to_string(),
      author: "n3wm4n".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** Hardening Testing ***********************************/
pub struct HardeningTesting { event: Event }
impl Eventable for HardeningTesting {
  fn on_init(&self) -> Vec<String> {
    print::println("Hardening Testing");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn hardeningtesting(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(HardeningTesting {
    event: Event {
      name:   name,
      desc:   "Testing tools to determine kernel hardening".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** Checksec ***********************************/
pub struct Checksec { event: Event }
impl Eventable for Checksec {
  fn on_init(&self) -> Vec<String> {
    // Prompt for file name
    let file_name = prompt_in_event!("checksec>", "Path to executable: ");
    if file_name.trim().eq("") {
      println!("Need a valid file path.");
      return Vec::new();
    }   
    print::print_custom(&file_name, "brightgreen");
    println!();
    return vec![file_name.trim().to_string()];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 { return self.event.usage.clone(); }
    let mut arg = String::from("--file=");
    arg.push_str(&args[0]);
    
    //run command
    let out = match Command::new("bash").args(vec!["ext/checksec/checksec","--extended",&arg]).output() {
      Ok(x) => x,
      Err(err) => {
        println!("Checksec failed: {}",err);
        return String::from("");
      }
    };
    let output = String::from_utf8_lossy(&out.stdout).to_string()
                                                     .replace("\t"," ")
                                                     .replace("   "," ")
                                                     .replace("    "," ")
                                                     .replace("     "," ")
                                                     .replace("      "," ")
                                                     .replace("       "," ")
                                                     .replace("        "," ")
                                                     .replace("         "," ")
                                                     .replace("  "," ");
    let output_nl = output.split("\n").collect::<Vec<&str>>();
    let output_ws1 = output_nl[0].split(" ").collect::<Vec<&str>>();
    let output_ws2 = output_nl[1].split(" ").collect::<Vec<&str>>();
    let mut pretty_output = String::new();
    for i in 0..output_ws2.len() {
      pretty_output.push_str("\x1b[38;5;11m"); 
      pretty_output.push_str(&output_ws1[i].replace("_"," "));
      pretty_output.push_str("\x1b[0m"); 
      pretty_output.push_str(": "); 
      pretty_output.push_str(&output_ws2[i].replace("_"," "));
      pretty_output.push_str("\n"); 
    }
    return pretty_output;
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/checksec/atf/atf.txt";
    let input_path = "tst/checksec/inp/vmlinux";
    let mut checksec_arg = String::from("--file=");
    checksec_arg.push_str(input_path);

    // Run command
    let check = match Command::new("ext/checksec/checksec").args(vec!["--extended".to_string(),checksec_arg]).output() {
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
        debug::print_debug(format!("checksec::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str {
      debug::print_debug(format!("stdout: {}",String::from_utf8_lossy(&check.stdout)));
      debug::print_debug(format!("filestr: {}",file_str));
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn checksec(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Checksec {
    event: Event {
      name:   name,
      desc:   "Checksec".to_string(),
      usage:  "checksec <path_to_binary>\n".to_string(),
      author: "Brian Davis (github.com/slimm609)".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
