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
 * events.rs -> stores static events structs
 * 
 * authors: $t@$h, r00r00, n3wmAn
 */

// Imports
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{stdin, stdout, Write};
use std::path::{Path};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering::*};
use std::{time, thread};
use termion::input::TermRead;
use zeroize::Zeroize;

// External files
use crate::alerts;
use crate::config;
use crate::constants;
use crate::debug;
use crate::filter;
use crate::log;
use crate::main_info;
use crate::print;
use crate::terminal;
use crate::util;
use crate::{simple_match, simple_test_match};

/* 
 * prompt_in_event
 * 
 * @brief If an event needs to ask the user for args, this is a
 * standard way to get their input back
 * @param prompt: Comes with the command prompt and question
 * @return the user's input
 */
macro_rules! prompt_in_event {
  ($event_name:expr, $question:expr) => {
    {
      let input = match terminal::get_input(format!("{} {} {}", constants::CONSOLE_PROMPT, $event_name, $question)) {
        Ok(out) => out,
        Err(_) => { return Vec::new(); }
      };
      if input == "forcequit" { return Vec::new(); }
      input.trim().to_string()
    }
  };
}

// Events folder
pub mod binaryanalysis;
pub mod codeanalyzers;
pub mod core;
pub mod cracking;
pub mod cyclicredundancycheckers;
pub mod cryptography;
pub mod denialofservice;
pub mod decompilereverse;
pub mod disassemblers;
pub mod errorcorrectioncoding;
pub mod eventmode;
pub mod exploitinjection;
pub mod fuzzers;
pub mod hashers;
pub mod kernelhardeners;
pub mod maninthemiddle;
pub mod netloaders;
pub use self::netloaders::servf_stop_all;
pub use self::scanners::stop_clamav;
pub use self::scanners::get_clamav_status;
pub mod networkanalyzers;
pub mod detectionevasion;
pub mod basictools;
pub mod rawtools;
pub mod secureboot;
pub mod signers;
pub mod snoopers;
pub mod scanners;
pub mod spoofers;
pub mod stegdetection;
pub mod steganography;
pub mod vulnerabilitydatabases;
pub mod forensics;
pub mod parsers;
pub mod reverseshell;

// Enums
#[derive(Copy, Clone, Debug)]
pub enum TestStatus {
  Passed,
  Failed,
  Unimplemented
}
impl std::fmt::Display for TestStatus {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match &self {
      TestStatus::Passed => write!(f, "PASSED"),
      TestStatus::Failed => write!(f, "FAILED"),
      TestStatus::Unimplemented => write!(f, "NO TEST"),
    }
  }
}

// Event parent struct
pub struct Event {
  pub name: String,
  pub desc: String,
  pub usage: String,
  pub author: String,
  pub easyrun: bool,
  pub secure: bool,
  pub parent: String,
  pub links: Vec<String>
}
pub trait Eventable {
  // If this is implemented the event has handholding functionality
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  // This is executed if a function was passed args, default go to on_init
  fn on_run(&self, _args: Vec<String>) -> String {
    return String::from("");
  }
  // Executes after init/run
  fn on_complete(&self) { print::print_custom("Success\n", "brightgreen"); }
  // Confirm that a tool works
  fn on_test(&self) -> TestStatus { return TestStatus::Unimplemented; }
  // Implementation doesn't change
  fn get_event(&self) -> &Event;
}

/*
 * is_module
 *
 * @brief We assume that events with only 2 links, and not named 'salvum'
 * are executable tools
 * @param event: The event in question
 * @return if it's a tool or not
 */
pub fn is_module(event: &Event) -> bool {
  return event.links.len() == 2 && event.name != "salvum";
}

/*
 * is_module
 * 
 * @brief 
 * @param input: Raw string to compare
 * @return if it is a builtin
 */
pub fn is_builtin(input: String) -> bool {
  for builtin in main_info::BUILTIN_CMNDS {
    if input == builtin {
      return true;
    }
  }
  return false;
}

/*
 * has_dir_access
 * 
 * @brief 
 * @param input: Raw string to compare
 * @return if it has top-level directory access
 */
thread_local!(static HAS_DIR_ACCESS: RefCell<Vec<String>> = RefCell::new(Vec::new()));
pub fn has_dir_access(input: String) -> bool {
  return HAS_DIR_ACCESS.with(|has_access| {
    let iter = &*has_access.borrow();
    for tool in iter {
      if *tool == input {
        return true;
      }
    }
    return false;
  });
}

