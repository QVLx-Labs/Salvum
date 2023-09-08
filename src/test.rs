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
 * test.rs -> 
 *
 * authors: $t@$h, r00r00
 */

// Imports
use colored::Colorize;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering::*};
use std::time;

// External files
use crate::events;
use crate::print;
use crate::terminal;

pub fn test<D>(args: Vec<D>,
               events: &Vec<Box<dyn events::Eventable + Send + Sync>>,
               events_map: &HashMap<String, usize>) 
  where D: std::fmt::Display + std::clone::Clone {
  let longest_name = match events_map.keys().map(String::len).max() {
    Some(out) => out,
    None => 20
  };

  // Title
  print::println_neutral(&format!("+{0:->width$}+{0:->10}+", "", width=(longest_name + 2)));
  print::println_neutral(&format!("|{:^width$}|{:^10}|", "Module Name", " Status", width=(longest_name + 2)));
  print::println_neutral(&format!("+{0:->width$}+{0:->10}+", "", width=(longest_name + 2)));

  let mut tests_passed: usize = 0;
  let mut tests_total: usize = 0;

  if args.len() == 1 {
    /********** Print individual tests **********/
      let test_name = terminal::replace_aliases(args[0].clone().to_string());
      let event = match events_map.get(&test_name) {
        Some(&id) => &events[id],
        None => {
          print::println_neutral(&format!("| {:>width$} |  {} |", test_name.yellow(), "UNKNOWN".magenta(), width=longest_name));
          return;
        }
      };
      run_test(event, &mut tests_passed, &mut tests_total, longest_name);
  }
  else if args.len() > 2 {
    /********** Print individual tests **********/
    for i in 2..args.len() {
      let test_name = terminal::replace_aliases(args[i].clone().to_string());
      let event = match events_map.get(&test_name) {
        Some(&id) => &events[id],
        None => {
          print::println_neutral(&format!("| {:>width$} |  {} |", test_name.yellow(), "UNKNOWN".magenta(), width=longest_name));
          continue;
        }
      };

      run_test(event, &mut tests_passed, &mut tests_total, longest_name);
    }
  } else {
    /********** Print all tests **********/
    for event in events {
      run_test(event, &mut tests_passed, &mut tests_total, longest_name);
    }
  }

  // Closing line
  print::println_neutral(&format!("+{0:->width$}+{0:->10}+", "", width=(longest_name + 2)));
  print::println_neutral(&format!("| {:>width$} | {:>3}/{:<3}  |", "Tests Passing", tests_passed, tests_total, width=longest_name));
  print::println_neutral(&format!("+{0:->width$}+{0:->10}+", "", width=(longest_name + 2)));
}

static TIMER_CONT: AtomicBool = AtomicBool::new(true);
static TIMER_END: AtomicBool = AtomicBool::new(false);
static TIMER_STATES: [&str; 6] = ["o.....",
                                  ".o....",
                                  "..o...",
                                  "...o..",
                                  "....o.",
                                  ".....o",];

/*
 * run_test
 *
 * @brief Takes the environment args and runs either tests or a single module
 * @param event: 
 * @param tests_passed: 
 * @param tests_total: 
 * @param longest_name: 
 */
fn run_test(event: &Box<dyn events::Eventable + Send + Sync>, tests_passed: &mut usize, tests_total: &mut usize, longest_name: usize) {
  TIMER_CONT.store(true, Release);
  TIMER_END.store(false, Release);

  // Only test modules
  if events::is_module(event.get_event()) {
    // Timer stuff
    let event_name = event.get_event().name.clone();
    std::thread::spawn(move || {
      while TIMER_CONT.load(Acquire) {
        for i in 0..6 {
          print::print_neutral(&format!("\r| {:>width$} |  {}  |", event_name.yellow(), TIMER_STATES[i], width=longest_name));
          terminal::flush();
          for _ in 0..16 {
            std::thread::sleep(time::Duration::from_millis(3));
            if !TIMER_CONT.load(Acquire) { break; }
          }
        }
      }
      TIMER_END.store(true, Release);
    });

    let status = event.on_test();

    // Inform the thread that it's done running the test
    TIMER_CONT.store(false, Release);
    while !TIMER_END.load(Acquire) {}

    // Print out the result
    *tests_total += 1;
    let msg = &format!("{}", status);
    print::print_neutral(&format!("\r| {:>width$} |  ", event.get_event().name.yellow(), width=longest_name));
    match status {
      events::TestStatus::Passed => {
        print::print_custom(msg, "brightgreen");
        print::println_neutral("  |");
        *tests_passed += 1;
      }
      events::TestStatus::Failed => {
        print::print_custom(msg, "red");
        print::println_neutral("  |");
      }
      events::TestStatus::Unimplemented => {
        print::print_custom(msg, "magenta");
        print::println_neutral(" |");
      }
    }
    terminal::flush();
  }
}
