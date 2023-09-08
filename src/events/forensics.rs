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
 * authors: $t@$h, r00r00, n3wmAn
 */
 use crate::events::*;
 use regex::Regex;
 //use std::str::pattern;
 //use crate::alerts::print_notice;
 
 /******************************* FORENSIC TOOLS *********************************/
 pub struct Forensics { event: Event }
 impl Eventable for Forensics {
   fn on_init(&self) -> Vec<String> {
     print::println("Forensic tools");
     return Vec::new();
   }
   fn get_event(&self) -> &Event { return &self.event; }
 } 
 pub fn forensics(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
   Box::new(Forensics {
     event: Event {
       name:   name,
       desc:   "Forensic analysis tools".to_string(),
       usage:  "".to_string(),
       author: "".to_string(),
       parent: parent,
       links:  links,
       easyrun: false,
       secure: false,
     }
   })
 }

/*********************************** SLEUTHKIT ***********************************/
pub struct SleuthKit { event: Event }
impl Eventable for SleuthKit {
  fn on_init(&self) -> Vec<String> {
    print::println("Sleuthkit tools");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn sleuthkit(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SleuthKit {
    event: Event {
      name:   name,
      desc:   "Sleuthkit digital forensics tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      parent: parent,
      links:  links,
      easyrun: false,
      secure: false,
    }
  })
}
/*********************************** MMLS ***********************************/
pub struct Mmls { event: Event }
impl Eventable for Mmls {
  fn on_init(&self) -> Vec<String> {
    let image_path = prompt_in_event!("Mmls>","Path to image: ");
    return vec![image_path];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    let mmls_output = match run_command(Command::new("ext/mmls/mmls").args(args)) {
      Ok(output) => output,
      Err(err) => {
        return format!("failed to execute mmls. {}\n", err);
      }
    };
    let mmls_output_str = String::from_utf8_lossy(&mmls_output.stdout);
    //print::println(&mmls_output_str);
    print::print_custom(&mmls_output_str,"brightgreen");
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let test = "tst/mmls/inp/image.img"; // This doesnt resolve
    let atf_path = "tst/mmls/atf/mmls_image";
    let mmls_output = match Command::new("ext/mmls/mmls").arg(test).output() {
      Ok(output) => output,
      Err(err) => {
        debug::print_debug(&format!("mmls::on_test: failed to execute mmls. {}", err));
        return TestStatus::Failed;
      }
    };

    let mmls_output_str = match String::from_utf8(mmls_output.stdout) {
      Ok(string) => string,
      Err(err) => {
        debug::print_debug(&format!("mmls::on_test: failed to read output to a string. {}", err));
        return TestStatus::Failed;
      }
    };

    let atf_str = match fs::read_to_string(atf_path) {
      Ok(string) => string,
      Err(err) => {
        debug::print_debug(&format!("mmls::on_test: failed to read artifact to a string. {}", err));
        return TestStatus::Failed;
      }
    };

    if mmls_output_str != atf_str {
      debug::print_debug("mmls::on_test: output does not match artifact.");
      return TestStatus::Failed;
    }

    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn mmls(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Mmls {
    event: Event {
      name:   name,
      desc:   "Displays layout of partitions in a volume system include tables and labels.".to_string(),
      usage:  "Prompts you for:\n\tImage file to ls\n".to_string(),
      author: "Brian Carrier & Basis Technology".to_string(),
      parent: parent,
      links:  links,
      easyrun: false,
      secure: false,
    }
  })
}
/*********************************** MMSTAT ***********************************/
pub struct Mmstat { event: Event }
impl Eventable for Mmstat {
  fn on_init(&self) -> Vec<String> {
    let image_path = prompt_in_event!("Mmstat>","Path to image: ");
    return vec![image_path];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    let mmstat_output = match run_command(Command::new("ext/mmstat/mmstat").args(args)) {
      Ok(output) => output,
      Err(err) => {
        return format!("failed to execute mmstat. {}\n", err);
      }
    };
    let mmstat_output_str = String::from_utf8_lossy(&mmstat_output.stdout);
    print::print_custom(&mmstat_output_str,"brightgreen");
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let test = "tst/mmstat/inp/image.img"; // This doesnt resolve
    let atf_path = "tst/mmstat/atf/mmstat_image";
    let mmstat_output = match Command::new("ext/mmstat/mmstat").arg(test).output() {
      Ok(output) => output,
      Err(err) => {
        debug::print_debug(&format!("mmstat::on_test: failed to execute mmstat. {}", err));
        return TestStatus::Failed;
      }
    };

    let mmstat_output_str = match String::from_utf8(mmstat_output.stdout) {
      Ok(string) => string,
      Err(err) => {
        debug::print_debug(&format!("mmstat::on_test: failed to read output to a string. {}", err));
        return TestStatus::Failed;
      }
    };

    let atf_str = match fs::read_to_string(atf_path) {
      Ok(string) => string,
      Err(err) => {
        debug::print_debug(&format!("mmstat::on_test: failed to read artifact to a string. {}", err));
        return TestStatus::Failed;
      }
    };

    if mmstat_output_str != atf_str {
      debug::print_debug(mmstat_output_str);      
      debug::print_debug("mmstat::on_test: output does not match artifact.");
      return TestStatus::Failed;
    }

    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn mmstat(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Mmstat {
    event: Event {
      name:   name,
      desc:   "Displays details about the volume system (partition tables)".to_string(),
      usage:  "Prompts you for:\n\tImage file to stat\n".to_string(),
      author: "Brian Carrier & Basis Technology".to_string(),
      parent: parent,
      links:  links,
      easyrun: false,
      secure: false,
    }
  })
}


/*********************************** HFIND ***********************************/
pub struct Hfind { event: Event }
impl Eventable for Hfind {
  fn on_init(&self) -> Vec<String> {
    let mut dir_str = prompt_in_event!("Hfind>","Path of directory to check: ");
    util::misc::reltoabs(&mut dir_str);
    let dir_path = Path::new(&dir_str);
    if !dir_path.exists() {
      print::println("Path does not exist.");
      return Vec::new();
    }
    let dir_name = match dir_path.file_name() {
      Some(name) => {
        match name.to_str() {
          Some(string) => string,
          None => {
            print::println("Invalid directory path.");
            return Vec::new();    
          }
        }
      },
      None => {
        print::println("Invalid directory path.");
        return Vec::new();
      }
    };
    let hash_path_str = format!("out/hfind/{}.md5",dir_name);
    let hash_path = Path::new(&hash_path_str);
    match visit_dirs(dir_path, hash_path) {
      Ok(()) => {},
      Err(err) => {
        print::println(&format!("Error recursing dir. {}", err));
        return Vec::new();
      }
    };
    
    let hash_lines = match fs::read_to_string(hash_path) {
      Ok(string) => string,
      Err(err) => {
        print::println(&format!("Error reading md5 file. {}", err));
        return Vec::new();
      }
    };
    let hash_lines = hash_lines.split('\n');
    let mut hfind_check_output: Vec<String> = Vec::new();
    for line in hash_lines {
      let (hash_found, hfind_output) = find_hash(line.to_string());
      hfind_check_output.push(hfind_output.clone());
      
      if hash_found {
        print::print(&hfind_output);
      }
      else {
        print::print_custom(&format!("{}\n",&hfind_output),"brightgreen");
      }
    }
    return Vec::new();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() == 0 { return String::from(""); }
    if args.len() > 1 { return self.event.usage.clone(); }
    let hfind_output = match run_command(Command::new("hfind").args(vec!["-e".to_string(),"NSRLFile.txt".to_string(),args[0].clone()]).current_dir("ext/util/hfind_db")) {
      Ok(output) => output,
      Err(err) => {
        return format!("Failed to execute hfind. {}\n",err);
      }
    };
    let mut hfind_output_string = match String::from_utf8(hfind_output.stdout) {
      Ok(string) => string,
      Err(err) => {
        return format!("Failed to read output to a string. {}\n",err);
      }
    };
    hfind_output_string.push_str(&String::from_utf8_lossy(&hfind_output.stderr));
    return hfind_output_string;
  }
  fn on_test(&self) -> TestStatus {
    //hfind DBFILE HASHVALUE
    let tests = vec![
      vec!["tst/hfind/inp/hashdb.txt","aa625c36ad5e139c0b130e859859fafe","tst/hfind/atf/hfind_hashdb_found"],
      vec!["tst/hfind/inp/hashdb.txt","aa625c36ad5e139c0b130e859859faff","tst/hfind/atf/hfind_hashdb_notfound"]
    ];
    for mut test in tests {
      let atf_path = match test.pop() {
        Some(string) => string,
        None => {
          debug::print_debug("hfind::on_test: invalid test vector.");
          return TestStatus::Failed;
        }
      };
      let atf_str = match fs::read_to_string(atf_path) {
        Ok(string) => string,
        Err(err) => {
          debug::print_debug(&format!("hfind::on_test: failed to read artifact file to a string. {}",err));
          return TestStatus::Failed;
        }
      };
      let hfind_output = match Command::new("hfind").args(test).output() {
        Ok(output) => output,
        Err(err) => {
          debug::print_debug(&format!("hfind::on_test: failed to execute hfind. {}",err));
          return TestStatus::Failed;
        }
      };
      let hfind_output_str = String::from_utf8_lossy(&hfind_output.stdout);
      if atf_str != hfind_output_str {
        debug::print_debug("hfind::on_test: output does not match artifact");
        return TestStatus::Failed;    
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn hfind(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Hfind {
    event: Event {
      name:   name,
      desc:   "Looks up MD5 or SHA-1 hashes in an NSRL database using binary search.".to_string(),
      usage:  "hfind <hash>\n\nExample: hfind aa625c36ad5e139c0b130e859859fafe\n".to_string(),
      author: "Brian Carrier & Basis Technology".to_string(),
      parent: parent,
      links:  links,
      easyrun: false,
      secure: false,
    }
  })
}
/*
  brief: this function recursively iterates through a directory path 
    and hashes each file it comes across. hash values are written
    to hash_file in the format "<hash_value>  <file_name>\n"
  
  dir: a path to a dir which will be recursed

  hash_file: path in which each file's hash value will be written too
*/
fn visit_dirs(dir: &Path, hash_file: &Path) -> std::io::Result<()> {
  if dir.is_dir() {
    let dir_entries = match fs::read_dir(dir) {
      Ok(entries) => entries,
      Err(err) => {
        print::println(&format!("Error reading directory. {}", err));
        return Err(err);
      }
    };
    for entry in dir_entries {
      let entry = match entry {
        Ok(e) => e,
        Err(err) => {
          print::println(&format!("Error iterating directory. {}", err));
          return Err(err);
        }
      };
      let entry_path = entry.path();
      if entry_path.is_dir() {
        match visit_dirs(&entry_path, hash_file) {
          Ok(_) => {},
          Err(err) => {
            print::println(&format!("Error recursing directory. {}", err));
            return Err(err);
          }
        }
      }
      else {
        match hash_entry(&entry, hash_file) {
          Ok(()) => {},
          Err(err) => {
            print::println(&format!("Error hashing entry. {}", err));
            return Err(err);
          }
        };
      }
    }
  }
  return Ok(());
}
/*
  brief: hashes an entry and writes the output to hash_file
  
  entry: reference to a directory entry that is a file that will be hashed
  
  hash_file: reference to a file path in which hash values will be written to
*/
fn hash_entry(entry: &fs::DirEntry, hash_file: &Path) -> std::io::Result<()> {

  if entry.path().is_dir() {
    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Directory found. Skipping hash."));
  }

  let mut file = match std::fs::OpenOptions::new().create(true).append(true).open(hash_file) {
    Ok(file) => file,
    Err(err) => {
      print::println(&format!("Unable to open hashes file. {}", err));
      return Err(err);
    }
  };

  let command = "md5sum";
  let entry_path = entry.path();
  let arg = match entry_path.to_str() {
    Some(string) => string,
    None => {
      print::println(&format!("Invalid entry path. {:?}", entry));
      return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid entry path."));
    }
  };
  let md5_output = match Command::new(command).arg(arg).output() {
    Ok(output) => output,
    Err(err) => {
      print::println(&format!("Unable to execute md5sum. {}", err));
      return Err(err);
    }
  };

  match file.write_all(&md5_output.stdout) {
    Ok(()) => {
      return Ok(());
    },
    Err(err) => {
      print::println(&format!("Unable to write hash to file. {}", err));
      return Err(err);
    }
  };
}
/*
  brief: this function takes in an md5 entry and searches for the hash value in the NSRL db
  
  md5_entry: a single line from an md5 hash file. formatted as "<hash_value>  <file_name>"
  
  return (bool, String): this function returns a tuple. returns true if the computed hash 
    value was found in the NSRLFile db. returns a string that will be printed to the console
*/
fn find_hash(md5_entry: String) -> (bool, String) {
  let mut split = md5_entry.split("  ");
  let hash_literal = match split.next() {
    Some(string) => string,
    None => {
      return (false, "".to_string());
    }
  };
  //let path_hashed = match split.next() {
  //  Some(string) => string,
  //  None => {
  //    return (false, "".to_string());
  //  }
  //};
  //TODO delete this print
  //println!("literal: {}\npath: {}", hash_literal, path_hashed);
  let working_dir = "ext/util/hfind_db";
  let args = vec!["NSRLFile.txt",hash_literal];
  let hfind_output = match Command::new("hfind").args(args).current_dir(working_dir).output() {
    Ok(output) => output,
    Err(err) => {
      print::println(&format!("Unable to execute hfind. {}", err));
      return (false, "".to_string());
    }
  };
  let hfind_stdout_str = String::from_utf8_lossy(&hfind_output.stdout);
  let mut hfind_split = hfind_stdout_str.split("\t");
  hfind_split.next();
  if hfind_stdout_str.contains("Hash Not Found") {
    return (false, format!("{}  File hash not found in NSRL db", md5_entry));
  }
  else {
    let nsrl_file = match hfind_split.next() {
      Some(string) => string,
      None => {
        //print::println("invalid nsrl md5 entry.");
        return (false, "".to_string());
      }
    };
    return (true, format!("{}  {}", md5_entry, nsrl_file));
  }
}

/******************************* FCAT *********************************/
pub struct FCat { event: Event }
impl Eventable for FCat {
  fn on_init(&self) -> Vec<String> {
    // Prompt for image file
    let path_in_image = prompt_in_event!("FCat>", "Path to File in Image: ");
    
    // Prompt for image file
    let image_file = prompt_in_event!("FCat>", "Image File: ");

    return vec![path_in_image, image_file];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 2 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_command(Command::new("fcat").args(args)));

    // Print output
    log::log("FCat", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["/home/hello.txt", "tst/fcat/inp/image.img", "tst/fcat/atf/atf.txt"]];
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("fcat").args(test[..2].to_vec()).output());

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[2]));

