/*
 * QVLx Salvum 
 *
 * edition_packager.rs
 * 
 * ./target/debug/salvum package <edition_name>
 * DO NOT RUN THIS AS SUDO, YOU CAN'T AUTO-COMPILE IF YOU DO
 * Will traverse through the config set in main.rs and copy over only
 * the necessary files from ext/ and tst/ to /home/shared/slm_<edition_name>
 * 
 * Ensure to comment out the package code in main::handle_cli and rebuild so that the
 * users don't have access to this. We could do this automatically, but it
 * would take some work to setup
 *
 * authors: r00r00, $t@$h
 */

// Imports
use colored::Colorize;

// External files
use crate::config;

#[allow(dead_code)]
fn copy_files(copy_path: &str, edition_path: String, docker_path: String) {
  match std::process::Command::new("sudo").args(vec!["cp", "-r", copy_path, &edition_path]).status() {
    Ok(_) => {}
    Err(err) => {
      println!("Failed to copy files over \"{}\": {}", copy_path, err);
      return;
    }
  }
  match std::process::Command::new("sudo").args(vec!["cp", "-r", copy_path, &docker_path]).status() {
    Ok(_) => {}
    Err(err) => {
      println!("Failed to copy files over \"{}\": {}", copy_path, err);
      return;
    }
  }
}

#[allow(dead_code)]
pub fn generate(edition: String) {
  let edition_path = format!("../slm_{}", edition);
  let docker_path = format!("../slm_dkr_{}/docker", edition);

  // Ensure that the necessary directories exist
  for path in [edition_path.clone(),
               docker_path.clone(),
               format!("{}/ext", edition_path.clone()),
               format!("{}/ext", docker_path.clone()),
               format!("{}/tst", edition_path.clone()),
               format!("{}/tst", docker_path.clone())] {
    match std::process::Command::new("sudo").args(vec!["mkdir", &path]).output() {
      Ok(_) => {}
      Err(err) => {
        println!("Failed to mkdir: {}", err);
        return;
      }
    }
  }

  // Make sure the Ubuntu 20.04 package has repair_salvum (each path should have custom install/package scripts)
  match std::process::Command::new("sudo").args(vec!["cp", "-r", "repair_salvum.sh", &edition_path.clone()]).status() {
    Ok(_) => {}
    Err(err) => {
      println!("Failed to copy files over \"repair_salvum.sh\": {}", err);
      return;
    }
  }

  // Copy over all top-level directories
  for path in ["cfg", "dep", "lic", "out"] {
    copy_files(path, edition_path.clone(), docker_path.clone());
  }

  // Special case where we need it in ext folder
  copy_files("ext/util", format!("{}/ext", edition_path.clone()), format!("{}/ext", docker_path.clone()));

  // Create the events by traversing through the entries
  fn traverse_entries(entry: config::Entry, edition_path: String, docker_path: String) {
    // Only look at included entries
    if entry.included {
      if entry.children.len() == 0 {
        // Copy over the ext and tst entries of included modules
        let ext_path = format!("ext/{}", entry.name);
        let tst_path = format!("tst/{}", entry.name);

        copy_files(&ext_path, format!("{}/ext", edition_path.clone()), format!("{}/ext", docker_path.clone()));
        copy_files(&tst_path, format!("{}/tst", edition_path.clone()), format!("{}/tst", docker_path.clone()));
      } else {
        // Loop through all children
        for child in entry.children {
          // Traverse through included children
          traverse_entries(child, edition_path.clone(), docker_path.clone());
        }
      }
    }
  }
  println!("{}", "You may see some file not found problems from ext/, this is OK\nIf any come from tst/ a test is probably not implemented".red());
  traverse_entries(config::get_entries(), edition_path.clone(), docker_path.clone());

  // Compile release for salvum
  match std::process::Command::new("cargo").args(vec!["build", "--release"]).status() {
    Ok(_) => {}
    Err(_) => {
      println!("Do not package as sudo, you can't automatically compile with cargo");
      return;
    }
  }
  copy_files("./target/release/salvum", edition_path.clone(), docker_path.clone())
}
