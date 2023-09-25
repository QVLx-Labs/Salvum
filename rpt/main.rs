/*
 * QVLx Salvum Reporting Engine
 *
 * main.rs -> orchestrating program for the Salvum Reporting Engine
 *
 * authors: r00r00
 */

use std::io::Write;

thread_local!(pub static FILE_NAME: std::cell::RefCell<String> = std::cell::RefCell::new(String::new()));

/******************** YOUR REPORTS GO HERE  ********************/

// You can put your code directly in the report function
fn report() {
  my_custom_report();
}

// Or create new functions to easily switch between
fn my_custom_report() {
  let mut modules = Vec::new();
  //             Module Name       Args
  modules.push(("mersennetwister", vec!["1", "64"]));
  modules.push(("mersennetwister", vec!["2", "64"]));
  modules.push(("mersennetwister", vec!["3", "64"]));
  modules.push(("mersennetwister", vec!["4", "64"]));
  modules.push(("mersennetwister", vec!["5", "64"]));

  // Execute all modules and collect results
  let results = run_modules(modules.clone());

  // Write results to a file
  for result in results {
    write_string(format!("-----{}", result));
  }

  println!("Token ({},{}): {}", 1, 0, grab_token(1, 0));
}

/******************** YOUR REPORTS END HERE ********************/

/*
 * run_modules
 * 
 * @brief This will execute all modules in a given collection and return each result
 * @param modules: A pair of the module name and its args for execution
 * @return a collection of all the results
 */
#[allow(dead_code)]
fn run_modules(modules: Vec<(&str, Vec<&str>)>) -> Vec<String> {
  let mut results = Vec::new();
  
  // Go through each individual module
  for (module, mut args) in modules {
    args.insert(0, module); // For the Salvum-CLI we need the module name as the first arg

    // Add the output to the results vector
    results.push(match std::process::Command::new("./target/debug/salvum").args(args).current_dir("..").output() {
      // Normal execution
      Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
      // Error when executing
      Err(_) => {
        println!("Problem executing {}.", module);
        String::from("Error")
      }
    });
  }

  return results;
}

/*
 * grab_token
 * 
 * @brief Uses line and token numbers as an index for grabbing a word
 * @param line_number: What line to pull from
 * @param token_number: What token to pull from
 * @return the word that is at location (l,t)
 */
#[allow(dead_code)]
fn grab_token(line_number: usize, token_number: usize) -> String {
  // Read file
  let content = match std::fs::read_to_string(get_file_name()) {
    Ok(out) => out,
    Err(err) => {
      println!("Can't open \"{}\": {}", get_file_name(), err);
      return String::from("");
    }
  };

  let stripped_content: String = String::from_utf8_lossy(&strip_ansi_escapes::strip(content).unwrap()).to_string();

  let lines: Vec<&str> = stripped_content.split("\n").collect();
  let tokens: Vec<&str> = lines[line_number].split(" ").collect();
  return tokens[token_number].to_string();
}

/*
 * write_string
 * 
 * @brief Simply opens up (or creates) a file and appends the new data to it
 * @param data: The new data to add
 */
#[allow(dead_code)]
fn write_string(data: String) {
  // Open file
  let mut file = match std::fs::OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open(get_file_name()) {
      Ok(f) => f,
      Err(err) => {
        println!("Can't create/open \"{}\": {}", get_file_name(), err);
        return;
      }
  };

  // Replace carriage returns with new-lines
  let filtered_data = data.replace("\x0d", "\x0a");

  // Write the string to the file
  match file.write_all(filtered_data.as_bytes()) {
    Ok(_) => {}
    Err(err) => {
      println!("Can't write to \"{}\": {}", get_file_name(), err);
      return;
    }
  }
}

/*
 * dump
 * 
 * @brief Dump the contents of the file normally and with token ID's
 */
fn dump() {
  // Read file
  let content = match std::fs::read_to_string(get_file_name()) {
    Ok(out) => out,
    Err(err) => {
      println!("Can't open \"{}\": {}", get_file_name(), err);
      return;
    }
  };

  println!("{}\n-----\n{}\nID Map\n-----", get_file_name(), content);

  let stripped_content: String = String::from_utf8_lossy(&strip_ansi_escapes::strip(content).unwrap()).to_string();

  // Print out all of the tokens with their IDs
  let lines: Vec<&str> = stripped_content.split("\n").collect();
  let mut line_number = 0;
  // Line by line
  for line in lines {
    let mut token_number = 0;
    print!("\x1b[42m{}\x1b[0m", line_number);

    let tokens: Vec<&str> = line.split(" ").collect();
    // Token by token
    for token in tokens {
      print!("\x1b[45m{}\x1b[0m{} ", token_number, token);

      token_number += 1;
    }
    println!();

    line_number += 1;
  }
}

/*
 * get_file_name
 * 
 * @brief Grab the global file name
 * @return the current file name
 */
fn get_file_name() -> String {
  return FILE_NAME.with(|file_name| {
    return file_name.borrow().clone();
  });
}

/*
 * set_file_name
 * 
 * @brief Set the global file name
 * @param name: The new name to set to
 */
fn set_file_name(name: String) {
  FILE_NAME.with(|file_name| {
    *file_name.borrow_mut() = name;
  });
}

/*
 * main
 * 
 * @brief Parse args and then load into the user's report
 */
fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args[1] == "help" || args.len() < 2 || args.len() > 3 {
    print!("\
      Generate report to a file:\n\
      \t./rpt <file name>\n\
      Dump contents of report:\n\
      \t./rpt dump <file name>
    \r");

    return;
  } else if args[1] == "dump" {
    set_file_name(args[2].clone());
    dump();

    return;
  }

  set_file_name(args[1].clone());

  report();
}