      // Compare
      if String::from_utf8_lossy(&output.stdout) != file_str {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn fcat(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(FCat {
    event: Event {
      name:   name,
      desc:   "Prints out the contents of a file within an image".to_string(),
      usage:  "Prompts you for:\n\
                \tPath to a file within an image (String)\n\
                \tPath to an image (String)\n".to_string(),
      author: "Brian Carrier & Basis Technology".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/******************************* FFIND *********************************/
pub struct FFind { event: Event }
impl Eventable for FFind {
  fn on_init(&self) -> Vec<String> {
    // Prompt for image file
    let image_file = prompt_in_event!("FFind>", "Image File: ");

    // Prompt for inode
    let inode = prompt_in_event!("FFind>", "inode: ");

    return vec![image_file, inode];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 2 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_command(Command::new("ffind").args(args)));

    // Print output
    log::log("FFind", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["tst/ffind/inp/image.img", "16", "tst/ffind/atf/atf.txt"]];
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("ffind").args(test[..2].to_vec()).output());

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[2]));

      // Compare
      if String::from_utf8_lossy(&output.stdout) != file_str {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ffind(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(FFind {
    event: Event {
      name:   name,
      desc:   "Prints out the name of a file/directory for a given inode".to_string(),
      usage:  "Prompts you for:\n\
                \tPath to an image (String)\n\
                \tAn inode within an image (String)\n".to_string(),
      author: "Brian Carrier & Basis Technology".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/******************************* FLS *********************************/
pub struct Fls { event: Event }
impl Eventable for Fls {
  fn on_init(&self) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    
    // Prompt for image file
    let image_file = prompt_in_event!("Fls>", "Image File: ");
    args.push(image_file);

    // Prompt for inode
    let inode = prompt_in_event!("Fls>", "inode (blank for root): ");
    if inode != "" {
      args.push(inode);
    }

    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 2 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_command(Command::new("fls").args(args)));

    // Print output
    log::log("Fls", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["tst/fls/inp/image.img", "12", "tst/fls/atf/atf.txt"]];
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("fls").args(test[..2].to_vec()).output());

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[2]));

      // Compare
      if String::from_utf8_lossy(&output.stdout) != file_str {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn fls(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Fls {
    event: Event {
      name:   name,
      desc:   "Lists out the files and directories for a given inode".to_string(),
      usage:  "Prompts you for:\n\
                \tPath to an image (String)\n\
                \tAn inode within an image (Optional String)\n".to_string(),
      author: "Brian Carrier & Basis Technology".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/******************************* FSSTAT *********************************/
pub struct FsStat { event: Event }
impl Eventable for FsStat {
  fn on_init(&self) -> Vec<String> {
    // Prompt for image file
    let image_file = prompt_in_event!("FsStat>", "Image File: ");

    return vec![image_file];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_command(Command::new("fsstat").args(args)));

    // Print output
    log::log("FsStat", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["-f", "ext3", "tst/fsstat/inp/image.img", "tst/fsstat/atf/atf.txt"]];
    let filter_regex = match Regex::new(r"Last (Written|Checked|Mounted) at: ([0-9]+)") {
      Ok(regex) => regex,
      Err(err) => {
        debug::print_debug(&format!("fsstat::on_test: failed to create filter regex. {}",err));
        return TestStatus::Failed;
      }
    };
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("fsstat").args(test[..3].to_vec()).output());
      // Read file
      let atf_str = simple_test_match!(fs::read_to_string(test[3]));
      // Compare
      let output_str = String::from_utf8_lossy(&output.stdout);
      let output_lines = output_str.split("\n");
      let atf_lines = atf_str.split("\n");
      let atf_lines: Vec<&str> = atf_lines.collect();
      for (index, line) in output_lines.enumerate() {
        if (!filter_regex.is_match(line)) && (line != atf_lines[index]) {
          return TestStatus::Failed;
        }
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn fsstat(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(FsStat {
    event: Event {
      name:   name,
      desc:   "Prints out details of the file system of an image file".to_string(),
      usage:  "Prompts you for:\n\
                \tPath to an image (String)\n".to_string(),
      author: "Brian Carrier & Basis Technology".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/********************************* IMG_STAT *********************************/
pub struct ImgStat { event: Event }
impl Eventable for ImgStat {
  fn on_init(&self) -> Vec<String> {
    // Prompt for image file
    let image_file = prompt_in_event!("ImgStat>", "Image File: ");

    return vec![image_file];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_command(Command::new("img_stat").args(args)));

    // Print output
    log::log("ImgStat", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["tst/img_stat/inp/image.img", "tst/img_stat/atf/atf.txt"]];
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("img_stat").arg(test[0]).output());

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[1]));

      // Compare
      if String::from_utf8_lossy(&output.stdout) != file_str {
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn img_stat(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ImgStat {
    event: Event {
      name:   name,
      desc:   "Prints out details of an image file".to_string(),
      usage:  "Prompts you for:\n\
                \tPath to an image (String)\n".to_string(),
      author: "Brian Carrier & Basis Technology".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/******************************* ISTAT *********************************/
pub struct IStat { event: Event }
impl Eventable for IStat {
  fn on_init(&self) -> Vec<String> {
    // Prompt for image file
    let image_file = prompt_in_event!("IStat>", "Image File: ");
    
    // Prompt for image file
    let inode = prompt_in_event!("IStat>", "inode: ");
    
    return vec![image_file, inode];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 2 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_command(Command::new("istat").args(args)));

    // Print output
    log::log("IStat", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["tst/istat/inp/image.img", "16", "tst/istat/atf/atf.txt"]];

    let filter_regex = match Regex::new(r"(Accessed|File Modified|Inode Modified):([ \t]+)([0-9\- :]+)") {
      Ok(regex) => regex,
      Err(err) => {
        debug::print_debug(&format!("fsstat::on_test: failed to create filter regex. {}",err));
        return TestStatus::Failed;
      }
    };
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("istat").args(test[..2].to_vec()).output());

      // Read file
      let atf_str = simple_test_match!(fs::read_to_string(test[2]));

      // Compare
      let output_str = String::from_utf8_lossy(&output.stdout);
      let output_lines = output_str.split("\n");
      let atf_lines = atf_str.split("\n");
      let atf_lines: Vec<&str> = atf_lines.collect();
      for (index, line) in output_lines.enumerate() {
        //if filter_regex.is_match(line) {
        //  println!("{}\nline matches regex",line);
        //}
        if (!filter_regex.is_match(line)) && (line != atf_lines[index]) {
          return TestStatus::Failed;
        }
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn istat(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(IStat {
    event: Event {
      name:   name,
      desc:   "Prints out the details of an inode within an image file".to_string(),
      usage:  "Prompts you for:\n\
                \tPath to an image (String)\n\
                \tAn inode within an image (String)\n".to_string(),
      author: "Brian Carrier & Basis Technology".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** FIWALK ***********************************/
pub struct FiWalk { event: Event }
impl Eventable for FiWalk {
  fn on_init(&self) -> Vec<String> {
    let iso_path = prompt_in_event!("FiWalk>","Path to iso file: ");
    return vec![iso_path];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 { return self.event.usage.clone(); }
    let command = "ext/fiwalk/fiwalk";
    //format the output path string
    let path_output = Path::new(&args[0]);
    let file_name = match path_output.file_name() {
      Some(osstr) => osstr,
      None => {
        return format!("Invalid file path\n");
      }
    };
    let file_name = match file_name.to_str() {
      Some(string) => string,
      None => {
        return format!("unable to read file name to a string\n");
      }
    };
    let output_file_path = format!("out/fiwalk/{}.fiwalk",file_name);
    let fiwalk_output = match run_command(Command::new(command).args(args)) {
      Ok(output) => output,
      Err(err) => {
        return format!("failed to execute fiwalk. {}\n", err);
      }
    };
    
    let fiwalk_output_str = String::from_utf8_lossy(&fiwalk_output.stdout);
    print::print_custom(&format!("{}",&fiwalk_output_str),"gold");
    match File::create(output_file_path.clone()) {
      Ok(mut file) => {
        match file.write_all(&fiwalk_output.stdout) {
          Ok(_) => {}
          Err(err) => {
            return format!("failed to write to output file. {}\n{}\n",err,output_file_path);
          }
        };
        return format!("output written to: {}\n", output_file_path);
      },
      Err(err) => {
        return format!("failed to write to output file. {}\n{}\n",err,output_file_path);
      }
    }
  }
  fn on_test(&self) -> TestStatus {
    let atf_path = "tst/fiwalk/atf/test.iso.fiwalk";
    let inp_path = "tst/fiwalk/inp/test.iso";
    let command = "ext/fiwalk/fiwalk";
    let arg = inp_path.clone();

    //execute fiwalk
    let fiwalk_output = match Command::new(command).arg(arg).output() {
      Ok(output) => output,
      Err(err) => {
        debug::print_debug(&format!("fiwalk::on_test: failed to execute fiwalk. {}",err));
        return TestStatus::Failed;
      }
    };

    //read output to string
    let fiwalk_output_str = String::from_utf8_lossy(&fiwalk_output.stdout);

    //read atf to string
    let atf_str = match fs::read_to_string(atf_path) {
      Ok(string) => string,
      Err(err) => {
        debug::print_debug(&format!("fiwalk::on_test: failed to read atf to a string. {}",err));
        return TestStatus::Failed;
      }
    };
    let _filter_regex = match Regex::new(r"(start_time|# clock|utime|stime|maxrss|minflt|majflt|clocktime|# stop_time|oublock|crtime|crtime_txt):([ \t]+)") {
      Ok(regex) => regex,
      Err(err) => {
        debug::print_debug(&format!("fsstat::on_test: failed to create filter regex. {}",err));
        return TestStatus::Failed;
      }
    };

    //compare atf to output
    let output_lines = fiwalk_output_str.split("\n");
    let output_lines: Vec<&str> = output_lines.collect();
    let atf_lines = atf_str.split("\n");
    let atf_lines: Vec<&str> = atf_lines.collect();
    /*for (index, line) in output_lines.enumerate() {
      //if filter_regex.is_match(line) {
      //  println!("{}\nline matches regex",line);
      //}
      if !filter_regex.is_match(line) && line != atf_lines[index] {
        debug::print_debug(format!("fiwalk::on_test: output doesnt match atf\noutput:{}\natf:{}",line,atf_lines[index]));
        return TestStatus::Failed;
      }
    }*/
    if output_lines.len() != atf_lines.len() {
        debug::print_debug("fiwalk::on_test: output doesnt match atf\n");
        return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn fiwalk(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(FiWalk {
    event: Event {
      name:   name,
      desc:   "Processes a disk image and outputs Digital Forensics XML, ARFF, or text.".to_string(),
      usage:  "Prompts you for:\n\tiso file to walk\n".to_string(),
      author: "Brian Carrier & Basis Technology".to_string(),
      parent: parent,
      links:  links,
      easyrun: false,
      secure: false,
    }
  })
}

