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
 * util.rs -> helpful code for salvum 
 *
 * authors: $t@$h, r00r00, n3wmAn
 */

#[macro_export]
macro_rules! simple_match {
  ($func:expr) => {
    {
      match $func {
        Ok(out) => out,
        Err(err) => {
          return format!("Error: {}\n", err);
        }
      }
    }
  };
}

#[macro_export]
macro_rules! simple_test_match {
  ($func:expr) => {
    {
      match $func {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("Error: {}", err));
          return TestStatus::Failed;
        }
      }
    }
  };
}

pub mod misc {
  use std::env::current_dir;
  use crate::print;
  //use std::path::{PathBuf,Path};

  pub fn reltoabs(rel: &mut String) -> bool {
    let mut converted: bool = false;
    //absolute paths contain / as the first character
    if !(&rel[..1] == "/") {
      //get the current working path
      let current_path = match  current_dir() {
        Ok(dir) => dir,
        Err(_) => {
          print::println(&format!("Found relative path and failed to convert to absolute : {}",rel));
          return converted;
        }
      };
      //append current absolute path with relative path
      let mut current_path = current_path.to_string_lossy().to_string();
      current_path.push('/');
      rel.insert_str(0,&current_path);
      converted = true;
    }
    return converted;
  }

  pub fn _basename(path: String) -> String {
    let remove_extn = path.split(".").collect::<Vec<&str>>()[0];
    let remove_prnt = remove_extn.split("/").collect::<Vec<&str>>();
    let base_name = remove_prnt[remove_prnt.len() - 1];
    return base_name.to_string(); 
  }

  pub fn cleanup(filepath: &str) {
    let _ = match std::process::Command::new("rm").args(vec!["-rf", filepath]).output() {
      Ok(_) => {}
      Err(err) => {
        println!("Error: {}", err);
      }
    };
  }

  pub fn write_file(message: String, out_file: String) {
    use std::fs::File;
    use std::io::Write;
    let mut file = match File::create(&out_file) {
      Ok(x) => x,
      Err(e) => { println!("Error creating file: {}", e); return; }
    };
    match write!(file,"{}",message) {
      Ok(x) => x,
      Err(e) => { println!("Error writing to file: {}",e); return; }
    };
  }

  pub fn get_files_in_path(path: String) -> Vec<String> {
    let output = match std::process::Command::new("ls").args(vec!["-av", &path]).output() {
      Ok(out) => out,
      Err(_) => { return Vec::new(); }
    };
    let ls_str = String::from_utf8_lossy(&output.stdout);
    let mut ls_vec: Vec<String> = ls_str.split("\n").map(|s| {
      if s.contains(" ") {
        return format!("\'{}\'", s);
      }
      return s.to_string();
    }).collect();

    // Get rid of '.' and '..' if they exist
    if ls_vec.len() <= 2 {
      return Vec::new();
    }
    ls_vec.drain(..2);

    // Get rid of empty strings
    let mut _ls_vec = Vec::new();
    for file in ls_vec {
      if file != "" {
        _ls_vec.push(file);
      }
    }

    return _ls_vec;
  }

  /*pub fn trim_stem(path_str: String) -> Result<PathBuf, &'static str> {
    let path = Path::new(&path_str);
    let mut trimmed_path = PathBuf::from(path);
    if path.extension() != None {
      trimmed_path.pop();
    }
    return Ok(trimmed_path);
  }*/
}

pub mod pipes {
  use std::process::{Stdio,Command,Child};
  use std::os::unix::io::{FromRawFd, AsRawFd};
  use std::io::{Error, ErrorKind, Result};

  /// Data structure used to hold processes
  /// and allows for the chaining of commands
  pub struct Pipe {
    child: Result<Child>,
  }
  impl Pipe {
    /// Creates a new `Pipe` by taking in a command
    /// as input. An empty string as input will
    /// cause the eventual end of the piping to have
    /// an error returned. Make sure you place in an
    /// actual command.
    pub fn new(command: &str) -> Pipe {
      let mut split = command.split_whitespace();
      let command = match split.next() {
        Some(x) => x,
        None => return pipe_new_error("No command as input"),
      };
      let args  = split.collect::<Vec<&str>>();

      Pipe {
        child: Command::new(command)
          .args(args.as_slice())
          .stdout(Stdio::piped())
          .spawn(),
      }
    }

    /// This is used to chain commands together. Use this for each
    /// command that you want to pipe.
    pub fn then(self, command: &str) -> Pipe {
      let stdout = match self.child {
        Ok(child) => match child.stdout {
          Some(stdout) => stdout,
          None => return pipe_new_error("No stdout for a command"),
        },
        Err(e) => return pipe_error(Err(e)),
      };

      let mut split = command.split_whitespace();
      let command = match split.next() {
          Some(x) => x,
          None => return pipe_new_error("No command as input"),
      };
      let args  = split.collect::<Vec<&str>>();
      let stdio = unsafe{ Stdio::from_raw_fd(stdout.as_raw_fd()) };

      Pipe {
        child: Command::new(command)
          .args(args.as_slice())
          .stdout(Stdio::piped())
          .stdin(stdio)
          .spawn(),
      }
    }

    /// Return the `Child` process of the final command that
    /// had data piped into it.
    pub fn finally(self) -> Result<Child> {
      self.child
    }

  }

  /// Helper method to generate a new error from a string
  /// but have it be a `Pipe` so that it can be passed through
  /// the chain.
  fn pipe_new_error(error: &str) -> Pipe {
    Pipe {
      child: Err(Error::new(ErrorKind::Other, error)),
    }
  }

  /// Helper method used to pass the error down the chain by creating
  /// a new pipe with the error passed in.
  fn pipe_error(error: Result<Child>) -> Pipe {
    Pipe {
      child: error,
    }
  }
}

pub mod security {
  use users::get_user_by_name;
	use users::switch::set_current_uid;
	use std::process::Command;
	
  pub fn deprivilege() {
		let slm_usr = get_user_by_name("salvum").unwrap();
		let slm_usr_id = slm_usr.uid();
		set_current_uid(slm_usr_id).unwrap();
	}

	pub fn create_salvum_user() {
		let slm_usr_exists: bool = match get_user_by_name("salvum") {
					Some(_user) => true,
					None => false
		};
		if !slm_usr_exists {
			//run the useradd command with sudo permission
			let args: Vec<String> = vec!["-s".to_string(),"/bin/bash".to_string(),"-M".to_string(),"-U".to_string(),"salvum".to_string()];
			match Command::new("useradd").args(args).output() {
				Ok(_) => {},
				Err(err) => { 
					println!("Salvum main failed to create Salvum user: {}",err);
					return;
				}    
			}    
		}
	}
}
