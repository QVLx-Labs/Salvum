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
 * codeanalyzers.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;
use std::ffi::OsStr;
   
 /*********************************** CODE ANALYZER ***********************************/
pub struct CodeAnalyzer { event: Event }
impl Eventable for CodeAnalyzer {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Code Analyzers");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn codeanalyzer(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(CodeAnalyzer {
    event: Event {
      name:   name,
      desc:   "Code Analyzing Tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** STATIC CODE ANALYZER ***********************************/
pub struct StaticCode { event: Event }
impl Eventable for StaticCode {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Static Code Analyzers");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn staticcode(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(StaticCode {
    event: Event {
      name:   name,
      desc:   "static analyzing tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** MODEL CHECKERS ***********************************/
pub struct ModelCheckers { event: Event }
impl Eventable for ModelCheckers {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Model Checkers");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn modelcheckers(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(ModelCheckers {
    event: Event {
      name:   name,
      desc:   "Model Checkers tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** INPUT SANITIZERS ***********************************/
pub struct InputSanitizers { event: Event }
impl Eventable for InputSanitizers {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Input Sanitizers");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn inputsanitizers(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(InputSanitizers {
    event: Event {
      name:   name,
      desc:   "input sanitizer tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** ADVISORY DETECTORS ***********************************/
pub struct AdvisoryDetectors { event: Event }
impl Eventable for AdvisoryDetectors {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Advisory Detectors");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn advisorydetectors(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(AdvisoryDetectors {
    event: Event {
      name:   name,
      desc:   "advisory detectors tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** CPPCHECK ***********************************/
pub struct CppCheck { event: Event }
impl Eventable for CppCheck {
  fn on_init(&self) -> Vec<String> {
    print::println(&format!("Welcome to cppcheck"));
    
    //prompt for files to scan
    let path_input = prompt_in_event!("CppCheck>","Enter a path to your source c++ files: ");
    if !Path::new(&path_input).exists() {
      print::print_custom(&format!("Source directory doesn't exist: {}\n",path_input),"orange");
      return Vec::new();
    }

    //create arguement vector
    let path_output = "out/cppcheck/cppcheck_report";
    let mut args: Vec<String> = Vec::new();
    args.push(format!("--enable=all"));
    args.push(format!("--output-file={}",path_output));
    args.push(path_input);
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    //run the cppcheck command
    match run_console_command(Command::new("ext/cppcheck/cppcheck").args(args)) {
      Ok(_) => {},
      Err(err) => {
        print::print_custom(&format!("Unable to execute cppcheck error: {}\n",err),"orange");
        return String::from("");   
      }
    }

    //verify output exists
    let path_output = "out/cppcheck/cppcheck_report.txt";
    if Path::new(&path_output).exists() {
      print::print_custom(&format!("Output file created: {}\n", path_output),"lightbluegreen");
    }
    else {
      print::print_custom(&format!("Unable to create output file: {}\n",path_output),"orange");
    }
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["--enable=all", "tst/cppcheck/foo1.cpp", "tst/cppcheck/cppcheck_--enable=all_foo1"],
                     vec!["--enable=all", "tst/cppcheck/foo2.cpp", "tst/cppcheck/cppcheck_--enable=all_foo2"]];
    
    for mut test in tests {
      let file_path = match test.pop() {
        Some(path) => path,
        None => {
          return TestStatus::Failed;
        }
      };

      //run the command
      let output = match Command::new("ext/cppcheck/cppcheck").args(test[..].to_vec()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("Error: {}", err));
          return TestStatus::Failed;
        }
      };

      let mut output_vec = output.stdout.clone();
      output_vec.append(&mut output.stderr.clone());

      //convert stdout and stderr to a String
      let output_str: String = match String::from_utf8(output_vec) {
        Ok(out_str) => out_str,
        Err(err) => {
          debug::print_debug(format!("Error: {}", err));
          return TestStatus::Failed;
        }
      };

      //read file
      let file_str = match fs::read_to_string(file_path) {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("Error: {}", err));
          return TestStatus::Failed;
        }
      };
      
      if output_str != file_str { return TestStatus::Failed;}
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn cppcheck(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(CppCheck {
    event: Event {
      name:   name,
      desc:   "Static analysis tool for C/C++ code.".to_string(),
      usage:  "Prompts you for: \
              \n\tPath to c++ source file(s)\n".to_string(),
      author: "Daniel Marjamaki (Cppcheck Team)".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** COBRA ***********************************/
pub struct Cobra { event: Event }
impl Eventable for Cobra {
  fn on_init(&self) -> Vec<String> {
    // Prompt for check type
    let mut check_type = prompt_in_event!("Cobra>", "Check type (stats, metrics, misra1997, or basic): ");
    if check_type == "" {
      check_type = String::from("stats");
    }

    // Prompt for files
    let files = prompt_in_event!("Cobra>", "Files to check: ");
    
    return vec![check_type, files];
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 2 {
      return self.event.usage.to_string();
    }

    let arguments = vec!["-terse".to_string(), "-f".to_string(), args[0].clone(), args[1].clone()];

    // Run command
    let output = simple_match!(run_command(Command::new("ext/cobra/cobra").args(arguments).env("C_BASE", "ext/cobra/rules")));
    
    log::log("Cobra", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    std::env::set_var("C_BASE", "ext/cobra/rules");
    let artifact_path = "tst/cobra/atf/atf.txt";
    let input_path = "tst/cobra/inp/test.c";

    std::env::set_var("C_BASE", "ext/cobra/rules");

    let output = simple_test_match!(Command::new("ext/cobra/cobra").args(vec!["-terse", "-f", "stats", input_path]).output());
    let out_str = String::from_utf8_lossy(&output.stdout);

    std::env::set_var("C_BASE", "");

    // Read file
    let file_str = simple_test_match!(fs::read_to_string(artifact_path));
    
    if out_str != file_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn cobra(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Cobra {
    event: Event {
      name:   name,
      desc:   "Fast code analyzer supporting various languages through pre-defined rules.".to_string(),
      usage:  "Requires a:\n\
               \tCheck type (stats, metrics, misra1997, or basic)\n\
               \tFiles (accepts regex)\n".to_string(),
      author: "Georg Holzmann".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** DEPENDENCY CHECKERS ***********************************/
pub struct DependencyCheckers { event: Event }
impl Eventable for DependencyCheckers {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Dependency Checkers");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn dependencycheckers(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(DependencyCheckers {
    event: Event {
      name:   name,
      desc:   "dependency checkers tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** SECURITY LINTERS ***********************************/
pub struct SecurityLinters { event: Event }
impl Eventable for SecurityLinters {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Security Linters");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn securitylinters(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SecurityLinters {
    event: Event {
      name:   name,
      desc:   "security linter tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** OCLINT ***********************************/
pub struct OCLint { event: Event }
impl Eventable for OCLint {
  fn on_init(&self) -> Vec<String> {
    print::println("OCLINT");
    
    //prompt user for path to source files
    let src_path = prompt_in_event!("OCLint>", "Enter a path to the source file(s) you wish to lint: ");

    //prompt user for path to source files
    let flags = prompt_in_event!("OCLint>", "Enter the compilation flags used: ");
    
    //create oclint command and args

    //fetch the current path
    let current_path = match env::current_dir() {
      Ok(dir) => dir,
      Err(err) => {
        println!("Something bad happened: {}",err);
        return Vec::new();
      }
    };
    
    //append the current path with the new date path
    let mut out_path: String = match current_path.to_str() {
      Some(s) => s.to_string(),
      None => return Vec::new()
    };
    let dt = chrono::offset::Local::now(); //create the DateTime struct
    let datetime: String = str::replace(&dt.to_string()," ","_"); //convert to a string and replace any ' ' with '_'
    let datetime: String = str::replace(&datetime,":","-");
    out_path.push_str("/out/oclint/report_");
    out_path.push_str(&datetime[..16]);

    let mut out_arg = String::from("-o=");
    out_arg.push_str(&out_path);

    let args = vec![src_path,out_arg,"--".to_string(),flags];
    
    //run oclint on the source files
    match run_console_command(Command::new("ext/oclint/bin/oclint").args(args)) {
      Ok(_) => {},
      Err(err) => {
        println!("Unable to execute oclint: {}",err);
      }
    };
    // /home/nickdev/ap/cppcheckstuff/*.cpp
    //tell the user where the report is written too
    if Path::new(&out_path).exists() {
      print::println(&format!("Report succesfully written to: {}",out_path));
    }
    else {
      print::println(&format!("Failed to write report to: {}",out_path));
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["tst/oclint/db1/drive.c","tst/oclint/oclint_drive"],
                     vec!["tst/oclint/db1/dbase.c","tst/oclint/oclint_dbase"],
                     vec!["tst/oclint/db1/erc.c","tst/oclint/oclint_erc"]];
    
    for mut test in tests {
      let file_path = match test.pop() {
        Some(path) => path,
        None => {
          return TestStatus::Failed;
        }
      };
      //run the command
      let output = match Command::new("ext/oclint/bin/oclint").args(test[..].to_vec()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("Error: {}", err));
          return TestStatus::Failed;
        }
      };
      
      let mut output_vec = output.stderr.clone();
      output_vec.append(&mut output.stdout.clone());

      //convert stdout and stderr to a String
      let output_str: String = match String::from_utf8(output_vec) {
        Ok(out_str) => out_str,
        Err(err) => {
          debug::print_debug(format!("Error: {}", err));
          return TestStatus::Failed;
        }
      };

      //read file
      let file_str = match fs::read_to_string(file_path) {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("Error: {}", err));
          return TestStatus::Failed;
        }
      };
      for line in file_str.split('\n') {
        if !line.contains("/") {
          //println!("{}",line);
          if !output_str.contains(line) {
            //println!("file\n{}\noutput\n{}",line,output_str);
            return TestStatus::Failed;
          }
        }
      }
      //if file_str != output_str {
      //  println!("file\n{}\noutput\n{}",file_str,output_str);
      //  return TestStatus::Failed;
      //}

    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn oclint(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(OCLint {
    event: Event {
      name:   name,
      desc:   "Static code analysis tool for inspecting C, C++ and Objective-C code.".to_string(),
      usage:  "Prompts you for: \
              \n\tPath to c source files \
              \n\tCompilation flags used\n".to_string(),
      author: "OCLint".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
/*********************************** CPPLINT ***********************************/
pub struct CppLint { event: Event }
impl Eventable for CppLint {
  fn on_init(&self) -> Vec<String> {
    //prompt user for path to source files
    let src_path = prompt_in_event!("CppLint>", "Enter a path to the source file you wish to check: ");
    
    //fetch the current path
    let current_path = match env::current_dir() {
      Ok(dir) => dir,
      Err(err) => {
        println!("Something bad happened {}",err);
        return Vec::new();
      }
    };
    
    //append the current path with the new date path
    let mut out_path: String = match current_path.to_str() {
      Some(s) => s.to_string(),
      None => return Vec::new()
    };
    let dt = chrono::offset::Local::now(); //create the DateTime struct
    let datetime: String = str::replace(&dt.to_string()," ","_"); //convert to a string and replace any ' ' with '_'
    let datetime: String = str::replace(&datetime,":","-");
    out_path.push_str("/out/cpplint/report_");
    out_path.push_str(&datetime[..16]);

    let mut out_file = match File::create(out_path.clone()) {
      Ok(file) => file,
      Err(err) => {
        print::println(&format!("Unable to create report file: {} Error : {}",out_path, err));
        return Vec::new();
      }
    };

    let args = vec!["ext/cpplint/cpplint.py".to_string(), "--recursive".to_string(),src_path];
    let output = match run_command(Command::new("python3").args(args)) {
      Ok(out) => out,
      Err(err) => {
        print::println(&format!("Unable to execute cpplint: {}",err));
        return Vec::new();
      }
    };
    match out_file.write_all(&output.stderr) {
      Ok(_) => {},
      Err(err) => {
        print::println(&format!("Failed writing to the report file. Error: {}",err));
        return Vec::new();
      }
    }
    match out_file.write_all(&output.stdout) {
      Ok(_) => {},
      Err(err) => {
        print::println(&format!("Failed writing to the report file. Error: {}",err));
        return Vec::new();
      }
    }

    if Path::new(&out_path).exists() {
      print::println(&format!("Report successfully written to: {}",out_path));
    }
    else {
      print::println(&format!("Failed to write the report to: {}",out_path));
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["ext/cpplint/cpplint.py","tst/cpplint/ARM.cpp","tst/cpplint/cpplint_ARM"],
                     vec!["ext/cpplint/cpplint.py","tst/cpplint/PPC.cpp","tst/cpplint/cpplint_PPC"],
                     vec!["ext/cpplint/cpplint.py","tst/cpplint/x86.cpp","tst/cpplint/cpplint_x86"]];
    for mut test in tests {
      let file_path = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("cpplint::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };
      //run the command
      let output = match Command::new("python3").args(test[..].to_vec()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("cpplint::on_test: failed to execute cpplint. {}",err));
          return TestStatus::Failed;
        }
      };
      let mut output_vec = output.stderr.clone();
      output_vec.append(&mut output.stdout.clone());
      
      //convert stdout and stderr to a String
      let output_str: String = match String::from_utf8(output_vec) {
        Ok(out_str) => out_str,
        Err(err) => {
          debug::print_debug(format!("cpplint::on_test: failed to read stdout. {}",err));
          return TestStatus::Failed;
        }
      };

      //read file
      let file_str = match fs::read_to_string(file_path) {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("cpplint::on_test: Failed to open the test file. {}", err));
          return TestStatus::Failed;
        }
      };

      if file_str != output_str {
        debug::print_debug(format!("file\n{}\noutput\n{}",file_str,output_str));
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn cpplint(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(CppLint {
    event: Event {
      name:   name,
      desc:   "Checks C/C++ files for style issues following Google's C++ style guide.".to_string(),
      usage:  "Prompts you for: \
              \n\tPath to c++ source files\n".to_string(),
      author: "Andrew Davis & Matt Clarkson".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** SPLINT ***********************************/
pub struct SpLint { event: Event }
impl Eventable for SpLint {
  fn on_init(&self) -> Vec<String> {
    //prompt user for path to source files
    let src_path = prompt_in_event!("Splint>", "Enter a path to the source file you wish to check: ");

    //search the user path for *.c files
      //find SRC_PATH -name *.c
        //where SRC_PATH is the user provided path to source files
    let args = vec![src_path,"-name".to_string(),"*.c".to_string()];
    let src_paths = match run_command(Command::new("find").args(args)) {
      Ok(paths) => paths,
      Err(err) => {
        print::println(&format!("{}",err));
        return Vec::new();
      }
    };

    //delimit find output by newlines
    let src_paths = String::from_utf8_lossy(&src_paths.stdout);
    let src_paths = src_paths.lines();

    //fetch the current path
    let current_path = match env::current_dir() {
      Ok(dir) => dir,
      Err(err) => {
        println!("Something bad happened: {}",err);
        return Vec::new();
      }
    };
    
    //append the current path with the new date path
    let mut out_path: String = match current_path.to_str() {
      Some(s) => s.to_string(),
      None => return Vec::new()
    };
    let dt = chrono::offset::Local::now(); //create the DateTime struct
    let datetime: String = str::replace(&dt.to_string()," ","_"); //convert to a string and replace any ' ' with '_'
    let datetime: String = str::replace(&datetime,":","-");
    out_path.push_str("/out/splint/report_");
    out_path.push_str(&datetime[..16]);

    let mut out_file = match File::create(out_path.clone()) {
      Ok(file) => file,
      Err(err) => {
        print::println(&format!("Unable to create report file: {} Error : {}",out_path, err));
        return Vec::new();
      }
    };

    //iterate over sourcefiles and run splint on each file
    for path in src_paths {
      //print::println(&format!("Checking : {}",path));
      //execute splint on path
      let args = vec![path];
      let splint_output = match run_command(Command::new("ext/splint/src/splint").args(args)) {
        Ok(output) => output,
        Err(err) => {
          print::println(&format!("Unable to execute splint on file: \nError : {}",err));
          return Vec::new();
        }
      };
      //append report with splint output
      match out_file.write_all(&splint_output.stdout) {
        Ok(_) => {},
        Err(err) => {
          print::println(&format!("Failed writing to the report file. Error: {}",err));
          return Vec::new();
        }
      }
      match out_file.write_all(&splint_output.stderr) {
        Ok(_) => {},
        Err(err) => {
          print::println(&format!("Failed writing to the report file. Error: {}",err));
          return Vec::new();
        }
      }
    }

    //verify that the report file exists
    if Path::new(&out_path).exists() {
      print::println(&format!("Report generated successfully at : {}",out_path));
    }
    else {
      print::println(&format!("Failed to generate report. Report path does not exists: {}",out_path));
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let mut tests = vec![vec!["tst/splint/db1/drive.c","tst/splint/splint_drive"],
                     vec!["tst/splint/db1/dbase.c","tst/splint/splint_dbase"],
                     vec!["tst/splint/db1/erc.c","tst/splint/splint_erc"]];
    let tests = &mut tests[..3];
    for test in tests {
      let file_path = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("splint::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };

      //run the command
      let output = match Command::new("ext/splint/src/splint").args(test[..].to_vec()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("splint::on_test: failed to execute splint. {}",err));
          return TestStatus::Failed;
        }
      };

      let output_vec = output.stdout.clone();
      //output_vec.append(&mut output.stderr.clone());

      //convert stdout and stderr to a String
      let output_str: String = match String::from_utf8(output_vec) {
        Ok(out_str) => out_str,
        Err(err) => {
          debug::print_debug(format!("splint::on_test: failed to read stdout. {}",err));
          return TestStatus::Failed;
        }
      };

      //read file
      let file_str = match fs::read_to_string(file_path) {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("splint::on_test: Failed to open the test file. {}", err));
          return TestStatus::Failed;
        }
      };

      let output_lines: Vec<&str> = output_str.split('\n').collect();
      for (pos, file_line) in file_str.split('\n').enumerate() {
        if file_line != output_lines[pos] {
          //println!("\nfile\n{}\noutput\n{}",file_line,output_lines[pos]);
          return TestStatus::Failed;
        }
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn splint(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SpLint {
    event: Event {
      name:   name,
      desc:   "Statically checks C programs for security vulnerabilities and coding mistakes.".to_string(),
      usage:  "Prompts you for: \
              \n\tPath to c source files\n".to_string(),
      author: "info@splint.org".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
/*********************************** DYNAMIC CODE ANALYZER ***********************************/
pub struct DynamicCode { event: Event }
impl Eventable for DynamicCode {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Dynamic Code Analyzers");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn dynamiccode(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(DynamicCode {
    event: Event {
      name:   name,
      desc:   "Dynamic analyzing tools".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** STOKE ***********************************/
pub struct Stoke { event: Event }
impl Eventable for Stoke {
  fn on_init(&self) -> Vec<String> {
    //this is a filter tool so first print usage
    print::println(&self.get_event().usage);

    // Prompt for args
    let args = prompt_in_event!("Stoke>", "");
    return args.split(" ").map(|s| s.to_string()).collect();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/stoke/bin/stoke").args(args)));

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    return TestStatus::Failed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn stoke(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Stoke {
    event: Event {
      name:   name,
      desc:   "Programming-language agnostic stochastic optimizer for the x86_64 ISA".to_string(),
      usage:  "Usage: stoke <subcommand> [options]\nType 'stoke <subcommand> --help' for help on a specific subcommand.\n\textract             extract the contents of a binary file\n\treplace             replace the contents of a binary file\n\tsynthesize          run STOKE search in synthesis mode\n\toptimize            run STOKE search in optimization mode\n\ttestcase            generate a STOKE testcase file\n\n\tdebug cfg           generate the control flow graph for a function\n\tdebug cost          evaluate a function using a STOKE cost function\n\tdebug diff          diff the resulting state of two functions\n\tdebug effect        show the effect of a function on the state\n\tdebug formula       show the SMT formula for a straight-line piece of code\n\tdebug invariant     find an invariant across a pair of sets of test cases\n\tdebug sandbox       step through a function execution\n\tdebug search        perform a program transformation\n\tdebug simplify      take an x86 program and simplify it (by removing redundant instructions)\n\tdebug state         check bit-wise operations\n\tdebug tunit         show the instruction sizes and RIP-offsets for a code\n\tdebug verify        check the equivalence of two functions\n\tbenchmark cfg       benchmark Cfg::recompute() kernel\n\tbenchmark cost      benchmark Cost::operator() kernel\n\tbenchmark sandbox   benchmark Sandbox::run() kernel\n\tbenchmark search    benchmark Transforms::modify() kernel\n\tbenchmark state     benchmark Memory::copy_defined() kernel\n\tbenchmark verify    benchmark Verifier::verify() kernel\n".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** TIS-INTERPRETER ***********************************/
pub struct TisInterpreter { event: Event }
impl Eventable for TisInterpreter {
  fn on_init(&self) -> Vec<String> {
    //prompt user for path to source files
    let src_path = prompt_in_event!("Tis-Interpreter>", "Enter a path to the source file(s) you wish to check: ");

    //prompt user for compiler flags
    let mut flags: String = "\"".to_string();
    flags.push_str(&prompt_in_event!("Tis-Interpreter>", "Flags used to compile your source code: "));
    flags.push_str("\"");

    //search the user path for *.c files
    //find SRC_PATH -name *.c
    //where SRC_PATH is the user provided path to source files
    let args = vec![src_path,"-name".to_string(),"*.c".to_string()];
    let src_paths = match Command::new("find").args(args).output() {
      Ok(paths) => paths,
      Err(err) => {
        print::println(&format!("{}",err));
        return Vec::new();
      }
    };

    //delimit find output by newlines
    let src_paths = String::from_utf8_lossy(&src_paths.stdout);
    let src_paths = src_paths.lines();

    //fetch the current path
    let current_path = match env::current_dir() {
      Ok(dir) => dir,
      Err(err) => {
        println!("Something bad happened: {}",err);
        return Vec::new();
      }
    };
    
    //append the current path with the new date path
    let mut out_path: String = match current_path.to_str() {
      Some(s) => s.to_string(),
      None => return Vec::new()
    };
    let dt = chrono::offset::Local::now(); //create the DateTime struct
    let datetime: String = str::replace(&dt.to_string()," ","_"); //convert to a string and replace any ' ' with '_'
    let datetime: String = str::replace(&datetime,":","-");
    out_path.push_str("/out/tisinterpreter/report_");
    out_path.push_str(&datetime[..16]);

    let mut out_file = match File::create(out_path.clone()) {
      Ok(file) => file,
      Err(err) => {
        print::println(&format!("Unable to create report file: {}. Error: {}",out_path, err));
        return Vec::new();
      }
    };

    //iterate over sourcefiles and run tis on each file
    for path in src_paths {
      //print::println(&format!("Checking : {}",path));
      //execute tis on path
      let args = vec!["--cc",&flags[..],path];
      let tis_output = match run_command(Command::new("ext/tis-interpreter/tis-interpreter.sh").args(args)) {
        Ok(output) => output,
        Err(err) => {
          print::println(&format!("Unable to execute tis-interpreter on file: {}\nError: {}",path,err));
          return Vec::new();
        }
      };
      //append report with splint output
      match out_file.write_all(&tis_output.stdout) {
        Ok(_) => {},
        Err(err) => {
          print::println(&format!("Failed writing to the report file. Error: {}",err));
          return Vec::new();
        }
      }
      match out_file.write_all(&tis_output.stderr) {
        Ok(_) => {},
        Err(err) => {
          print::println(&format!("Failed writing to the report file. Error: {}",err));
          return Vec::new();
        }
      }
    }
    //verify that the report file exists
    if Path::new(&out_path).exists() {
      print::println(&format!("Report generated successfully at: {}",out_path));
    }
    else {
      print::println(&format!("Failed to generate report. Report path does not exists: {}",out_path));
    }
    return Vec::new();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["tst/tisinterpreter/examples/01_multiple_files/main.c","tst/tisinterpreter/examples/01_multiple_files/concat.c","tst/tisinterpreter/tis_main_concat"],
                     vec!["tst/tisinterpreter/examples/02_keccak_sha_3/readable_sha3/main.c","tst/tisinterpreter/examples/02_keccak_sha_3/readable_sha3/sha3.c","tst/tisinterpreter/tis_main_sha3"]];

    for mut test in tests {
      let file_path = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("tis-interpreter::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };

      //run the command
      let output = match Command::new("ext/tis-interpreter/tis-interpreter.sh").args(test[..].to_vec()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("tis-interpreter::on_test: failed to execute tis-interpreter. {}",err));
          return TestStatus::Failed;
        }
      };
      
      //let mut output_vec = output.stderr.clone();
      //output_vec.append(&mut output.stdout.clone());

      //convert stdout and stderr to a String
      let output_str: String = match String::from_utf8(output.stdout) {
        Ok(out_str) => out_str,
        Err(err) => {
          debug::print_debug(format!("tis-interpreter::on_test: failed to read stdout. {}",err));
          return TestStatus::Failed;
        }
      };

      //read file
      let file_str = match fs::read_to_string(file_path) {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("tis-interpreter::on_test: Failed to open the test file. {}", err));
          return TestStatus::Failed;
        }
      };

      if file_str.trim() != output_str.trim() {
        debug::print_debug("\n");
        let mut oline = output_str.split("\n");
        for line in file_str.split("\n") {
          let o_line = match oline.next() {
            Some(s) => s,
            None => {
              debug::print_debug("end of lines");
              return TestStatus::Failed;
            }
          };
          if line != o_line {
            debug::print_debug(format!("{} -- {}",line, o_line));
          }
        }
        //debug::print_debug(format!("\nfile\n{}\n",file_str));
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn tisinterpreter(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(TisInterpreter {
    event: Event {
      name:   name,
      desc:   "Detects subtle bugs and undefined behavior in C programs.".to_string(),
      usage:  "Prompts you for:\
              \n\tPath to .c source files to be checked \
              \n\tFlags used to compile your source code\n".to_string(),
      author: "TrustInSoft".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** VALGRIND ***********************************/
pub struct Valgrind { event: Event }
impl Eventable for Valgrind {
  fn on_init(&self) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();

    // Prompt for file name
    let filename = prompt_in_event!("Valgrind>", "File Name: ");
    args.push(filename);

    // Prompt for flags
    let flags = prompt_in_event!("Valgrind>", "Flags: ");
    if flags != "" {
      args.push(flags);
    }
    
    return args;
  }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 {
      return self.event.usage.to_string();
    }

    // Run command
    let output = simple_match!(run_console_command(Command::new("valgrind").args(args)));

    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["tst/valgrind/inp/hello-gcc", "tst/valgrind/atf/atf.txt"]];
    for test in tests {
      // Run command
      let output = simple_test_match!(Command::new("valgrind").args(vec![test[0], "&>", "tst/valgrind/out"]).output());
      let out_str = String::from_utf8_lossy(&output.stderr);

      // Read file
      let file_str = simple_test_match!(fs::read_to_string(test[1]));

      let out_vec: Vec<&str> = out_str.split("\n").collect();
      let str_vec: Vec<&str> = file_str.split("\n").collect();

      // Compare
      if out_vec.len() != str_vec.len() {
        debug::print_debug(format!("\n-----\n{}\n-----\n{}\n-----\n", out_vec.len(), str_vec.len()));
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn valgrind(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Valgrind {
    event: Event {
      name:   name,
      desc:   "Memory leak checking tool.".to_string(),
      usage:  "Prompts you for:\n\
                \tFile name (string) \n\
                \tFlags (string)\n".to_string(),
      parent: parent,
      author: "Julian Seward".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** Spin ***********************************/
pub struct Spin { event: Event }
impl Eventable for Spin {
  fn on_init(&self) -> Vec<String> {
    //get source file input
    let path_input_str = prompt_in_event!("Spin>","Enter a path to model and check: ");
    let path_input_str_clone = path_input_str.clone();
    let path_input = Path::new(&path_input_str_clone);
    if !path_input.exists() {
      print::println(&format!("Path does not exist: {}",path_input_str));
      return Vec::new();
    }
    let args: Vec<String> = vec![path_input_str];
    return args;    
  }
  fn get_event(&self) -> &Event { return &self.event; }
  fn on_run(&self, args: Vec<String>) -> String {
    //fetch the current path
    if args.len() != 1 {
      return String::from("error. args must contain only 1 arguement\n");
    }
    let mut fmted_args: Vec<Vec<String>> = Vec::new();
    let mut spin_output_buff = String::new();
    let path_input_str = &args[0];
    let path_input = Path::new(&path_input_str);
    if path_input.extension() == None {
      //the path is a directory search for .c files
      //search the user path for *.c files
      //find SRC_PATH -name *.c
      //where SRC_PATH is the user provided path to source files
      let find_args: Vec<&str> = vec![&path_input_str,"-name","*.c"];
      let src_paths = match run_command(Command::new("find").args(find_args)) {
        Ok(paths) => paths,
        Err(err) => {
          return format!("{}",err);
        }
      };
      //delimit find output by newlines
      let src_paths = String::from_utf8_lossy(&src_paths.stdout);
      let src_paths = src_paths.lines();
      //iterate through paths vec and execute spin for each path
      for path in src_paths {
        let new_args: Vec<String> = vec!["-run".to_string(),path.to_string()];
        fmted_args.push(new_args);
      }
    }
    else { //path is not a dir
      let new_args: Vec<String> = vec!["-run".to_string(),path_input_str.to_string()];
      return execute_spin(new_args);
    }
    
    for fmted_arg in fmted_args {
      let spin_output: &str = &execute_spin(fmted_arg);
      let spin_output = format!("{}\n",spin_output);
      spin_output_buff.push_str(&spin_output);
    }
    return spin_output_buff;

    fn execute_spin(_args: Vec<String>) -> String {
      let current_path = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
          return format!("failed to get current dir. {}\n",err);
        }
      };
      //append the current path with the new date path
      let mut out_path: String = match current_path.to_str() {
        Some(s) => s.to_string(),
        None => return format!("failed to convert path to string\n")
      };
      let dt = chrono::offset::Local::now(); //create the DateTime struct
      let datetime: String = str::replace(&dt.to_string()," ","_"); //convert to a string and replace any ' ' with '_'
      let datetime: String = str::replace(&datetime,":","-");
      out_path.push_str("/out/spin/report_");
      let stem = match Path::new(&_args[_args.len() - 1]).file_stem() {
        Some(stem) => stem,
        None => OsStr::new("")
      };
      let stem = match stem.to_str() {
        Some(stem) => stem,
        None => ""
      };
      out_path.push_str(stem);
      out_path.push_str("_");
      out_path.push_str(&datetime[..16]);
      
      //create the output file
      let mut out_file = match File::create(out_path.clone()) {
        Ok(file) => file,
        Err(err) => {
          return format!("Unable to create report file: {}. Error: {}\n",out_path, err);
        }
      };
      
      //run the spin command with args
      let spin_output = match run_command(Command::new("ext/spin/spin").args(_args)) {
        Ok(output) => {output},
        Err(err) => {
          return format!("failed to execute spin. {}\n",err); //handle error executing spin
        }
      };
      //append report with spin output
      match out_file.write_all(&spin_output.stdout) {
        Ok(_) => {},
        Err(err) => {
          return format!("Failed writing to the report file. Error: {}\n",err);
        }
      }
      match out_file.write_all(&spin_output.stderr) { //also write stderr
        Ok(_) => {},
        Err(err) => {
          return format!("Failed writing to the report file. Error: {}\n",err);
        }
      }
      //verify that report exists
      if Path::new(&out_path).exists() {
        return format!("report written to: {}\n",out_path);
      }
      else {
        return format!("failed to write to report file: {}\n",out_path);
      }
    }
  }
  fn on_test(&self) -> TestStatus {
    let input_path = "tst/spin/inp/test.pml";
    let artifact_path = "tst/spin/atf/atf.txt";

    let spin_args = vec!["-run", input_path];
    
    // Run command
    let check = match Command::new("ext/spin/spin").args(spin_args).output() {
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
        debug::print_debug(format!("spin::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    let tmp = String::from_utf8_lossy(&check.stdout);
    let mut lines1: Vec<&str> = tmp.split("\n").collect();
    let mut lines2: Vec<&str> = file_str.split("\n").collect();
    lines1.pop();
    lines2.pop();
    if lines1.join("\n") != lines2.join("\n") {
      debug::print_debug("spin::on_test: artifact doesnt match output");
      return TestStatus::Failed;
    }

    //spin places output files into current working directory. move them to tst dir
    match Command::new("mv").args(vec!["pan","ext/spin"]).output() {
      Ok(_) => {},
      Err(err) => {
        debug::print_debug(format!("spin::on_test: unable to move pan to spin dir. {}",err));
        return TestStatus::Failed;
      }
    };
    
    return TestStatus::Passed;
  }
}
pub fn spin(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Spin {
    event: Event {
      name:   name,
      desc:   "An Efficient Logic Model Checker for the Verification of Multi-threaded Code".to_string(),
      usage:  filter::parse_help_menu(Command::new("ext/spin/spin").args(vec!["--help"]),
                vec!["!ALL!".to_string()], "\n", 0, ""),
      author: "Bell Labs".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** Modex ***********************************/
pub struct Modex { event: Event }
impl Eventable for Modex {
  fn on_init(&self) -> Vec<String> {
    //get source file input
    let path_input_str = prompt_in_event!("Modex>","Enter a path to model and check : ");
    let path_input = Path::new(&path_input_str);
    if !path_input.exists() {
      print::println(&format!("Path does not exist: {}",path_input_str));
      return Vec::new();
    }
    let args: Vec<String> = vec![path_input_str];
    return args;

    
  }
  fn get_event(&self) -> &Event { return &self.event; }
  fn on_run(&self, args: Vec<String>) -> String {
    //fetch the current path
    if args.len() != 1 {
      return String::from("Error. args must contain only 1 arguement\n");
    }
    let mut fmted_args: Vec<Vec<String>> = Vec::new();
    let mut modex_output_buff = String::new();
    let path_input_str = &args[0];
    let path_input = Path::new(&path_input_str);
    if path_input.extension() == None {
      //the path is a directory search for .c files
      //search the user path for *.c files
      //find SRC_PATH -name *.c
      //where SRC_PATH is the user provided path to source files
      let find_args: Vec<&str> = vec![&path_input_str,"-name","*.c"];
      let src_paths = match run_command(Command::new("find").args(find_args)) {
        Ok(paths) => paths,
        Err(err) => {
          return format!("{}",err)
        }
      };
      //delimit find output by newlines
      let src_paths = String::from_utf8_lossy(&src_paths.stdout);
      let src_paths = src_paths.lines();
      //iterate through paths vec and execute modex for each path
      for path in src_paths {
        let new_args: Vec<String> = vec!["-run".to_string(),path.to_string()];
        fmted_args.push(new_args);
      }
    }
    else { //input path isn't a dir so execute modex just on input_str
      let new_args: Vec<String> = vec!["-run".to_string(),path_input_str.to_string()];
      return execute_modex(new_args);
    }

    for fmted_arg in fmted_args {
      let modex_output: &str = &execute_modex(fmted_arg);
      let modex_output = format!("{}\n",modex_output);
      modex_output_buff.push_str(&modex_output);
    }
    return modex_output_buff;

    fn execute_modex(_args: Vec<String>) -> String {
      let current_path = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
          return format!("Failed to get the current directory {}\n",err);
        }
      };
      
      //append the current path with the new date path
      let mut out_path: String = match current_path.to_str() {
        Some(s) => s.to_string(),
        None => {return format!("unable to get current path\n");}
      };
      let dt = chrono::offset::Local::now(); //create the DateTime struct
      let datetime: String = str::replace(&dt.to_string()," ","_"); //convert to a string and replace any ' ' with '_'
      let datetime: String = str::replace(&datetime,":","-");
      out_path.push_str("/out/modex/report_");
      let stem = match Path::new(&_args[1]).file_stem() {
        Some(stem) => stem,
        None => OsStr::new("")
      };
      let stem = match stem.to_str() {
        Some(stem) => stem,
        None => ""
      };
      out_path.push_str(stem);
      out_path.push_str("_");
      out_path.push_str(&datetime[..16]);
      
      //create the output file
      let mut out_file = match File::create(out_path.clone()) {
        Ok(file) => file,
        Err(err) => {
          return format!("Unable to create report file: {}. Error: {}\n",out_path, err);
        }
      };
      
  
      //run the modex command with args
      let modex_output = match run_command(Command::new("ext/spin/modex").args(_args)) {
        Ok(output) => {output},
        Err(err) => {
          return format!("{}\n",err); //handle error executing modex
        }
      };
  
      //append report with modex output
      match out_file.write_all(&modex_output.stdout) {
        Ok(_) => {},
        Err(err) => {
          return format!("Failed writing to the report file. Error: {}\n",err);
        }
      }
      match out_file.write_all(&modex_output.stderr) { //also write stderr
        Ok(_) => {},
        Err(err) => {
          return format!("Failed writing to the report file. Error: {}\n",err);
        }
      }
      
      //verify that report exists
      if !Path::new(&out_path).exists() {
        return format!("failed to write to report file : {}\n",out_path);
      }
      let move_files = vec!["_modex_.cln","_modex_.drv","_modex_.h","_modex_.run","model","model.trail","pan.b","pan.c","pan.h","pan.m","pan.p","pan.t","pan"];
      for file in move_files {
        //print::println(&format!("moving {} to out/modex",file));
        let args = vec![file,"out/modex"];
        match Command::new("mv").args(args).output() {
          Ok(_) => {
          },
          Err(_) => {
            return format!("Unable to move modex's output files\n");
          }
        }
      }
      return format!("report written to : {}\n",out_path);
    }
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["-run", "tst/modex/1_bounds.c", "tst/modex/modex_-run_1_bounds"],
                     vec!["-run", "tst/modex/2_pointers.c", "tst/modex/modex_-run_2_pointers"],
                     vec!["-run", "tst/modex/3_threads.c", "tst/modex/modex_-run_3_threads"]];
    
    //
    for mut test in tests {
      //pop the file path from the test vec
      let file_path = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("modex::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      };

      //run the command
      let output = match Command::new("ext/spin/modex").args(test[..].to_vec()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("modex::on_test: failed to execute pass-rs. {}",err));
          return TestStatus::Failed;
        }
      };
      
      //modex places output files into current working directory. move them to tst dir
      let move_files = vec!["_modex_.cln","_modex_.drv","_modex_.h","_modex_.run","model","model.trail","pan.b","pan.c","pan.h","pan.m","pan.p","pan.t","pan"];
      for file in move_files {
        //print::println(&format!("moving {} to out/modex",file));
        let args = vec![file,"tst/modex"];
        match Command::new("mv").args(args).output() {
          Ok(_) => {},
          Err(err) => {
            debug::print_debug(format!("modex::on_test: unable to move modex's output files. {}",err));
            return TestStatus::Failed;
          }
        }
      }

      //convert stdout to a String
      let output_str: String = match String::from_utf8(output.stdout) {
        Ok(out_str) => out_str,
        Err(err) => {
          debug::print_debug(format!("modex::on_test: failed to read stdout. {}",err));
          return TestStatus::Failed;
        }
      };

      //read file
      let file_str = match fs::read_to_string(file_path) {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("modex::on_test: Failed to open the test file : {}. {}", file_path, err));
          return TestStatus::Failed;
        }
      };

      //compare the output of test file and command
      let output_split = output_str.split('\n');
      let file_split: Vec<&str> = file_str.split('\n').collect();

      let skip_lines = vec!["pan: elapsed time"];
      for (pos, output_line) in output_split.enumerate() {
        for skip in &skip_lines {
          if !output_line.contains(skip) {
            if output_line != file_split[pos] {
              debug::print_debug(format!("modex::on_test: artifact does not match output."));
              return TestStatus::Failed;
            }
          }
        }
      }
    }
    return TestStatus::Passed;
  }
}

pub fn modex(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Modex {
    event: Event {
      name:   name,
      desc:   "Mechanically extracts verification models from implementation level C code.".to_string(),
      usage:  filter::parse_help_menu(Command::new("ext/spin/modex").args(vec!["-h"]),
                vec!["!ALL!".to_string()], "\n", 0, ""),
      parent: parent,
      author: "Bell Labs".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** Cbmc ***********************************/
pub struct Cbmc { event: Event }
impl Eventable for Cbmc {
  fn on_init(&self) -> Vec<String> {
    //need to accredit the original authors of the tool
    let cbmc_legal = "CBMC (c) 2018 - Daniel Kroening, Edmund Clarke";
    print::println(cbmc_legal);
    
    //get source file input
    let path_input_str = prompt_in_event!("Cbmc>","Enter a path to model and check: ");
    let path_input = Path::new(&path_input_str);
    if !path_input.exists() {
      print::println(&format!("Path does not exist: {}",path_input_str));
      return Vec::new();
    }
    let args: Vec<String> = vec![path_input_str];
    return args;

    
  }
  fn get_event(&self) -> &Event { return &self.event; }
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 {
      return String::from("error. args must only contain 1 arguement\n");
    }
    let mut fmted_args: Vec<Vec<String>> = Vec::new();
    let mut cbmc_output_buff = String::new();
    let path_input_str = &args[0];
    let path_input = Path::new(&path_input_str);

    if path_input.extension() == None {
      //the path is a directory search for .c files
      //search the user path for *.c files
      //find SRC_PATH -name *.c
      //where SRC_PATH is the user provided path to source files
      let find_args: Vec<&str> = vec![&path_input_str,"-name","*.c"];
      let src_paths = match Command::new("find").args(find_args).output() {
        Ok(paths) => paths,
        Err(err) => {
          return format!("{}",err);
        }
      };
      //delimit find output by newlines
      let src_paths = String::from_utf8_lossy(&src_paths.stdout);
      let src_paths = src_paths.lines();
      //iterate through paths vec and execute cbmc for each path
      for path in src_paths {
        //print::println(&format!("executing cbmc on {}",path));
        let new_args: Vec<String> = vec![path.to_string()];
        fmted_args.push(new_args);
      }
    }
    else { //input path isn't a dir so execute cbmc just on input_str
      let args: Vec<String> = vec![path_input_str.to_string()];
      return execute_cbmc(args);
    }

    for fmted_arg in fmted_args {
      let cbmc_output: &str = &execute_cbmc(fmted_arg);
      let cbmc_output = format!("{}\n",cbmc_output);
      cbmc_output_buff.push_str(&cbmc_output);
    }
    return cbmc_output_buff;

    fn execute_cbmc(_args: Vec<String>) -> String {
      let current_path = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => {
          return format!("failed to get current directory.\n");
        }
      };
      //append the current path with the new date path
      let mut out_path: String = match current_path.to_str() {
        Some(s) => s.to_string(),
        None => {return format!("Cbmc: failed to convert path to string\n");}
      };
      let dt = chrono::offset::Local::now(); //create the DateTime struct
      let datetime: String = str::replace(&dt.to_string()," ","_"); //convert to a string and replace any ' ' with '_'
      let datetime: String = str::replace(&datetime,":","-");
      out_path.push_str("/out/cbmc/report_");
      let stem = match Path::new(&_args[0]).file_stem() {
        Some(stem) => stem,
        None => OsStr::new("")
      };
      let stem = match stem.to_str() {
        Some(stem) => stem,
        None => ""
      };
      out_path.push_str(stem);
      out_path.push_str("_");
      out_path.push_str(&datetime[..16]);
      
      //create the output file
      let mut out_file = match File::create(out_path.clone()) {
        Ok(file) => file,
        Err(err) => {
          return format!("Unable to create report file: {} Error: {}\n",out_path, err);
        }
      };
  
      //run the cbmc command with args
      let cbmc_output = match run_command(Command::new("ext/cbmc/cbmc").args(_args)) {
        Ok(output) => {output},
        Err(err) => {
          return format!("{}\n",err); //handle error executing cbmc
        }
      };
  
      //append report with cbmc output
      match out_file.write_all(&cbmc_output.stdout) {
        Ok(_) => {},
        Err(err) => {
          return format!("Failed writing to the report file. Error: {}\n",err);
        }
      }
      match out_file.write_all(&cbmc_output.stderr) { //also write stderr
        Ok(_) => {},
        Err(err) => {
          return format!("Failed writing to the report file. Error: {}\n",err);
        }
      }
      //verify that report exists
      if Path::new(&out_path).exists() {
        return format!("Report written to: {}\n",out_path);
      }
      else {
        return format!("Failed to write to report file: {}\n",out_path);
      }
    }
  }
  fn on_test(&self) -> TestStatus {
    let tests = vec![vec!["tst/cbmc/1_bounds.c","tst/cbmc/cbmc_bounds"],
                     vec!["tst/cbmc/2_pointers.c","tst/cbmc/cbmc_pointers"],
                     vec!["tst/cbmc/3_threads.c","tst/cbmc/cbmc_threads"]];

    
    for mut test in tests {
      //pop file path from test vec
      let file_path = match test.pop() {
        Some(path) => path,
        None => {
          debug::print_debug("cbmc::on_test: invalid test vector");
          return TestStatus::Failed;
        }
      }; 

      //run the command
      let output = match Command::new("ext/cbmc/cbmc").args(test[..].to_vec()).output() {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("cbmc::on_test: failed to execute cbmc. {}",err));
          return TestStatus::Failed;
        }
      };

      //convert stdout to a String
      let output_str: String = match String::from_utf8(output.stdout) {
        Ok(out_str) => out_str,
        Err(err) => {
          debug::print_debug(format!("cbmc::on_test: failed to read stdout. {}",err));
          return TestStatus::Failed;
        }
      };
      
      //read file
      let file_str = match fs::read_to_string(file_path) {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("cbmc::on_test: Failed to open the test file: {}. {}",file_path, err));
          return TestStatus::Failed;
        }
      };

      //certain output lines will never be the same therefore they need to be skipped
      let skip_lines = vec!["Runtime Symex".to_string(),"Runtime Postprocess Equation".to_string()];
      //split up the strings to compare
      let output_str_split: Vec<&str> = output_str.split('\n').collect();
      let file_str_split = file_str.split('\n');
      //iterate through each line of the file
      for (pos, line) in file_str_split.enumerate() {
        let mut skip = false;
        //check to see if the current line needs to be skipped
        for skip_str in &skip_lines {
          if line.contains(skip_str) { 
            skip = true;
          }
        }
        //if the current line doesnt need to be skipped compare it against command output
        if !skip && (line != output_str_split[pos]) {
          debug::print_debug(format!("line\n{}\noutput_str_split\n{}",line,output_str_split[pos]));
          return TestStatus::Failed;
        }
      }
    }
    return TestStatus::Passed;
  }
}
pub fn cbmc(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Cbmc {
    event: Event {
      name:   name,
      desc:   "A bounded model checker that verifies memory safety.".to_string(),
      usage:  "Prompts you for: \
              \n\tPath to source file(s)\n".to_string(),
      author: "Daniel Kroening\nkroening@kroening.com".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
} 
