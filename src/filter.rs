/*
 * QVLx Salvum 
 *
 * authors: $t@$h, r00r00, n3wmAn
 */

/*
 * NOTES
 * 
 * usage::parse_help_menu(Command::new("ext/ciphey").args(vec!["--help"]), vec!["rule1", "rule2"], "\n  -");
 * usage::parse_help_menu(Command::new("ext/jtr/john").args(vec!["-h"]), vec!["rule1", "rule2"], "\n--");
 * usage::parse_help_menu(Command::new("ext/reveng").args(vec!["-h"]), vec!["rule1", "rule2"], "\x0A\x09-");
 * 
 * If you find yourself working with something as "nice" as reveng, here
 * is how you can tackle it:
 *    ext/reveng -h &> reveng_help
 *    hexdump -C reveng_help > reveng_dump
 * And then search for the hex values of those lovely characters.
 *
 * Design Box
 *      child                                  filter                  parent
 * +-------------+                             +----+            +----------------+  
 * |             | cmd1_flg1_flg2   <--------->|cmd1| <--------->|                |
 * |    Tool     | cmd2_flg1        <--------->|kmd*| <--------->|     Salvum     |
 * |             | kmd3_flg1_flg2   <--------->|    | <--------->|                |
 * |             | cmd4_flg1_flg2_flg3 <------>|    | <--------->|                |
 * +-------------+                             +----+            +----------------+
 * End Design Box
 */

use std::fs;

/*
 * load_rules_cfg
 * 
 * @brief Loads a vec with rules from a .cfg file
 * @param tool_name: What config to load
 * @return A vector of rules
 */
pub fn load_rules_cfg(tool_name: &str) -> Vec<String> {
  // Create the path
  let mut path = String::from("cfg/");
  path.push_str(tool_name);
  path.push_str(".cfg");

  // Load the file into a string
  let cfg = match fs::read_to_string(path) {
    Ok(out) => out,
    Err(err) => {
      println!("load_rules_cfg: Failed to open the file. {}", err);
      return vec![String::from("")];
    }
  };

  // Delimit the file by newline
  let lines: Vec<&str> = cfg.split("\n").collect();

  let mut rules: Vec<String> = Vec::new();
  for line in lines {
    // Add any line that isn't a comment as a rule
    if line != "" && line.len() > 1 && &line[0..2] != "//" {
      rules.push(line.to_string());
    }
  }
  return rules;
}

/*
 * parse_help_menu
 * 
 * @brief Prints out a set of commandline arguments based on a group of rules
 * @param cmnd: The command with only -h or --help as an arg
 * @param rules: What options you want to see (allowlist)
 * @param delimeter: What to split the help menu on
 */
pub fn parse_help_menu(cmnd: &mut std::process::Command, rules: Vec<String>, delimeter: &str, pop_lines: usize, first_line: &str) -> String {
  // Run the command
  let output = match cmnd.output() {
    Ok(out) => out,
    Err(err) => {
      println!("parse_help_menu ({:?}): {}", cmnd, err);
      return "".to_string();
    }
  };

  // Grab the stdout and stderr, push into the same string
  let mut output_str = String::from(String::from_utf8_lossy(&output.stdout));
  output_str.push_str(&String::from_utf8_lossy(&output.stderr));
  let lines: Vec<&str> = output_str.split(delimeter).collect();

  let mut filtered_string = String::from(first_line);

  // For each line, excluding the very first
  for line in lines[pop_lines..].iter() {
    let mut good_line = false;
    // For each rule
    for rule in rules.iter() {
      // If it contains a rule set good to true, but keep checking
      if rule == "!ALL!" || line.contains(rule) {
        good_line = true;
      }
      // If it detects a blocklisted term, it's a bad line
      else if rule.len() > 4 && &rule[0..4] == "!BL!" && line.contains(&rule[4..]) {
        good_line = false;
        break;
      }
    }
    // Add good lines
    if good_line {
      filtered_string.push_str(line);
      filtered_string.push_str(delimeter);
    }
  }
  filtered_string.push_str("\n");
  return filtered_string;
}

/*
 * if_command_contains_rule
 * 
 * @brief Determines if user input is allowed based on the rules
 * @param usr_args: Arguments that were delimited on a space
 * @param rules: The set to check against
 * @return Whether or not the user passes the rules
 */
pub fn if_command_contains_rule(usr_args: &Vec<&str>, rules: Vec<String>) -> bool {
  for arg in usr_args.iter() {
    // If the arg starts with a hyphen...
    if arg.len() > 1 && &arg[0..1] == "-" {
      let mut found = false;
      // ...search each rule to see...
      for rule in &rules {
        // ...if a rule is found in the args...
        if rule == "!ALL!" || arg.contains(&rule.to_string()) {
          found = true;
        }
      }
      // ...if one isn't found, bad arg
      if !found {
        return false;
      }
    }
  }
  // Each arg is good
  return true;
}
