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
 * reports_coder.rs -> 
 *
 * authors: $t@$h, r00r00, n3wmAn
 */

// Imports
use std::collections::HashMap;
use std::fs::{self, File};
use heck::TitleCase;
use std::io::Write;

// External files
use crate::config;
use crate::errno;

pub fn generate() {
  // Create a file
  let mut file = match File::create("rpt/reports.rs") {
    Ok(filename) => filename,
    Err(_err) => {
      errno::print_err("Report", "Can't create file.");
      return;
    }
  };

  let mut report_files: HashMap<String, File> = HashMap::new();
  match fs::create_dir("rpt/reports") {
    Ok(_) => {}
    Err(_) => {}
  };

  // Static input
  let mut file_string = String::from("");

  // Create the events by traversing through the entries
  fn traverse_entries(entry: config::Entry,
                      report_files: &mut HashMap<String, File>,
                      parent_str: String,
                      mod_str: &mut String,
                      match_str: &mut String) {
    // Only look at included entries
    if entry.included {
      if entry.children.len() == 0 {
        match_str.push_str(&format!("    \"{}\" => {{ {}::{}::report(); }}\n", entry.name, parent_str, entry.name.to_title_case()));
        let mut report_file_str = String::from("");
        report_file_str.push_str(&format!("\npub struct {} {{}}", entry.name.to_title_case()));
        report_file_str.push_str(&format!("\nimpl Reportable for {} {{}}\n", entry.name.to_title_case()));
        let mut report_file = &report_files[&parent_str];
        match report_file.write_all(report_file_str.as_bytes()) {
          Ok(_) => {}
          Err(_err) => {
            errno::print_err("Report", "Can't write to file.");
            return;
          }
        }
      }
      // Loop through all children
      for child in entry.children {
        // Traverse through included children
        if child.included {
          let parent_new = if entry.name == "blue" || entry.name == "red" {
            // Create a file
            report_files.insert(
              child.name.to_string(),
              match File::create(format!("rpt/reports/{}.rs", child.name)) {
                Ok(filename) => filename,
                Err(_err) => {
                  errno::print_err("Report", "Can't create file.");
                  return;
                }
              }
            );
            let mut report_file = &report_files[child.name];
            match report_file.write_all(b"use crate::reports::*;\n") {
              Ok(_) => {}
              Err(_err) => {
                errno::print_err("Report", "Can't write to file.");
                return;
              }
            }
            mod_str.push_str(&format!("mod {};\n", child.name));

            child.name.to_string()
          } else {
            parent_str.clone()
          };
          traverse_entries(child, report_files, parent_new, mod_str, match_str);
        }
      }
    }
  }
  let mut mod_str = String::from("");
  let mut match_str = String::from("");
  traverse_entries(config::get_entries(), &mut report_files, String::from(""), &mut mod_str, &mut match_str);

  file_string.push_str(&mod_str);
  file_string.push_str("\npub trait Reportable {\n");
  file_string.push_str("  fn report() {\n");
  file_string.push_str("    println!(\"No report available\");\n");
  file_string.push_str("  }\n");
  file_string.push_str("}\n\n");
  file_string.push_str("pub fn invoke_report(report_name: &str) {\n");
  file_string.push_str("  match report_name {\n");
  file_string.push_str(&match_str);
  file_string.push_str("    _ => {}\n");
  file_string.push_str("  }\n");
  file_string.push_str("}\n");

  match file.write_all(file_string.as_bytes()) {
    Ok(_) => {}
    Err(_err) => {
      errno::print_err("Report", "Can't write to file.");
      return;
    }
  }
}