/*
 * has_ip_access
 * 
 * @brief 
 * @param input: Raw string to compare
 * @return if it has non-local ip access
 */
thread_local!(static HAS_IP_ACCESS: RefCell<Vec<String>> = RefCell::new(Vec::new()));
pub fn has_ip_access(input: String) -> bool {
  return HAS_IP_ACCESS.with(|has_access| {
    let iter = &*has_access.borrow();
    for tool in iter {
      if *tool == input {
        return true;
      }
    }
    return false;
  });
}

/*
 * prompt_passwd_in_event
 * 
 * @brief Special input to prevent displaying the characters to the terminal
 * @param prompt: Comes with the command prompt and question
 * @return the user's input
 */
fn prompt_passwd_in_event(event_name: &str,
                          question: &str) -> String {
  print::print(&format!("{} {} {}", constants::CONSOLE_PROMPT, event_name, question));
  let stdout = stdout();
  let mut stdout = stdout.lock();
  terminal::flush();
  let passwd = match stdin().read_passwd(&mut stdout) {
    Ok(out) => match out {
      Some(s) => s,
      None => return "".to_string(),
    },
    Err(_) => return "".to_string(),
  };
  println!();
  return passwd;
}

/* 
 * run_command
 * 
 * @brief Executes a command (w/ a timer) and returns the output
 * @param cmnd: The Command object you created
 * @return the output (contains stdout and stderr)
 */
fn run_command(cmnd: &mut std::process::Command) -> std::io::Result<std::process::Output> {
  if !main_info::is_bg() { start_prompt_timer(); }
  let output = cmnd.output();
  if !main_info::is_bg() { stop_prompt_timer(); }
  return output;
}

// Used for determining when to kill the timer thread
static TIMER_CONT: AtomicBool = AtomicBool::new(true);
static TIMER_END: AtomicBool = AtomicBool::new(false);
const  TIMER_STATES: &[&str] = &["Processing . . . ",
                                 "Processing o . . ",
                                 "Processing . o . ",
                                 "Processing . . o "];
/* 
 * start_prompt_timer
 * 
 * @brief Launches a new thread to display a counter while waiting on processes
 * @param none
 * @return none
 */
fn start_prompt_timer() {
  let mut time_in_secs: usize = 0;
  let mut timer_state: usize = 0;
  
  TIMER_CONT.store(true, Release);
  thread::spawn(move || {
    while TIMER_CONT.load(Acquire) {
      for _ in 0..4 {
        print!("\r{}{}s", TIMER_STATES[timer_state], time_in_secs);
        terminal::flush();
        thread::sleep(time::Duration::from_millis(250));
        if !TIMER_CONT.load(Acquire) { break; }
        timer_state = (timer_state + 1) % TIMER_STATES.len();
      }
      time_in_secs += 1;
    }
    TIMER_END.store(true, Release);
  });
}

/* 
 * stop_prompt_timer
 * 
 * @brief Signals the timer thread to stop
 * @param none
 * @return none
 */
fn stop_prompt_timer() {
  TIMER_CONT.store(false, Release);
  println!();
  while !TIMER_END.load(Acquire) {}
}

/* 
 * run_console_command
 * 
 * @brief Executes a command (w/o a timer) and will display output until it's done
 * @param cmnd: The Command object you created
 * @return none
 */
fn run_console_command(cmnd: &mut std::process::Command) -> std::io::Result<std::process::Output> {
  if main_info::is_bg() {
    cmnd.stdout(Stdio::piped()).stderr(Stdio::piped());
  }
  
  let child = match cmnd.spawn() {
    Ok(out) => out,
    Err(err) => return Err(err)
  };

  return child.wait_with_output();
}

/* 
 * run_bounded_command
 * 
 * @brief Executes a command (w/o a timer) and will end execution after max_time seconds
 * @param cmnd: The Command object you created
 * @param max_time: The maximum amount of seconds to execute
 * @return none
 */
