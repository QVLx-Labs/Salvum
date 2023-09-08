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
 * netloaders.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;
use std::fs::OpenOptions;
use users::get_user_by_name;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

/*********************************** NET LOADERS ***********************************/
pub struct NetLoader { event: Event }
impl Eventable for NetLoader {
  fn on_init(&self) -> Vec<String> {
    print::println("Net loader");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn netloader(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync>  {
  Box::new(NetLoader {
    event: Event {
      name:   name,
      desc:   "Net Loader Tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
/*********************************** STOPSERVF ***********************************/
pub struct StopServF { event: Event }
impl Eventable for StopServF {
  fn on_init(&self) -> Vec<String> {    
    //print current status
    servf_print_status();
    //prompt for file serving option
    let serv_option = prompt_in_event!("ServF>","Enter a file server to stop 'tftp' 'ftp' 'pxe' 'all': ");
    //handle the user selected option
    match &serv_option[..] {
      "tftp" => servf_tftp_stop(),
      "ftp" => servf_ftp_stop(),
      "pxe" => servf_pxe_stop(),
      "all" => {
        servf_tftp_stop();
        servf_ftp_stop();
        servf_pxe_stop();
      }
      _ => {
        print::println(&format!("Invalid option : {}",serv_option));
      }
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    match servf_tftp_stop_test() {
      TestStatus::Failed | TestStatus::Unimplemented => {
        debug::print_debug("stopservf::on_test: failed to stop tftp service.");
        return TestStatus::Failed;
      },
      TestStatus::Passed => {}
    };
    match servf_ftp_stop_test() {
      TestStatus::Failed | TestStatus::Unimplemented => {
        debug::print_debug("stopservf::on_test: failed to stop ftp service.");
        return TestStatus::Failed;
      },
      TestStatus::Passed => {}
    };
    match servf_pxe_stop_test() {
      TestStatus::Failed | TestStatus::Unimplemented => {
        debug::print_debug("stopservf::on_test: failed to stop pxe service.");
        return TestStatus::Failed;
      },
      TestStatus::Passed => {}
    };
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn stopservf(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(StopServF {
    event: Event {
      name:   name,
      desc:   "A utility used to stop background file services.".to_string(),
      usage:  "Prompts you for: \
              \n\tFile service to stop tftp, ftp, pxe, all\n".to_string(),
      author: "n3wm4n".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

fn servf_tftp_stop_test() -> TestStatus {
//start tftp server
  //path to the configuration file the tftp daemon will read
  let path_config = "/etc/default/tftpd-hpa".to_string();
  //create args vec
  let mut args: Vec<String> = Vec::new();
  let path_new_config = "cfg/servf/tftpd-hpa-template".to_string();
  args.push(path_new_config);
  let path_new_config = "cfg/servf/tftpd-hpa".to_string();
  args.push(path_new_config.clone());
  //copy template config file
  match Command::new("cp").args(args).output() {
    Ok(_) => {},
    Err(err) => {
      println!("stopservf::on_test: failed to copy configuration file. {}",err);
      return TestStatus::Failed;
    }
  }
  //open up the new config file
  let mut config_file = match OpenOptions::new().write(true).append(true).open(path_new_config.clone()) {
    Ok(file) => file,
    Err(err) => {
      println!("stopservf::on_test: failed to open the configuration file. {}",err);
      return TestStatus::Failed;
    }
  };
  //get the absolute path to srv dir
  let path_srv = match servf_get_srv_path() {
    Ok(path) => path,
    Err(err) => {
      println!("stopservf::on_test: failed to get srv path {}",err);
      return TestStatus::Failed;
    }
  };
  //append the config file with the path to the users source files
  match writeln!(config_file,"TFTP_DIRECTORY=\"{}\"",path_srv) {
    Ok(_) => {},
    Err(err) => {
      println!("stopservf::on_test: failed to append to configuration file. {}",err);
      return TestStatus::Failed;
    }
  }
  //replace the old tftp config file with the new one
  let mut args: Vec<String> = Vec::new();
  args.push(path_new_config);
  args.push(path_config.clone());
  match Command::new("mv").args(args).output() {
    Ok(_) => {},
    Err(err) => {
      println!("stopservf::on_test: failed to replace the configuration file. {}",err);
      return TestStatus::Failed;
    }
  }
  //enable tftp daemon
  let command = "service";
  /*let args = vec!["enable","tftpd-hpa"];
  match Command::new(command).args(args).output() {
    Ok(_) => {},
    Err(err) => {
      println!("stopservf::on_test: failed to enable tftp daemon. {}",err);
      return TestStatus::Failed;
    }
  }*/
  //start tftp daemon
  let args = vec!["tftpd-hpa","start"];
  match Command::new(command).args(args).output() {
    Ok(_) => {},
    Err(err) => {
      println!("stopservf::on_test: failed to start tftp daemon. {}",err);
      return TestStatus::Failed;
    }
  }
  //stop tftp daemon
  let args = vec!["tftpd-hpa","stop"];
  match Command::new(command).args(args).output() {
    Ok(_) => {},
    Err(err) => {
      println!("stopservf::on_test: failed to stop tftp daemon. {}",err);
      return TestStatus::Failed;
    }
  }
  return TestStatus::Passed;
}

fn servf_ftp_stop_test() -> TestStatus {
  //check to see if ftpqvlx exists
  let _ftp_usr_exist: bool = match get_user_by_name("ftpqvlx") {
    Some(_user) => true,
    None => false
  };
  //create ftpqvlx if it doesn't exist
  /*if !ftp_usr_exist {
    //get the absolut srv path
    let path_srv: String = match servf_get_srv_path() {
      Ok(path) => path,
      Err(err) => {
        println!("stopservf::on_test: failed to get srv path {}",err);
        return TestStatus::Failed;
      }
    };
    //run the useradd command with sudo permission
    let args: Vec<String> = vec!["-s".to_string(),"/bin/bash".to_string(),"-d".to_string(),path_srv,"ftpqvlx".to_string()];
    match Command::new("useradd").args(args).output() {
      Ok(_) => {},
      Err(err) => {
        println!("stopservf::on_test: failed to create ftpqvlx {}",err);
        return TestStatus::Failed;
      }
    }
    //run the passwd command to set ftpqvlx password
    print::println("Please set the password for the 'ftpqvlx' user.");
    let args = vec!["ftpqvlx"];
    match run_console_command(Command::new("passwd").args(args)) {
      Ok(_) => {},
      Err(err) => {
        println!("stopservf::on_test: failed to set ftpqvlx password {}",err);
        return TestStatus::Failed;
      }
    }
  }*/
  //enable ftp daemon
  //sudo systemctl enable pure-ftpd
  /*let args = vec!["enable","pure-ftpd"];
  match Command::new("systemctl").args(args).output() {
    Ok(_) => {},
    Err(err) => {
      println!("stopservf::on_test: failed to enable the ftp daemon {}",err);
      return TestStatus::Failed;
    }
  }*/
  //start ftp daemon
  //sudo systemctl start pure-ftpd
  let args = vec!["vsftpd","start"];
  match Command::new("service").args(args).output() {
    Ok(_) => {},
    Err(err) => {        
      println!("stopservf::on_test: failed to start the ftp daemon {}",err);
      return TestStatus::Failed;
    }
  }
  //stop ftp daemon
  //sudo systemctl start pure-ftpd
  let args = vec!["vsftpd","stop"];
  match Command::new("service").args(args).output() {
    Ok(_) => {},
    Err(err) => {        
      println!("stopservf::on_test: failed to stop the ftp daemon {}",err);
      return TestStatus::Failed;
    }
  }
  return TestStatus::Passed;
}

/*********************************** SERVF ***********************************/
pub struct ServF { event: Event }
impl Eventable for ServF {
  fn on_init(&self) -> Vec<String> {    
    //print current status
    servf_print_status();
    //prompt for file serving option
    let serv_option = prompt_in_event!("ServF>","Enter a file server option 'tftp' 'ftp' 'pxe': ");
    //handle the user selected option
    let servf_b: bool = match &serv_option[..] {
      "tftp" => servf_tftp_start(),
      "ftp" => servf_ftp_start(),
      "pxe" => servf_pxe_start(false),
      _ => {
        print::println(&format!("Invalid option : {}",serv_option));
        return Vec::new();
      }
    }; 
    //check to see if the server initialization was successful
    if servf_b {
      print::println(&format!("Your {} file server started successfully and will run in the background as long as Salvum is running. You may stop the service at any time via 'stopservf'",serv_option));
    }
    else {
      print::println(&format!("Your {} file server did not start successfully.",serv_option));
      return Vec::new();
    }
    //prompt for ip addr option
    let ip_option = prompt_in_event!("ServF>","Salvum can retrieve your system's IP addresses. Would you like to print them to the console? (y/n): ");
    //handle ip addr option
    if ip_option.trim().eq("y") {
      let addresses = match get_ip_addrs() {
        Ok(addrs) => addrs,
        Err(err) => {
          print::println(&format!("{}",err));
          return Vec::new();
        }
      };
      for addr in addresses {
        print::println(&format!("{}",addr));
      }
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    match servf_tftp_start_test() {
      TestStatus::Passed => {},
      TestStatus::Failed | TestStatus::Unimplemented => {
        debug::print_debug("servf::on_test: failed to start tftp service.");
        return TestStatus::Failed;
      }
    };
    match servf_ftp_start_test() {
      TestStatus::Passed => {},
      TestStatus::Failed | TestStatus::Unimplemented => {
        debug::print_debug("servf::on_test: failed to start ftp service.");
        return TestStatus::Failed;
      }
    };
    match servf_pxe_start_test() {
      TestStatus::Passed => {},
      TestStatus::Failed | TestStatus::Unimplemented => {
        debug::print_debug("servf::on_test: failed to start pxe service.");
        return TestStatus::Failed;
      }
    };
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
fn servf_tftp_start_test() -> TestStatus {
  //hard coded to fail until test is finished being fixed to ATF format
  //return TestStatus::Failed;
  _pxe_stop();
  //vec![<loop_back_destination_path>,<loop_back_script_path>,<srv_payload_path>,<original_atf_path>]
  let tests = vec![vec!["tst/servf/tftp/hello","./loopback_hello.sh","srv/hello","tst/servf/tftp/atf/hello"],
                   vec!["tst/servf/tftp/dbase.c","./loopback_dbase.sh","srv/dbase.c","tst/servf/tftp/atf/dbase.c"]];
  //path to the configuration file the tftp daemon will read
  let path_config = "/etc/default/tftpd-hpa".to_string();
  //create args vec
  let mut args: Vec<String> = Vec::new();
  let path_new_config = "cfg/servf/tftpd-hpa-template".to_string();
  args.push(path_new_config);
  let path_new_config = "cfg/servf/tftpd-hpa".to_string();
  args.push(path_new_config.clone());
  //copy template config file
  match Command::new("cp").args(args).output() {
    Ok(_) => {},
    Err(err) => {
      println!("servf::on_test: failed to copy configuration file. {}",err);
      return TestStatus::Failed;
    }
  }
  //open up the new config file
  let mut config_file = match OpenOptions::new().write(true).append(true).open(path_new_config.clone()) {
    Ok(file) => file,
    Err(err) => {
      println!("servf::on_test: failed to open the configuration file. {}",err);
      return TestStatus::Failed;
    }
  };
  //get the absolute path to srv dir
  let path_srv = match servf_get_srv_path() {
    Ok(path) => path,
    Err(err) => {
      println!("servf::on_test: failed to get srv path {}",err);
      return TestStatus::Failed;
    }
  };
  //append the config file with the path to the users source files
  match writeln!(config_file,"TFTP_DIRECTORY=\"{}\"",path_srv) {
    Ok(_) => {},
    Err(err) => {
      println!("servf::on_test: failed to append to configuration file. {}",err);
      return TestStatus::Failed;
    }
  }
  //replace the old tftp config file with the new one
  let mut args: Vec<String> = Vec::new();
  args.push(path_new_config);
  args.push(path_config.clone());
  match Command::new("mv").args(args).output() {
    Ok(_) => {},
    Err(err) => {
      println!("servf::on_test: failed to replace the configuration file. {}",err);
      return TestStatus::Failed;
    }
  }
  //println!("config data copied");
  fn tftp_stop() {
    let stop_args = vec!["tftpd-hpa","stop"];
    match Command::new("service").args(stop_args).output() {
      Ok(_) => {
        //println!("ftp service stopped");
      },
      Err(err) => {        
        println!("servf::on_test: failed to stop the ftp daemon {}",err);
      }
    };
  }
  fn tftp_start() {
    let start_args = vec!["tftpd-hpa","start"];
    match Command::new("service").args(start_args.clone()).output() {
      Ok(_out) => {
        //println!("starting ftp service {}{}",String::from_utf8_lossy(&out.stderr),String::from_utf8_lossy(&out.stdout));
      },
      Err(err) => {        
        println!("servf::on_test: failed to start the ftp daemon {}",err);
      }
    };
  }
  fn tftp_status() {
    let status_args = vec!["tftpd-hpa","status"];
    match Command::new("service").args(status_args)/*.stdin(Stdio::null()).stdout(Stdio::null())*/.output() {
      Ok(_out) => {
        //println!("status of ftp service {}{}",String::from_utf8_lossy(&out.stderr),String::from_utf8_lossy(&out.stdout));
      },
      Err(err) => {        
        println!("servf::on_test: failed to get status of the ftp daemon {}",err);
      }
    }
  }
  
  let working_path = "tst/servf/tftp";
  tftp_start();
  tftp_status();
  for test in tests {
    //println!("{:?}", test);
    let atf_file_path = test[3];
    let srv_file_path = test[2];
    let loopback_script_path_str = test[1];
    let loopback_file_path_str = test[0];

    match Command::new("cp").args(vec![atf_file_path,srv_file_path]).output() {
      Ok(_) => {
        //println!("artifact copied to srv dir");
      },
      Err(err) => {
        debug::print_debug(format!("{}",err));
        tftp_stop();
        return TestStatus::Failed;
      }
    };

    let args = vec![loopback_script_path_str];
    let _output = match Command::new("sh").args(args).current_dir(working_path).output() {
      Ok(output) => {
        //println!("executed loopback script");
        output
      },
      Err(err) => {
        debug::print_debug(format!("servf::on_test: failed to run tftp loopback script. {}", err));
        tftp_stop();
        return TestStatus::Failed;
      }
    };
    //println!("here");
    //println!("{}{}",String::from_utf8_lossy(&_output.stderr), String::from_utf8_lossy(&_output.stdout));
    let artifact_path = Path::new(atf_file_path);
    let artifact_str = match fs::read_to_string(artifact_path) {
      Ok(string) => string,
      Err(err) => {
        debug::print_debug(format!("servf::on_test: failed to read artifact file. \"{}\" {}",atf_file_path,err));
        tftp_stop();
        return TestStatus::Failed;
      }
    };
    let loopback_file_path = Path::new(loopback_file_path_str);
    let loopback_str = match fs::read_to_string(loopback_file_path) {
      Ok(string) => string,
      Err(err) => {
        debug::print_debug(format!("servf::on_test: failed to read loopback file. \"{}\" {}",loopback_file_path_str,err));
        tftp_stop();
        return TestStatus::Failed;
      }
    };
    //println!("artifact: \"{}\"\nloopback: \"{}\"",artifact_str,loopback_str);
    if artifact_str != loopback_str {
      tftp_stop();
      debug::print_debug("artifact does not match loopback");
      return TestStatus::Failed;
    }
    match fs::remove_file(&loopback_file_path) {
      Ok(()) => {},
      Err(err) => {
        debug::print_debug(format!("servf::on_test: failed to remove loopback file. \"{}\" {}",loopback_file_path_str,err));
        tftp_stop();
        return TestStatus::Failed;
      }
    }
  }
  tftp_stop();
  return TestStatus::Passed; 
}
fn servf_ftp_start_test() -> TestStatus {
  //vec![<loop_back_destination_path>,<loop_back_script_path>,<srv_payload_path>,<original_atf_path>]
  let tests = vec![vec!["tst/servf/ftp/hello","./loopback_hello.sh","srv/hello","tst/servf/ftp/atf/hello"],
                   vec!["tst/servf/ftp/dbase.c","./loopback_dbase.sh","srv/dbase.c","tst/servf/ftp/atf/dbase.c"]];
  //check to see if ftpqvlx exists
  let _ftp_usr_exist: bool = match get_user_by_name("ftpqvlx") {
    Some(_user) => true,
    None => false
  };
  
  fn ftp_stop() {
    let stop_args = vec!["vsftpd","stop"];
    match Command::new("service").args(stop_args).output() {
      Ok(_) => {
        //println!("ftp service stopped");
      },
      Err(err) => {        
        println!("servf::on_test: failed to stop the ftp daemon {}",err);
      }
    };
  }
  fn ftp_start() {
    let start_args = vec!["vsftpd","start"];
    match Command::new("service").args(start_args.clone()).output() {
      Ok(_out) => {
        //println!("starting ftp service {}{}",String::from_utf8_lossy(&out.stderr),String::from_utf8_lossy(&out.stdout));
      },
      Err(err) => {        
        println!("servf::on_test: failed to start the ftp daemon {}",err);
      }
    };
  }
  fn ftp_status() {
    let status_args = vec!["vsftpd","status"];
    match Command::new("service").args(status_args).stdin(Stdio::null()).stdout(Stdio::null()).output() {
      Ok(_out) => {
        //println!("status of ftp service {}{}",String::from_utf8_lossy(&out.stderr),String::from_utf8_lossy(&out.stdout));
      },
      Err(err) => {        
        println!("servf::on_test: failed to get status of the ftp daemon {}",err);
      }
    }
  }
  
  let working_path = "tst/servf/ftp";
  ftp_start();
  ftp_status();
  for test in tests {
    let atf_file_path = test[3];
    let srv_file_path = test[2];
    let loopback_script_path_str = test[1];
    let loopback_file_path_str = test[0];

    match Command::new("cp").args(vec![atf_file_path,srv_file_path]).output() {
      Ok(_) => {},
      Err(err) => {
        println!("{}",err);
        ftp_stop();
        return TestStatus::Failed;
      }
    };

    let args = vec![loopback_script_path_str];
    let _output = match Command::new("sh").args(args).current_dir(working_path).output() {
      Ok(output) => {
        output
      },
      Err(err) => {
        println!("servf::on_test: failed to run ftp loopback script. {}", err);
        ftp_stop();
        return TestStatus::Failed;
      }
    };
    //println!("{}{}",String::from_utf8_lossy(&_output.stderr), String::from_utf8_lossy(&_output.stdout));
    let artifact_path = Path::new(atf_file_path);
    let artifact_str = match fs::read_to_string(artifact_path) {
      Ok(string) => string,
      Err(err) => {
        println!("servf::on_test: failed to read artifact file. \"{}\" {}",atf_file_path,err);
        ftp_stop();
        return TestStatus::Failed;
      }
    };
    let loopback_file_path = Path::new(loopback_file_path_str);
    let srv_file_path = Path::new(srv_file_path);
    let loopback_str = match fs::read_to_string(loopback_file_path) {
      Ok(string) => string,
      Err(err) => {
        println!("servf::on_test: failed to read loopback file. \"{}\" {}",loopback_file_path_str,err);
        ftp_stop();
        return TestStatus::Failed;
      }
    };
    //println!("artifact: \"{}\"\nloopback: \"{}\"",artifact_str,loopback_str);
    if artifact_str != loopback_str {
      ftp_stop();
      return TestStatus::Failed;
    }
    for path in vec![loopback_file_path, srv_file_path] {
      match fs::remove_file(&path) {
        Ok(()) => {},
        Err(err) => {
          println!("servf::on_test: failed to remove file. \"{:?}\" {}",path,err);
          ftp_stop();
          return TestStatus::Failed;
        }
      }
    }
    
  }
  ftp_stop();
  return TestStatus::Passed; 
}
fn get_ip_addrs() -> Result<Vec<String>, &'static str> {
  //get the ipaddr of the system
  let output = match Command::new("hostname").args(vec!["-I"]).output() {
    Ok(out) => out,
    Err(_) => {
      //return error enum
      return Err("Unable to retrieve hostname addresses.");
    }
  };
  //convert standard out to a string
  let output = String::from_utf8_lossy(&output.stdout);
  //split by whitespace
  let output = output.split_whitespace();
  let mut ipaddrs: Vec<String> = Vec::new();
  //push addresses into a string vector
  for addr in output {
    ipaddrs.push(addr.to_string());
  }
  //return Ok enum
  return Ok(ipaddrs);
}
pub fn servf(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ServF {
    event: Event {
      name:   name,
      desc:   "App used to create/setup tftp, ftp, or pxe services with minimal setup required.".to_string(),
      usage:  "Prompts you for: \
              \n\tFile service to start tftp, ftp, pxe \
              \n\tPrint machine ip addresses (optional)\n".to_string(),
      author: "n3wm4n".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*
 * tftp_start
 * 
 * @brief starts the salvum tftp service
 * @param none
 * @return bool will return true if the service successfully started. false if it didnt start successfully
 */
fn servf_tftp_start() -> bool {
  _pxe_stop();
  //make sure to stop the current tftp service running if it is running
  if servf_tftp_status() {
    print::println("The TFTP service is already running. Stopping the current ftp service...");
    servf_tftp_stop();
  }
  //path to the configuration file the tftp daemon will read
  let path_config = "/etc/default/tftpd-hpa".to_string();
  //create args vec
  let mut args: Vec<String> = Vec::new();
  let path_new_config = "cfg/servf/tftpd-hpa-template".to_string();
  args.push(path_new_config);
  let path_new_config = "cfg/servf/tftpd-hpa".to_string();
  args.push(path_new_config.clone());
  //copy template config file
  match Command::new("cp").args(args).output() {
    Ok(_) => {},
    Err(err) => {
      print::println(&format!("unable to copy configuration file. Error : {}",err));
      return false;
    }
  }
  //open up the new config file
  let mut config_file = match OpenOptions::new().write(true).append(true).open(path_new_config.clone()) {
    Ok(file) => file,
    Err(err) => {
      print::println(&format!("unable to create configuration file. Error : {}",err));
      return false;
    }
  };
  //get the absolute path to srv dir
  let path_srv = match servf_get_srv_path() {
    Ok(path) => path,
    Err(err) => {
      print::println(&format!("{}",err));
      return false;
    }
  };
  //append the config file with the path to the users source files
  match writeln!(config_file,"TFTP_DIRECTORY=\"{}\"",path_srv) {
    Ok(_) => {},
    Err(err) => {
      print::println(&format!("unable to append configurationg file : {} Error : {}",path_new_config,err));
      return false;
    }
  }
  //replace the old tftp config file with the new one
  let mut args: Vec<String> = Vec::new();
  args.push(path_new_config);
  args.push(path_config.clone());
  match Command::new("mv").args(args).output() {
    Ok(_) => {},
    Err(err) => {
      print::println(&format!("unable to replace configuration file. Error : {}",err));
      return false;
    }
  }
  //verify that the new config file was migrated
  if !Path::new(&path_config.clone()).exists() {
    println!("unable to replace configuration file.");
    return false;
  }
  //enable tftp daemon
  /*let args = vec!["enable","tftpd-hpa"];
  match Command::new("systemctl").args(args).output() {
    Ok(_) => {},
    Err(err) => {
      print::println(&format!("Failed to enable tftp daemon. Error : {}",err));
      return false;
    }
  }*/
  //start tftp daemon
  let args = vec!["tftpd-hpa","start"];
  match Command::new("service").args(args).output() {
    Ok(_) => {},
    Err(err) => {
      print::println(&format!("Failed to start tftp daemon. Error : {}",err));
      return false;
    }
  }
  //get the status of the daemon
  let args = vec!["tftpd-hpa","status"];
  match run_console_command(Command::new("systemctl").args(args)) {
    Ok(_) => {},
    Err(err) => {
      print::println(&format!("Failed to get status of tftp daemon. Error : {}",err));
      return false;
    }
  }
  if servf_tftp_status() {
    return true;
  }
  return false;
}

/*
 * tftp_stop
 * 
 * @brief stops the salvum tftp service
 * @param none
 * @return noe
 */
fn servf_tftp_stop() {
  let cur_mode = terminal::get_color_mode();
  terminal::set_color_mode(terminal::Mode::Magenta);
  //is the service running already or not?
  if servf_tftp_status() {
    print::println("An TFTP service was found still running. Sudo permissions will be needed to stop the service.");
    if !servf_stop_prompt("TFTP") {
      terminal::set_color_mode(cur_mode);
      return;
    }
    //create systemctl args
    let args = vec!["tftpd-hpa","stop"];
    //execute the stop command
    match Command::new("service").args(args).output() {
      Ok(_) => {
        print::println("TFTP service has successfully stopped.");
      },
      Err(err) => {
        print::println(&format!("Unable to stop tftp server. Error : {}",err));
        terminal::set_color_mode(cur_mode);
        return;
      }
    }
  }
  terminal::set_color_mode(cur_mode);
}

/*
 * tftp_status
 * 
 * @brief gets the current status of the tftp service
 * @param none
 * @return true if the service is running. false is the service is not running
 */
fn servf_tftp_status() -> bool {
  //create args vec for ps command
  let args = vec!["-eo","tty,pid,comm"];
  //launch ps command to get currently running proccesses
  let output = match Command::new("ps").args(args).output() {
    Ok(out) => out,
    Err(_) => {
      return false;
    }
  };
  let output_str = String::from_utf8_lossy(&output.stdout).to_string();
  //check to see if there is a tftp process running
  if output_str.contains("in.tftpd") {
    return true;
  }
  return false;
}

/*
 * ftp_start
 * 
 * @brief starts the salvum ftp service
 * @param none
 * @return bool will return true if the service successfully started. false if it didnt start successfully
 */
fn servf_ftp_start() -> bool {
  //make sure to stop the current ftp service running if it is running
  if servf_ftp_status() {
    print::println("The FTP service is already running. Stopping the current ftp service...");
    servf_ftp_stop();
  }
  //check to see if ftpqvlx exists
  let _ftp_usr_exist: bool = match get_user_by_name("ftpqvlx") {
    Some(_user) => true,
    None => false
  };
  //create ftpqvlx if it doesn't exist
  /*if !ftp_usr_exist {
    print::println("Salvum has detected that the 'ftpqvlx' user does not exist. Creating ther user...");
    //get the absolut srv path
    let path_srv: String = match servf_get_srv_path() {
      Ok(path) => path,
      Err(err) => {
        print::println(&format!("{}",err));
        return false;
      }
    };
    //run the useradd command with sudo permission
    let args: Vec<String> = vec!["-s".to_string(),"/bin/bash".to_string(),"-d".to_string(),path_srv,"ftpqvlx".to_string()];
    match Command::new("useradd").args(args).output() {
      Ok(_) => {
        print::println("Created 'ftpqvlx' user account successfully")
      },
      Err(err) => {
        print::println(&format!("'ftpqvlx' user does not exists. Unable to create ftp user. Error : {}",err));
        return false;
      }
    }
    //run the passwd command to set ftpqvlx password
    print::println("Please set the password for the 'ftpqvlx' user.");
    let args = vec!["ftpqvlx"];
    match run_console_command(Command::new("passwd").args(args)) {
      Ok(_) => {},
      Err(err) => {
        print::println(&format!("Unable to set ftpqvlx password. Error : {}",err));
        return false;
      }
    }
  }*/
  //enable ftp daemon
  //sudo systemctl enable pure-ftpd
  /*let args = vec!["enable","pure-ftpd"];
  match Command::new("systemctl").args(args).output() {
    Ok(_) => {},
    Err(err) => {
      print::println(&format!("Failed to enable ftp daemon. Error : {}",err));
      return false;
    }
  }*/
  //start ftp daemon
  //sudo systemctl start pure-ftpd
  let args = vec!["vsftpd","start"];
  match Command::new("service").args(args).output() {
    Ok(_) => {},
    Err(err) => {
      print::println(&format!("Failed to start ftp daemon. Error : {}",err));
      return false;
    }
  }
  //verify that daemon is running
  //sudo systemctl status pure-ftp
  let args = vec!["vsftpd","status"];
  match run_console_command(Command::new("service").args(args)) {
    Ok(_) => {},
    Err(err) => {
      print::println(&format!("Failed to get the status of the ftp daemon. Error : {}",err));
      return false;
    }
  }
  if servf_ftp_status() {
    print::println("You may connect to the FTP server using the 'ftpqvlx' user");
    return true;
  }
  return false;
}

/*
 * ftp_stop
 * 
 * @brief stops the salvum ftp service
 * @param none
 * @return noe
 */
fn servf_ftp_stop() {
  let cur_mode = terminal::get_color_mode();
  terminal::set_color_mode(terminal::Mode::Magenta);
  //is the service running already or not?
  if servf_ftp_status() {
    print::println("An FTP service was found still running. Sudo permissions will be needed to stop the service.");
    //prompt the user to confirm they want to stop ftp
    if !servf_stop_prompt("FTP") {
      terminal::set_color_mode(cur_mode);
      return;
    }
    //create systemctl args
    let args = vec!["vsftpd","stop"];
    //execute the stop command
    match Command::new("service").args(args).output() {
      Ok(_) => {
        print::println("FTP service has successfully stopped.");
      },
      Err(err) => {
        print::println(&format!("Unable to stop ftp server. Error : {}",err));
        terminal::set_color_mode(cur_mode);
        return;
      }
    }
  }
  terminal::set_color_mode(cur_mode);
}

/*
 * ftp_status
 * 
 * @brief gets the current status of the ftp service
 * @param none
 * @return true if the service is running. false is the service is not running
 */
fn servf_ftp_status() -> bool {
  //create args vec for ps command
  let mut args: Vec<String> = Vec::new();
  args.push("-eo".to_string());
  args.push("tty,pid,comm".to_string());
  //launch ps command to get currently running proccesses
  let output = match Command::new("ps").args(args).output() {
    Ok(out) => out,
    Err(_) => {
      return false;
    }
  };
  let output_str = String::from_utf8_lossy(&output.stdout).to_string();
  //check to see if there is a ftp process running
  if output_str.contains("vsftpd") {
    return true;
  }
  return false;
}
fn servf_pxe_start(silent: bool) -> bool {
  servf_tftp_stop();
  //scan pxe dir for boot files
  let mut boot_files: Vec::<String> = Vec::new();
  let boot_files_path = "srv/pxe";
  let dir_entries = match fs::read_dir(boot_files_path) {
    Ok(entries) => entries,
    Err(err) => {
      print::println(&format!("error reading directory. {}", err));
      debug::print_debug(&format!("pxe_start_test: failed to read dir. {}",err));
      return false;
    }
  };
  for entry in dir_entries {
    let entry = match entry {
      Ok(e) => e,
      Err(err) => {
        print::println(&format!("error iterating directory. {}", err));
        debug::print_debug(&format!("pxe_start_test: failed to iterate dir. {}",err));
        return false;
      }
    };
    let entry_str = match entry.file_name().into_string() {
      Ok(string) => string,
      Err(err) => {
        debug::print_debug(&format!("pxe_start_test: failed to read entry filename into a string {:?}", err));
        return false;
      }
    };
    
    boot_files.push(entry_str);
    
  }
  
  //execute: sudo pixiecore boot <boot_files>
  let command = "../../ext/pxecore/pxecore";
  let mut args = vec!["boot".to_string()];
  args.append(&mut boot_files);
  let mut pxecmd = Command::new(command);
  let mut silent_pxecmd = Command::new(command);
  let silent_cmd = silent_pxecmd.args(args.clone()).current_dir(boot_files_path).stdout(Stdio::null()).stderr(Stdio::null());
  let cmd = pxecmd.args(args).current_dir(boot_files_path);
  if silent {
    match silent_cmd.spawn() {
      Ok(child) => child,
      Err(err) => {
        print::println(&format!("failed to spawn pixiecore child. {}",err));
        debug::print_debug(&format!("pxe_start_test: failed to spawn pxecore child. {}",err));
        return false;
      }
    };
  }
  else {
    match cmd.spawn() {
      Ok(child) => child,
      Err(err) => {
        print::println(&format!("failed to spawn pixiecore child. {}",err));
        debug::print_debug(&format!("pxe_start_test: failed to spawn pxecore child. {}",err));
        return false;
      }
    };
  }
  //check for pixiecore proces
  let mut check = 0;
  let mut status = false;
  while check < 10 {
    check += 1;
    if servf_pxe_status() {
      //debug::print_debug(&format!("pxe_start_test: status check returned TRUE on attempt #{}",check.to_string()));
      //status = true;
      return true;
    }
    else {
      status = false;
    }
    std::thread::sleep(std::time::Duration::from_secs(1));
  }
  if !status {
    debug::print_debug("pxe_start_test: status check returned FALSE on all attempts");
  }
  return status;
}
fn servf_pxe_start_test() -> TestStatus {
  if !servf_pxe_start(true) {
    _pxe_stop();
    debug::print_debug("servf::on_test: failed to start pxe service");
    return TestStatus::Failed;
  }
  _pxe_stop();
  return TestStatus::Passed;
}
fn servf_pxe_stop_test() -> TestStatus {
  if !servf_pxe_start(true) || !_pxe_stop() {
    debug::print_debug("servf::on_test: failed to stop pxe service");
    return TestStatus::Failed;
  }
  return TestStatus::Passed;
}
fn servf_pxe_stop() {
  if servf_pxe_status() {
    print::println("Salvum is attempting to stop the pxe server proccess.");
    if !servf_stop_prompt("PXE") {
      return;
    }
    _pxe_stop();
  }
  
}
fn _pxe_stop() -> bool {
  let arg = "pxecore";
  let pid_output = match Command::new("pidof").arg(arg).output() {
    Ok(output) => {
      if output.status.success() {
        output
      }
      else {
        return false;
      }
    },
    Err(err) => {
      print::println(&format!("{}",err));
      return false;
    }
  };
  let pid_str = String::from_utf8_lossy(&pid_output.stdout);
  let pid_str = pid_str.trim();
  
  let pid_i32: i32 = match pid_str.parse() {
    Ok(pid) => pid,
    Err(err) => {
      print::println(&format!("Unable to parse process id string into an integer. {}",err));
      return false;
    }
  };
  //kill proccess id
  if pid_i32 > 0 {
    let clam_pid = Pid::from_raw(pid_i32);
    let sig_kill = Signal::SIGKILL;
    match kill(clam_pid, sig_kill) {
      Ok(()) => {return true;},
      Err(err) => {
        print::println(&format!("{}",err));
        return false;
      }
    };
  }
  return false;
}
fn servf_pxe_status() -> bool {
  //pidof pxecore
  let command = "pidof";
  let arg = "pxecore";
  let _output = match Command::new(command).arg(arg).output() {
    Ok(out) => {
      if out.status.success() {
        return true;
      }
      else {
        //debug::print_debug("pxe: pidof returned with nonzero exit status");
        return false;
      }
    },
    Err(err) => {
      print::println(&format!("failed to execute pidof. {}",err));
      debug::print_debug(&format!("failed to execute pidof. {}",err));
      return false;
    }
  };
}

/*
 * stop_all
 * 
 * @brief stops all servf services
 * @param none
 * @return none
 */
pub fn servf_stop_all() {
  servf_tftp_stop();
  servf_ftp_stop();
  servf_pxe_stop();
}

/*
 * stop_prompt
 * 
 * @brief prints out a prompt confirming that the user actually wants to stop a service
 * @param proto string name of the protocol the user is stopping
 * @return true if the user selects yes. false if the user selects no
 */
fn servf_stop_prompt(proto: &str) -> bool {
  let msg: &str = &format!("Are you sure you'd like to stop the {} service?",proto);
  return alerts::confirm_task(msg);
}

/*
 * print_status
 * 
 * @brief prints the current status of all servf services
 * @param none
 * @return none
 */
fn servf_print_status() {
  let tftp_status: String = match servf_tftp_status() {
    true => "running".to_string(),
    false => "disabled".to_string()
  };
  let ftp_status: String = match servf_ftp_status() {
    true => "running".to_string(),
    false => "disabled".to_string()
  };
  let pxe_status: String = match servf_pxe_status() {
    true => "running".to_string(),
    false => "disabled".to_string()
  };

  print::println(&format!("SERVF STATUS\ntftp : {}\nftp  : {}\npxe  : {}",tftp_status,ftp_status,pxe_status));
}

/*
 * get_srv_path
 * 
 * @brief gets the absolute path to the salvum srv dir
 * @param none
 * @return Result<String, &'static str>. 
 *    returns the srv path as Ok() enum if no error. 
 *    returns Err() enum if an error occurs
 */
fn servf_get_srv_path() -> Result<String, &'static str> {
  //get the current path
  let mut current_path = match env::current_dir() {
    Ok(dir) => dir,
    Err(_err) => {
      return Err("Unable to get current path.");
    }
  };
  //append the path with the srv directory
  current_path.push("srv/");
  let path_srv = current_path.to_string_lossy();
  //return the path 
  return Ok(path_srv.to_string());
}
