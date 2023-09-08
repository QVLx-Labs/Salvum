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

/*********************************** listfiles ***********************************/
pub struct Listfiles { event: Event }
impl Eventable for Listfiles {
	fn on_run(&self, args: Vec<String>) -> String {
		let mut arguments = Vec::new();
    if args.len() == 0 {
      arguments = vec!["-lgn".to_string()]; 
			let out = match run_console_command(Command::new("ext/listfiles/natls").args(arguments)) {
				Ok(out) => out,
				Err(e) => { return format!("Failed to run natls command. {}\n", e); }
			};
      return String::from_utf8_lossy(&out.stdout).to_string();
    }
		else if args.len() == 1 {
      let file = args[0].clone();
      arguments.push("-lgn".to_string());
      let mut tmp = file.clone();
			let last = match tmp.pop() {
				Some(c) => c,
				None => return String::from(""),
			};
			if last != '/' {
				tmp.push(last);
			}
      arguments.push(tmp);
			let out = match run_console_command(Command::new("ext/listfiles/natls").args(arguments)) {
				Ok(out) => out,
				Err(e) => { return format!("Failed to run natls command. {}\n", e); }
			};
      return String::from_utf8_lossy(&out.stdout).to_string();
    }
    else if args.len() == 2 {
      if !args[0].eq("size") && !args[0].eq("archive") { return self.event.usage.clone(); };
      if args[0] == "archive" {
				match run_console_command(Command::new("ext/listfiles/als").arg(args[1].clone())) {
					Ok(out) => out,
					Err(e) => { return format!("Failed to run als command. {}\n", e); }  
				};
        return String::from("");
      }
      let file = args[1].clone();
			use std::fs::metadata;
			let md = match metadata(file.clone()) {
        Ok(o) => o, 
       Err(e) => { return format!("Failed getting directory metadata: {}\n", e); }
      };
			if md.is_dir() {
				let files = match Command::new("ls").args(vec!["-1".to_string(),file.clone()]).output() {
					Ok(out) => out,
					Err(e) => { return format!("Failed to run ls command. {}\n", e); }  
				};
        let files_str = String::from_utf8_lossy(&files.stdout);
        let files_str_nl = files_str.split("\n").collect::<Vec<&str>>();
        let mut cmd = String::from("ext/listfiles/durt ");
        cmd.push_str("-st");
        for f in 0..files_str_nl.len() - 1 {
          cmd.push_str(" ");
          cmd.push_str(&file);
          cmd.push_str("/");
          cmd.push_str(files_str_nl[f]);
        }
				let output = match util::pipes::Pipe::new(&cmd.to_string()).finally() {
					Ok(sel) => sel,
				  Err(err) => { return format!("Command failed. Error: {}\n", err); }
				};
				let out = match output.wait_with_output() {
					Ok(sel) => sel,
					Err(err) => { return format!("Command failed. Error: {}\n", err); }
				};
        let totals = String::from_utf8_lossy(&out.stdout);
        let totals_split = totals.split("---------").collect::<Vec<&str>>(); 
        print::print_custom(&format!("{}",totals_split[0]),"rose");
        print::print_custom("---------","grey");
        print::print_custom(&format!("{}",totals_split[1]),"lightbluegreen");
			}
      else {
        arguments = vec!["-st".to_string(),file];
				match run_console_command(Command::new("ext/listfiles/durt").args(arguments)) {
					Ok(out) => out,
					Err(e) => { return format!("Failed to run durt command. {}\n", e); }  
				};
      }
    }
		else { print::print_custom("ls only takes zero or one argument.\n", "orange"); return self.event.usage.clone(); }
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
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn listfiles(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Listfiles {
    event: Event {
      name:   name,
      desc:   "Like the standard ls command but with improvements.".to_string(),
      usage:  "Usage:\n\tlistfiles\n\tlistfiles <path>\n\tlistfiles size <path>\n\tlistfiles archive <path>\n".to_string(),
      parent: parent,
      author: "William Lane".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }
  })
}

/*********************************** BATCAT ***********************************/
pub struct BatCat { event: Event }
impl Eventable for BatCat {
	fn on_run(&self, args: Vec<String>) -> String {
		let mut arguments = Vec::new();
		if args.len() == 1 { // Regular mode
			arguments.push("--paging");
			arguments.push("never");
			arguments.push(args[0].trim());
		}
		else if args.len() == 2 { // Hidden mode
      if args[0].eq("classic") {
				let output = match run_console_command(Command::new("cat").arg(&args[1])) {
					Ok(out) => out,
					Err(err) => {
						return format!("Failed to run cat: {}\n", err);
					}
				};
        return String::from_utf8_lossy(&output.stdout).to_string();
      }
			if !(args[0] == "hidden") { return self.event.usage.clone(); }
			arguments.push("--paging");
			arguments.push("never");
			arguments.push("-A");
			arguments.push(args[1].trim());
		}
		else { print::print_custom("cat needs a file path.\n", "orange"); return self.event.usage.clone(); } // Handle invalid

		// Run command
		let output = match run_console_command(Command::new("ext/batcat/batcat").args(&arguments[..])) {
			Ok(out) => out,
			Err(err) => {
				return format!("Failed to run batcat: {}\n", err);
			}
		};

		// Print output
		log::log("BatCat", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
	}
	fn on_test(&self) -> TestStatus {
		let input_path = "tst/batcat/inp/test.txt";
		let artifact_path = "tst/batcat/atf/atf.txt";

		let cat_args = vec!["--paging=never", input_path];

		// Run command
		let check = match Command::new("ext/batcat/batcat").args(cat_args).output() {
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
					debug::print_debug(format!("batcat::on_test: Failed to open the test file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn batcat(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(BatCat {
    event: Event {
      name:   name,
      desc:   "Improved file cat tool for easy inspection.".to_string(),
      usage:  "cat <file_path>\ncat hidden <file_path>\ncat classic <file_path>\n".to_string(),
      parent: parent,
      author: "David Peter and bat-developers\n\nCopyright (c) 2018-2021 bat-developers.".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** vericopy ***********************************/
pub struct Vericopy { event: Event }
impl Eventable for Vericopy {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() != 3 { // Handle invalid args
      return self.event.usage.clone();
    }
		if args[0].eq("md5") {
			// Run command
			let output = match run_console_command(Command::new("ext/vericopy/md5_cp").args(&args.clone()[1..])) {
				Ok(out) => out,
				Err(e) => { return format!("Failed to run md5_cp: {}", e); }
			};
			// Print output
      log::log("vericopy", &String::from_utf8_lossy(&output.stderr));
			return String::from_utf8_lossy(&output.stdout).to_string();
		}
		else if args[0].eq("sha256") {
			// Run command
			let output = match run_console_command(Command::new("ext/vericopy/sha256_cp").args(&args.clone()[1..])) {
				Ok(out) => out,
				Err(e) => { return format!("Failed to run sha256_cp: {}", e); }  
			};
			// Print output
      log::log("vericopy", &String::from_utf8_lossy(&output.stderr));
      return String::from_utf8_lossy(&output.stdout).to_string();
		}
		else {
      return self.event.usage.clone();
    } // Handle invalid args 
	}
	fn on_test(&self) -> TestStatus {
		let staging_path = "tst/vericopy/test.txt";
		let input_path = "tst/vericopy/inp/test.txt";
		let artifact_path = "tst/vericopy/atf/atf.txt";
		let cp_args = vec![input_path, staging_path];

		// Run command
		let check = match Command::new("ext/vericopy/sha256_cp").args(cp_args).output() {
			Ok(o) => o,
				Err(e) => {
					debug::print_debug(format!("Error: {}", e));
					return TestStatus::Failed;
				}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
				Err(err) => {
					debug::print_debug(format!("vericopy::on_test: Failed to read artifact file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if !String::from_utf8_lossy(&check.stdout).contains(&file_str) { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn vericopy(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Vericopy {
event: Event {
name:   name,
desc:   "Standard cp command but with checksum verification.".to_string(),
usage:  "cp [md5|sha256] source_path destination_path\n".to_string(),
parent: parent,
author: "Martin Page".to_string(),
easyrun: false,
secure: false,
links:  links
}
})
}

/*********************************** ASCIIPEEP ***********************************/
pub struct AsciiPeep { event: Event }
impl Eventable for AsciiPeep {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() != 1 {
			print::print_custom("strings takes one argument - a path to a binary.\n", "orange");
      return String::from("");
		}

		let mut arg_vec = vec!["-n", "3", "-t", "d"];
		arg_vec.push(args[0].trim());

		// Run command
		let output = match Command::new("ext/asciipeep/strings").args(arg_vec).output() {
			Ok(out) => out,
			Err(e) => { return format!("Failed to run the ascii_peep command. {}\n", e); }  
		};

		//convert stdout into string
		//split up string by white space
		print::print_custom("+--------------+--------------------------------+\n", "orange");
		print::print_custom("|    line#     |          ascii string          |\n", "orange");
		print::print_custom("+--------------+--------------------------------+\n", "orange");

		let output_str_nl = String::from_utf8_lossy(&output.stdout).to_string();
		let output_str: Vec<&str> = output_str_nl.split('\n').collect(); 
		for i in 0..output_str.len() {
			let output_tok: Vec<&str> = output_str[i].split_whitespace().collect();
			if output_tok.len() == 1{
				print::print_custom(&format!("{: >14}", output_tok[0]), "white");
				print::print_custom(" | \n", "orange");
			}
			if output_tok.len() == 2 {
				print::print_custom(&format!("{: >14}", output_tok[0]), "white");
				print::print_custom(" | ", "orange");
				print::print_custom(&format!("{}\n", &output_tok[1]), "white");
			}
			if output_tok.len() > 2 {
				print::print_custom(&format!("{: >14}", output_tok[0]), "white");
				print::print_custom(" | ", "orange");
				print::print_custom(&format!("{}", output_tok[1]), "white");
				for j in &output_tok[2..] {
          print::print_custom(&format!(" {}", j), "white");
				}
        print::print_custom("\n", "white");
			}
		}
		//log::log("asciipeep", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let input_path = "tst/asciipeep/inp/test.txt";
		let artifact_path = "tst/asciipeep/atf/atf.txt";
		let string_args = vec!["-n", "2", input_path];

		// Run command
		let ascii = match Command::new("ext/asciipeep/strings").args(string_args).output() {
			Ok(a) => a,
				Err(err) => {
					debug::print_debug(format!("Error: {}", err));
					return TestStatus::Failed;
				}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
				Err(err) => {
					debug::print_debug(format!("asciipeep::on_test: Failed to open the test file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&ascii.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn asciipeep(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(AsciiPeep {
event: Event {
name:   name,
desc:   "Print the ASCII strings found in a file.".to_string(),
usage:  "This utility takes one argument: path to the file to analyze.\n".to_string(),
parent: parent,
author: "".to_string(),
easyrun: false,
secure: false,
links:  links
}
})
}

/*********************************** TSCD ***********************************/
pub struct TSCD { event: Event }
impl Eventable for TSCD {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() < 1 || args.len() > 2 {
      print::print_custom("dump takes two arguments: bin|oct|hex|dec and binary path.\n", "orange");
      return self.event.usage.clone();
    }
		let mut base = "";
		let mut filename = "";
		if args.len() == 2 {
			match args[0].trim() {
				"bin" => base = "-b",
					"dec" => base = "-d",
					"oct" => base = "-o",
					"hex" => {},
					_ => return self.event.usage.clone()
			}
			filename = args[1].trim();
		}
		if args.len() == 1 { filename = args[0].trim(); }
		let arg_vec = vec![base, filename];
    let md = match fs::metadata(filename) {
      Ok(o) => o,
      Err(e) => {
        print::print_custom(&format!("Error getting metadata in TSCD: {}", e),"orange");
        return String::from("");
      }
    };
    let filesize = md.len();
    if main_info::get_file_redirect() && filesize > 9000000 {
			// Run command
			let out = match run_command(Command::new("ext/tscd/hexdump").arg(filename)) {
				Ok(o) => o,
				Err(err) => { return format!("Failed to run hx command: {}\n", err); }
			};
			return String::from_utf8_lossy(&out.stdout).to_string(); 
    }
    if filesize > 4000000 && filesize < 9000000 {
      if main_info::is_bg() && !main_info::get_file_redirect() {
        print!("Very large binary dumps can't be run in the background.");
        return String::from("Very large binary dumps can't be run in the background.\n");
      }
      print::print_custom("Large file detected.\n","orange");
			let mut last_chance = String::from("Want us to use a faster dumper to dump ");
			last_chance.push_str(&filename);
			last_chance.push_str(" ?");
			if alerts::confirm_task(&last_chance) == constants::CONFIRMED {
				// Run command
        print::print_custom("","reset");
        //print::print_custom_uncapped("","rose");
				let out = match run_command(Command::new("ext/tscd/hx").arg(filename)) {
					Ok(o) => o,
					Err(err) => { return format!("Failed to run hx command: {}\n", err); }
				};
        print::print_custom(&String::from_utf8_lossy(&out.stdout)
                              .to_string()
                              .replace("\x1b[38;5;8m","\x1b[38;5;203m")
                              .replace(" 00","\x1b[38;5;8m 00\x1b[38;5;203m")
                              .replace(".","\x1b[38;5;8m.\x1b[38;5;203m")
                              .replace(" |","\x1b[38;5;7m |\x1b[38;5;203m")
                              .replace("|\n","\x1b[38;5;7m|\n\x1b[38;5;203m"),"rose");
				print::print_custom("","reset");
				return String::from("");
			}
    };
    if filesize > 9000000 {
      if main_info::is_bg() && !main_info::get_file_redirect() {
        print!("Large binary dumps can't be run in the background.");
        return String::from("Large binary dumps can't be run in the background.\n");
      }
      print::print_custom("Very large file detected.\n","orange");
			let mut last_chance = String::from("Want us to use an even faster dumper to dump ");
			last_chance.push_str(&filename);
			last_chance.push_str(" ?");
			if alerts::confirm_task(&last_chance) == constants::CONFIRMED {
				// Run command
				let out = match run_command(Command::new("ext/tscd/hexdump").arg(filename)) {
					Ok(o) => o,
					Err(err) => { return format!("Failed to run hx command: {}\n", err); }
				};
        print::print_custom(&String::from_utf8_lossy(&out.stdout)
                               .to_string()
                               .replace(" 0000","\x1b[38;5;8m 0000\x1b[38;5;154m")
                               .replace("*","\x1b[38;5;7m*\x1b[38;5;154m"),"neongreen");
				return String::from("");
			}
    };

		// Run command
		let output = match run_console_command(Command::new("ext/tscd/tscd").args(arg_vec)) {
			Ok(out) => out,
				Err(err) => {
          return format!("Failed to run the tscd command: {}\n", err);
				}
		};

		// Print output
		print::println("");
		log::log("dump", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
	fn on_test(&self) -> TestStatus {
		let input_path = "tst/tscd/inp/test.txt";
		let artifact_path = "tst/tscd/atf/atf.txt";

		// Run command
		let check = match Command::new("ext/tscd/tscd").arg(input_path).output() {
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
					debug::print_debug(format!("tscd::on_test: Failed to open the test file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn tscd(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(TSCD {
		event: Event {
			name:   name,
			desc:   "Improved file dumping tool for easy inspection.".to_string(),
			usage:  "This hexdump takes the following arguments:\n\t1) hex|bin|dec|oct\n\t2) path to file\n\ntscd hex <file_name>\n".to_string(),
			parent: parent,
			author: "Tanveer Salim\tTanveer.Salim@ttu.edu\n\nCopyright (C) 2019 FOSRES\nCopyright (C) 2019 Tanveer Salim".to_string(),
			easyrun: false,
			secure: false,
			links:  links
	  }
  })
}

/*********************************** CCDIFF ***********************************/
pub struct CCDiff { event: Event }
impl Eventable for CCDiff {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() != 2 {
			print::print_custom("Diff needs 2 arguments - source and destination paths.\n", "orange");
      return self.event.usage.clone();
		}

		let left_file = args[0].clone();
		let right_file = args[1].clone();
		if !Path::new(&left_file).exists() {
			return format!("diff: \"{}\" doesn't exist.\n", left_file);
		}
		if !Path::new(&right_file).exists() {
			return format!("diff: \"{}\" doesn't exist.\n", right_file);
		}
    
    if left_file == right_file {
      print::print_custom("Left and right are the same file.\n","neongreen");
      return String::from("");
    }

		// Handle directory diffs
		let left_md = match std::fs::metadata(left_file.trim()) {
			Ok(md) => md,
				Err(err) => {
					return format!("Failed to get metadata of the left file. {}\n", err);
				}
		};
		let right_md = match std::fs::metadata(right_file.trim()) {
			Ok(md) => md,
				Err(err) => {
					return format!("Failed to get metadata of the right file. {}\n", err);
				}
		};

		if left_md.is_dir() && right_md.is_dir() {
			let mut cmd = String::from("diff -qr ");
			cmd.push_str(left_file.trim());
			cmd.push_str(" ");
			cmd.push_str(right_file.trim());

			// run this: diff -qr left_file right_file
			let output = match util::pipes::Pipe::new(&cmd.to_string()).finally() {
				Ok(sel) => sel,
					Err(err) => {
						return format!("Command failed. Error: {}\n", err);
					}
			};
			let output = match output.wait_with_output() {
				Ok(sel) => sel,
					Err(err) => {
						return format!("Command failed. Error: {}\n", err);
					}
			};

			let output_str_nl = String::from_utf8_lossy(&output.stdout).to_string();
			let output_str: Vec<&str> = output_str_nl.split('\n').collect();
			for i in 0..output_str.len(){
				let output_tok: Vec<&str> = output_str[i].split_whitespace().collect();
				let mut color = "purple";
				if output_tok.len() > 0{
					if output_tok[0].contains("Files"){
						color = "orange";
					}
					print::print_custom(&format!("{}\n", output_str[i]), color);
				}
        else{
					if output_str.len() == 1 {
						print::print_custom("No differences found between directories.\n", "bluegreen");
					}
          continue;
        }
			}
			return String::from("");
		}
		else if (left_md.is_dir() && right_md.is_file())
         || (left_md.is_file() && right_md.is_dir())
         || (left_md.is_dir() && !right_md.is_dir())
         || (!left_md.is_dir() && right_md.is_dir())
    {
			print::print_custom("Can't diff a file with a directory.\n", "orange");
			return String::from("");
		}

    // Handle binary diffs
    let is_leftbin = match binaryornot::is_binary(&left_file) {
      Ok(o) => o,
     Err(_) => return String::from(""),
    };
    let is_rightbin = match binaryornot::is_binary(&right_file) {
      Ok(o) => o,
     Err(_) => return String::from(""),
    };
    if is_leftbin || is_rightbin {
      print::print_custom("This diff does not handle binaries.\n","orange");
      return String::from("");
    }

		// Run command
		let output = simple_match!(run_command(Command::new("ext/ccdiff/ccdiff").args(vec!["-p", "--bg=ansi228", "--new=ansi29", &left_file, &right_file])));

		// Get the terminal width
		let (width, _) = match termion::terminal_size() {
			Ok(size) => size,
			Err(err) => { return format!("Error: {}\n", err); }
		};
		let print_width: usize = (width as usize / 2) - 7;

		// Take the diff output as iterable lines
		let out = String::from_utf8_lossy(&output.stdout);
		let mut lines = out.lines().peekable();

		// Diff Header
		let left_header = match lines.next() {
			Some(h) => h,
				None => "",
		};
		let right_header = match lines.next() {
			Some(h) => h,
				None => "",
		};
		print::println_neutral(&format!("\x1b[38;5;204m|Line#|\x1b[35m{1:0$}\x1b[38;5;204m|Line#|\x1b[32m{2:0$}\x1b[0m",
				print_width + 4,
				left_header,
				right_header));

		// Generic formatted print for both left and right sides
		let print_diff_format = |line: &str, line_num: usize, line_j: &mut usize, more: &mut bool, last_fmt: &mut String| {
      let start_fmt = last_fmt.clone();

			// Line numbers
			if *line_j > 0 || line == "" { print::print_neutral(&format!("\x1b[0m\x1b[38;5;208m|{:^5}|\x1b[0m", ' ')); }
      else { print::print_neutral(&format!("\x1b[0m\x1b[38;5;208m|{:^5}|\x1b[0m", line_num)); }

			// Loop through each byte
			let mut do_count = true;
			let mut length = 0;
      let mut print_str = String::from("");

      let mut grapheme_cursor = strcursor::StrCursor::new_at_end(&line[*line_j..]);
      let line_len = grapheme_cursor.byte_pos();
      grapheme_cursor = strcursor::StrCursor::new_at_start(&line[*line_j..]);

			//for b in line[*line_j..].bytes() {
      while grapheme_cursor.byte_pos() < line_len {
				*line_j += 1;

        //print_str.push(b as char);
        let grapheme = match grapheme_cursor.next() {
          Some((g, c)) => {
            grapheme_cursor = c;
            g
          }
          None => { continue; }
        };
        print_str.push_str(grapheme.as_str());
				if grapheme.as_str() == "\x1B" {        // Start of ansi format
					do_count = false;
				} else if do_count {  // Visible character
					length += 1;
					if length >= print_width {
						*more = true;
						break;
					}
				} else if grapheme.as_str() == "\x6D" { // End of ansi format
          last_fmt.push_str(grapheme.as_str());
					do_count = true;
				}
        
        // Multi-line formatting
        if !do_count {
          last_fmt.push_str(grapheme.as_str());
        }
			}

      print::print_neutral(&start_fmt);
      print::print_neutral(&print_str.replace("\t", " "));

			// Pad extra spaces if it didn't reach the end
			if length < print_width {
				*more = false;
				for _ in length..print_width { print::print(" "); }
			}
		};

		// This loop makes the assumption that each diff follows this pattern:
		// 1,1c1,1
		// line1_file1
		// line1_file2
		// ...
		// Where the first number pair is the line counts for file 1, and the
		// second number pair is the line counts for file 2 in a single diff
		// With this info we can calculate how many lines will follow until the
		// next line counts appears. 
		loop {
			// Get the line that has the line numbers for the diffs
			let line_counts_str = match lines.next() {
				Some(item) => item,
				None => break,
			};

			// Parse the line counts diff data
      let line_counts: Vec<&str>;
      let left_line_counts: Vec<&str>;
      let right_line_counts: Vec<&str>;
      let mut left_start: isize = 0;
      let left_end: isize;
      let mut left_diff: isize = -1;
      let mut right_start: isize = 0;
      let right_end: isize;
      let mut right_diff: isize = -1;

      // The 'a' denotes an addition of the right file compared to the other
      if line_counts_str.contains("a") {
        line_counts = line_counts_str.split('a').collect();
        right_line_counts = line_counts[1].split(',').collect();
        left_start = match line_counts[0].parse() {
          Ok(us) => us,
          Err(err) => { return format!("Failed to parse the left start \"{}\". {}\n", line_counts[0], err); }
        };
        right_start = match right_line_counts[0].parse() {
          Ok(us) => us,
          Err(err) => { return format!("Failed to parse the right start \"{}\". {}\n", right_line_counts[0], err); }
        };
        right_end = match right_line_counts[1].parse() {
          Ok(us) => us,
          Err(err) => { return format!("Failed to parse the right end \"{}\". {}\n", right_line_counts[1], err); }
        };
        right_diff = right_end - right_start;
      // The 'c' denotes a comparison between the two files
      } else if line_counts_str.contains("c") {
        line_counts = line_counts_str.split('c').collect();
        left_line_counts = line_counts[0].split(',').collect();
        right_line_counts = line_counts[1].split(',').collect();
        left_start = match left_line_counts[0].parse() {
          Ok(us) => us,
          Err(err) => { return format!("Failed to parse the left start \"{}\". {}\n", left_line_counts[0], err); }
        };
        left_end = match left_line_counts[1].parse() {
          Ok(us) => us,
          Err(err) => { return format!("Failed to parse the left end \"{}\". {}\n", left_line_counts[1], err); }
        };
        left_diff = left_end - left_start;
        right_start = match right_line_counts[0].parse() {
          Ok(us) => us,
          Err(err) => { return format!("Failed to parse the right start \"{}\". {}\n", right_line_counts[0], err); }
        };
        right_end = match right_line_counts[1].parse() {
          Ok(us) => us,
          Err(err) => { return format!("Failed to parse the right end \"{}\". {}\n", right_line_counts[1], err); }
        };
        right_diff = right_end - right_start;
      // The 'd' denotes an addition of the left file compared to the other
      } else if line_counts_str.contains("d") {
        line_counts = line_counts_str.split('d').collect();
        left_line_counts = line_counts[0].split(',').collect();
        left_start = match left_line_counts[0].parse() {
          Ok(us) => us,
          Err(err) => { return format!("Failed to parse the left start \"{}\". {}\n", left_line_counts[0], err); }
        };
        left_end = match left_line_counts[1].parse() {
          Ok(us) => us,
          Err(err) => { return format!("Failed to parse the left end \"{}\". {}\n", left_line_counts[1], err); }
        };
        left_diff = left_end - left_start;
        right_start = match line_counts[1].parse() {
          Ok(us) => us,
          Err(err) => { return format!("Failed to parse the right start \"{}\". {}\n", line_counts[1], err); }
        };
      }
			let max_diff = if left_diff > right_diff { left_diff } else { right_diff };

			// Fill the vectors with their respective lines
			let mut left_lines: Vec<&str> = Vec::new();
			let mut right_lines: Vec<&str> = Vec::new();
			// The next left_diff pushes will be for the left side
      if left_diff != -1 {
        for _ in 0..=left_diff {
          let next_line = match lines.next() {
            Some(l) => l,
            None => "",
          };
          left_lines.push(next_line);
        }
        // If left isn't the largest difference, pad blank strings
        for _ in 0..(max_diff - left_diff) { left_lines.push(""); }
      } else {
        for _ in 0..=max_diff { left_lines.push(""); }
      }
			// The next right_diff pushes will be for the right side
      if right_diff != -1 {
        for _ in 0..=right_diff {
          let next_line = match lines.next() {
            Some(l) => l,
            None => "",
          };
          right_lines.push(next_line);
        }
        // If right isn't the largest difference, pad blank strings
        for _ in 0..(max_diff - right_diff) { right_lines.push(""); }
      } else {
        for _ in 0..=max_diff { right_lines.push(""); }
      }

			// Loop through all the lines just added
			for i in 0..=max_diff {
				let mut left_more = true;
				let mut right_more = true;
				let mut left_j = 0;
				let mut right_j = 0;
        let mut last_left_fmt = String::from("");
        let mut last_right_fmt = String::from("");

				// Continue until there is no more
				while left_more || right_more {
					// Left side
					if left_more {
						print_diff_format(left_lines[i as usize], (left_start + i) as usize, &mut left_j, &mut left_more, &mut last_left_fmt);
					} else {
            print::print_neutral(&format!("\x1b[0m\x1b[38;5;208m|{1:^5}|\x1b[0m{1:0$}\x1b[0m", print_width, ' '));
          }

					// Right side
					if right_more {
						print_diff_format(right_lines[i as usize], (right_start + i) as usize, &mut right_j, &mut right_more, &mut last_right_fmt);
					} else {
            print::print_neutral(&format!("\x1b[0m\x1b[38;5;208m|{1:^5}|\x1b[0m{1:0$}\x1b[0m", print_width, ' '));
          }
					print::println("");
				}
			}
			// Add a blank line (with columns) if there is more to format
			if lines.peek().is_some() {
				print::println_neutral(&format!("\x1b[38;5;208m|{1:^5}|\x1b[0m{1:0$}\x1b[38;5;208m|{1:^5}|\x1b[0m", print_width, ' '));
			}
		}
		terminal::flush();
		log::log("CCDiff", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/ccdiff/atf/atf.txt";

		// Run check
		let check = match util::pipes::Pipe::new(&"ext/ccdiff/ccdiff tst/ccdiff/inp/left.txt tst/ccdiff/inp/right.txt".to_string())
			.then("wc -l")
			.finally() {
				Ok(sel) => sel,
					Err(err) => {
						debug::print_debug(format!("ccdiff::on_test: Failed to create the pipe. {}", err));
						return TestStatus::Failed;
					}
			};

		let check = match check.wait_with_output() {
			Ok(sel) => sel,
				Err(err) => {
					debug::print_debug(format!("ccdiff::on_test: Failed when waiting for the process to finish. {}", err));
					return TestStatus::Failed;
				}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
				Err(err) => {
					debug::print_debug(format!("ccdiff::on_test: Failed to open the test file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ccdiff(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(CCDiff {
    event: Event {
      name:   name,
			desc:   "File diffing tool for easy comparison with our own modifications.".to_string(),
			usage:  "Provide two file paths and the differences will be shown.\n".to_string(),
			parent: parent,
			author: "r00r00 and H.Merijn Brand\n\n\tCopyright (c) 2018-2021 H.Merijn Brand".to_string(),
			easyrun: false,
			secure: false,
			links:  links
		}
	})
}

/*********************************** tre ***********************************/
pub struct Tre { event: Event }
impl Eventable for Tre {
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let arguments;
    if args.len() == 2 { arguments = vec!["-ad".to_string(), "-l".to_string(), args[0].clone(), args[1].clone()]; }
    else {
      if !main_info::is_bg() {
        print::print_custom("tre <traversal_depth> <path>\n", "orange");
        return String::from("");
      }
      else { return String::from("Tre command does not support running in the background.\n"); }
    }
    let output = match run_console_command(Command::new("ext/tre/tre").args(arguments)) {
      Ok(out) => out,
      Err(e) => { return format!("Running tre command failed: {}\n", e); }
    };
    // Print output
    log::log("tre", &String::from_utf8_lossy(&output.stderr));
    //return String::from();
    if main_info::is_bg() {
      print!("Tre command does not support running in the background.");
      return String::from("Tre command does not support running in the background.\n"); }
    else { return String::from_utf8_lossy(&output.stdout).to_string(); }
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/tre/atf/atf.txt";
    let input_path = "tst/tre/inp";
    let arguments = vec!["-ad", "-l", "2", input_path];

    // Run check 
    let check = match Command::new("ext/tre/tre").args(arguments).output() {
      Ok(out) => out, 
      Err(_) => { return TestStatus::Failed; }  
    };   
    
    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out, 
      Err(err) => { 
        debug::print_debug(format!("tre::on_test: Failed to read artifact. {}", err));
        return TestStatus::Failed;
      }    
    };   

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }    
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn tre (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Tre {
    event: Event {
      name:   name,
      desc:   "Like the standard tree command but with improvements.".to_string(),
      usage:  "tre <traversal_depth> <path>\n".to_string(),
      parent: parent,
      author: "Daniel Duan\nDave Lee\nJanek Spaderna".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }    
  })
}

/*********************************** bingrep ***********************************/
pub struct Bingrep { event: Event }
impl Eventable for Bingrep {
  fn on_run(&self, args: Vec<String>) -> String {
    if !(args.len() == 2) {
      print::print_custom("Needs two arguments for (1)option and (2)path to binary.\n","orange");
      return self.event.usage.clone();
    }
    if args[0] != "all" && args[0] != "info" && args[0] != "phdr" && args[0] != "shdr" && args[0] != "syms" && args[0] != "dsyms" &&
       args[0] != "reloc" && args[0] != "libs" && args[0] != "ranges" {
      return self.event.usage.clone();
    }

    if main_info::is_bg() {
			// Run command
			let output = match Command::new("ext/bingrep/bgrep").args(vec![args[0].trim(),args[1].trim(),"0"]).output() {
				Ok(out) => out,
				Err(err) => {
					print::print_custom(&format!("Error: {}\n", err), "orange");
					return format!("Running bgrep command failed: {}\n", err);
				}
			};
			log::log("bingrep", &String::from_utf8_lossy(&output.stderr));
			return String::from_utf8_lossy(&output.stdout).to_string();
    }
    else {
			// Run command
			let output = match Command::new("ext/bingrep/bgrep").args(vec![args[0].trim(),args[1].trim(),"1"]).output() {
				Ok(out) => out,
				Err(err) => {
					print::print_custom(&format!("Error: {}\n", err), "orange");
					return format!("Running bgrep no color command failed: {}\n", err);
				}
			};
			log::log("bingrep", &String::from_utf8_lossy(&output.stderr));
			return String::from_utf8_lossy(&output.stdout).to_string();
    }
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/bingrep/atf/atf.txt";
    let input_path = "tst/bingrep/inp/test.elf";

    let check = match Command::new("ext/bingrep/bgrep").args(vec!["info", input_path,"1"]).output() {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("bingrep::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("bingrep::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str {
      debug::print_debug("Output differs");
      debug::print_debug(format!("bingrep stdout: {}", String::from_utf8_lossy(&check.stdout)));
      debug::print_debug(format!("filestr: {}", file_str));
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn bingrep(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Bingrep {
    event: Event {
      name:   name,
      desc:   "Bingrep binary analysis utility.".to_string(),
      usage:  "This ELF analysis tool takes the following arguments:\n\
                \t1) all|info|phdr|shdr|syms|dsyms|reloc|libs|ranges\n\
                \t2) Path to binary\n".to_string(),
      author: "m4b".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** readelf ***********************************/
pub struct Readelf { event: Event }
impl Eventable for Readelf {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 2 {
      print::print_custom("Needs (1)option and (2)path to binary.\n","orange");
      return self.event.usage.clone();
    }

    let option;
    match args[0].trim() {
      "all" => option = "--all",
     "phdr" => option = "--program-headers",
     "shdr" => option = "--sections",
     "sgrp" => option = "--section-groups",
     "sdet" => option = "--section-details",
     "syms" => option = "--syms",
   "relocs" => option = "--relocs",
      "dyn" => option = "--dynamic",
           _=> return self.event.usage.clone(),
    }

    // Run command
    let output = match Command::new("ext/readelf/creadelf.pl").args(vec![option, args[1].trim()]).output() {
      Ok(out) => out,
      Err(err) => {
        return format!("Creadelf command failed to run: {}\n", err);
      }
    };

    // Print output
    log::log("readelf", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/readelf/atf/atf.txt";
    let input_path = "tst/readelf/inp/test.elf";

    let check = match Command::new("ext/readelf/creadelf.pl").args(vec!["-l",input_path]).output() {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("readelf::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("readelf::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn readelf(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Readelf {
    event: Event {
      name:   name,
      desc:   "Readelf ELF analysis applet.".to_string(),
      usage:  "This ELF analysis tool takes the following arguments:\n\
                \t1) all|phdr|shdr|sgrp|sdet|syms|relocs|dyn\n\
                \t2) Path to binary\n".to_string(),
      author: "m4b\n\nCopyright (c) m4b 2017-2018".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** Nm ***********************************/
pub struct Nm{ event: Event }
impl Eventable for Nm {
 
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 2 { return self.event.usage.clone(); }
    if args.len() == 2 {
      if args[0] != "demangle" { return self.event.usage.clone(); }
      let mut path = args[1].clone();
      util::misc::reltoabs(&mut path);

      // Run command
      let _output = match run_console_command(Command::new("acoc").args(vec!["nm", "-na", "--demangle", "--special-syms", "--with-symbol-versions", &path]).current_dir("ext/nm/bin")) {
        Ok(out) => out,
        Err(err) => {
          return format!("Acoc command failed to run: {}\n", err);
        }
      };
      return String::from("");
    }
    let mut path = args[0].clone();
    util::misc::reltoabs(&mut path);

    // Run command
    let _output = match run_console_command(Command::new("acoc").args(vec!["nm", "-na", "--special-syms", "--with-symbol-versions", &path]).current_dir("ext/nm/bin")) {
      Ok(out) => out,
      Err(err) => {
        return format!("Acoc command failed to run: {}\n", err);
      }
    };

    // Print output
    log::log("nm", &String::from_utf8_lossy(&_output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/nm/atf/atf.txt";
    
    // Run check
    let check = match Command::new("ext/nm/bin/nm").arg("-V").output() {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("nm::on_test: Failed to run command {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("nm::on_test: Failed to open the artifact file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn nm(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Nm {
    event: Event {
      name:   name,
      desc:   "Nm object file analysis utility.".to_string(),
      usage:  "Takes a path to an object file, executable, or object-file library.\n\nCan also demangle like so:\n\tnm demangle <path>\n".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** xxd ***********************************/
pub struct Xxd{ event: Event }
impl Eventable for Xxd {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 3 { return self.event.usage.clone(); } // Handle invalid args
    util::misc::reltoabs(&mut args[2].clone());

    // Run command
    let output = match Command::new("ext/xxd/xxd-rs").args(vec!["--seek",args[0].trim(),"--length",args[1].trim(),"dump",args[2].trim()]).output() {
      Ok(o) => o,
      Err(err) => {
        return format!("Failed to run xxd command: {}\n", err);
      }
    };

    // Print output
    print::print_custom("","reset");
    let mut out = String::from_utf8_lossy(&output.stdout).to_string();
    out.insert_str(0,"\x1b[38;5;85m");
    out = out.replace(" 0000","\x1b[38;5;8m 0000\x1b[38;5;85m").replace(".","\x1b[38;5;8m.\x1b[38;5;85m").replace(":","\x1b[38;5;7m:\x1b[38;5;85m");
    print::print(&out);
    print::print_custom("","reset");
    log::log("xxd", &out);
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/xxd/atf/atf.txt";
    let input_path = "tst/xxd/inp/test.elf";

    let check = match Command::new("ext/xxd/xxd-rs").args(vec!["--seek","0","--length","100","dump",input_path]).output() {
      Ok(c) => c,
      Err(err) => {
        debug::print_debug(format!("xxd::on_test: Failed to run test command. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("xxd::on_test: Failed to open artifact file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn xxd(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Xxd {
    event: Event {
      name:   name,
      desc:   "Xxd binary analysis utility.".to_string(),
      usage:  "Binary analysis tool requires these three arguments:\n\
                \t1) Starting offset in file to dump\n\
                \t2) Ending offset in file to dump\n\
                \t3) Path to binary file\n".to_string(),
      author: "Nicola Coretti\n\nCopyright (c) 2017, Nicola Coretti".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** procs ***********************************/
pub struct Procs { event: Event }
impl Eventable for Procs {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() > 0 { return self.event.usage.clone(); }
    // Run command
    let out = match run_console_command(Command::new("ext/procs/procs").args(vec!["-t","-p","disable"])) {
      Ok(o) => o,
      Err(err) => {
        return format!("Procs command failed to run: {}\n", err);
      }
    };
    return String::from_utf8_lossy(&out.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/procs/atf/atf.txt";
    let check = match Command::new("ext/procs/procs").arg("-V").output() {
      Ok(out) => out,
      Err(e) => {
        debug::print_debug(format!("procs::on_test: Failed to open the test file. {}", e));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(e) => {
        debug::print_debug(format!("procs::on_test: Failed to open the test file. {}", e));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn procs(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Procs {
    event: Event {
      name:   name,
      desc:   "Procs process manager.".to_string(),
      usage:  "No arguments needed to invoke this process manager.\n".to_string(),
      author: "dalance\tdalance@gmail.com".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** readelfmaster ***********************************/
pub struct Readelfmaster{ event: Event }
impl Eventable for Readelfmaster {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() > 2 || args.len() < 2 { return self.event.usage.clone(); }
    let option;
    let mut second_cmd = String::new();
    match args[0].trim() {
      "sect" => {
                  option = " -S";
                  second_cmd.push_str("python3 ext/util/colorize/colorize.py -a yellow,green");
                }
      "phdr" => { 
                  option = " -l";
                  second_cmd.push_str("python3 ext/util/colorize/colorize.py -a blue,cyan");
                 }
      "dsym" => {
                  option = " -d";
                  second_cmd.push_str("python3 ext/util/colorize/colorize.py -c white,grey");
                }
           _ => return self.event.usage.clone()
    }

    let mut cmd = String::from("ext/readelfmaster/readelfmaster ");
    cmd.push_str(args[1].trim());
    cmd.push_str(option);

    // Run check
    let out = match util::pipes::Pipe::new(&cmd)
                                        .then(&second_cmd)
                                        .finally() {
      Ok(sel) => sel, 
      Err(err) => { 
        return format!("Failed execute readelfmaster command: {}\n", err);
      }    
    };   

    let output = match out.wait_with_output() {
      Ok(sel) => sel, 
      Err(err) => { 
        return format!("Failed in waiting for readelfmaster command: {}\n", err);
      }    
    };
    // Print output
    log::log("readelfmaster", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/readelfmaster/atf/atf.txt";

    // Run check
    let out = match util::pipes::Pipe::new("ext/readelfmaster/readelfmaster -d tst/readelfmaster/inp/test.elf")
      .then("python3 ext/util/colorize/colorize.py -c white,grey")
      .finally() {
        Ok(sel) => sel, 
        Err(err) => { 
        debug::print_debug(format!("readelfmaster::on_test Error: {}", err));
        print::print_custom(&format!("Failed to run command. {}\n", err), "orange");
        return TestStatus::Failed;
        }    
    };   

    let check = match out.wait_with_output() {
      Ok(sel) => sel, 
      Err(err) => { 
        print::print_custom(&format!("Failed to run command. {}", err),"orange");
        return TestStatus::Failed;
      }    
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("binwalk::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn readelfmaster(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Readelfmaster {
    event: Event {
      name:   name,
      desc:   "ReadELFmaster binary analysis utility.".to_string(),
      usage:  "This ELF analyis tool takes two arguments:\n\
                \t1) sect|phdr|dsym\n\
                \t2) Path to binary\n".to_string(),
      author: "Jonathan Dewein".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** shred ***********************************/
pub struct Shred { event: Event }
impl Eventable for Shred {
  fn on_run(&self, args: Vec<String>) -> String {
    let mut iter = "9".to_string();
    let mut file = args[0].clone();
    if args.len() < 1 || args.len() > 2 { return self.event.usage.clone(); } // Handle invalid args
    if args.len() == 2 {
      iter = args[0].clone();
      file = args[1].clone();
    }
    if file.eq("/") {
        print::print_custom("Directory off limits to shredding.\n","orange");
        return String::from("");
    }
    use std::fs::metadata;
    let md = match metadata(file.clone()) {
      Ok(o) => o,
     Err(_) => return String::from("Shred command error: can't get file metadata. Are you sure it exists?\n"),
    };
    if md.is_dir() {
      print::print_custom("Can't shred a directory. Use rmdir instead.\n","orange");
      return String::from("");
    }
    let file_vec: Vec<&str> = file.trim().split("/").collect();
    let root_dir = file_vec[1];
    let nono_direc = vec!["boot", "dev", "etc", "root", "sys" , "usr"];
    for dir in nono_direc {
      if root_dir.eq(dir) {
        print::print_custom("Directory off limits to shredding.\n","orange");
        return String::from("");
      }
    }

    // Last chance warning
    let mut last_chance = String::from("You sure you want to delete ");
    last_chance.push_str(&file);
    last_chance.push_str("?");
    if alerts::confirm_task(&last_chance) == constants::UNCONFIRMED {
      print::print_custom("Operation cancelled.\n","orange");
      return String::from("");
    }
    let arguments = vec!["shred".to_string(),"-vzuf".to_string(),"-n".to_string(),iter,file.clone()];
    // Run shred command
    let output = match run_console_command(Command::new("ext/util/coreutils").args(&arguments)) {
      Ok(out) => out,
      Err(e) => { return format!("Failed to run coreutils-shred command: {}\n",e); }
    };
    if !std::path::Path::new(&file).exists() {
      print::print_custom("File removal successful.\n","brightgreen");
    }
    log::log("shred", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/shred/test.txt";
    let input_path = "tst/shred/inp/test.txt";
    let artifact_path = "tst/shred/atf/atf.txt";
    util::misc::cleanup(staging_path);
    let copy_args = vec![input_path, staging_path];

    // Run command
    match Command::new("cp").args(copy_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Run command
    match Command::new("ext/util/coreutils").args(vec!["shred","-uf",staging_path]).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    // Run check
    let check_out = match util::pipes::Pipe::new(&"ls tst/shred".to_string())
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("shred::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel, 
      Err(err) => { 
        debug::print_debug(format!("shred::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }    
    };   

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("shred::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn shred(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Shred {
    event: Event {
      name:   name,
      desc:   "The Shred file destruction tool. Performs various overwrites before deleting.".to_string(),
      usage:  "This applet takes one argument: path to file to delete.\n".to_string(),
      parent: parent,
      author: "Jordi Boggiano\n\nCopyright (c) Jordi Boggiano".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** srm ***********************************/
pub struct Srm { event: Event }
impl Eventable for Srm {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 2 { return self.event.usage.clone(); }
    let arguments = vec!["-v", args[0].trim()];
    if arguments[1].eq("-rf") {
      println!("This applet won't do recursive delete. There are other applets for that.");
    }
    let mut last_chance = String::from("You sure you want to delete ");
    last_chance.push_str(&args[0].trim());
    last_chance.push_str(" ?");
    if alerts::confirm_task(&last_chance) == constants::UNCONFIRMED {
      print::print_custom("Operation cancelled.\n","orange");
      return String::from("");
    };

    // Run shred command
    let output = match run_console_command(Command::new("ext/srm/srm").args(&arguments)) {
      Ok(out) => out,
      Err(e) => { return format!("Srm command failed: {}\n", e); }
    };
    log::log("srm", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/srm/test.txt";
    let input_path = "tst/srm/inp/test.txt";
    let artifact_path = "tst/srm/atf/atf.txt";
    util::misc::cleanup(staging_path);
    let copy_args = vec![input_path, staging_path];

    // Run command
    match Command::new("cp").args(copy_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Run command
    match Command::new("ext/srm/srm").arg(staging_path).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    // Run check
    let check_out = match util::pipes::Pipe::new(&"find tst/srm/ -maxdepth 1 -name test.txt".to_string())
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("srm::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("srm::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("shred::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn srm(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Srm {
    event: Event {
      name:   name,
      desc:   "Secure 38-pass overwrite file removal.".to_string(),
      usage:  "Input your file path and the file shall be no more.\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** findfile ***********************************/
pub struct Findfile { event: Event }
impl Eventable for Findfile {
  fn on_run(&self, mut args: Vec<String>) -> String {
    // Run command
    if args.len() == 2 { args.push("-aHLIs".to_string()); }
    else { return self.event.usage.clone(); }
    let output = match run_console_command(Command::new("ext/findfile/fd").args(args)) {
      Ok(out) => out,
      Err(e) => { return format!("fd command failed: {}\n", e); }
    };
    // Print output
    log::log("find", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/findfile/atf/atf.txt";

     // Run check
    let check_out = match util::pipes::Pipe::new(&"ext/findfile/fd -aHLIs test tst/findfile/inp".to_string())
      .then("wc -l")
      .finally() {
        Ok(sel) => sel,
        Err(err) => {
          debug::print_debug(format!("fd::on_test: Failed to execute test command. {}", err));
          return TestStatus::Failed;
        }
      };

    let check = match check_out.wait_with_output() {
      Ok(o) => o,
      Err(e) => {
        debug::print_debug(format!("fd::on_test: Failed to open the test file. {}", e));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("fd::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn findfile (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Findfile {
    event: Event {
      name:   name,
      desc:   "A modification of the standard find command.".to_string(),
      usage:  "This finder takes two arguments:\n\t1) File or directory name to find\n\t2) Path to directory to search\n".to_string(),
      parent: parent,
      author: "David Peter and the fd developers\n\nCopyright (c) 2017-2021 The fd developers".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }
  })
}

/*********************************** file ***********************************/
pub struct File { event: Event }
impl Eventable for File {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() > 1 || args.len() < 1 { return self.event.usage.clone(); }
    let mut cmd = String::from("file ");
    cmd.push_str(args[0].trim());
    // Run command
    let check_out = match util::pipes::Pipe::new(&cmd)
      .then("ext/file/slm_file")
      .finally() {
        Ok(sel) => sel,
        Err(err) => return format!("Failed to execute file command. {}\n", err)
    };
    let output = match check_out.wait_with_output() {
      Ok(o) => o,
      Err(e) => return format!("Failed to execute file command. {}\n", e)
    };
    log::log("file", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/file/atf/atf.txt";

     // Run check
    let check_out = match util::pipes::Pipe::new(&"file tst/file/inp/test.elf".to_string())
      .then("ext/file/slm_file")
      .finally() {
        Ok(sel) => sel,
        Err(err) => {
          debug::print_debug(format!("file::on_test: Failed to execute test command. {}", err));
          return TestStatus::Failed;
        }
      };

    let check = match check_out.wait_with_output() {
      Ok(o) => o,
      Err(e) => {
        debug::print_debug(format!("file::on_test: Failed to open the test file. {}", e));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("fd::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn file (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(File {
    event: Event {
      name:   name,
      desc:   "Like the standard file command but, naturally, with some improvements.".to_string(),
      usage:  "Input a path to file to analyze metadata.\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }
  })
}

/*********************************** ping ***********************************/
pub struct Ping { event: Event }
impl Eventable for Ping {
  fn on_run(&self, args: Vec<String>) -> String {
    let mut arguments = Vec::new();
    let interface; let selections_str;
    if args.len() < 1 || args.len() > 3 { return self.event.usage.clone(); }
    if args[0] != "::1" && args[0] != "0:0:0:0:0:0:0:1" && args[0].matches(":").count() > 1 {
      arguments.push("ping6");
			if args.len() == 1 { // ping [ipaddr, 20, .2]
				arguments.push("-c");
				arguments.push("20");
				arguments.push("-i");
				arguments.push(".2");
			}
			if args.len() == 2 { // ping [ipaddr, iterations, .2]
				arguments.push("-c");
				arguments.push(args[1].trim()); //iterations -- optional
				arguments.push("-i");
				arguments.push(".2");
			}
			if args.len() == 3 { // ping [ipaddr, iterations, interval]
				arguments.push("-c");
				arguments.push(args[1].trim()); //iterations -- optional
				arguments.push("-i");
				arguments.push(args[2].trim()); //interval-- optional
			}
      arguments.push("-I");
      let selections = match util::pipes::Pipe::new("netstat -i")
      .then("tail -n +3").then("ext/util/choose 0").finally() {
        Ok(sel) => sel,
        Err(err) => {
          print::println(&format!("Unable to retrieve network devices. Error : {}",err));
          return String::from("");
        }
      };
			let selections2 = match selections.wait_with_output() {
				Ok(sel) => sel,
				Err(err) => {
					print::println(&format!("Unable to retrieve network devices. Error : {}",err));
					return String::from("");
				}
			};
			selections_str = String::from_utf8_lossy(&selections2.stdout).to_string();
			let slcs: Vec<&str> = selections_str.split_whitespace().collect();
			interface = match terminal::get_selection(slcs.clone()) {
				Some(opt) => slcs[opt],
				_ => {
					print::print_custom("Bad selection.\n","orange");
					return String::from("");
				}
			};
			arguments.push(interface);
    }
    else { arguments.push("ping"); }
    arguments.push(args[0].trim()); //ipaddr -- required
    if args.len() == 1 { // ping [ipaddr, 20, .2]
      arguments.push("-c");
      arguments.push("20");
      arguments.push("-i");
      arguments.push(".2");
    }
    if args.len() == 2 { // ping [ipaddr, iterations, .2]
      arguments.push("-c");
      arguments.push(args[1].trim()); //iterations -- optional
      arguments.push("-i");
      arguments.push(".2");
    }
    if args.len() == 3 { // ping [ipaddr, iterations, interval]
      arguments.push("-c");
      arguments.push(args[1].trim()); //iterations -- optional
      arguments.push("-i");
      arguments.push(args[2].trim()); //interval-- optional
    }
    let output = match run_console_command(Command::new("ext/ping/clrr").args(arguments)) {
      Ok(out) => out,
      Err(_) => { return String::from("Failed to execute\n"); }
    };

    log::log("ping", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/ping/atf/atf.txt";
    let arguments = vec!["ping","-V"];
    let check = match Command::new("ext/ping/clrr").args(arguments).output() {
      Ok(out) => out,
      Err(e) => {
        debug::print_debug(format!("ping::on_test: Failed to open the test file. {}", e));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(e) => {
        debug::print_debug(format!("ping::on_test: Failed to open the test file. {}", e));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ping (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Ping {
    event: Event {
      name:   name,
      desc:   "The ping command with Salvum style.".to_string(),
      usage:  "There are 3 ways you can run ping:\
               \n\tping <IP_addr>\
               \n\tping <IP_addr> [iterations]\
               \n\tping <IP_addr> [iterations] [interval]\n\n\
               Default values:\
               \n\titerations = 20\
               \n\tinterval = .2 (seconds)\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }
  })
}

/*********************************** asm ***********************************/
pub struct Asm { event: Event }
impl Eventable for Asm {
  fn on_run(&self, args: Vec<String>) -> String {
    let mut arguments = Vec::new();
    let mut instruction = String::new();
    if args.len() < 2 { return self.event.usage.clone(); }
    arguments.push(args[0].clone());
    for arg in &args[1..(args.len() - 1)] { instruction.push_str(arg); instruction.push_str(" "); }
    instruction.push_str(&args[args.len() - 1]);
    arguments.push(instruction);
    let output = match Command::new("ext/asm/aas").args(arguments).output() {
      Ok(out) => out,
      Err(e) => { return format!("aas command failed: {}\n", e); }
    };
    // Print output
    let out = String::from_utf8_lossy(&output.stdout);
    print::print_custom(&out,"brightgreen");
    log::log("asm", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact_ppc = "tst/asm/atf/atf_ppc.txt";
    let artifact_ppc64 = "tst/asm/atf/atf_ppc64.txt";
    let artifact_arm = "tst/asm/atf/atf_arm.txt";
    let artifact_arm64 = "tst/asm/atf/atf_arm64.txt";
    let artifact_x86 = "tst/asm/atf/atf_x86.txt";
    let artifact_x64 = "tst/asm/atf/atf_x64.txt";
    let artifact_sparc = "tst/asm/atf/atf_sparc.txt";
    let artifact_sparc64 = "tst/asm/atf/atf_sparc64.txt";
    let artifact_mips = "tst/asm/atf/atf_mips.txt";
    let artifact_mips64 = "tst/asm/atf/atf_mips64.txt";
    let artifact_riscv = "tst/asm/atf/atf_riscv.txt";
    let artifact_riscv64 = "tst/asm/atf/atf_riscv64.txt";
    let artifact_avr = "tst/asm/atf/atf_avr.txt";

    let artifact_vec = vec![artifact_ppc,artifact_ppc64,artifact_arm,artifact_arm64,artifact_x86,
                            artifact_x64,artifact_sparc,artifact_sparc64,artifact_mips,
                            artifact_mips64,artifact_riscv,artifact_riscv64,artifact_avr];
    
    let input_ppc = "addi 3,3,48"; 
    let input_ppc64 = "ld 3, 8(4)";
    let input_arm = "ADD r0, r0, r1";
    let input_arm64 = "add x0, x1, x2";
    let input_x86 = "mov %ebp, %esp";
    let input_x64 = "mov %rbp, %rsp";
    let input_sparc = "add %l1, %l2, %l3";
    let input_sparc64 = "add %l1, %l2, %l3";
    let input_mips = "sll $t0, $t0, 2";
    let input_mips64 = "addu $4, $4, $11";
    let input_risc = "addi a4,a0,1400";
    let input_riscv64 = "addi a4,a0,1400";
    let input_avr = "ldi r24,0x00";

    let input_vec = vec![input_ppc,input_ppc64,input_arm,input_arm64,input_x86,input_x64,
                         input_sparc,input_sparc64,input_mips,input_mips64,input_risc,
                         input_riscv64,input_avr];

    let architecture_vec = vec!["ppc32","ppc64","arm32","arm64","x86","x64","sparc32",
                                "sparc64","mips32","mips64","riscv32","riscv64","avr"];
    
    // Run check
    for i in 0..architecture_vec.len() {
			let check = match Command::new("ext/asm/aas").args(vec![architecture_vec[i], input_vec[i]])
                                                    .output() {
				Ok(out) => out,
				Err(e) => {
					debug::print_debug(format!("asm::on_test: Failed to run command: {}: {}", i, e));
					return TestStatus::Failed;
				}
      };
      // Read file
			let file_str = match fs::read_to_string(artifact_vec[i]) {
				Ok(out) => out,
				Err(err) => {
					debug::print_debug(format!("asm::on_test: Failed to open artifact file: {}: {}", i, err));
					return TestStatus::Failed;
				}
			};

			// Compare
			if String::from_utf8_lossy(&check.stdout) != file_str {
				debug::print_debug(format!("asm::on_test: Output doesn't match artifact: {}", i));
        return TestStatus::Failed;
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn asm (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Asm {
    event: Event {
      name:   name,
      desc:   "Salvum's built-in assembler.".to_string(),
      usage:  "This assembler takes the following arguments:\
              \n\t1) x86|x64|ppc32|ppc64|arm32|arm64|sparc32|sparc64|mips32|mips64|riscv32|riscv64|avr\
              \n\t2) Instruction\
              \n\nExample: asm x64 mov %rbp, %rsp\n".to_string(),
      parent: parent,
      author: "(c) 2021 QVLX LLC. All rights reserved.".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }
  })
}

/*********************************** dasm ***********************************/
pub struct Dasm { event: Event }
impl Eventable for Dasm {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 2 { return self.event.usage.clone(); }

    let output = match Command::new("ext/dasm/das").args(args).output() {
      Ok(out) => out,
      Err(e) => { return format!("das command did not run correctly: {}\n", e); }
    };
    // Print output
    let out = String::from_utf8_lossy(&output.stdout);
    print::print_custom(&out.replace("\t"," ")
                               .replace("  "," ")
                               .replace("   "," ")
                               .replace("    "," ")
                               .replace("     "," ")
                               .replace("      "," "),"brightgreen");
    
    log::log("dasm", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact_ppc = "tst/dasm/atf/atf_ppc.txt";
    let artifact_ppc64 = "tst/dasm/atf/atf_ppc64.txt";
    let artifact_arm = "tst/dasm/atf/atf_arm.txt";
    let artifact_arm64 = "tst/dasm/atf/atf_arm64.txt";
    let artifact_x86 = "tst/dasm/atf/atf_x86.txt";
    let artifact_x64 = "tst/dasm/atf/atf_x64.txt";
    let artifact_sparc = "tst/dasm/atf/atf_sparc.txt";
    let artifact_sparc64 = "tst/dasm/atf/atf_sparc64.txt";
    let artifact_mips = "tst/dasm/atf/atf_mips.txt";
    let artifact_mips64 = "tst/dasm/atf/atf_mips64.txt";
    let artifact_riscv = "tst/dasm/atf/atf_riscv.txt";
    let artifact_riscv64 = "tst/dasm/atf/atf_riscv64.txt";
    let artifact_avr = "tst/dasm/atf/atf_avr.txt";

    let artifact_vec = vec![artifact_ppc,artifact_ppc64,artifact_arm,artifact_arm64,artifact_x86,
                            artifact_x64,artifact_sparc,artifact_sparc64,artifact_mips,
                            artifact_mips64,artifact_riscv,artifact_riscv64,artifact_avr];
    
    let input_ppc = "a602687f781b6d7c78238e7c30006338"; 
    let input_ppc64 = "380081e8080064e8";
    let input_arm = "e3a0000ae3a01003e0800001e3a00018e51f1000ef12345600020026";
    let input_arm64 = "8b020020cb0200208a220020f9400020";
    let input_x86 = "5589ec292504000000575689ec5e5f";
    let input_x64 = "554889ec4829242520000000";
    let input_sparc = "a6044012a2046001b620001b8024600aa4044000a8000000";
    let input_sparc64 = "a6044012a2046001b620001b8024600aa4044000a8000000";
    let input_mips = "8f8800008f8900040109482a1120ffff0000000000084080011c4020ad00001c";
    let input_mips64 = "008b2021009530210000000000000000";
    let input_risc = "57850713";
    let input_riscv64 = "57850713";
    let input_avr = "80e094e060e170e090930106";
    
    let input_vec = vec![input_ppc,input_ppc64,input_arm,input_arm64,input_x86,input_x64,
                         input_sparc,input_sparc64,input_mips,input_mips64,input_risc,
                         input_riscv64,input_avr];

    let architecture_vec = vec!["ppc32","ppc64","arm32","arm64","x86","x64","sparc32",
                                "sparc64","mips32","mips64","riscv32","riscv64","avr"];
    
    // Run check
    for i in 0..architecture_vec.len() {
			let check = match Command::new("ext/dasm/das").args(vec![architecture_vec[i], input_vec[i]])
                                                    .output() {
				Ok(out) => out,
				Err(e) => {
					debug::print_debug(format!("dasm::on_test: Failed to run command: {}: {}", i, e));
					return TestStatus::Failed;
				}
      };
			// Read file
			let file_str = match fs::read_to_string(artifact_vec[i]) {
				Ok(out) => out,
				Err(err) => {
					debug::print_debug(format!("dasm::on_test: Failed to open the test file. {}", err));
					return TestStatus::Failed;
				}
			};

			// Compare
      let check_str = String::from_utf8_lossy(&check.stdout);
			if check_str != file_str {
        debug::print_debug("dasm::on_test: output does not match artifact");
        debug::print_debug(&format!("\ncheck: {}\nfile: {}\n", check_str, file_str));
        return TestStatus::Failed; 
      }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn dasm (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Dasm {
    event: Event {
      name:   name,
      desc:   "Salvum built-in disassembler.".to_string(),
      usage:  "This disassembler takes the following arguments:\
              \n\t1) x86|x64|ppc32|ppc64|arm32|arm64|sparc32|sparc64|mips32|mips64|riscv32|riscv64|avr\
              \n\t2) Instruction\
              \n\nExample: dasm riscv64 57850713\n".to_string(),
      parent: parent,
      author: "(c) 2021 QVLX LLC. All rights reserved.".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }
  })
}

/*********************************** clc ***********************************/
pub struct Clc { event: Event }
impl Eventable for Clc {
  fn on_run(&self, mut args: Vec<String>) -> String {
    let mut arguments = Vec::new();
    let mut instruction = String::new();
    if args.len() < 1 { return self.event.usage.clone(); }
    let mut temp  = String::from("--");
    if args[0].eq("hex") || args[0].eq("oct") || args[0].eq("bin") || args[0].eq("all") {
      temp.push_str(&args[0]);
      arguments.push(&temp[..]);
      args.remove(0);
    }
    else if args[0].eq("dec") {
      args.remove(0);
    }
    for arg in &args[0..(args.len() - 1)] { instruction.push_str(arg); instruction.push_str(" "); }
    instruction.push_str(&args[args.len() - 1]);
    arguments.push(&instruction[..]);
    let output = match Command::new("ext/clc/clc").args(arguments).output() {
      Ok(out) => out,
      Err(e) => { return format!("clc command did not run successfully: {}\n", e); }
    };
    // Print output
    let out = String::from_utf8_lossy(&output.stdout);
    print::print_custom(&out,"brightgreen");
    log::log("asm", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact = "tst/clc/atf/atf.txt";
    let arg = "5 + 0xa - 0o17 * 0b11";
    let check = match Command::new("ext/clc/clc").arg(arg).output() {
      Ok(out) => out,
      Err(_) => { return TestStatus::Failed; }
    };
		// Read file
		let file_str = match fs::read_to_string(artifact) {
			Ok(out) => out,
			Err(err) => {
				debug::print_debug(format!("clc::on_test: Failed to open artifact: {}", err));
				return TestStatus::Failed;
			}
		};
    // Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn clc (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Clc {
    event: Event {
      name:   name,
      desc:   "Command line calculator and radix converter.".to_string(),
      usage:  "clc [dec|hex|oct|bin|all] <value>\nclc <arithmetic_expression>\n\nExamples:\n\tclc dec 0b1010101\n\tclc 2 + 2 * 4\n\nValue can be in the following formats:\n\t123\n\t0xab\n\t0b0111\n\t0o12\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }
  })
}

/*********************************** TypeInfo ***********************************/
pub struct Typeinfo { event: Event }
impl Eventable for Typeinfo {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); }
    let output = match Command::new("ext/typeinfo/b0x").arg(args[0].clone()).output() {
      Ok(out) => out,
      Err(e) => { return format!("b0x command failed to run: {}\n", e); }
    };
    // Print output
    let out = String::from_utf8_lossy(&output.stdout);
    print::print_custom(&out,"brightgreen");
    log::log("box", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact = "tst/typeinfo/atf/atf.txt";
    let check = match Command::new("ext/typeinfo/b0x").arg("10").output() {
      Ok(out) => out,
      Err(_) => { return TestStatus::Failed; }
    };
		// Read file
		let file_str = match fs::read_to_string(artifact) {
			Ok(out) => out,
			Err(err) => {
				debug::print_debug(format!("typeinfo::on_test: Failed to open artifact: {}", err));
				return TestStatus::Failed;
			}
		};
    // Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn typeinfo (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Typeinfo {
    event: Event {
      name:   name,
      desc:   "Display information about a datastructure of your choosing.".to_string(),
      usage:  "typeinfo <value>\n\nExamples:\n\ttypeinfo 10111010\n\ttypeinfo \"hello world\"\n\ttypeinfo [21,45,123]\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }
  })
}

/*********************************** Micro ***********************************/
pub struct Micro { event: Event }
impl Eventable for Micro {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() > 1 || args.len() < 1 { return self.event.usage.clone(); }
    // Run command
    let output = match run_console_command(Command::new("ext/micro/micro").arg(args[0].trim())) {
      Ok(out) => out, 
      Err(e) => { return format!("micro command failed to run successfully: {}\n", e) }  
    };   
    log::log("micro", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/micro/atf/atf.txt";
    
    // Run check
    let check_out = match util::pipes::Pipe::new(&"ext/micro/micro -version".to_string())
                                        .then("grep hash")
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel, 
      Err(err) => { 
        debug::print_debug(format!("micro::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }    
    };   

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel, 
      Err(err) => { 
        debug::print_debug(format!("micro::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }    
    };   
    
    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out, 
      Err(err) => { 
        debug::print_debug(format!("micro::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }    
    };   

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }    
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn micro(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Micro {
    event: Event {
      name:   name,
      desc:   "Nice and lean file editor.".to_string(),
      usage:  "micro <file_path>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }    
  })
}

/*********************************** Svim ***********************************/
pub struct Svim { event: Event }
impl Eventable for Svim {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); } 
    alerts::print_advisory();
    print::println("");
    if alerts::confirm_task("Do you want to continue?") == constants::UNCONFIRMED { 
      print::print_custom("Cancelled.\n", "orange");
      return String::from("");
    } 
    let mut arguments = Vec::new();
    arguments.push("-u");
    arguments.push("ext/svim/.vimrc"); 
    arguments.push("-x");
    arguments.push(args[0].trim());

    // Run command
    let output = match run_console_command(Command::new("vim").args(arguments)) {
      Ok(out) => out, 
      Err(e) => { return format!("Svim command failed to run: {}\n", e); }  
    };   
    log::log("vim", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/vim/test.txt";
    let artifact_path = "tst/vim/atf/atf.txt";
    util::misc::cleanup(staging_path);

    let vim_args = vec![staging_path, "-c", ":wq"];
    
    // Run command
    match Command::new("vim").args(vim_args).output() {
      Ok(_) => (),
      Err(err) => { 
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }    
    };   
    
    // Run check
    let check = match Command::new("ls").arg("tst/vim/test.txt").output() {
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
        debug::print_debug(format!("svim::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }    
    };   

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }    
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn svim(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Svim {
    event: Event {
      name:   name,
      desc:   "Classic vim editor but set for Blowfish2 encryption.".to_string(),
      usage:  "svim <file_path>\n\n**Keep in mind that the file created will be encrypted.\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }    
  })
}

/*********************************** vim ***********************************/
pub struct Vim { event: Event }
impl Eventable for Vim {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); } 

    // Run command
    let output = match run_console_command(Command::new("vim").arg(args[0].clone())) {
      Ok(out) => out, 
      Err(e) => { return format!("vim command failed: {}\n", e); }  
    };   
    log::log("vim", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/vim/test.txt";
    let artifact_path = "tst/vim/atf/atf.txt";
    util::misc::cleanup(staging_path);
    let vim_args = vec![staging_path, "-c", ":wq"];
    
    // Run command
    match Command::new("vim").args(vim_args).output() {
      Ok(_) => (),
      Err(err) => { 
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }    
    };   
    
    // Run check
    let check = match Command::new("ls").arg("tst/vim/test.txt").output() {
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
        debug::print_debug(format!("vim::on_test: Failed to open the test file. {}", err));
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
pub fn vim(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Vim {
    event: Event {
      name:   name,
      desc:   "Classic vim editor.".to_string(),
      usage:  "vim <file_path>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }    
  })
}

/*********************************** Emacs ***********************************/
pub struct Emacs { event: Event }
impl Eventable for Emacs {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() > 1 || args.len() < 1 { return self.event.usage.clone(); }
    // Run command
    let output = match run_console_command(Command::new("emacs").arg(args[0].trim())) {
      Ok(out) => out,
      Err(e) => { return format!("emacs command failedn: {}\n", e); }
    };
    log::log("emacs", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/emacs/atf/atf.txt";
    // Run check
    let check = match Command::new("emacs").arg("--help").output() {
      Ok(out) => out, 
      Err(_) => { return TestStatus::Failed; }  
    };   
    
    // Read file
    let atf_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out, 
      Err(err) => { 
        debug::print_debug(format!("emacs::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }    
    };   

    // Compare
    if String::from_utf8_lossy(&check.stdout) != atf_str {
      debug::print_debug(format!("-- Output --\n{}\n-- Artifact --\n{}", String::from_utf8_lossy(&check.stdout), atf_str));
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn emacs(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Emacs {
    event: Event {
      name:   name,
      desc:   "Classic GNU Emacs editor.".to_string(),
      usage:  "emacs <file_path>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }    
  })
}

/*********************************** uwc ***********************************/
pub struct Uwc{ event: Event }
impl Eventable for Uwc {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() > 1 || args.len() < 1 { return self.event.usage.clone(); }
    let second_cmd = "python3 ext/util/colorize/colorize.py -c purple,yellow,red,green,cyan,blue";
    let mut cmd = String::from("ext/uwc/uwc -a ");
    cmd.push_str(args[0].trim());

    // Run check
    let out = match util::pipes::Pipe::new(&cmd).then(&second_cmd).finally() {
      Ok(sel) => sel, 
      Err(err) => { 
        return format!("Failed to run the uwc command: {}\n", err);
      }    
    };   

    let output = match out.wait_with_output() {
      Ok(sel) => sel, 
      Err(err) => { 
        return format!("Failed to wait on the uwc command: {}\n", err);
      }    
    };
    log::log("uwc", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/uwc/atf/atf.txt";
    let input_path = "tst/uwc/inp/test.txt";

    // Run command
    let check = match Command::new("ext/uwc/uwc").arg(input_path).output() {
      Ok(out) => out, 
      Err(_) => { return TestStatus::Failed; }  
    };   

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("uwc::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn uwc(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Uwc {
    event: Event {
      name:   name,
      desc:   "Uwc unicode-aware word counting utility.".to_string(),
      usage:  "This word counter takes the path to a text file as an argument.\n".to_string(),
      author: "Skyler Hawthorne\n\nskyler@dead10ck.com".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** wc ***********************************/
pub struct Wc{ event: Event }
impl Eventable for Wc {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 { return self.event.usage.clone(); }
    let second_cmd = "python3 ext/util/colorize/colorize.py -c purple,red,green,cyan";
    let mut cmd = String::from("ext/util/coreutils wc -clw ");
    cmd.push_str(&args[0]);

    let out = match util::pipes::Pipe::new(&cmd).then(&second_cmd).finally() {
      Ok(sel) => sel, 
      Err(err) => { 
        return format!("Failed to run the wc command: {}\n", err);
      }    
    };   

    let output = match out.wait_with_output() {
      Ok(sel) => sel, 
      Err(err) => { 
        return format!("Failed to wait on the wc command: {}\n", err);
      }    
    };
    let result = String::from_utf8_lossy(&output.stdout).to_string().replace("\n","").replace("  "," ").replace("   "," ").replace("    "," ");
    let mut split_ws = result.split(" ").collect::<Vec<&str>>();
    split_ws.retain(|&x| x.len() > 0);
    if split_ws.len() == 6 {
      print::print_custom(&format!("lines: {}\n",split_ws[2]),"red");
      print::print_custom(&format!("words: {}\n",split_ws[3]),"green");
      print::print_custom(&format!("bytes: {}\n",split_ws[4]),"cyan");
      print::print_custom(&format!("filename: {}\n",split_ws[5]),"magenta");
    }
    if split_ws.len() == 5 {
      print::print_custom(&format!("lines: {}\n",split_ws[1]),"red");
      print::print_custom(&format!("words: {}\n",split_ws[2]),"green");
      print::print_custom(&format!("bytes: {}\n",split_ws[3]),"cyan");
      print::print_custom(&format!("filename: {}\n",split_ws[4]),"magenta");
    }
    if split_ws.len() == 4 {
      print::print_custom(&format!("lines: {}\n",split_ws[0]),"red");
      print::print_custom(&format!("words: {}\n",split_ws[1]),"green");
      print::print_custom(&format!("bytes: {}\n",split_ws[2]),"cyan");
      print::print_custom(&format!("filename: {}\n",split_ws[3]),"magenta");
    }
    log::log("wc", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/wc/atf/atf.txt";
    let input_path = "tst/wc/inp/test.txt";

    // Run command
    let check = match Command::new("ext/util/coreutils").args(vec!["wc","-clw",input_path]).output() {
      Ok(out) => out, 
      Err(_) => { return TestStatus::Failed; }  
    };   

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("wc::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn wc(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Wc {
    event: Event {
      name:   name,
      desc:   "The classic word count utility written in pure Rust.".to_string(),
      usage:  "Word count takes a path to a binary or text file as an argument.\n".to_string(),
      author: "Jordi Boggiano\n\nCopyright (c) Jordi Boggiano".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}

/*********************************** rem ***********************************/
pub struct Rem { event: Event }
impl Eventable for Rem {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 { return self.event.usage.clone(); }
    let mut is_new = false;
    let mut arguments = Vec::new();
    if args[0].eq("new") || args[0].eq("add") {
      arguments.push("add");
      for arg in &args[1..args.len()] { arguments.push(arg); }
      if args.len() > 1 { is_new = true; }
    }
    else if args[0].eq("show") || args[0].eq("list") || args[0].eq("all") {
      arguments.push("cat");
      arguments.push("-n");
    }
    else if args[0].eq("del") || args[0].eq("delete") || args[0].eq("remove") {
      arguments.push("del");
      arguments.push("-f");
      arguments.push(args[1].trim());
    } 
    else if args[0].eq("edit") {
      arguments.push("edit");
      arguments.push(args[1].trim());
    } 
    else { return self.event.usage.clone(); }
  
    // Run command
    let output = match run_console_command(Command::new("ext/rem/rem-cli").args(arguments)) {
      Ok(out) => out, 
      Err(e) => { return format!("rem command failed: {}\n", e); }
    };

    if is_new { print::print_custom("Note added to collection.\n","neongreen"); }
    log::log("rem", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/rem/atf/atf.txt";
    // Run check
    let check = match Command::new("ext/rem/rem-cli").arg("-h").output() {
      Ok(out) => out, 
      Err(_) => { return TestStatus::Failed; }  
    };   
    
    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out, 
      Err(err) => { 
        debug::print_debug(format!("rem::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }    
    };   

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }    
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn rem(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Rem {
    event: Event {
      name:   name,
      desc:   "Rem note taking utility.".to_string(),
      usage:  "This note taking applet takes the following arguments:\n\
               \t1) add <note_text_here>\n\
               \t2) show|list|all <note_id>\n\
               \t3) del|delete|remove <note_id>\n\
               \t4) edit <note_id>\n\n\
               Example: rem add here is some test\n".to_string(),
      parent: parent,
      author: "Max Wagner\tmax@wagnerm.dev".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }    
  })
}

/*********************************** lz4 ***********************************/
pub struct Lz4 { event: Event }
impl Eventable for Lz4 { 
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); }
    // Run command
    let output = match run_console_command(Command::new("ext/lz4/lz").arg(args[0].clone())) {
      Ok(out) => out,
      Err(e) => { return format!("lz command was not successful: {}\n", e); }  
    };
    log::log("lz", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/lz4/atf/atf.txt";

    // Run command
    let check = match Command::new("ext/lz4/lz").arg("-H").output() {
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
        debug::print_debug(format!("lz::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }    
    };   

    // Compare
    if String::from_utf8_lossy(&check.stderr) != file_str {
      return TestStatus::Failed;
    }    
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn lz4(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Lz4 {
    event: Event {
      name:   name,
      desc:   "Lz4 compression tool.".to_string(),
      usage:  "lz4 <file_path>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }    
  })
}

/*********************************** 7zip ***********************************/
pub struct P7Zip { event: Event }
impl Eventable for P7Zip {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); }
    let file_name = args[0].clone();
    let mut arguments = Vec::new();
    let mut out_file_name = String::from(file_name.trim());
    let baseish_name = str::replace(&file_name, ".7z", "");
    
    // Decompress file
    if file_name.contains(".7z") {
      arguments.push("ext/7zip/p7zip");
      arguments.push("-d");
      arguments.push(file_name.trim());
      arguments.push(&baseish_name);
    }
    // Compress file
    else {
      arguments.push("ext/7zip/p7zip");
      arguments.push(file_name.trim());
      out_file_name.push_str(".7z");
      arguments.push(&out_file_name[..]);
    }
    // Run command
    let output = match run_command(Command::new(args[0].clone()).args(&args[1..])) {
      Ok(out) => out,
      Err(err) => {
        return format!("p7zip command failed: {}\n", err);
      }
    };

    // Print output
    log::log("7zip", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/7zip/test.txt";
    let cleaning_path = "tst/7zip/test.txt.7z";
    let input_path = "tst/7zip/inp/test.txt";
    let artifact_path = "tst/7zip/atf/atf.txt";
    util::misc::cleanup(cleaning_path); 
    util::misc::cleanup(staging_path); 
    let copy_args = vec![input_path, staging_path];
    
    // Run command
    match Command::new("cp").args(copy_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    let zip_args = vec![staging_path, cleaning_path];

    // Run command
    match Command::new("ext/7zip/p7zip").args(zip_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Run check
    let check_out = match util::pipes::Pipe::new(&"file tst/7zip/test.txt.7z".to_string())
                                        .then("grep 7-zip")
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("7zip::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("7zip::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("7zip::on_test: Failed to open the test file. {}", err));
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
pub fn p7zip(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(P7Zip {
    event: Event {
      name:   name,
      desc:   "7-zip commpression utility".to_string(),
      usage:  "7zip <file_to_[de]compress>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** lzma ***********************************/
pub struct Lzma { event: Event }
impl Eventable for Lzma {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); }
    let file_name = args[0].clone();
    let mut arguments = Vec::new();
    arguments.push("-v"); // Verbose option
    if file_name.contains(".lzma") { arguments.push("-d"); } // Decompress file
    arguments.push(file_name.trim()); // File to compress or decompress

    // Run command
    let output = match run_command(Command::new("ext/lzma/lzma").args(args)) {
      Ok(out) => out,
      Err(err) => {
        return format!("lzma command failed: {}\n", err);
      }
    };

    // Print output
    log::log("Lzma", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/lzma/test.txt";
    let cleaning_path = "tst/lzma/test.txt.lzma";
    let input_path = "tst/lzma/inp/test.txt";
    let artifact_path = "tst/lzma/atf/atf.txt";
    util::misc::cleanup(cleaning_path); 
    util::misc::cleanup(staging_path); 
    let copy_args = vec![input_path, staging_path];
    
    // Run command
    match Command::new("cp").args(copy_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Run command
    match Command::new("ext/lzma/lzma").arg(staging_path).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Run check: file tst/lzma/test.txt* | wc
    let check_out = match util::pipes::Pipe::new(&"file tst/lzma/test.txt.lzma".to_string())
                                        .then("grep LZMA")
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("lzma::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("lzma::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("lzma::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn lzma(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Lzma {
    event: Event {
      name:   name,
      desc:   "LZMA compression utility".to_string(),
      usage:  "lzma <file_to_[de]compress>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** tar ***********************************/
pub struct Tar { event: Event }
impl Eventable for Tar {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); }
    let file_name = args[0].clone();
    let mut arguments = Vec::new();
    let mut out_file_name = String::from(file_name.trim());
    let baseish_name = str::replace(&file_name, ".tar", "");
    
    // Decompress file
    if file_name.contains(".tar") {
      arguments.push("ext/tar/tar");
      arguments.push("-xvf");
      arguments.push(file_name.trim());
      arguments.push(&baseish_name);
    }
    // Compress file
    else {
      arguments.push("ext/tar/tar");
      arguments.push("-cvf");
      out_file_name.push_str(".tar");
      arguments.push(&out_file_name[..]);
      arguments.push(file_name.trim());
    }

    // Run command
    let output = match run_command(Command::new(args[0].clone()).args(&args[1..])) {
      Ok(out) => out,
      Err(err) => {
        return format!("tar command failed: {}\n", err);
      }
    };

    // Print output
    log::log("tar", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/tar/test.txt";
    let cleaning_path = "tst/tar/test.txt.tar";
    let input_path = "tst/tar/inp/test.txt";
    let artifact_path = "tst/tar/atf/atf.txt";
    util::misc::cleanup(cleaning_path); 
    util::misc::cleanup(staging_path); 
    let copy_args = vec![input_path, staging_path];
    
    // Run command
    match Command::new("cp").args(copy_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    let tar_args = vec![staging_path, cleaning_path];

    // Run command
    match Command::new("ext/tar/tar").args(tar_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Run check
    let check_out = match util::pipes::Pipe::new(&"file tst/tar/test.txt.tar".to_string())
                                        .then("grep tar")
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("tar::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("tar::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("tar::on_test: Failed to open the test file. {}", err));
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
pub fn tar(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Tar {
    event: Event {
      name:   name,
      desc:   "Tar archiving utility".to_string(),
      usage:  "tar <directory_to_[un]pack>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** rar ***********************************/
pub struct Rar { event: Event }
impl Eventable for Rar {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); }
    let file_name = args[0].clone();
    let mut arguments = Vec::new(); 
    let mut out_file_name = String::from(file_name.trim());
    
    // Decompress file
    if file_name.contains(".rar") {
      arguments.push("ext/rar/unrar");
      arguments.push("x");
    }
    // Compress file
    else {
      arguments.push("ext/rar/rar");
      arguments.push("a");
      out_file_name.push_str(".rar");
      arguments.push(&out_file_name[..]);
    }

    // File to compress or decompress
    arguments.push(file_name.trim());

    // Run command
    let output = match run_command(Command::new(args[0].clone()).args(&args[1..])) {
      Ok(out) => out,
      Err(err) => {
        return format!("rar command failed: {}\n", err);
      }
    };

    // Print output
    let output_str = String::from_utf8_lossy(&output.stdout);
    let output_str_nl: Vec<&str> = output_str.split('\n').collect();             
      for i in 0..output_str_nl.len(){
        if output_str_nl[i].contains("Trial") {
          continue;
        }
        if output_str_nl[i].contains("Evaluation"){
          continue;
        }
      print::println(&format!("{}", output_str_nl[i]));
      }
    log::log("Rar", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/rar/test.txt";
    let cleaning_path = "tst/rar/test.txt.rar";
    let input_path = "tst/rar/inp/test.txt";
    let artifact_path = "tst/rar/atf/atf.txt";
    let artifact2_path = "tst/rar/atf/atf2.txt";
    util::misc::cleanup(cleaning_path); 
    util::misc::cleanup(staging_path); 
    let copy_args = vec![input_path, staging_path];
    
    // Run command
    match Command::new("cp").args(copy_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    let zip_args = vec!["a", cleaning_path, staging_path];

    // Run command
    match Command::new("ext/rar/rar").args(zip_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Run check
    let check_out = match util::pipes::Pipe::new(&"file tst/rar/test.txt.rar".to_string())
                                        .then("grep RAR")
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("rar::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("rar::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("rar::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str {
      return TestStatus::Failed;
    }
    
    util::misc::cleanup(staging_path); 

    // Run command
    match Command::new("ext/rar/unrar").args(vec!["x", cleaning_path]).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Run check
    let check_out = match util::pipes::Pipe::new(&"file tst/rar/test.txt".to_string())
                                        .then("grep ASCII")
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("rar::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("rar::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact2_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("rar::on_test: Failed to open the test file. {}", err));
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
pub fn rar(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Rar {
    event: Event {
      name:   name,
      desc:   "Rar archiving utility.".to_string(),
      usage:  "rar <directory_to_[un]pack>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** zip ***********************************/
pub struct Zip { event: Event }
impl Eventable for Zip {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 2 || args.len() > 2 { return self.event.usage.clone(); }
    let mut arguments = Vec::new();
    let file_name = args[0].clone();
    let mut out_file_name = String::from(args[1].clone());

    // Decompress file
    if file_name.contains(".zip") {
      arguments.push("ext/zip/unzip");
    }
    // Compress file
    else {
      arguments.push("ext/zip/zip");
      out_file_name.push_str(".zip");
      arguments.push(&out_file_name[..]);
    }

    // File to compress or decompress
    arguments.push(file_name.trim());

    // Run command
    let output = match run_command(Command::new(args[0].clone()).args(&args[1..])) {
      Ok(out) => out,
      Err(err) => {
        return format!("zip command failed: {}\n", err);
      }
    };

    log::log("Zip", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/zip/test.txt";
    let cleaning_path = "tst/zip/test.txt.zip";
    let input_path = "tst/zip/inp/test.txt";
    let artifact_path = "tst/zip/atf/atf.txt";
    let artifact2_path = "tst/zip/atf/atf2.txt";
    util::misc::cleanup(cleaning_path); 
    util::misc::cleanup(staging_path); 
    let copy_args = vec![input_path, staging_path];
    
    // Run command
    match Command::new("cp").args(copy_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    let zip_args = vec![cleaning_path, staging_path];

    // Run command
    match Command::new("ext/zip/zip").args(zip_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Run check
    let check_out = match util::pipes::Pipe::new(&"file tst/zip/test.txt.zip".to_string())
                                        .then("grep Zip")
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("zip::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("zip::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("zip::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str {
      return TestStatus::Failed;
    }
    
    util::misc::cleanup(staging_path); 

    // Run command
    match Command::new("ext/zip/unzip").arg(cleaning_path).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Run check
    let check_out = match util::pipes::Pipe::new(&"file tst/zip/test.txt".to_string())
                                        .then("grep ASCII")
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("zip::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("zip::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact2_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("zip::on_test: Failed to open the test file. {}", err));
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
pub fn zip(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Zip {
    event: Event {
      name:   name,
      desc:   "Zip archiving tool.".to_string(),
      usage:  "zip <directory_to_[un]pack>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** gzip ***********************************/
pub struct Gzip { event: Event }
impl Eventable for Gzip {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); }
    let mut arguments = Vec::new();
    let file_name = args[0].clone();
    arguments.push("-v");  // Verbose option
    if file_name.contains(".gz") { arguments.push("-d"); } // Decompress file
    arguments.push(file_name.trim()); // File to compress or decompress

    // Run command
    let output = match run_command(Command::new("ext/gzip/gzip").args(args)) {
      Ok(out) => out,
      Err(err) => {
        return format!("Error running gzip command: {}\n", err);
      }
    };
    log::log("Gzip", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/gzip/test.txt";
    let cleaning_path = "tst/gzip/test.txt.gz";
    let input_path = "tst/gzip/inp/test.txt";
    let artifact_path = "tst/gzip/atf/atf.txt";
    util::misc::cleanup(cleaning_path); 
    util::misc::cleanup(staging_path); 
    let copy_args = vec![input_path, staging_path];
    
    // Run command
    match Command::new("cp").args(copy_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Run command
    match Command::new("ext/gzip/gzip").arg(staging_path).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Run check: file tst/lzma/test.txt* | wc
    let check_out = match util::pipes::Pipe::new(&"file tst/gzip/test.txt.gz".to_string())
                                        .then("grep gzip")
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("gzip::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("gzip::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("gzip::on_test: Failed to open the test file. {}", err));
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
pub fn gzip(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Gzip {
    event: Event {
      name:   name,
      desc:   "Gzip compression utility.".to_string(),
      usage:  "gzip <file_to_[de]compress>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** lzop ***********************************/
pub struct Lzop { event: Event }
impl Eventable for Lzop {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); }
    let mut arguments = Vec::new();
    let file_name = args[0].clone();
    arguments.push("-v"); // Verbose option 
    if file_name.contains(".lzo") { arguments.push("-d"); } // Decompress file
    arguments.push(file_name.trim()); // File to compress or decompress

    // Run command
    let output = match run_command(Command::new("ext/lzop/lzop").args(args)) {
      Ok(out) => out,
      Err(err) => {
        return format!("Error running lzop command: {}\n", err);
      }
    };
    log::log("lzop", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/lzop/test.txt";
    let cleaning_path = "tst/lzop/test.txt.lzo";
    let input_path = "tst/lzop/inp/test.txt";
    let artifact_path = "tst/lzop/atf/atf.txt";
    util::misc::cleanup(cleaning_path); 
    util::misc::cleanup(staging_path); 
    let copy_args = vec![input_path, staging_path];
    
    // Run command
    match Command::new("cp").args(copy_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Run command
    match Command::new("ext/lzop/lzop").arg(staging_path).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Run check
    let check_out = match util::pipes::Pipe::new(&"file tst/lzop/test.txt.lzo".to_string())
                                        .then("grep lzop")
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("lzop::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("lzop::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("lzop::on_test: Failed to open the test file. {}", err));
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
pub fn lzop(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Lzop {
    event: Event {
      name:   name,
      desc:   "Lzop compression utility.".to_string(),
      usage:  "lzop <file_to_[de]compress>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** lzip ***********************************/
pub struct Lzip { event: Event }
impl Eventable for Lzip {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); }
    let mut arguments = Vec::new();
    let file_name = args[0].clone();
    arguments.push("-v"); // Verbose option
    if file_name.contains(".lz") { arguments.push("-d"); } // Decompress file
    arguments.push(file_name.trim()); // File to compress or decompress
    self.on_run(vec![]);

    // Run command
    let output = match run_command(Command::new("ext/lzip/lzip").args(args)) {
      Ok(out) => out,
      Err(err) => {
        return format!("Error running lzip command: {}\n", err);
      }
    };
    log::log("lzip", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/lzip/test.txt";
    let cleaning_path = "tst/lzip/test.txt.lz";
    let input_path = "tst/lzip/inp/test.txt";
    let artifact_path = "tst/lzip/atf/atf.txt";
    util::misc::cleanup(cleaning_path); 
    util::misc::cleanup(staging_path); 
    let copy_args = vec![input_path, staging_path];
    
    // Run command
    match Command::new("cp").args(copy_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Run command
    match Command::new("ext/lzip/lzip").arg(staging_path).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Run check
    let check_out = match util::pipes::Pipe::new(&"file tst/lzip/test.txt.lz".to_string())
                                        .then("grep lzip")
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("lzip::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("lzip::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("lzip::on_test: Failed to open the test file. {}", err));
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
pub fn lzip(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Lzip {
    event: Event {
      name:   name,
      desc:   "Lzip compression utility.".to_string(),
      usage:  "lzip <file_to_[de]compress>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** bzip ***********************************/
pub struct Bzip { event: Event }
impl Eventable for Bzip {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); }    let mut arguments = Vec::new();
    let file_name = args[0].clone();    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); }
    arguments.push("-v"); // Verbose
    if file_name.contains(".bz2") { arguments.push("-d"); } // Decompress file
    else { arguments.push("-z"); } // Compress file

    arguments.push(file_name.trim()); // File to compress or decompress
    // Run command
    let output = match run_command(Command::new("ext/bzip2/bzip2").args(args)) {
      Ok(out) => out,
      Err(err) => {
        return format!("bzip2 command failed to run: {}\n", err);
      }
    };

    // Print output
    log::log("Bzip2", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/bzip2/test.txt";
    let cleaning_path = "tst/bzip2/test.txt.bz2";
    let input_path = "tst/bzip2/inp/test.txt";
    let artifact_path = "tst/bzip2/atf/atf.txt";
    util::misc::cleanup(cleaning_path); 
    util::misc::cleanup(staging_path); 
    let copy_args = vec![input_path, staging_path];
    
    // Run command
    match Command::new("cp").args(copy_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Run command
    match Command::new("ext/bzip2/bzip2").arg(staging_path).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
   
    // Run check 
    let check_out = match util::pipes::Pipe::new(&"file tst/bzip2/test.txt.bz2".to_string())
                                        .then("grep bzip2")
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("bzip::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("bzip::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("bzip::on_test: Failed to open the test file. {}", err));
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
pub fn bzip(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Bzip {
    event: Event {
      name:   name,
      desc:   "bzip2 compression utility.".to_string(),
      usage:  "bzip2 <file_to_[de]compress>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** zstd ***********************************/
pub struct Zstd { event: Event }
impl Eventable for Zstd {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); }
    let mut arguments = Vec::new();
    let file_name = args[0].clone();
    if file_name.contains(".zst") { arguments.push("-d"); } // Decompress file
    arguments.push(file_name.trim()); // File to compress or decompress

    // Run command
    let output = match run_command(Command::new("ext/zstd/zstd").args(args)) {
      Ok(out) => out,
      Err(err) => {
        return format!("zstd command failed: {}\n", err);
      }
    };
    log::log("zstd", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/zstd/test.txt";
    let cleaning_path = "tst/zstd/test.txt.zst";
    let input_path = "tst/zstd/inp/test.txt";
    let artifact_path = "tst/zstd/atf/atf.txt";
    util::misc::cleanup(cleaning_path); 
    util::misc::cleanup(staging_path); 
    let copy_args = vec![input_path, staging_path];
    
    // Run command
    match Command::new("cp").args(copy_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Run command
    match Command::new("ext/zstd/zstd").arg(staging_path).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Run check: file tst/lzma/test.txt* | wc
    let check_out = match util::pipes::Pipe::new(&"file tst/zstd/test.txt.zst".to_string())
                                        .then("grep Zstandard")
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("zstd::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("zstd::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("zstd::on_test: Failed to open the test file. {}", err));
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
pub fn zstd(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Zstd {
    event: Event {
      name:   name,
      desc:   "Zstd compression utility.".to_string(),
      usage:  "zstd <file_to_[de]compress>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** xz ***********************************/
pub struct Xz { event: Event }
impl Eventable for Xz {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); }
    let mut arguments = Vec::new();
    let file_name = args[0].clone();

    arguments.push("-v"); // Verbose option
    if file_name.contains(".xz") { arguments.push("-d"); } // Decompress file
    else { arguments.push("-z"); } // Compress file
    arguments.push(file_name.trim()); // File to compress or decompress
    // Run command
    let output = match run_command(Command::new("ext/xz/xz").args(args)) {
      Ok(out) => out,
      Err(err) => {
        return format!("xz command was no successful: {}\n", err);
      }
    };
    log::log("Xz", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/xz/test.txt";
    let cleaning_path = "tst/xz/test.txt.xz";
    let input_path = "tst/xz/inp/test.txt";
    let artifact_path = "tst/xz/atf/atf.txt";
    util::misc::cleanup(cleaning_path); 
    util::misc::cleanup(staging_path); 
    let copy_args = vec![input_path, staging_path];
    
    // Run command
    match Command::new("cp").args(copy_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Run command
    match Command::new("ext/xz/xz").arg(staging_path).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    
    // Run check: file tst/lzma/test.txt* | wc
    let check_out = match util::pipes::Pipe::new(&"file tst/xz/test.txt.xz".to_string())
                                        .then("grep XZ")
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("xz::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("xz::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("xz::on_test: Failed to open the test file. {}", err));
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
pub fn xz(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Xz {
    event: Event {
      name:   name,
      desc:   "XZ compression utility.".to_string(),
      usage:  "xz <file_to_[de]compress>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** cnv ***********************************/
pub struct Cnv { event: Event }
impl Eventable for Cnv {
  fn on_run(&self, args: Vec<String>) -> String {
    let mut arguments = Vec::new();
    let mut instruction = String::new();
    if args.len() < 1 { return self.event.usage.clone(); }
    for arg in &args[0..(args.len() - 1)] { instruction.push_str(arg); instruction.push_str(" "); }
    instruction.push_str(&args[args.len() - 1]);
    //if !instruction.contains(" to ") { return self.event.usage.clone() };
    arguments.push(&instruction[..]);
    let output = simple_match!(Command::new("ext/cnv/cpc").args(arguments).output());
    // Print output
    let out = String::from_utf8_lossy(&output.stdout);
    print::print_custom(&out,"brightgreen");
    log::log("cnv", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact = "tst/cnv/atf/atf.txt";
    let check = match Command::new("ext/cnv/cpc").arg("--version").output() {
      Ok(out) => out,
      Err(_) => { return TestStatus::Failed; }
    };
		// Read file
		let file_str = match fs::read_to_string(artifact) {
			Ok(out) => out,
			Err(err) => {
				debug::print_debug(format!("cnv::on_test: Failed to open artifact: {}", err));
				return TestStatus::Failed;
			}
		};
    // Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn cnv (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Cnv {
    event: Event {
      name:   name,
      desc:   "Applet that converts units and performs calculations.".to_string(),
      usage:  "cnv <value> <unit1> to <unit2>.\n\nExamples:\n\tcnv 3 meters to kilometers\n\tcnv log(34)\n\tcnv (4 + 1)km to light years\n\tcnv round(sqrt(2)^4)! liters\n\tcnv 10% of abs(sin(pi)) horsepower to watts\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }
  })
}

/*********************************** hw ***********************************/
pub struct Hw { event: Event }
impl Eventable for Hw {
  fn on_run(&self, args: Vec<String>) -> String {
    if main_info::is_bg() || main_info::get_file_redirect() {
      print::print_custom("Hw currently does not support background or file redirection.\n\nComing soon.\n","orange");
      return String::from("");
    }
    if args.len() < 1 || args.len() > 1 { return self.event.usage.clone(); }
    if args[0].eq("serial") { // Print SCSI information
      let output = match Command::new("ext/hw/lser").output() {
        Ok(o) => o,
        Err(err) => {
          print::print_custom(&format!("Lser command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      let out = String::from_utf8_lossy(&output.stdout);
      print::print_custom(&out,"magenta");
    }
    if args[0].eq("all"){ // Print a hardware information summary
      match run_console_command(Command::new("ext/hw/neofetch").arg("--off")) {
				Ok(_) => (),
					Err(err) => {
						print::println(&format!("Neofetch command failed. Error : {}",err));
            return String::from("");
					}
			};
      match run_console_command(Command::new("inxi").args(vec!["-Fxz","-v8","-c","11"])) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("inxi ommand failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      print::print_custom_uncapped("{}","gold");
      match run_console_command(Command::new("ext/hw/lshw").arg("-short")) {
				Ok(_) => (),
					Err(err) => {
						print::print_custom(&format!("lshw command failed. Error : {}",err),"orange");
            return String::from("");
					}
			};
      print::print_custom("","reset");
    }
    if args[0].eq("cpu") { // Print CPU information
      print::print_custom_uncapped("","orange");
      match run_console_command(&mut Command::new("ext/hw/lscpu")) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("lscpu command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      print::print_custom_uncapped("","grey");
      match run_console_command(&mut Command::new("cpu-info")) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("cpu-info command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      print::print_custom_uncapped("","gold");
      match run_console_command(&mut Command::new("ext/hw/procinfo")) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("procinfo command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      let output = match Command::new("dmidecode").args(vec!["-t","processor"]).output() {
        Ok(out) => out,
        Err(err) => {
          print::print_custom(&format!("dmidecode command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      let out = String::from_utf8_lossy(&output.stdout);
      print::print_custom(&out,"brightgreen");
      return String::from("");
    }
    if args[0].eq("cache") { // Print Cache information
      print::print_custom_uncapped("","gold");
      match run_console_command(&mut Command::new("cache-info")) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("cache-info command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      let output = match Command::new("dmidecode").args(vec!["-t","cache"]).output() {
        Ok(out) => out,
        Err(err) => {
          print::print_custom(&format!("Command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      let out = String::from_utf8_lossy(&output.stdout);
      print::print_custom(&out,"brightgreen");
    }
    if args[0].eq("mem") || args[0].eq("memory") { // Print Memory information
      match run_console_command(&mut Command::new("ext/hw/mprober").args(vec!["memory","-l"])) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("Command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      print::print_custom_uncapped("","grey");
      match run_console_command(&mut Command::new("ext/hw/free").args(vec!["-htl"])) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("Command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      print::print_custom_uncapped("","gold");
      match run_console_command(&mut Command::new("ext/hw/lshw").args(vec!["-short", "-C", "memory"])) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("Command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      print::print_custom_uncapped("","orange");
      match run_console_command(&mut Command::new("ext/hw/lsmem").args(vec!["--summary=always"])) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("Command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      let output = match Command::new("dmidecode").args(vec!["-t","memory"]).output() {
        Ok(out) => out,
        Err(err) => {
          print::print_custom(&format!("Command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      let out = String::from_utf8_lossy(&output.stdout);
      print::print_custom(&out,"brightgreen");
    }
    if args[0].eq("disk") { // Print CPU information
      print::print_custom_uncapped("","grey");
      match run_console_command(&mut Command::new("ext/hw/lshw").args(vec!["-short", "-C" ,"memory"])) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("Command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      print::print_custom_uncapped("","gold");
      match run_console_command(&mut Command::new("ext/hw/lsblk").args(vec!["-fpz","-o","name,uuid,fstype,fsavail,fsuse%,mountpoint,zoned"])) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("Command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      print::print_custom_uncapped("","neongreen");
      match run_console_command(&mut Command::new("ext/hw/df").args(vec!["-Th"])) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("Command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      print::print_custom("","reset");
      /*
      match run_console_command(&mut Command::new("ext/hw/mprober").args(vec!["volume","-l"])) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("Command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      */
    }
    if args[0].eq("pci") { // Print PCI information
      print::print_custom_uncapped("","orange");
      match run_console_command(&mut Command::new("lspci")) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("Command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      print::print_custom("---------------------------------------------------------------\n","grey");
      print::print_custom_uncapped("","gold");
      match run_console_command(&mut Command::new("lspci").arg("-t")) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("Command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      print::print_custom("---------------------------------------------------------------\n","grey");
      print::print_custom_uncapped("","purple");
      match run_console_command(&mut Command::new("dmidecode").args(vec!["-t","slot"])) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("Command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      print::print_custom("","reset");
    }
    if args[0].eq("scsi") { // Print SCSI information
      print::print_custom_uncapped("","gold");
      let output = match Command::new("ext/hw/lsscsi").arg("-ldis").output() {
        Ok(o) => o,
        Err(err) => {
          print::print_custom(&format!("Command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      let out = String::from_utf8_lossy(&output.stdout);
      print::print_custom(&out,"brightgreen");
    }
    if args[0].eq("usb") { // Print USB information
      print::print_custom_uncapped("","gold");
      match run_console_command(&mut Command::new("lsusb")) {
        Ok(_) => (),
        Err(err) => {
          print::print_custom(&format!("Command failed. Error : {}",err),"orange");
          return String::from("");
        }
      };
      print::print_custom("--------------------------------------------------------------------\n","grey");
      let output = match Command::new("lsusb").arg("-t").output() {
        Ok(o) => o,
          Err(err) => {
            print::print_custom(&format!("Command failed. Error : {}",err),"orange");
            return String::from("");
        }
      };
      let out = String::from_utf8_lossy(&output.stdout);
      print::print_custom(&out,"brightgreen");
    }
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact1 = "tst/hw/atf/atf1.txt"; // lscpu
    let artifact2 = "tst/hw/atf/atf2.txt"; // lshw
    let artifact3 = "tst/hw/atf/atf3.txt"; // procinfo
    let artifact4 = "tst/hw/atf/atf4.txt"; // dmidecode
    let artifact5 = "tst/hw/atf/atf5.txt"; // mprober
    let artifact6 = "tst/hw/atf/atf6.txt"; // free
    let artifact7 = "tst/hw/atf/atf7.txt"; // lsmem
    let artifact8 = "tst/hw/atf/atf8.txt"; // lsblk
    let artifact9 = "tst/hw/atf/atf9.txt"; // df
    let artifact10 = "tst/hw/atf/atf10.txt"; // lspci
    let artifact11 = "tst/hw/atf/atf11.txt"; // lsscsi
    let artifacts = vec![artifact1, artifact2, artifact3, artifact4, artifact5, artifact6,
                         artifact7, artifact8, artifact9, artifact10, artifact11];
    
    let arg1 = "--version";
    let arg2 = "-version";
    let arg3 = "-version";
    let arg4 = "--version";
    let arg5 = "-V";
    let arg6 = "--version";
    let arg7 = "--version";
    let arg8 = "--version";
    let arg9 = "--version";
    let arg10 = "--version";
    let arg11 = "--version";
    let arguments = vec![arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11];
    let test1 = "ext/hw/lscpu";
    let test2 = "ext/hw/lshw";
    let test3 = "ext/hw/procinfo";
    let test4 = "dmidecode";
    let test5 = "ext/hw/mprober";
    let test6 = "ext/hw/free";
    let test7 = "ext/hw/lsmem";
    let test8 = "ext/hw/lsblk";
    let test9 = "ext/hw/df";
    let test10 = "lspci";
    let test11 = "ext/hw/lsscsi";
    let tests = vec![test1,test2,test3,test4,test5,test6,test7,test8,test9,test10,test11];
    for i in 0..arguments.len() {
      let check = match Command::new(tests[i]).arg(arguments[i]).output() {
        Ok(out) => out,
        Err(_) => { return TestStatus::Failed; }
      };
      let file_str = match fs::read_to_string(artifacts[i]) {
        Ok(out) => out,
        Err(err) => {
          debug::print_debug(format!("hw::on_test: Failed to open artifact: {}", err));
          return TestStatus::Failed;
        }
      };
      let result = String::from_utf8_lossy(&check.stdout);
      let error = String::from_utf8_lossy(&check.stderr); 
      // Compare
      if result != file_str && error != file_str { return TestStatus::Failed; }
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn hw (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Hw {
    event: Event {
      name:   name,
      desc:   "Host hardware querying utilities.".to_string(),
      usage:  "hw [all|mem|cpu|disk|cache|pci|usb|scsi|serial]\n".to_string(),
      parent: parent,
      author: "$t@$h".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }
  })
}

/*********************************** ripgrep ***********************************/
pub struct Ripgrep { event: Event }
impl Eventable for Ripgrep {
  fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() < 2 { 
      return self.event.usage.clone(); 
    }
    let arguments = vec!["--hidden".to_string(), args[0].clone(), args[1].clone()];
    let output = match run_console_command(Command::new("ext/ripgrep/rg").args(arguments)) {
			Ok(out) => out,
      Err(err) => { 
        return format!("failed to execute ripgrep. {}\n",err); 
      }  
		};
		// Print output
		log::log("natls", &String::from_utf8_lossy(&output.stderr));
		return String::from_utf8_lossy(&output.stdout).to_string();
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/ripgrep/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/ripgrep/rg").arg("--help").output() {
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
					debug::print_debug(format!("ripgrep::on_test: Failed to open the test file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ripgrep(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Ripgrep {
event: Event {
name:   name,
desc:   "Like the standard grep command but with improvements.".to_string(),
usage:  "ripgrep <token_to_search> <directory_path>.\n".to_string(),
parent: parent,
author: "Andrew Gallant (burntsushi)".to_string(),
easyrun: true,
secure: false,
links:  links
}
})
}

/*********************************** smv ***********************************/
pub struct Smv { event: Event }
impl Eventable for Smv {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() != 2 { return self.event.usage.clone(); }
    let arguments = vec![args[0].clone(), args[1].clone()];
		let last_chance = String::from("Are you sure you want to move this file?");
    if alerts::confirm_task(&last_chance) == constants::UNCONFIRMED {
      print::print_custom("Operation cancelled.\n","orange");
      return String::from("");
    }
    let output = match run_console_command(Command::new("ext/smv/smv").args(arguments)) {
			Ok(out) => out,
			Err(err) => { return format!("failed to execute smv. {}\n",err); }  
		};
		// Print output
		log::log("smv", &String::from_utf8_lossy(&output.stderr));
		return String::from_utf8_lossy(&output.stdout).to_string();
	}
	fn on_test(&self) -> TestStatus {
		let input_path = "tst/smv/inp/test.txt";
		let input_cp_path = "tst/smv/inp/test2.txt";
		let staging_folder_path = "tst/smv";
		let staging_file_path = "tst/smv/test.txt";
		let artifact_path = "tst/smv/atf/atf.txt";
    util::misc::cleanup(staging_file_path);

		// Run check
		match Command::new("cp").args(vec![input_path,input_cp_path]).output() {
			Ok(_) => (),
				Err(err) => {
					debug::print_debug(format!("Smv test error: {}", err));
					return TestStatus::Failed;
				}
		};
		match Command::new("ext/smv/smv").args(vec![input_cp_path,staging_file_path]).output() {
			Ok(_) => (),
				Err(err) => {
					debug::print_debug(format!("Smv test error: {}", err));
					return TestStatus::Failed;
				}
		};
		let check = match Command::new("ls").args(vec!["-1",staging_folder_path]).output() {
			Ok(o) => o,
				Err(err) => {
					debug::print_debug(format!("Smv test error: {}", err));
					return TestStatus::Failed;
				}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
				Err(err) => {
					debug::print_debug(format!("smv::on_test: Failed to open the test file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn smv(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Smv {
			event: Event {
			name:   name,
			desc:   "Like the standard mv command but with improvements.".to_string(),
			usage:  "Input a source and destination path and the file or directory will be moved.\n".to_string(),
			parent: parent,
			author: "Copyright (c) 2018, Michael Monsivais".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** feel ***********************************/
pub struct Feel { event: Event }
impl Eventable for Feel {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if !(args.len() == 1) { return self.event.usage.clone(); }
    let out = match run_console_command(Command::new("ext/feel/feel").arg(args[0].clone())) {
			Ok(o) => o,
			Err(e) => { return format!("feel(touch) command failed: {}\n", e); }  
		};
    if std::path::Path::new(&args[0].clone()).exists() {
      print::print_custom("Created path successfully.\n","brightgreen");
    }
    return String::from_utf8_lossy(&out.stdout).to_string();
	}
	fn on_test(&self) -> TestStatus {
		let staging_path = "tst/feel/test.txt";
		let artifact_path = "tst/feel/atf/atf.txt";
    util::misc::cleanup(staging_path);

		// Run check
		match Command::new("ext/feel/feel").arg(staging_path).output() {
			Ok(_) => (),
			Err(err) => {
				debug::print_debug(format!("Error running feel test: {}", err));
				return TestStatus::Failed;
			}
		};
		let check = match Command::new("ls").args(vec!["-1","tst/feel"]).output() {
			Ok(o) => o,
			Err(err) => {
				debug::print_debug(format!("Error running feel test: {}", err));
				return TestStatus::Failed;
			}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
			Err(err) => {
				debug::print_debug(format!("feel::on_test: Failed to open the test file. {}", err));
				return TestStatus::Failed;
			}
		};

		// Compare
    let result = String::from_utf8_lossy(&check.stdout);
		if result != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn feel(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Feel {
			event: Event {
			name:   name,
			desc:   "Like the touch command but with improvements and reborn in Rust.".to_string(),
			usage:  "Takes a file path and creates a blank file at that location.\n\nNote: Creates the path to the file if it does not exist.\n".to_string(),
			parent: parent,
			author: "Jacob Rothstein\thi@jbr.me".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** ln ***********************************/
pub struct Ln { event: Event }
impl Eventable for Ln {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() < 1 || args.len() > 2 { return self.event.usage.clone(); }
    let mut arguments = Vec::new();
    if args.len() == 1 {
      arguments.push("unlink".to_string());
      arguments.push(args[0].clone());
			match run_console_command(Command::new("ext/util/coreutils").args(&arguments)) {
				Ok(_) => { print::print_custom("Symlink successfully unlinked.\n","neongreen"); },
				Err(err) => { return format!("{}\n",err); }
			};
		  return String::from("");
    }
    else if args.len() == 2 {
      arguments.push("ln".to_string());
      arguments.push("-s".to_string());
      arguments.push(args[0].clone());
      arguments.push(args[1].clone());
			match run_console_command(Command::new("ext/util/coreutils").args(&arguments)) {
				Ok(_) => { print::print_custom("Symlink created sucessfully\n","neongreen"); },
				Err(err) => { return format!("{}\n",err); }
			};
		  return String::from("");
    }
    else { return self.event.usage.clone(); }
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/ln/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/util/coreutils").args(vec!["--help"]).output() {
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
				debug::print_debug(format!("ln::on_test: Failed to open the test file. {}", err));
				return TestStatus::Failed;
			}
		};
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ln(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Ln {
			event: Event {
			name:   name,
			desc:   "Applet to create or unlink symbolic links.".to_string(),
			usage:  "ln <symlink_to_unlink>\nln <path_to_file_or_dir> <symlink_name>\n".to_string(),
			parent: parent,
			author: "Rust Coreutils community".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** Net ***********************************/
pub struct Net { event: Event }
impl Eventable for Net {
  fn on_run(&self, _args: Vec<String>) -> String {
    if main_info::is_bg() || main_info::get_file_redirect() {
      print::print_custom("Net currently does not support background or file redirection.\n\nComing soon.\n","orange");
      return String::from("");
    }
    // Run command
    let public_ip = match Command::new("ext/net/gip").arg("-4").output() {
      Ok(out) => out, 
      Err(err) => { 
        return format!("Error running gip command: {}\n", err);
      }    
    };  
    let pub_ip: String = String::from_utf8_lossy(&public_ip.stdout).to_string();
    let arguments = vec![pub_ip.replace("\n",""), "-a".to_string()]; 
    print::print_custom("Public\n","orange");
    // Run command
    match run_console_command(Command::new("ext/net/ipgeo").args(arguments)) {
      Ok(_) => (), 
      Err(err) => { 
        return format!("Error running ipgeo: {}\n", err);
      }    
    };   
   
    let arguments = vec!["-c","yes","dev","status"];
    print::print_custom("\nStatus\n","orange");
    // Run command
    match run_console_command(Command::new("nmcli").args(arguments)) {
      Ok(_) => (), 
      Err(err) => { 
        return format!("Error running nmcli command: {}\n", err);
      }    
    };   

    let arguments = vec!["-c","-s","addr"]; 
    print::print_custom("\nLocal\n","orange");
    // Run command
    match run_console_command(Command::new("ip").args(arguments)) {
      Ok(_) => (), 
      Err(err) => { 
        return format!("Error running ip command: {}\n", err);
      }    
    };   
    
    print::print_custom("\nARP","orange");
    print::print_custom_uncapped("\n","grey");
    // Run check
    let arguments = vec!["-v"]; 
    // Run command
    match run_console_command(Command::new("arp").args(arguments)) {
      Ok(out) => out, 
      Err(err) => { 
        return format!("Error running arp command: {}\n", err);
      }    
    };
    print::print_custom("\nRouting","orange");
    print::print_custom_uncapped("\n","purple");
    // Run command
    match run_console_command(&mut Command::new("route")) {
      Ok(out) => out, 
      Err(err) => { 
        return format!("Error running route command: {}\n", err);
      }    
    };
    print::print_custom("","reset");
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/net/atf/atf.txt";

    // Run command
    let check = match Command::new("nmcli").arg("-h").output() {
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
        debug::print_debug(format!("net::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }   
    };  

    // Compare
    if String::from_utf8_lossy(&check.stdout).contains(&file_str) {
      return TestStatus::Failed;
    }   
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn net(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Net {
    event: Event {
      name:   name,
      desc:   "Displays an assortment of host network information.".to_string(),
      usage:  "Simply invoke and the network will be analyzed.\n".to_string(),
      author: "$t@$h".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }   
  })  
}

/*********************************** smkdir ***********************************/
pub struct Smkdir { event: Event }
impl Eventable for Smkdir {
	fn on_run(&self, args: Vec<String>) -> String {
		if !(args.len() == 1) { return self.event.usage.clone(); }
    match run_console_command(Command::new("ext/smkdir/smkdir").arg(args[0].clone())) {
			Ok(out) => out,
			Err(err) => { return format!("failed to execute smkdir. {}\n",err); }  
		};
	  return String::from("");
  }
	fn on_test(&self) -> TestStatus {
		let staging_path = "tst/smkdir/test_dir";
		let artifact_path = "tst/smkdir/atf/atf.txt";
    util::misc::cleanup(staging_path);

		// Run check
		match Command::new("ext/smkdir/smkdir").arg(staging_path).output() {
			Ok(_) => (),
			Err(err) => {
				debug::print_debug(format!("Error with smkdir test: {}", err));
				return TestStatus::Failed;
			}
		};
		let check = match Command::new("ls").args(vec!["-1","tst/smkdir"]).output() {
			Ok(o) => o,
			Err(err) => {
				debug::print_debug(format!("Error running smkdir test: {}", err));
				return TestStatus::Failed;
			}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
			Err(err) => {
				debug::print_debug(format!("smkdir::on_test: Failed to open the test file. {}", err));
				return TestStatus::Failed;
			}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn smkdir(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Smkdir {
			event: Event {
			name:   name,
			desc:   "Like the standard mkdir command but with improvements.".to_string(),
			usage:  "Input a path and a directory will created at that location.\n".to_string(),
			parent: parent,
			author: "$t@$h".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** srmdir ***********************************/
pub struct Srmdir { event: Event }
impl Eventable for Srmdir {
	fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 { return self.event.usage.clone(); }
    use fs_utils::check::is_folder_empty;
    let is_empty = match is_folder_empty(args[0].clone()) {
      Ok(o) => o,
      Err(_) => {
        print::print_custom("Not a directory or invalid. Deletion declined.\n","orange");
        return String::from("");
      }
    };
    if !is_empty {
      print::print_custom("Directory is not empty. Deletion declined.\n","orange");
      return String::from("");
    }
    match run_console_command(Command::new("ext/util/coreutils").args(vec!["rmdir".to_string(),args[0].clone()])) {
			Ok(_) => (),
			Err(e) => {
        print::print_custom(&format!("{}",e),"orange");
        return String::from("");
      }  
		};
    print::print_custom("Directory removal successful.\n","brightgreen");
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let input_path = "tst/srmdir/inp/test_dir";
    let staging_path = "tst/srmdir/test_dir";
		let artifact_path = "tst/srmdir/atf/atf.txt";
    util::misc::cleanup(staging_path);

		// Run check
		match Command::new("cp").args(vec!["-r",input_path,staging_path]).output() {
			Ok(_) => (),
				Err(err) => {
					debug::print_debug(format!("Error: {}", err));
					return TestStatus::Failed;
				}
		};
		let check = match Command::new("ls").args(vec!["-1","tst/srmdir"]).output() {
			Ok(o) => o,
			Err(err) => {
				debug::print_debug(format!("Error running srmdir test: {}", err));
				return TestStatus::Failed;
			}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
			Err(err) => {
				debug::print_debug(format!("srmdir::on_test: Failed to open the artifact file. {}", err));
				return TestStatus::Failed;
			}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn srmdir(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Srmdir {
			event: Event {
			name:   name,
			desc:   "Like the standard rmdir command but imported via Rust coreutils.".to_string(),
			usage:  "Pass a path to a directory and it will be deleted if it is empty.\n".to_string(),
			parent: parent,
			author: "Rust coreutils contributors".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** cksum ***********************************/
pub struct Cksum { event: Event }
impl Eventable for Cksum {
	fn on_run(&self, args: Vec<String>) -> String {
		if !(args.len() == 1) { return self.event.usage.clone(); }
    let output = match Command::new("ext/util/coreutils").args(vec!["cksum".to_string(),args[0].clone()]).output() {
			Ok(out) => out,
			Err(e) => {
        print::print_custom(&format!("{}",e),"orange");
        return String::from("");
      }  
		};
		// Print output
    let out = String::from_utf8_lossy(&output.stdout); 
    let out_split = out.split(" ").collect::<Vec<&str>>();
		print::print_custom(&format!("Checksum: {}\t", out_split[0]),"purple");
    print::print_custom(&format!("Size: {} bytes\n", out_split[1]), "purple");
    log::log("srmdir", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let input_path = "tst/cksum/inp/test.txt";
		let artifact_path = "tst/cksum/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/util/coreutils").args(vec!["cksum",input_path]).output() {
			Ok(o) => o,
				Err(err) => {
					debug::print_debug(format!("Cksum error: {}", err));
					return TestStatus::Failed;
				}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
				Err(err) => {
					debug::print_debug(format!("cksum::on_test: Failed to open the artifact file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn cksum(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Cksum {
			event: Event {
			name:   name,
			desc:   "Rust coreutils implementation of the standard cksum command.".to_string(),
			usage:  "cksum <file_name>\n".to_string(),
			parent: parent,
			author: "Rust coreutils contributors".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** head ***********************************/
pub struct Head { event: Event }
impl Eventable for Head {
	fn on_run(&self, args: Vec<String>) -> String {
		if !(args.len() == 1) { return self.event.usage.clone(); }
    let output = match Command::new("ext/util/coreutils").args(vec!["head".to_string(),args[0].clone()]).output() {
			Ok(out) => out,
			Err(e) => {
        print::print_custom(&format!("{}",e),"orange");
        return String::from("");
      }  
		};
		// Print output
    let out = String::from_utf8_lossy(&output.stdout); 
		print::print_custom(&out,"grey");
    log::log("head", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let input_path = "tst/head/inp/test.txt";
		let artifact_path = "tst/head/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/util/coreutils").args(vec!["head","-n","1",input_path]).output() {
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
					debug::print_debug(format!("head::on_test: Failed to open the test file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn head(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Head {
			event: Event {
			name:   name,
			desc:   "The head command implemented in Rust.".to_string(),
			usage:  "Takes a single argument that is the path to a file.\n".to_string(),
			parent: parent,
			author: "Jordi Boggiano\n\nCopyright (c) Jordi Boggiano".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** tail ***********************************/
pub struct Tail { event: Event }
impl Eventable for Tail {
	fn on_run(&self, args: Vec<String>) -> String {
		if !(args.len() == 1) { return self.event.usage.clone(); }
    let output = match Command::new("ext/util/coreutils").args(vec!["tail".to_string(),args[0].clone()]).output() {
			Ok(out) => out,
			Err(e) => {
        print::print_custom(&format!("{}",e),"orange");
        return String::from("");
      }  
		};
		// Print output
    let out = String::from_utf8_lossy(&output.stdout); 
		print::print_custom(&out,"grey");
    log::log("srmdir", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let input_path = "tst/tail/inp/test.txt";
		let artifact_path = "tst/tail/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/util/coreutils").args(vec!["tail","-n","1",input_path]).output() {
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
					debug::print_debug(format!("tail::on_test: Failed to open the test file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn tail(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Tail {
			event: Event {
			name:   name,
			desc:   "The tail command implemented in Rust.".to_string(),
			usage:  "Takes a single argument that is the path to a file.\n".to_string(),
			parent: parent,
			author: "Jordi Boggiano\n\nCopyright (c) Jordi Boggiano".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** pwd ***********************************/
pub struct Pwd { event: Event }
impl Eventable for Pwd {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() > 0 { 
      return self.event.usage.clone(); 
    }
    let output = match Command::new("ext/util/coreutils").arg("pwd").output() {
			Ok(out) => out,
			Err(e) => {
        print::print_custom(&format!("{}",e),"orange");
        return String::from("");
      }  
		};
		// Print output
    let out = String::from_utf8_lossy(&output.stdout); 
		print::print_custom(&out,"grey");
    log::log("srmdir", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/pwd/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/util/coreutils").args(vec!["pwd","-V"]).output() {
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
					debug::print_debug(format!("pwd::on_test: Failed to open the artifact. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn pwd(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Pwd {
			event: Event {
			name:   name,
			desc:   "Rust implementation of pwd.".to_string(),
			usage:  "Takes no arguments, just invoke.\n".to_string(),
			parent: parent,
			author: "Jordi Boggiano\n\nCopyright (c) Jordi Boggiano".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** readlink ***********************************/
pub struct Readlink { event: Event }
impl Eventable for Readlink {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() != 1 { 
      return self.event.usage.clone(); 
    }
    let output = match Command::new("ext/util/coreutils").args(vec!["readlink".to_string(),args[0].clone()]).output() {
			Ok(out) => out,
			Err(e) => {
        print::print_custom(&format!("{}",e),"orange");
        return String::from("");
      }  
		};
		// Print output
    let out = String::from_utf8_lossy(&output.stdout); 
		print::print_custom(&out,"purple");
    log::log("srmdir", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/readlink/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/util/coreutils").args(vec!["readlink","-V"]).output() {
			Ok(o) => o,
			Err(err) => {
				debug::print_debug(format!("Error running readlink test: {}", err));
				return TestStatus::Failed;
			}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
			Err(err) => {
				debug::print_debug(format!("readlink::on_test: Failed to open the test file. {}", err));
				return TestStatus::Failed;
			}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn readlink(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Readlink {
			event: Event {
			name:   name,
			desc:   "Like the standard readlink but from Rust coreutils.".to_string(),
			usage:  "readlink <path_to_symlink>\n".to_string(),
			parent: parent,
			author: "Rust coreutils contributors".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** seq ***********************************/
pub struct Seq { event: Event }
impl Eventable for Seq {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if !(args.len() == 2) { return self.event.usage.clone(); }
    let output = match Command::new("ext/util/coreutils").args(vec!["seq".to_string(),args[0].clone(),args[1].clone()]).output() {
			Ok(out) => out,
			Err(e) => {
        print::print_custom(&format!("{}",e),"orange");
        return String::from("");
      }  
		};
		// Print output
    let out = String::from_utf8_lossy(&output.stdout); 
		print::print_custom(&out,"grey");
    log::log("srmdir", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/seq/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/util/coreutils").args(vec!["seq","0","3"]).output() {
			Ok(o) => o,
			Err(err) => {
				debug::print_debug(format!("Error with seq test: {}", err));
				return TestStatus::Failed;
			}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
				Err(err) => {
					debug::print_debug(format!("seq::on_test: Failed to open artifact file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn seq(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Seq {
			event: Event {
			name:   name,
			desc:   "Like the standard seq command but written in Rust.".to_string(),
			usage:  "Usage: seq <start_int> <end_int>\n".to_string(),
			parent: parent,
			author: "Jordi Boggiano\n\nCopyright (c) Jordi Boggiano".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** sort ***********************************/
pub struct Sort { event: Event }
impl Eventable for Sort {
	fn on_run(&self, args: Vec<String>) -> String {
		if !(args.len() == 1) { return self.event.usage.clone(); }
    let output = match Command::new("ext/util/coreutils").args(vec!["sort".to_string(),args[0].clone()]).output() {
			Ok(out) => out,
			Err(e) => { 
        print::print_custom(&format!("{}",e),"orange");
        return String::from("");
      }  
		};
		// Print output
    let out = String::from_utf8_lossy(&output.stdout); 
		print::print_custom(&out,"brightgreen");
    log::log("srmdir", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let input_path = "tst/sort/inp/test.txt";
		let artifact_path = "tst/sort/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/util/coreutils").args(vec!["sort",input_path]).output() {
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
					debug::print_debug(format!("sort::on_test: Failed to open the test file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn sort(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Sort {
			event: Event {
			name:   name,
			desc:   "Rust rewrite of the standard sort command.".to_string(),
			usage:  "Pass this tool a file and it will sort it to stdout.\n".to_string(),
			parent: parent,
			author: "Jordi Boggiano\n\nCopyright (c) Jordi Boggiano".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** strip ***********************************/
pub struct Strip { event: Event }
impl Eventable for Strip {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() < 1 || args.len() > 2 { return self.event.usage.clone(); }
    let arguments;
    if args.len() == 2 {
      match args[0].as_str() {
              "all" => arguments = vec!["-s".to_string(),args[0].clone()],
            "debug" => arguments = vec!["-d".to_string(),args[0].clone()],
        "nonglobal" => arguments = vec!["-x".to_string(),args[0].clone()],
         "compiler" => arguments = vec!["-X".to_string(),args[0].clone()],
                   _=> return self.event.usage.clone(),
      }  
    }
    else { arguments = vec!["-s".to_string(),args[0].clone()]; }
    match Command::new("ext/strip/strip").args(&arguments).output() {
			Ok(_) => { print::print_custom("Binary stripped successfully.\n","brightgreen"); },
			Err(e) => {
        print::print_custom_uncapped("","orange");
        return format!("{}\n",e);
      }  
		};
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/strip/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/strip/strip").arg("/dev/null").output() {
			Ok(o) => o,
			Err(err) => {
				debug::print_debug(format!("Error with strip: {}", err));
				return TestStatus::Failed;
			}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
			Err(err) => {
				debug::print_debug(format!("strip::on_test: Failed to open the test file. {}", err));
				return TestStatus::Failed;
			}
		};

		// Compare
		if String::from_utf8_lossy(&check.stderr) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn strip(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Strip {
			event: Event {
			name:   name,
			desc:   "Removes symbols and sections from files.".to_string(),
			usage:  "This utility needs the following arguments:\n\
              \t1) all|debug|nonglobal|compiler\n\
              \t2) Path to file\n".to_string(),
			parent: parent,
			author: "".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** merge ***********************************/
pub struct Merge { event: Event }
impl Eventable for Merge {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() != 4 { 
      return self.event.usage.clone(); 
    }
    match Command::new("ext/slm_merge/slm_merge").args(&args).output() {
			Ok(_) => (),
			Err(e) => {
        print::print_custom_uncapped("","orange");
        return format!("{}\n",e);
      }  
		};
		// Print output
		print::print_custom("Directories merged successfully.\n","brightgreen");
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/merge/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/merge/slm_merge").arg("__test__").output() {
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
					debug::print_debug(format!("merge::on_test: Failed to open the test file. {}", err));
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
pub fn merge(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Merge {
			event: Event {
			name:   name,
			desc:   "Command that merges files from two directories into a new third directory.".to_string(),
			usage:  "merge <dir1> <dir2> <extension> <out_dir>\n\n(extension is a way to filter file types)\n".to_string(),
			parent: parent,
			author: "$t@$h".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** deedoo ***********************************/
pub struct Deedoo { event: Event }
impl Eventable for Deedoo {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() != 1 { 
      return self.event.usage.clone(); 
    }
    let output = match Command::new("ext/deedoo/deedoo").args(vec!["-E".to_string(),args[0].clone()]).output() {
			Ok(o) => o,
			Err(e) => {
        print::print_custom_uncapped("","orange");
        return format!("{}\n",e);
      }  
		};
		// Print output
    print::print_custom(&format!("{}",&String::from_utf8_lossy(&output.stdout)),"grey");
		print::print_custom("Files in directory deduplicated successfully.\n","brightgreen");
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/deedoo/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/deedoo/deedoo").arg("--help").output() {
			Ok(o) => o,
				Err(err) => {
					debug::print_debug(format!("Error(deedoo): {}", err));
					return TestStatus::Failed;
				}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
				Err(err) => {
					debug::print_debug(format!("deedoo::on_test: Failed to open the artifact file. {}", err));
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
pub fn deedoo(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Deedoo {
			event: Event {
			name:   name,
			desc:   "Applet that deduplicates files in one directory into a new directory.".to_string(),
			usage:  "Input a directory path and it will be copied and deduplicated.\n".to_string(),
			parent: parent,
			author: "Kris\n\nversbinarii@gmail.com".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** deleteall ***********************************/
pub struct Deleteall { event: Event }
impl Eventable for Deleteall {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() != 1 { 
      return self.event.usage.clone(); 
    }
    // Last chance warning
    let mut last_chance = String::from("You sure you want to delete ");
    last_chance.push_str(&args[0]);
    last_chance.push_str(" ?");
    if alerts::confirm_task(&last_chance) == constants::UNCONFIRMED {
      print::print_custom("Operation cancelled.\n","orange");
      return String::from("");
    }
    match Command::new("ext/deleteall/slm_rm_rf").arg(args[0].clone()).output() {
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
		let staging_dir = "tst/deleteall/tmp";
		let staging_path = "tst/deleteall";
		let artifact_path = "tst/deleteall/atf/atf.txt";

		match Command::new("mkdir").arg(staging_dir).output() {
			Ok(_) => (),
			Err(err) => {
				debug::print_debug(format!("Error: {}", err));
				return TestStatus::Failed;
			}
		};
		match Command::new("ext/deleteall/slm_rm_rf").arg(staging_dir).output() {
			Ok(_) => (),
			Err(err) => {
				debug::print_debug(format!("Error: {}", err));
				return TestStatus::Failed;
			}
		};
		// Run check
		let check = match Command::new("ls").args(vec!["-1",staging_path]).output() {
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
				debug::print_debug(format!("deleteall::on_test: Failed to open the test file. {}", err));
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
pub fn deleteall(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Deleteall {
			event: Event {
			name:   name,
			desc:   "Like the rm -rf command. Deletes directory and contents.".to_string(),
			usage:  "deleteall <directory_path>.\n".to_string(),
			parent: parent,
			author: "".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** Stringsext ***********************************/
pub struct Stringsext { event: Event }
impl Eventable for Stringsext {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() < 1 || args.len() > 1 {
			print::print_custom("strings takes one argument - a path to a binary.\n", "orange");
      return self.event.usage.clone();
		}

		// Run command
		let output = match Command::new("ext/stringsext/stringsext").args(vec!["-n".to_string(),"3".to_string(),"-t".to_string(),"x".to_string(),"--".to_string(),args[0].clone()]).output() {
			Ok(out) => out,
			Err(e) => { return format!("Failed to run the stringsext command. {}\n", e); }  
		};

		print::print_custom("+----------------+--------------------------------+\n", "lightbluegreen");
		print::print_custom("|   Hex Offset   |      unicode/ascii string      |\n", "lightbluegreen");
		print::print_custom("+----------------+--------------------------------+\n", "lightbluegreen");

		let output_str_nl = String::from_utf8_lossy(&output.stdout).to_string();
		let output_str: Vec<&str> = output_str_nl.split('\n').collect(); 
		for i in 1..output_str.len() - 1 {
			if output_str[i].len() > 1 {
				let output_tb: Vec<&str> = output_str[i].split('\t').collect();
				print!("{: >15} ",output_tb[0].replace("<","").replace(">",""));
				print::print_custom(" | ", "lightbluegreen");
				println!("{}",output_tb[1].replace("\n","").replace("\r",""));
			}
    } 
		log::log("stringsext", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/stringsext/atf/atf.txt";

		// Run command
		let ascii = match Command::new("ext/stringsext/stringsext").arg("--help").output() {
			Ok(a) => a,
			Err(err) => {
				debug::print_debug(format!("Error: {}", err));
				return TestStatus::Failed;
			}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
			Err(err) => {
				debug::print_debug(format!("stringsext::on_test: Failed to open artifact file. {}", err));
				return TestStatus::Failed;
			}
		};

		// Compare
		if String::from_utf8_lossy(&ascii.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn stringsext(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Stringsext {
event: Event {
name:   name,
desc:   "Nice and lean file editor.".to_string(),
usage:  "Just give us the file path and we'll take it from there.\n".to_string(),
parent: parent,
author: "".to_string(),
easyrun: false,
secure: false,
links:  links
}
})
}

/*********************************** base64 ***********************************/
pub struct Base64 { event: Event }
impl Eventable for Base64 {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		let mut base_cmd = String::new();
    let mut first_cmd = String::new();
    let mut enc_str = String::new();
    if args.len() < 1 { return self.event.usage.clone(); }
    if args[0].eq("d") {
      if args.len() < 2 { return self.event.usage.clone(); }
      first_cmd.push_str("echo ");
      for i in 1..args.len() - 1 {
        enc_str.push_str(&args[i]);
        enc_str.push_str(" ");
      }
      enc_str.push_str(&args[args.len() - 1]);
      first_cmd.push_str(&enc_str);
      base_cmd.push_str("ext/util/coreutils base64 -d"); 
    }
    else {
      first_cmd.push_str("echo ");
      for i in 0..args.len() - 1 {
        enc_str.push_str(&args[i]);
        enc_str.push_str(" ");
      }
      enc_str.push_str(&args[args.len() - 1]);
      first_cmd.push_str(&enc_str);
      base_cmd.push_str("ext/util/coreutils base64"); 
    }
		let output = match util::pipes::Pipe::new(&first_cmd.to_string())
                                         .then(&base_cmd.to_string())
                                         .finally() {
			Ok(sel) => sel,
				Err(err) => {
					return format!("Command failed. Error: {}\n", err);
				}
		};
		let output = match output.wait_with_output() {
			Ok(sel) => sel,
				Err(err) => {
					return format!("Command failed. Error: {}\n", err);
				}
		};
		 
    // Print output
    let mut out = String::from_utf8_lossy(&output.stdout).to_string(); 
    out.push('\n');
		print::print_custom(&out,"bluegreen");
    log::log("base64", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/base64/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/util/coreutils").args(vec!["base64","--help"]).output() {
			Ok(o) => o,
				Err(err) => {
					debug::print_debug(format!("base64 error: {}", err));
					return TestStatus::Failed;
				}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
				Err(err) => {
					debug::print_debug(format!("base64::on_test: Failed to open the test file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn base64(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Base64 {
			event: Event {
			name:   name,
			desc:   "Classic Base64 encoding/decoding utility but written in Rust.".to_string(),
			usage:  "This applet can be run in the following ways:\n\
              \tEncoding: base64 <string>\n\
              \tDecoding: base64 d <string>\n".to_string(),
			parent: parent,
			author: "Jordi Boggiano\n\nCopyright (c) Jordi Boggiano".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** base32 ***********************************/
pub struct Base32 { event: Event }
impl Eventable for Base32 {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		let mut base_cmd = String::new();
    let mut first_cmd = String::new();
    let mut enc_str = String::new();
    if args.len() < 1 { return self.event.usage.clone(); }
    if args[0].eq("d") {
      if args.len() < 2 { return self.event.usage.clone(); }
      first_cmd.push_str("echo ");
      for i in 1..args.len() - 1 {
        enc_str.push_str(&args[i]);
        enc_str.push_str(" ");
      }
      enc_str.push_str(&args[args.len() - 1]);
      first_cmd.push_str(&enc_str);
      base_cmd.push_str("ext/util/coreutils base32 -d"); 
    }
    else {
      first_cmd.push_str("echo ");
      for i in 0..args.len() - 1 {
        enc_str.push_str(&args[i]);
        enc_str.push_str(" ");
      }
      enc_str.push_str(&args[args.len() - 1]);
      first_cmd.push_str(&enc_str);
      base_cmd.push_str("ext/util/coreutils base32"); 
    }
		let output = match util::pipes::Pipe::new(&first_cmd.to_string())
                                         .then(&base_cmd.to_string())
                                         .finally() {
			Ok(sel) => sel,
				Err(err) => {
					return format!("Command failed. Error: {}\n", err);
				}
		};
		let output = match output.wait_with_output() {
			Ok(sel) => sel,
				Err(err) => {
					return format!("Command failed. Error: {}\n", err);
				}
		};
		 
    // Print output
    let mut out = String::from_utf8_lossy(&output.stdout).to_string();
    out.push('\n');
		print::print_custom(&out,"bluegreen");
    log::log("base32", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/base32/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/util/coreutils").args(vec!["base32","--help"]).output() {
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
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn base32(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Base32 {
			event: Event {
			name:   name,
			desc:   "Classic Base64 encoding/decoding utility but written in Rust.".to_string(),
			usage:  "This applet can be run in the following ways:\n\
              \tEncoding: base32 <string>\n\
              \tDecoding: base32 d <string>\n".to_string(),
			parent: parent,
			author: "Jordi Boggiano\n\nCopyright (c) Jordi Boggiano".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** findBytes ***********************************/
pub struct FindBytes { event: Event }
impl Eventable for FindBytes {
	fn on_run(&self, args: Vec<String>) -> String {
		let mut cmd_args = Vec::new();
    if args.len() == 2 { 
		  cmd_args.push("-b");
		  cmd_args.push(&args[0]);
		  cmd_args.push(&args[1]);
    }
    else if args.len() == 3 {
      if args[0].eq("-s") {
		    cmd_args.push("-t");
		    cmd_args.push(&args[1]);
		    cmd_args.push(&args[2]);
      }
      else { return self.event.usage.clone(); }
    }
    else { return self.event.usage.clone(); }
		match run_console_command(Command::new("ext/findbytes/hxd").args(cmd_args)) {
			Ok(_) => (),
			Err(e) => {
				print::print_custom(&format!("{}",e),"orange");
				return String::from("");
			}  
		};
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/findbytes/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/findbytes/hxd").output() {
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
					debug::print_debug(format!("findbytes::on_test: Failed to open the test file. {}", err));
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
pub fn findbytes(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(FindBytes {
			event: Event {
			name:   name,
			desc:   "Print occurance of bytes of interest in a binary.".to_string(),
			usage:  "findbytes <bytes> <path_to_binary>\n\nExample: findbytes abcdef12 salvum\n".to_string(),
			parent: parent,
			author: "".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** scrub ***********************************/
pub struct Scrub { event: Event }
impl Eventable for Scrub {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 { return self.event.usage.clone(); } 
    let file = args[0].clone();
    use std::fs::metadata;
    let md = match metadata(file.clone()) {
      Ok(o) => o,
     Err(_) => return String::from("Error getting metadata for scrub command\n"),
    };
    if md.is_dir() {
      print::print_custom("Can't scrub a directory. Use rmdir instead.\n","orange");
      return String::from("");
    }
    let file_vec: Vec<&str> = file.trim().split("/").collect();
    let root_dir = file_vec[1];
    let nono_direc = vec!["boot", "dev", "etc", "root", "sys" , "usr"];
    for dir in nono_direc {
      if root_dir.eq(dir) {
        print::print_custom("Directory off limits to scrubbing.\n","orange");
        return String::from("");
      }
    }
    let selections = vec!["NNSA NAP-14.1-C 3-pass",
                          "DoD 5220.22-M 3-pass",
                          "BSI 9-pass",
                          "US Army AR380-19 9-pass",
                          "Bruce Schneier Algorithm",
                          "Roy Pfitzner 33-random-pass",
                          "Gutmann 35-pass"];
    //get interface option from user
    let algorithm = match terminal::get_selection_custom(selections.clone(),"white") {
      Some(opt) => selections[opt],
			_=> {
				print::print_custom("Bad selection.\n","orange");
				return String::from("");
			}
    };
    print::print_custom(&format!("{}\n",algorithm).to_string(),"bluegreen");
    
    // Last chance warning
    let mut last_chance = String::from("You sure you want to delete ");
    last_chance.push_str(&file);
    last_chance.push_str(" ?");
    if alerts::confirm_task(&last_chance) == constants::UNCONFIRMED {
      print::print_custom("Operation cancelled.\n","orange");
      return String::from("");
    }
    let algo_arg;
    match algorithm {
      "NNSA NAP-14.1-C 3-pass" => algo_arg = "nnsa",
        "DoD 5220.22-M 3-pass" => algo_arg = "dod",
                  "BSI 9-pass" => algo_arg = "bsi",
     "US Army AR380-19 9-pass" => algo_arg = "usarmy",
    "Bruce Schneier Algorithm" => algo_arg = "schneier",
 "Roy Pfitzner 33-random-pass" => algo_arg = "pfitzner33",
             "Gutmann 35-pass" => algo_arg = "gutmann",
                              _=> algo_arg = "verify"
    }
    let arguments = vec!["-f".to_string(),"-r".to_string(),"-p".to_string(),algo_arg.to_string(),args[0].clone()];
    // Run scrub command
    print::print_custom_uncapped("","grey");
    match run_console_command(Command::new("ext/scrub/scrub").args(&arguments)) {
      Ok(out) => out,
      Err(e) => { return format!("Failed to run scrub command: {}\n",e); }
    };
    print::print_custom("","reset");
    print::print_custom("File removal successful.\n","brightgreen");
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/scrub/test.txt";
    let input_path = "tst/scrub/inp/test.txt";
    let artifact_path = "tst/scrub/atf/atf.txt";
    util::misc::cleanup(staging_path);
    let copy_args = vec![input_path, staging_path];

    // Run command
    match Command::new("cp").args(copy_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error with cp command in scrub test: {}", err));
        return TestStatus::Failed;
      }
    };

    // Run command
    match Command::new("ext/scrub/scrub").args(vec!["-f","-r","-p","verify",staging_path]).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error running scrub test: {}", err));
        return TestStatus::Failed;
      }
    };
    // Run check
    let check_out = match util::pipes::Pipe::new(&"ls tst/scrub".to_string())
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("scrub::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel, 
      Err(err) => { 
        debug::print_debug(format!("scrub::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }    
    };   

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("scrub::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn scrub(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Scrub {
    event: Event {
      name:   name,
      desc:   "The Scrub file destruction tool. Performs various overwrites before deleting.".to_string(),
      usage:  "scrub [nnsa|dod|bsi|usarmy|schneier|pfitzner33|gutmann|verify] <file_name>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** wipe ***********************************/
pub struct Wipe { event: Event }
impl Eventable for Wipe {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 { return self.event.usage.clone(); } 
    let file = args[0].clone();
    use std::fs::metadata;
    let md = match metadata(file.clone()) {
      Ok(o) => o,
     Err(_) => return String::from("Error getting metadata for wipe command.\n"), 
    };
    if md.is_dir() {
      print::print_custom("Can't scrub a directory. Use rmdir instead.\n","orange");
      return String::from("");
    }
    let file_vec: Vec<&str> = file.trim().split("/").collect();
    let root_dir = file_vec[1];
    let nono_direc = vec!["boot", "dev", "etc", "root", "sys" , "usr"];
    for dir in nono_direc {
      if root_dir.eq(dir) {
        print::print_custom("Directory off limits to scrubbing.\n","orange");
        return String::from("");
      }
    }
    // Last chance warning
    let mut last_chance = String::from("You sure you want to delete ");
    last_chance.push_str(&file);
    last_chance.push_str(" ?");
    if alerts::confirm_task(&last_chance) == constants::UNCONFIRMED {
      print::print_custom("Operation cancelled.\n","orange");
      return String::from("");
    }
    // Run command
    print::print_custom_uncapped("","grey");
    match run_console_command(Command::new("ext/wipe/wipe").args(vec!["-f".to_string(),file])) {
      Ok(out) => out,
      Err(e) => { return format!("Failed to run wipe command: {}\n",e); }
    };
    print::print_custom("","reset");
    print::print_custom("File removal successful.\n","neongreen");
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let staging_path = "tst/wipe/test.txt";
    let input_path = "tst/wipe/inp/test.txt";
    let artifact_path = "tst/wipe/atf/atf.txt";
    util::misc::cleanup(staging_path);
    let copy_args = vec![input_path, staging_path];

    // Run command
    match Command::new("cp").args(copy_args).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Run command
    match Command::new("ext/wipe/wipe").args(vec!["-f",staging_path]).output() {
      Ok(_) => (),
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };
    // Run check
    let check_out = match util::pipes::Pipe::new(&"ls tst/wipe".to_string())
                                        .then("wc -l")
                                        .finally() {
      Ok(sel) => sel,
      Err(err) => {
        debug::print_debug(format!("wipe::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    let check = match check_out.wait_with_output() {
      Ok(sel) => sel, 
      Err(err) => { 
        debug::print_debug(format!("wipe::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }    
    };   

    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("wipe::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn wipe(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Wipe {
    event: Event {
      name:   name,
      desc:   "The Wipe file destruction tool. Performs various overwrites before deleting.".to_string(),
      usage:  "Give the file path and Wipe will destroy it for you.\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** text ***********************************/
pub struct Text { event: Event }
impl Eventable for Text {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
    let result: String;
    if args.len() == 2 { // length
      let subcommand = args[0].clone();
      let file = args[1].clone();
			use std::fs::metadata;
			let md = match metadata(file.clone()) {
        Ok(o) => o, 
       Err(e) => { return format!("Failed getting directory metadata: {}\n", e); }
      };
			if !md.is_file() || md.is_dir() {
		    print::print_custom("Input is not a file. Need a file.\n","orange");
			  return String::from("");
      };
			let mut cmd = String::from("cat ");
			cmd.push_str(&file);
      let cmd2;
      if subcommand.eq("length") { cmd2 = String::from("ext/text/string length"); }
      else { return self.event.usage.clone(); }
			let output = match util::pipes::Pipe::new(&cmd.to_string()).then(&cmd2.to_string()).finally() {
				Ok(sel) => sel,
				Err(err) => { return format!("Command failed. Error: {}\n", err); }
			};
			let out = match output.wait_with_output() {
				Ok(sel) => sel,
				Err(err) => { return format!("Command failed. Error: {}\n", err); }
			};
		  result = String::from_utf8_lossy(&out.stdout).to_string();
		  print::print_custom(&format!("{}",&result).to_string(),"brightgreen");
      log::log("text", &String::from_utf8_lossy(&out.stderr));
    }
    else if args.len() == 3 { // line
      let subcommand = args[0].clone();
      let file = args[1].clone();
      let linenumber = args[2].clone();
			use std::fs::metadata;
			let md = match metadata(file.clone()) {
        Ok(o) => o, 
       Err(e) => { return format!("Failed getting directory metadata: {}\n", e); }
      };
			if !md.is_file() || md.is_dir() {
		    print::print_custom("Input is not a file. Need a file.\n","orange");
			  return String::from("");
      };
			let mut cmd = String::from("cat ");
			cmd.push_str(&file);
      let mut cmd2;
      if subcommand.eq("line") {
        cmd2 = String::from("ext/text/string line ");
        cmd2.push_str("-n ");
        cmd2.push_str(&linenumber); 
      }
      else { return self.event.usage.clone(); }
			let output = match util::pipes::Pipe::new(&cmd.to_string()).then(&cmd2.to_string()).finally() {
				Ok(sel) => sel,
				Err(err) => { return format!("Command failed. Error: {}\n", err); }
			};
			let out = match output.wait_with_output() {
				Ok(sel) => sel,
				Err(err) => { return format!("Command failed. Error: {}\n", err); }
			};
		  result = String::from_utf8_lossy(&out.stdout).to_string();
		  print::print_custom(&format!("{}",&result).to_string(),"brightgreen");
      log::log("text", &String::from_utf8_lossy(&out.stderr));
    }
    else if args.len() == 4 { // replace
      let subcommand = args[0].clone();
      let file = args[1].clone();
      let original = args[2].clone();
      let replacement = args[3].clone();
			use std::fs::metadata;
			let md = match metadata(file.clone()) {
        Ok(o) => o, 
       Err(e) => { return format!("Failed getting directory metadata: {}\n", e); }
      };
			if !md.is_file() || md.is_dir() {
		    print::print_custom("Input is not a file. Need a file.\n","orange");
			  return String::from("");
      };
			let mut cmd = String::from("cat ");
			cmd.push_str(&file);
      let mut cmd2;
      if subcommand.eq("replace") {
        cmd2 = String::from("ext/text/string replace ");
        cmd2.push_str("--match ");
        cmd2.push_str(&original);
        cmd2.push_str(" --with " );
        cmd2.push_str(&replacement); 
      }
      else { return self.event.usage.clone(); }
			let output = match util::pipes::Pipe::new(&cmd.to_string()).then(&cmd2.to_string()).finally() {
				Ok(sel) => sel,
				Err(err) => { return format!("Command failed. Error: {}\n", err); }
			};
			let out = match output.wait_with_output() {
				Ok(sel) => sel,
				Err(err) => { return format!("Command failed. Error: {}\n", err); }
			};
		  result = String::from_utf8_lossy(&out.stdout).to_string().trim_end().to_string();
		  print::print_custom(&format!("{}\n",&result).to_string(),"brightgreen");
      log::log("text", &String::from_utf8_lossy(&out.stderr));
      util::misc::write_file(result,"out/text/text_result.txt".to_string());
		  print::print_custom("Replace result written to --> out/text/text_result.txt\n","neongreen");
    }
    else { return self.event.usage.clone(); }
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
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn text(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Text {
			event: Event {
			name:   name,
			desc:   "The text applet performs various string operations.".to_string(),
			usage:  "Print string at line in file:\n\ttext line <file_path> <line_number>\n\nPrint total string length of a file:\n\ttext length <file_path>\n\nReplace string in file and write new file to out/text/text_result.txt:\n\ttext replace <file_path> <string> <replacement>\n".to_string(),
			parent: parent,
			author: "Nils Martel".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** lddsafe ***********************************/
pub struct Lddsafe { event: Event }
impl Eventable for Lddsafe {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() != 1 { return self.event.usage.clone(); }
    /*
    if !args[0].contains(".so") {
      print::print_custom("ldd requires a shared library.\n","orange");
      return String::from("");
    }
    */
    let output = match Command::new("ext/lddsafe/lddsafe").arg(args[0].clone()).output() {
			Ok(out) => out,
			Err(e) => {
        print::print_custom(&format!("{}\n",e),"orange");
        return String::from("");
      }  
		};
		// Print output
    let out = String::from_utf8_lossy(&output.stdout); 
		print::print_custom(&out,"purple");
    log::log("lddsafe", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let input_path = "tst/lddsafe/inp/test.so";
		let artifact_path = "tst/lddsafe/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/lddsafe/lddsafe").arg(input_path).output() {
			Ok(o) => o,
				Err(err) => {
					debug::print_debug(format!("Lddsafe error: {}", err));
					return TestStatus::Failed;
				}
		};

		// Read file
		let atf_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out,
				Err(err) => {
					debug::print_debug(format!("lddsafe::on_test: Failed to open the test file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != atf_str {
      debug::print_debug(format!("-- Output --\n{}\n-- Artifact --\n{}", String::from_utf8_lossy(&check.stdout), atf_str));
      return TestStatus::Failed;
    }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn lddsafe(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Lddsafe {
			event: Event {
			name:   name,
			desc:   "Like the standard ldd command but uses objdump to avoid security flaws of ldd.".to_string(),
			usage:  "Input a path to a shared library or binary.\n".to_string(),
			parent: parent,
			author: "Ricardo Garcia\nr@rg3.name".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** ipv4 ***********************************/
pub struct Ipv4 { event: Event }
impl Eventable for Ipv4 {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 7 { return self.event.usage.clone(); }
    let protocol = args[0].clone();
    let source_port = args[1].clone();
    let dest_port = args[2].clone();
    let source_ip = args[3].clone();
    let dest_ip = args[4].clone();
    let mut iters  = match args[5].parse::<u32>(){
      Ok(o) => o,
      Err(e) => {
        print::print_custom("ipv4: error parsing int for loop iterations: \n","orange"); 
        print::println(&format!("{}",e));
        return String::from("");
      }
    };
    if iters == 0 { iters = 1 };
    let mut payload = String::new();
    for i in 6..args.len(){
      payload.push_str(&args[i]);
      payload.push_str(" ");
    }
    payload = payload.trim_end().to_string();
    let arguments;
    if protocol.eq("TCP") || protocol.eq("tcp") {
      arguments = vec!["-p".to_string(),"ipv4".to_string(),"-is".to_string(),source_ip,"-p".to_string(),"tcp".to_string(),"-ts".to_string(),source_port,"-td".to_string(),dest_port,"-d".to_string(),payload,"-v".to_string(),dest_ip];
    }
    else if protocol.eq("UDP") || protocol.eq("udp") {
      arguments = vec!["-p".to_string(),"ipv4".to_string(),"-is".to_string(),source_ip,"-p".to_string(),"udp".to_string(),"-us".to_string(),source_port,"-ud".to_string(),dest_port,"-d".to_string(),payload,"-v".to_string(),dest_ip];
    }
    else { return self.event.usage.clone(); }
		print::print_custom("--------------------------\n","purple");
    for i in 0..iters {
			print::print_custom(&format!("\tPacket: {}\n--------------------------\n",(i + 1)),"purple");
			print::print_custom_uncapped("","rose");
			match run_console_command(Command::new("sendip").args(&arguments)) {
				Ok(_) => (), 
				Err(e) => { return format!("Running ipv4 command failed: {}\n", e); }  
			};
      print::print_custom("","reset");
			print::print_custom("--------------------------\n","purple");
			let one_sec = time::Duration::from_secs(1);
			thread::sleep(one_sec);
    }
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/ipv4/atf/atf.txt";

    // Run check 
    let check = match Command::new("sendip").arg("-h").output() {
      Ok(out) => out, 
      Err(_) => { return TestStatus::Failed; }  
    };   
    
    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out, 
      Err(err) => { 
        debug::print_debug(format!("ipv4::on_test: Failed to read artifact. {}", err));
        return TestStatus::Failed;
      }    
    };   

    // Compare
    if String::from_utf8_lossy(&check.stderr) != file_str { return TestStatus::Failed; }    
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ipv4 (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Ipv4 {
    event: Event {
      name:   name,
      desc:   "Applet to send a custom ipv4 IP packet.".to_string(),
      usage:  "ipv4 [udp|tcp] <src_port> <dst_port> <src_IP> <dst_IP> <iterations> <message>\n".to_string(),
      parent: parent,
      author: "rickettm".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }    
  })
}

/*********************************** ipv6 ***********************************/
pub struct Ipv6 { event: Event }
impl Eventable for Ipv6 {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 7 { return self.event.usage.clone(); }
    let protocol = args[0].clone();
    let source_port = args[1].clone();
    let dest_port = args[2].clone();
    let source_ip = args[3].clone();
    let dest_ip = args[4].clone();
    let mut iters = match args[5].parse::<u32>(){
      Ok(o) => o,
      Err(e) => {
        print::print_custom("ipv6: error parsing int for loop iterations: \n","orange"); 
        print::println(&format!("{}",e));
        return String::from("");
      }
    };
    if iters == 0 { iters = 1 };
    let mut payload = String::new();
    for i in 6..args.len(){
      payload.push_str(&args[i]);
      payload.push_str(" ");
    }
    payload = payload.trim_end().to_string();
    let arguments;
    if protocol.eq("TCP") || protocol.eq("tcp") {
      arguments = vec!["-p".to_string(),"ipv6".to_string(),"-6s".to_string(),source_ip,"-p".to_string(),"tcp".to_string(),"-ts".to_string(),source_port,"-td".to_string(),dest_port,"-d".to_string(),payload,"-v".to_string(),dest_ip];
    }
    else if protocol.eq("UDP") || protocol.eq("udp") {
      arguments = vec!["-p".to_string(),"ipv6".to_string(),"-6s".to_string(),source_ip,"-p".to_string(),"udp".to_string(),"-us".to_string(),source_port,"-ud".to_string(),dest_port,"-d".to_string(),payload,"-v".to_string(),dest_ip];
    }
    else { return self.event.usage.clone(); }
		print::print_custom("--------------------------\n","purple");
    for i in 0..iters {
			print::print_custom(&format!("\tPacket: {}\n--------------------------\n",(i + 1)),"purple");
			print::print_custom_uncapped("","lightbluegreen");
			match run_console_command(Command::new("sendip").args(&arguments)) {
				Ok(_) => (), 
				Err(e) => { return format!("Running ipv6 command failed: {}\n", e); }  
			};
      print::print_custom("","reset");
			print::print_custom("--------------------------\n","purple");
			let one_sec = time::Duration::from_secs(1);
			thread::sleep(one_sec);
    }
    return String::from("");
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/ipv6/atf/atf.txt";

    // Run check 
    let check = match Command::new("sendip").arg("-h").output() {
      Ok(out) => out, 
      Err(_) => { return TestStatus::Failed; }  
    };   
    
    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out, 
      Err(err) => { 
        debug::print_debug(format!("ipv6::on_test: Failed to read artifact. {}", err));
        return TestStatus::Failed;
      }    
    };   

    // Compare
    if String::from_utf8_lossy(&check.stderr) != file_str { return TestStatus::Failed; }    
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ipv6 (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Ipv6 {
    event: Event {
      name:   name,
      desc:   "Applet to send a custom ipv6 packet.".to_string(),
      usage:  "ipv6 [udp|tcp] <src_port> <dst_port> <src_IP> <dst_IP> <iterations> <message>.\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }    
  })
}

/*********************************** infer ***********************************/
pub struct Infer { event: Event }
impl Eventable for Infer {
	fn on_run(&self, args: Vec<String>) -> String {
    if !(args.len() == 1) { return self.event.usage.clone(); }
    match run_console_command(Command::new("ext/infer/slm_infer").arg(args[0].clone())) {
      Ok(out) => out,
      Err(e) => {
        print::print_custom(&format!("Failed to run infer command: {}\n",e),"orange");
        return String::from("");
      }
    };
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/infer/atf/atf.txt";
		let input_path = "tst/infer/inp/test.txt";

		// Run check
		let check = match Command::new("ext/infer/slm_infer").arg(input_path).output() {
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
					debug::print_debug(format!("infer::on_test: Failed to open the test file. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stderr) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn infer(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Infer {
			event: Event {
			name:   name,
			desc:   "Applet used to identify file type based on magic.".to_string(),
			usage:  "infer <file_path>\n".to_string(),
			parent: parent,
			author: "Bojan Djurkovic".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}
 
/*********************************** Phrack Article Viewer ***********************************/
pub struct Phrack { event: Event }
impl Eventable for Phrack {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 || args.len() > 2 {
      return self.event.usage.to_string();
    }

    // Get the volume
    let volume = args[0].clone();
    let dir_path = format!("ext/phrack/phrack{}", volume);

    if !std::path::Path::new(&dir_path).exists() {
      return format!("Volume {} doesn't exist.\n", volume);
    }

    // Get the issue
    let issue =
      if args.len() == 2 {
        args[1].clone()
      } else {
        let mut paths = util::misc::get_files_in_path(dir_path);
        paths = paths.iter().map(|p| {
          match p.find('.') {
            Some(idx) => {
              let (issue_n, _) = p.split_at(idx);
              issue_n.to_string()
            }
            None => p.to_string(),
          }
        }).collect();
        match terminal::get_selection(paths.clone()) {
          Some(id) => paths[id].clone(),
          None => {
            return String::from("No issue selected.\n");
          }
        }
      };
    let file_path = format!("ext/phrack/phrack{}/{}.txt", volume, issue);

    if !std::path::Path::new(&file_path).exists() {
      return format!("Volume {} Issue {} doesn't exist.\n", volume, issue);
    }

    // Print out the article
    return match std::fs::read_to_string(file_path) {
      Ok(out) => out,
      Err(err) => {
        format!("Failed to read the phrack article: {}\n", err)
      }
    };
  }
  fn on_test(&self) -> TestStatus {
    let actual = simple_test_match!(fs::read("ext/phrack/phrack1/1.txt"));

    // Load the file into a string
    let expected = simple_test_match!(fs::read("tst/phrack/atf/1.txt"));

    if actual.ne(&expected) {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn phrack(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Phrack {
    event: Event {
      name:   name,
      desc:   "Applet to retrieve and read Phrack hacker magazine articles.".to_string(),
      usage:  "Requires:\n\
                \tVolume number (decimal integer)\n\
                \tIssue number (decimal integer)\n".to_string(),
      parent: parent,
      author: "r00r00 and staff@phrack.org".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }
  })
}
 
/*********************************** Sallie ***********************************/
pub struct Sallie { event: Event }
impl Eventable for Sallie {
  fn on_run(&self, _args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(
      run_console_command(
        Command::new("ext/util/sallie/eliza")
          .arg("ext/util/sallie/scripts/slm.json")
      )
    );

    log::log("Sallie", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
		// Run check
		let check = match Command::new("ext/util/sallie/eliza").arg("__test__").output() {
			Ok(o) => o,
				Err(err) => {
					debug::print_debug(format!("Error: {}", err));
					return TestStatus::Failed;
				}
		};
		// Compare
		if String::from_utf8_lossy(&check.stdout) != "test successful\n" { return TestStatus::Failed; }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn sallie(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Sallie {
    event: Event {
      name:   name,
      desc:   "Sallie artificial assistant for cybersecurity.".to_string(),
      usage:  "Just type Sallie and she shall appear.\n".to_string(),
      author: "r00r00 and $t@$h".to_string(),
      parent: parent,
      easyrun: true,
      secure: false,
      links:  links
    }
  })
}

/*********************************** chardump ***********************************/
pub struct Chardump { event: Event }
impl Eventable for Chardump {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() > 0 { return self.event.usage.clone(); }
    let output = match run_console_command(&mut Command::new("ext/chardump/chd")) {
      Ok(out) => out,
      Err(e) => { return format!("Running chardump command failed: {}\n", e); }
    };
    // Print output
    log::log("chardump", &String::from_utf8_lossy(&output.stderr));
    if main_info::is_bg() {
      print!("Chardump doesn't support running in the background.");
      return String::from("Chardump doesn't support running in the background.\n"); }
    else { return String::from_utf8_lossy(&output.stdout).to_string(); }
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/chardump/atf/atf.txt";

    // Run check 
    let check = match run_bounded_command(&mut Command::new("ext/chardump/chd"),false,1) {
      Ok(out) => out, 
      Err(_) => { return TestStatus::Failed; }  
    };   
    
    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out, 
      Err(err) => { 
        debug::print_debug(format!("chardump::on_test: Failed to read artifact. {}", err));
        return TestStatus::Failed;
      }    
    };   

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }    
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn chardump (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Chardump {
    event: Event {
      name:   name,
      desc:   "Applet that dumps a string character by character in different radixes.".to_string(),
      usage:  "Takes no args. Just invoke chardump.\n".to_string(),
      parent: parent,
      author: "Copyright 2004,2005,2006 Luigi Auriemma".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }    
  })
}

/*********************************** QBUILD ***********************************/
pub struct QBuild { event: Event }
impl Eventable for QBuild {
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() < 3 || args.len() > 4 {
      return self.event.usage.to_string();
    }

    // Convert relative path to absolute
    match std::env::current_dir() {
      Ok(mut d) => {
        d.push(args[2].clone());
        args[2] = match d.into_os_string().into_string() {
          Ok(s) => s,
          Err(_) => args[2].clone(),
        };
      }
      Err(_) => {}
    };
		use std::fs::metadata;
		let md = match metadata(args[2].clone()) {
			Ok(o) => o, 
		 Err(e) => { return format!("Failed getting directory metadata: {}\n", e); }
		};
		if !md.is_dir() {
      print::print_custom("Qbuild needs a directory for src_path, not a file.\n","orange");
      return String::from("");
    }

    // Run command
    simple_match!(Command::new("rm").args(vec!["-rf","out/qbuild/bin"]).output());
    simple_match!(Command::new("rm").args(vec!["-rf","out/qbuild/build"]).output());
    let output = simple_match!(run_command(Command::new("./qbuild").args(args.clone()).current_dir("ext/qbuild")));
    simple_match!(Command::new("mv").args(vec!["ext/qbuild/bin", "out/qbuild"]).output());
    simple_match!(Command::new("mv").args(vec!["ext/qbuild/build", "out/qbuild"]).output());
    simple_match!(Command::new("unlink").arg(&format!("ext/qbuild/{}", args[0])).output());
    //util::misc::cleanup(&format!("ext/qbuild/{}", args[1]));
    print::println("You can find your build at ./out/qbuild");

    log::log("QBuild", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let atf_path = "tst/qbuild/atf/atf.txt";

    // Run command
    let output = simple_test_match!(Command::new("./ext/qbuild/qbuild").args(vec!["-h"]).output());

    // Read file
    let atf_string = simple_test_match!(fs::read_to_string(atf_path));

    if String::from_utf8_lossy(&output.stdout).to_string() != atf_string {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn qbuild(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(QBuild {
    event: Event {
      name:   name,
      desc:   "Will attempt to compile your C project using a generic Makefile.".to_string(),
      usage:  "Usage: qbuild <output_binary_name> [gcc|clang] <src_path> [library_path]\n\
               Compilers: gcc or clang\n".to_string(),
      parent: parent,
      author: "Michael Crawford & m0nZSt3r".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }
  })
}

/*********************************** QBUILDPLUS ***********************************/
pub struct QBuildPlus { event: Event }
impl Eventable for QBuildPlus {
  fn on_run(&self, mut args: Vec<String>) -> String {
    if args.len() < 3 || args.len() > 4 {
      return self.event.usage.to_string();
    }

    // Convert relative path to absolute
    match std::env::current_dir() {
      Ok(mut d) => {
        d.push(args[2].clone());
        args[2] = match d.into_os_string().into_string() {
          Ok(s) => s,
          Err(_) => args[2].clone(),
        };
      }
      Err(_) => {}
    };
		use std::fs::metadata;
		let md = match metadata(args[2].clone()) {
			Ok(o) => o, 
		 Err(e) => { return format!("Failed getting directory metadata: {}\n", e); }
		};
		if !md.is_dir() {
      print::print_custom("Qbuildplus needs a directory for src_path, not a file.\n","orange");
      return String::from("");
    }

    // Run command
    simple_match!(Command::new("rm").args(vec!["-rf","out/qbuildplus/bin"]).output());
    simple_match!(Command::new("rm").args(vec!["-rf","out/qbuildplus/build"]).output());
    let output = simple_match!(run_command(Command::new("./qbuildplus").args(args.clone()).current_dir("ext/qbuildplus")));
    simple_match!(Command::new("mv").args(vec!["ext/qbuildplus/bin", "out/qbuildplus"]).output());
    simple_match!(Command::new("mv").args(vec!["ext/qbuildplus/build", "out/qbuildplus"]).output());
    simple_match!(Command::new("unlink").arg(&format!("ext/qbuildplus/{}", args[0])).output());
    //util::misc::cleanup(&format!("ext/qbuildplus/{}", args[1]));
    print::println("You can find your build at ./out/qbuildplus");

    log::log("QBuildPlus", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let atf_path = "tst/qbuildplus/atf/atf.txt";

    // Run command
    let output = simple_test_match!(Command::new("./ext/qbuildplus/qbuildplus").args(vec!["-h"]).output());

    // Read file
    let atf_string = simple_test_match!(fs::read_to_string(atf_path));

    if String::from_utf8_lossy(&output.stdout).to_string() != atf_string {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn qbuildplus(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(QBuildPlus {
    event: Event {
      name:   name,
      desc:   "Will attempt to compile your C++ project using a generic Makefile.".to_string(),
      usage:  "Usage: qbuildplus <binary_name> [g++|clang++] <src_path> [library_path]\n\
               Compilers: g++ or clang++\n".to_string(),
      parent: parent,
      author: "Michael Crawford & Matzr3lla".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }
  })
}

/*********************************** tagstr ***********************************/
pub struct Tagstr { event: Event }
impl Eventable for Tagstr {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 { return self.event.usage.clone(); }
    let output = match run_console_command(&mut Command::new("ext/tagstr/colorizer").args(vec!["--email","ORANGE","--ipv4","RED",&args[0]])) {
      Ok(out) => out,
      Err(e) => { return format!("Running tagstr command failed: {}\n", e); }
    };
    // Print output
    log::log("tagstr", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/tagstr/atf/atf.txt";
    let input_path = "tst/tagstr/inp/test.txt";
    // Run check 
    let check = match Command::new("ext/tagstr/colorizer").args(vec!["--email","ORANGE","--ipv4","RED",input_path]).output() {
      Ok(out) => out, 
      Err(err) => {
        debug::print_debug(format!("tagstr::on_test: Failed to run test command. {}", err));
        return TestStatus::Failed;
      }  
    };   
    
    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out, 
      Err(err) => { 
        debug::print_debug(format!("tagstr::on_test: Failed to read artifact. {}", err));
        return TestStatus::Failed;
      }    
    };   

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }    
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn tagstr (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Tagstr {
    event: Event {
      name:   name,
      desc:   "Applet that prints email addresses and IP addresses in a file.".to_string(),
      usage:  "tagstr <file_to_analyze>\n".to_string(),
      parent: parent,
      author: "$t@$h".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }    
  })
}

/*********************************** signsig ***********************************/
pub struct Signsig { event: Event }
impl Eventable for Signsig {
  fn on_run(&self, args: Vec<String>) -> String {
    if args.len() != 1 { return self.event.usage.clone(); }
    let output = match run_console_command(&mut Command::new("ext/signsig/signsrch").arg(&args[0])) {
      Ok(out) => out,
      Err(e) => { return format!("Running signsig command failed: {}\n", e); }
    };
    // Print output
    log::log("signsig", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/signsig/atf/atf.txt";

    // Run check 
    let check = match Command::new("ext/signsig/signsrch").arg("-h").output() {
      Ok(out) => out, 
      Err(err) => {
        debug::print_debug(format!("signsig::on_test: Failed to run test command. {}", err));
        return TestStatus::Failed;
      }  
    };   
    
    // Read file
    let file_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out, 
      Err(err) => { 
        debug::print_debug(format!("signsig::on_test: Failed to read artifact. {}", err));
        return TestStatus::Failed;
      }    
    };   

    // Compare
    if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }    
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn signsig (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Signsig {
    event: Event {
      name:   name,
      desc:   "Applet that analyzes files and prints any signatures found.".to_string(),
      usage:  "signsig <file_to_analyze>\n".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: true,
      secure: false,
      links:  links
    }    
  })
}

/*********************************** dojo ***********************************/
pub struct Dojo { event: Event }
impl Eventable for Dojo {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() > 0 { 
      return self.event.usage.clone(); 
    }
		if main_info::is_bg() {
			print!("The Salvum Dojo does not support the alternate screen.");
			return String::from("Salvum Dojo can't be run in the background.\n");
		}
    let output = match run_console_command(&mut Command::new("ext/dojo/slm_dojo")) {
			Ok(out) => out,
			Err(e) => {
        print::print_custom(&format!("{}",e),"orange");
        return String::from("");
      }  
		};
		// Print output
    let out = String::from_utf8_lossy(&output.stdout);
		print::print_custom(&out,"grey");
    log::log("dojo", &String::from_utf8_lossy(&output.stderr));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact1_path = "tst/dojo/atf/atf.txt";
		let artifact2_path = "tst/dojo/atf/atf2.txt";
		let artifact3_path = "tst/dojo/atf/atf3.txt";
		let input_path = "tst/dojo/inp/test.c";

		// Run check
		let check = match Command::new("ext/dojo/cinterp").arg(input_path).output() {
			Ok(o) => o,
				Err(err) => {
					debug::print_debug(format!("dojo_cinterp::on_test: Error running test1. {}", err));
					return TestStatus::Failed;
				}
		};

		// Read file
		let file_str = match fs::read_to_string(artifact1_path) {
			Ok(out) => out,
				Err(err) => {
					debug::print_debug(format!("dojo_cinterp::on_test: Failed to open artifact 1. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare 1
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }

		// Run check
		let check2 = match Command::new("ext/dojo/slm_dojo").arg("__TEST__").output() {
			Ok(o) => o,
				Err(err) => {
					debug::print_debug(format!("dojo_core::on_test: Error running test2: {}", err));
					return TestStatus::Failed;
				}
		};

		// Read file
		let file_str2 = match fs::read_to_string(artifact2_path) {
			Ok(out) => out,
				Err(err) => {
					debug::print_debug(format!("dojo_core::on_test: Failed to open artifact 2. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare 2
		if String::from_utf8_lossy(&check2.stdout) != file_str2 { return TestStatus::Failed; }
    
		// Run check
		let check3 = match Command::new("ext/dojo/slm_dojo").arg("__TEST__").output() {
			Ok(o) => o,
				Err(err) => {
					debug::print_debug(format!("dojo_type::on_test: Error running test3: {}", err));
					return TestStatus::Failed;
				}
		};

		// Read file
		let file_str3 = match fs::read_to_string(artifact3_path) {
			Ok(out) => out,
				Err(err) => {
					debug::print_debug(format!("dojo_type::on_test: Failed to open artifact 3. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare 3
		if String::from_utf8_lossy(&check3.stdout) != file_str3 { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn dojo(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Dojo {
			event: Event {
			name:   name,
			desc:   "The Salvum cybersecurity training dojo.".to_string(),
			usage:  "Takes no arguments, just invoke.\n".to_string(),
			parent: parent,
			author: "Copyright (c) 2009-2011, Zik Saleeba\nCopyright (c) 2015, Joseph Poirier\nAll rights reserved.\ngeri1701 (geri@sdf.org)\nand $t@$h".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** lnxmods ***********************************/
pub struct Lnxmods { event: Event }
impl Eventable for Lnxmods {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() > 2 || args.len() < 1 { return self.event.usage.clone(); }
    if args.len() == 2 {
      if args[0].eq("add") {
				let output = match run_console_command(&mut Command::new("ext/lnxmods/nms").args(vec!["insert", &args[1]])) {
					Ok(out) => out,
					Err(e) => { return format!("Running nms command failed: {}\n", e); }
				};
		    log::log("lnxmods", &String::from_utf8_lossy(&output.stderr));
		    return String::from_utf8_lossy(&output.stdout).to_string();
      }
      else if args[0].eq("remove") || args[0].eq("rem") {
				let output = match run_console_command(&mut Command::new("ext/lnxmods/nms").args(vec!["remove", &args[1]])) {
					Ok(out) => out,
					Err(e) => { return format!("Running nms command failed: {}\n", e); }
				};
		    log::log("nms", &String::from_utf8_lossy(&output.stderr));
		    return String::from_utf8_lossy(&output.stdout).to_string();
      }
      else { return self.event.usage.clone(); }
    }
    else if args.len() == 1 {
      if args[0].eq("list") {
				let output = match run_console_command(&mut Command::new("ext/lnxmods/nms").arg("list")) {
					Ok(out) => out,
					Err(e) => { return format!("Running nms command failed: {}\n", e); }
				};
		    log::log("lnxmods", &String::from_utf8_lossy(&output.stderr));
		    return String::from_utf8_lossy(&output.stdout).to_string();
      } 
    }
		return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/lnxmods/atf/atf.txt";

		// Run check 
		let check = match Command::new("ext/lnxmods/nms").arg("--help").output() {
			Ok(out) => out, 
			Err(err) => {
				debug::print_debug(format!("lnxmods::on_test: Failed to run test command. {}", err));
				return TestStatus::Failed;
			}  
		};   
		
		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out, 
			Err(err) => { 
				debug::print_debug(format!("lnxmods::on_test: Failed to read artifact. {}", err));
				return TestStatus::Failed;
			}    
		};   

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }    
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn lnxmods (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Lnxmods {
		event: Event {
			name:   name,
			desc:   "Applet that lists, adds, and removes Linux kernel modules.".to_string(),
			usage:  "lnxmods list\nlnxmods add <module_path>\nlnxmods rem <module_name>\n".to_string(),
			parent: parent,
			author: "".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}    
	})
}

/*********************************** fdtdump ***********************************/
pub struct Fdtdump { event: Event }
impl Eventable for Fdtdump {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() != 1 { return self.event.usage.clone(); }
    if !Path::new(&args[0]).exists() {
      print::print_custom("Path to .dtb doesn't exist. Aborting.\n","orange");
      return String::from("");
    }
		let output = match run_console_command(&mut Command::new("ext/fdtdump/fdtdump").arg(&args[0])) {
			Ok(out) => out,
			Err(e) => { return format!("Running fdtdump command failed: {}\n", e); }
		};
		log::log("fdtdump", &String::from_utf8_lossy(&output.stderr));
		return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/fdtdump/atf/atf.txt";
    let input_path = "tst/fdtdump/inp/test.dtb";

		// Run check 
		let check = match Command::new("ext/fdtdump/fdtdump").arg(input_path).output() {
			Ok(out) => out, 
			Err(err) => {
				debug::print_debug(format!("nms::on_test: Failed to run test command. {}", err));
				return TestStatus::Failed;
			}  
		};   
		
		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out, 
			Err(err) => { 
				debug::print_debug(format!("nms::on_test: Failed to read artifact. {}", err));
				return TestStatus::Failed;
			}    
		};   

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }    
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn fdtdump (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Fdtdump {
		event: Event {
			name:   name,
			desc:   "Applet that reverses a device tree blob (.dtb) file.".to_string(),
			usage:  "fdtdump <path_to_dtb>\n".to_string(),
			parent: parent,
			author: "Copyright (c) 2020 Sean Wilson".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}    
	})
}

/*********************************** tabtospc ***********************************/
pub struct Tabtospc { event: Event }
impl Eventable for Tabtospc {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() != 3 { return self.event.usage.clone(); }
    if !Path::new(&args[1]).exists() {
      print::print_custom("Path to source text file doesn't exist. Aborting.\n","orange");
      return String::from("");
    }
    let num = match args[0].clone().parse::<i32>() {
      Ok(o) => o,
     Err(_) => return String::from("tabtospc: parsing int failed.\n"),
    };
    if num < 0 { return String::from("tabtospc: negative argument not allowed\n"); }
    let mut in_path = args[1].clone();
    let mut out_path = args[2].clone();
    util::misc::reltoabs(&mut in_path);
    util::misc::reltoabs(&mut out_path);
		let output = match run_console_command(&mut Command::new("ext/tabtospc/tab2spc").args(vec![&args[0],&in_path,&out_path])) {
			Ok(out) => out,
			Err(e) => { return format!("Running tabtospc command failed: {}\n", e); }
		};
		log::log("tabtospc", &String::from_utf8_lossy(&output.stderr));
		return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/tabtospc/atf/atf.txt";

		// Run check 
		let check = match Command::new("ext/tabtospc/tab2spc").arg("").output() {
			Ok(out) => out, 
			Err(err) => {
				debug::print_debug(format!("tabtospc::on_test: Failed to run test command. {}", err));
				return TestStatus::Failed;
			}  
		};   
		
		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out, 
			Err(err) => { 
				debug::print_debug(format!("tabtospc::on_test: Failed to read artifact. {}", err));
				return TestStatus::Failed;
			}    
		};   

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }    
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn tabtospc (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Tabtospc {
		event: Event {
			name:   name,
			desc:   "Applet that replaces tabs in a file to spaces.".to_string(),
			usage:  "tabtospc <num_of_spaces> <path_to_in_file> <path_to_out_file>\n".to_string(),
			parent: parent,
			author: "Copyright 2004,2005,2006 Luigi Auriemma".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}    
	})
}

/*********************************** rmchar ***********************************/
pub struct Rmchar { event: Event }
impl Eventable for Rmchar {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() != 3 { return self.event.usage.clone(); }
    if !Path::new(&args[1]).exists() {
      print::print_custom("Path to source file doesn't exist. Aborting.\n","orange");
      return String::from("");
    }
    let mut in_path = args[1].clone();
    let mut out_path = args[2].clone();
    util::misc::reltoabs(&mut in_path);
    util::misc::reltoabs(&mut out_path);
		let output = match run_console_command(&mut Command::new("ext/rmchar/rmchar").args(vec![&in_path,&out_path,&args[0]])) {
			Ok(out) => out,
			Err(e) => { return format!("Running tabtospc command failed: {}\n", e); }
		};
		log::log("rmchar", &String::from_utf8_lossy(&output.stderr));
		return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/rmchar/atf/atf.txt";

		// Run check 
		let check = match Command::new("ext/rmchar/rmchar").arg("").output() {
			Ok(out) => out, 
			Err(err) => {
				debug::print_debug(format!("rmchar::on_test: Failed to run test command. {}", err));
				return TestStatus::Failed;
			}  
		};   
		
		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out, 
			Err(err) => { 
				debug::print_debug(format!("rmchar::on_test: Failed to read artifact. {}", err));
				return TestStatus::Failed;
			}    
		};   

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }    
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn rmchar (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Rmchar {
		event: Event {
			name:   name,
			desc:   "Applet that removes character occurences from a file.".to_string(),
			usage:  "rmchar <hex_byte_of_char_to_delete> <path_to_in_file> <path_to_out_file>\n".to_string(),
			parent: parent,
			author: "Copyright 2004,2005,2006 Luigi Auriemma".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}    
	})
}

/*********************************** binstitch ***********************************/
pub struct Binstitch { event: Event }
impl Eventable for Binstitch {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() != 3 { return self.event.usage.clone(); }
    if !Path::new(&args[0]).exists() {
      print::print_custom("Path to source binary 1 doesn't exist. Aborting.\n","orange");
      return String::from("");
    }
    if !Path::new(&args[1]).exists() {
      print::print_custom("Path to source binary 2 doesn't exist. Aborting.\n","orange");
      return String::from("");
    }
    let mut in_path1 = args[0].clone();
    let mut in_path2 = args[1].clone();
    let mut out_path = args[2].clone(); 
    util::misc::reltoabs(&mut in_path1);
    util::misc::reltoabs(&mut in_path2);
    util::misc::reltoabs(&mut out_path);
    
		let output = match run_console_command(&mut Command::new("ext/binstitch/bincat").args(vec![&in_path1, &in_path2, &out_path])) {
			Ok(out) => out,
			Err(e) => { return format!("Running binstitch command failed: {}\n", e); }
		};
		log::log("binstitch", &String::from_utf8_lossy(&output.stderr));
		return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/binstitch/atf/atf.txt";

		// Run check 
		let check = match Command::new("ext/binstitch/bincat").arg("").output() {
			Ok(out) => out, 
			Err(err) => {
				debug::print_debug(format!("binstitch::on_test: Failed to run test command. {}", err));
				return TestStatus::Failed;
			}  
		};   
		
		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out, 
			Err(err) => { 
				debug::print_debug(format!("binstitch::on_test: Failed to read artifact. {}", err));
				return TestStatus::Failed;
			}    
		};   

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }    
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn binstitch (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Binstitch {
		event: Event {
			name:   name,
			desc:   "Applet that stitches two binaries together into a third binary.".to_string(),
			usage:  "binstitch <path_to_in_binary1> <path_to_in_binary2> <path_to_out_binary>\n".to_string(),
			parent: parent,
			author: "Copyright 2004,2005,2006 Luigi Auriemma".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}    
	})
}

/*********************************** bindiff ***********************************/
pub struct Bindiff { event: Event }
impl Eventable for Bindiff {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() != 2 { return self.event.usage.clone(); }
    if !Path::new(&args[0]).exists() {
      print::print_custom("Path to source binary 1 doesn't exist. Aborting.\n","orange");
      return String::from("");
    }
    if !Path::new(&args[1]).exists() {
      print::print_custom("Path to source binary 2 doesn't exist. Aborting.\n","orange");
      return String::from("");
    }
    let mut in_path1 = args[0].clone();
    let mut in_path2 = args[1].clone();
    util::misc::reltoabs(&mut in_path1);
    util::misc::reltoabs(&mut in_path2);
    
		let output = match run_console_command(&mut Command::new("ext/bindiff/fcomp").args(vec![&in_path1, &in_path2])) {
			Ok(out) => out,
			Err(e) => { return format!("Running bindiff command failed: {}\n", e); }
		};
		log::log("bindiff", &String::from_utf8_lossy(&output.stderr));
		return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/bindiff/atf/atf.txt";

		// Run check 
		let check = match Command::new("ext/bindiff/fcomp").arg("").output() {
			Ok(out) => out, 
			Err(err) => {
				debug::print_debug(format!("bindiff::on_test: Failed to run test command. {}", err));
				return TestStatus::Failed;
			}  
		};   
		
		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out, 
			Err(err) => { 
				debug::print_debug(format!("bindiff::on_test: Failed to read artifact. {}", err));
				return TestStatus::Failed;
			}    
		};   

		// Compare
		if String::from_utf8_lossy(&check.stderr) != file_str { return TestStatus::Failed; }    
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn bindiff (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Bindiff {
		event: Event {
			name:   name,
			desc:   "Applet that reveals differences between binaries.".to_string(),
			usage:  "bindiff <path_to_binary1> <path_to_binary2>\n".to_string(),
			parent: parent,
			author: "Copyright 2004,2005,2006 Luigi Auriemma".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}    
	})
}

/*********************************** filestitch ***********************************/
pub struct Filestitch { event: Event }
impl Eventable for Filestitch {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() != 2 { return self.event.usage.clone(); }
    if !Path::new(&args[0]).exists() {
      print::print_custom("Path to source directory doesn't exist. Aborting.\n","orange");
      return String::from("");
    }
    let mut in_path1 = args[0].clone();
    let mut in_path2 = args[1].clone();
    util::misc::reltoabs(&mut in_path1);
    util::misc::reltoabs(&mut in_path2);
    
		let output = match run_console_command(&mut Command::new("ext/filestitch/concatenate").args(vec![&in_path1, &in_path2])) {
			Ok(out) => out,
			Err(e) => { return format!("Running filestitch command failed: {}\n", e); }
		};
    print::print_custom("Files were combined successfully.\n","bluegreen");
		log::log("filestitch", &String::from_utf8_lossy(&output.stderr));
		return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/filestitch/atf/atf.txt";

		// Run check 
		let check = match Command::new("ext/filestitch/concatenate").arg("--help").output() {
			Ok(out) => out, 
			Err(err) => {
				debug::print_debug(format!("filestitch::on_test: Failed to run test command. {}", err));
				return TestStatus::Failed;
			}  
		};   
		
		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out, 
			Err(err) => { 
				debug::print_debug(format!("filestitch::on_test: Failed to read artifact. {}", err));
				return TestStatus::Failed;
			}    
		};   

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }    
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn filestitch (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Filestitch {
		event: Event {
			name:   name,
			desc:   "Applet that combines all files in a directory into a single file.".to_string(),
			usage:  "filestitch <path_to_source_dir> <path_to_out_file>\n".to_string(),
			parent: parent,
			author: "Copyright (c) 2018 Natu Lauchande".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}    
	})
}

/*********************************** ascii ***********************************/
pub struct Ascii { event: Event }
impl Eventable for Ascii {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() != 0 { return self.event.usage.clone(); }
     
		let output = match run_console_command(&mut Command::new("ext/ascii/ascii").args(vec!["-l","1"])) {
			Ok(out) => out,
			Err(e) => { return format!("Running ascii command failed: {}\n", e); }
		};
		log::log("ascii", &String::from_utf8_lossy(&output.stderr));
		return String::from_utf8_lossy(&output.stdout).to_string();
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/ascii/atf/atf.txt";

		// Run check 
		let check = match Command::new("ext/ascii/ascii").arg("--help").output() {
			Ok(out) => out, 
			Err(err) => {
				debug::print_debug(format!("ascii::on_test: Failed to run test command. {}", err));
				return TestStatus::Failed;
			}  
		};   
		
		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out, 
			Err(err) => { 
				debug::print_debug(format!("ascii::on_test: Failed to read artifact. {}", err));
				return TestStatus::Failed;
			}    
		};   

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }    
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ascii (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Ascii {
		event: Event {
			name:   name,
			desc:   "Applet that prints the ASCII table to the console.".to_string(),
			usage:  "Takes no args, simply invoke.\n".to_string(),
			parent: parent,
			author: "Copyright (c) 2021 Marek Barvíř".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}    
	})
}

/*********************************** dd ***********************************/
pub struct Dd { event: Event }
impl Eventable for Dd {
	fn on_run(&self, args: Vec<String>) -> String {
		if args.len() != 4 { return self.event.usage.clone(); }
    let start = args[0].clone();
    let count = args[1].clone();
    let block = args[2].clone();
    let mut file = args[3].clone();
    util::misc::reltoabs(&mut file);
    if !Path::new(&file.replace("./","")).exists() {
      print::print_custom("Input file does not appear to exist.\n","orange");
      return String::from("");
    }
    let mut arg1 = String::from("skip=");
    arg1.push_str(&start);
    let mut arg2 = String::from("count=");
    arg2.push_str(&count);
    let mut arg3 = String::from("if=");
    arg3.push_str(&file.replace("./",""));
    let mut arg4 = String::from("of=out/dd/");
    arg4.push_str("slice_");
    arg4.push_str(&start);
    arg4.push_str("_");
    arg4.push_str(&count);
    let mut arg5 = String::from("bs=");
    arg5.push_str(&block);
		let output = match run_console_command(&mut Command::new("ext/util/coreutils").args(vec!["dd".to_string(),arg1,arg2,arg3,arg4,arg5])) {
			Ok(out) => out,
			Err(e) => { return format!("Running dd command failed: {}\n", e); }
		};
		log::log("dd", &String::from_utf8_lossy(&output.stderr));
		return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/dd/atf/atf.txt";

		// Run check 
		let check = match Command::new("ext/util/coreutils").args(vec!["dd","--version"]).output() {
			Ok(out) => out, 
			Err(err) => {
				debug::print_debug(format!("dd::on_test: Failed to run test command. {}", err));
				return TestStatus::Failed;
			}  
		};   
		
		// Read file
		let file_str = match fs::read_to_string(artifact_path) {
			Ok(out) => out, 
			Err(err) => { 
				debug::print_debug(format!("dd::on_test: Failed to read artifact. {}", err));
				return TestStatus::Failed;
			}    
		};   

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }    
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn dd (links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Dd {
		event: Event {
			name:   name,
			desc:   "Applet that extracts a slice of a file and writes to a new file.".to_string(),
			usage:  "dd <starting_offset> <count_to_extract> <input_file_path> <block_size>\n".to_string(),
			parent: parent,
			author: "Rust Coreutils".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}    
	})
}

/*********************************** run ***********************************/
pub struct Run { event: Event }
impl Eventable for Run {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() < 1{ 
      return self.event.usage.clone(); 
    }
    if args[0].eq("-h") {
      return self.event.usage.clone(); 
    }
    if args[0].eq("cd") {
      print::print_custom("Restricted command. Run declined.\n","orange");
      return String::from(""); 
    }
    if args.len() > 2 {
			if args[0].eq("rm") && args[1].eq("-rf") && args[2].eq("/") || args[2].eq("/*") {
				print::print_custom("Restricted command. Run declined.\n","orange");
				return String::from("");  
			}
    }
    if args.len() > 1 {
			if args[0].eq("mv") && args[1].contains("/") {
				print::print_custom("Restricted command. Run declined.\n","orange");
				return String::from("");  
			}
			for arg in 0..args.len() {
				if args[arg].eq(">") && args[arg + 1].contains("/dev"){
					print::print_custom("Restricted command. Run declined.\n","orange");
					return String::from("");  
				}
				if args[0].eq("dd") && args[arg].contains("of") && args[arg].contains("/dev") {
					print::print_custom("Restricted command. Run declined.\n","orange");
					return String::from("");  
				}
				if args[0].contains("mkfs") && args[arg].contains("/dev") {
					print::print_custom("Restricted command. Run declined.\n","orange");
					return String::from("");  
				}
			}
    }
		alerts::print_generic_warning("The Salvum \'run\' command is written in Safe Rust but\n\
										allows for execution of non-validated executables outside of\n\
										the Salvum environment. These aren't gauranteed to be secure.\n");
	  let last_chance = String::from("Are you sure you want to run this executable? ");
    if alerts::confirm_task(&last_chance) == constants::UNCONFIRMED {
      print::print_custom("Operation cancelled.\n","orange");
      return String::from("");
    }
    simple_match!(run_console_command(Command::new("ext/run/env").args(args.clone())));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/run/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/run/env").arg("-h").output() {
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
					debug::print_debug(format!("run::on_test: Failed to open the artifact. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn run(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Run {
			event: Event {
			name:   name,
			desc:   "Executable runner written in Rust.".to_string(),
			usage:  "run <Linux_command_or_path_to_executable> [args.....]\n\nTakes as many arguments as you need to give it.\n".to_string(),
			parent: parent,
			author: "DoumanAsh && $t@$h".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** games ***********************************/
pub struct Games { event: Event }
impl Eventable for Games {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() > 0 { 
      return self.event.usage.clone(); 
    }
    simple_match!(run_console_command(&mut Command::new("ext/games/game_runner")));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/games/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/games/game_runner").arg("__TEST__").output() {
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
					debug::print_debug(format!("games::on_test: Failed to open the artifact. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn games(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Games {
			event: Event {
			name:   name,
			desc:   "The Salvum game chest.".to_string(),
			usage:  "Takes no arguments, simply invoke.\n".to_string(),
			parent: parent,
			author: "$t@$h && dear game developers".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** strace ***********************************/
pub struct Strace { event: Event }
impl Eventable for Strace {
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() != 1 { 
      return self.event.usage.clone(); 
    }
    simple_match!(run_console_command(&mut Command::new("ext/strace/strace_clr").args(vec!["-i".to_string(), args[0].clone()])));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/strace/atf/atf.txt";

		// Run check
		let check = match Command::new("ext/strace/strace_clr").arg("--version").output() {
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
					debug::print_debug(format!("strace::on_test: Failed to open the artifact. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout) != file_str { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn strace(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Strace {
			event: Event {
			name:   name,
			desc:   "Dumps Linux system calls.".to_string(),
			usage:  "Takes one argument:\n\tLinux command name or path to binary.\n".to_string(),
			parent: parent,
			author: "$t@$h".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}

/*********************************** uftrace ***********************************/
pub struct Uftrace { event: Event }
impl Eventable for Uftrace {
	fn on_run(&self, args: Vec<String>) -> String {
    if args.len() < 1 {
			simple_match!(run_console_command(&mut Command::new("uftrace").arg("--help")));
			return String::from("");
    }
    simple_match!(run_console_command(&mut Command::new("uftrace").args(args.clone())));
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
		let artifact_path = "tst/uftrace/atf/atf.txt";

		// Run check
		let check = match Command::new("uftrace").arg("tst/uftrace/inp/a.out").output() {
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
					debug::print_debug(format!("uftrace::on_test: Failed to open the artifact. {}", err));
					return TestStatus::Failed;
				}
		};

		// Compare
		if String::from_utf8_lossy(&check.stdout).contains(&file_str) { return TestStatus::Failed; }
		return TestStatus::Passed;
	}
	fn get_event(&self) -> &Event { return &self.event; }
}
pub fn uftrace(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
	Box::new(Uftrace {
			event: Event {
			name:   name,
			desc:   "Dumps Linux library calls.".to_string(),
			usage:  "uftrace [flag] <path>\n".to_string(),
			parent: parent,
			author: "Namhyung Kim".to_string(),
			easyrun: true,
			secure: false,
			links:  links
		}
	})
}