fn run_bounded_command(cmnd: &mut std::process::Command, is_stdout: bool, max_time: u64) -> std::io::Result<std::process::Output> {
  let child = match if is_stdout { cmnd.spawn() } 
                    else {
                      cmnd.stdout(Stdio::piped());
                      cmnd.stderr(Stdio::null());
                      cmnd.spawn()
                    }
  {
    Ok(out) => out,
    Err(err) => return Err(err)
  };

  thread::sleep(time::Duration::from_secs(max_time));
  let id = child.id();

  match Command::new("kill").args(vec!["-9", &id.to_string()]).spawn() {
    Ok(_) => (),
    Err(e) => {
      println!("run_bounded_command failed: {}", e);
    }
  };

  return child.wait_with_output();
}

/* 
 * run_bounded_command_err
 * 
 * @brief Executes a command (w/o a timer) and will end execution after max_time seconds
 * @param cmnd: The Command object you created
 * @param max_time: The maximum amount of seconds to execute
 * @return none
 */
fn run_bounded_command_err(cmnd: &mut std::process::Command, is_stderr: bool, max_time: u64) -> std::io::Result<std::process::Output> {
  let child = match if is_stderr { cmnd.spawn() } 
                    else {
                      cmnd.stdout(Stdio::null());
                      cmnd.stderr(Stdio::piped());
                      cmnd.spawn()
                    } 
  {
    Ok(out) => out,
    Err(err) => return Err(err)
  };

  thread::sleep(time::Duration::from_secs(max_time));
  let id = child.id();

  match Command::new("kill").args(vec!["-9", &id.to_string()]).spawn() {
    Ok(_) => (),
    Err(e) => {
      println!("run_bounded_command_err failed: {}", e);
    }
  };

  return child.wait_with_output();
}
/*
 * get_events
 * 
 * @brief Creates all of the events and adds them to the vector
 * @param none
 * @return a vector of events
 */
pub fn get_events() -> Vec<Box<dyn Eventable + Send + Sync>> {
  let entry_config = config::read_config();

  // We have to box up the Eventables as they have an undeterministic size
  let mut events: Vec<Box<dyn Eventable + Send + Sync>> = Vec::new();

  // Create the events by traversing through the entries
  fn traverse_entries(entry: config::Entry, parent: &str,
                      entry_config: &HashMap<String, (config::EntryStatus, config::EntryStatus)>,
                      events: &mut Vec<Box<dyn Eventable + Send + Sync>>) -> bool {
    // Used to prevent adding either blue or red if excluded
    static INCLUDE_BLUE: AtomicBool = AtomicBool::new(true);
    static INCLUDE_RED: AtomicBool = AtomicBool::new(true);

    let (included_status, _) = match entry_config.get(entry.name) {
      Some(&out) => out,
      None => (config::EntryStatus::Default, config::EntryStatus::Default)
    };
    
    // Only look at included entries
    if (included_status == config::EntryStatus::Default && entry.included)
    || (included_status == config::EntryStatus::Enabled) {
      // Store all entrys with special directory access in the vector
      HAS_DIR_ACCESS.with(|has_access| {
        if entry.unsafe_dir_access {
          let borrow = &mut *has_access.borrow_mut();
          borrow.push(entry.name.to_string());
        }
      });

      // Store all entries with non-local ip access in the vector
      HAS_IP_ACCESS.with(|has_access| {
        if entry.nonlocal_ip_access {
          let borrow = &mut *has_access.borrow_mut();
          borrow.push(entry.name.to_string());
        }
      });

      let mut links: Vec<String> = Vec::new();
      let children_len = entry.children.len();

      // Loop through all children
      for child in entry.children {
        let child_name = child.name.clone();
        let included = traverse_entries(child, entry.name, entry_config, events);

        // Traverse through included children
        if included {
          links.push(child_name.to_string());
        // If the child isn't included, check to see if it's blue or red
        } else {
          if child_name == "blue" {
            INCLUDE_BLUE.store(false, Release);
          } else if child_name == "red" {
            INCLUDE_RED.store(false, Release);
          }
        }
      }

      if (children_len > 0 && links.len() > 0)
      || (children_len == 0 && links.len() == 0) {
        // Add blue and red to the events where appropriate
        if entry.name != "salvum" {
          if INCLUDE_BLUE.load(Acquire) && entry.name != "blue" { links.push("blue".to_string()); }
          if INCLUDE_RED.load(Acquire) && entry.name != "red" { links.push("red".to_string()); }
        }

        events.push((entry.fn_ptr)(links, entry.name.to_string(), parent.to_string()));
        return true;
      }
    }
    return false;
  }
  traverse_entries(config::get_entries(), "", &entry_config, &mut events);

  return events;
}
