/*
 * QVLx Salvum 
 *
 * main.rs -> orchestrating program for Salvum
 *
 * authors: $t@$h, r00r00, n3wm4n
 */

// Imports
use colored::Colorize;
use ctrlc;
use heck::TitleCase;
use std::collections::HashMap;
use std::fs;

// External files
mod alerts;
mod constants;
mod debug;
mod edition_packager;
mod errno;
mod events;
mod filter;
mod log;
mod print;
mod reports_coder;
mod terminal;
mod test;
mod util;

// Different configs
//mod community_config;
mod full_config;

//use community_config as config;
use full_config as config;

/*
 * print_splash
 *
 * @brief print branding splash
 * @param none
 * @return none
 */
fn print_splash() {
 let splash: &'static str = "  \x1b[38;37;97m_________________\x1b[0m \n\
                              _\x1b[38;37;97m|:::::::::::::::::|\x1b[0m______________________________________\n\
                              =\x1b[38;37;97m|:            ::::|\x1b[0m======================================\n\
                              =\x1b[38;37;97m|     \x1b[38;5;115mQ V L x\x1b[0m   \x1b[38;37;97m::|\x1b[0m======== \x1b[38;37;97m\x1b[38;5;115mSecurity Engine CLI\x1b[0m =========\n\
                              =\x1b[38;37;97m|                :|\x1b[0m======================================\n\
                              -\x1b[38;37;97m|   \x1b[38;5;115mS a l v u m\x1b[0m  \x1b[38;37;97m |\x1b[0m--== \x1b[38;37;97mOptions\x1b[0m ==================-------\n\
                              --\x1b[38;37;97m\\               /\x1b[0m---=============================-------\n\
                              ---\x1b[38;37;97m\\   \x1b[38;5;115mCE  1.0\x1b[0m \x1b[38;37;97m :/\x1b[0m----===    \x1b[38;5;152m(m)enu\x1b[38;5;15m for menu\x1b[0m    ===-------\n\
                              ---=\x1b[38;37;97m\\         ::/\x1b[0m-----=============================-------\n\
                              ---==\x1b[38;37;97m\\        :/\x1b[0m------===    \x1b[38;5;152mred\x1b[38;5;15m for offense\x1b[0m    ===-------\n\
                              ---===\x1b[38;37;97m\\     ::/\x1b[0m-------=============================-------\n\
                              ---===-\x1b[38;37;97m\\    :/\x1b[0m--------===    \x1b[38;5;152mblue\x1b[38;5;15m for defense\x1b[0m   ===-------\n\
                              ---===--\x1b[38;37;97m\\  :/\x1b[0m---------=============================-------\n\
                              ---===---\x1b[38;37;97m\\_/\x1b[0m----------===    \x1b[38;5;152m(c)lear\x1b[38;5;15m to purge\x1b[0m   ===-------\n\
                              ---===----------------=============================-------\n\
                              =========================    \x1b[38;5;152m(b)ack\x1b[38;5;15m to go back\x1b[0m  ===-------\n\
                              =~==~~==~~==~~==~~==~~=============================-------\n\
                              ~==~~==~~==~~==~~==~~====    \x1b[38;5;152m(n)ote\x1b[38;5;15m for legal\x1b[0m   ==========\n\
                              ==~~==~~==~~==~~==~~===============================~~==~~=\n\
                              =~~==~~==~~==~~==~~======    \x1b[38;5;152m(e)xit\x1b[38;5;15m to quit\x1b[0m     ===~==~~==\n\
                              ~===~~==~~==~~==~~====================================~~==\n\
                              ===~~==~~==~~==~~==  \x1b[38;5;152m(u)sage <module>\x1b[38;5;15m for help\x1b[0m  =====~~==~\n\
                              ==~~==~~==~~==~~====================================~~==~~\n\
                              =~~==~~==~~==~~==    \x1b[38;5;152m(d)esc <module>\x1b[38;5;15m for info\x1b[0m   ===~~==~~=\n\
                              ==========================================================\n\
                              === \x1b[38;5;15mYou can press \x1b[38;5;152mEsc\x1b[38;5;15m to toggle array mode from a menu\x1b[0m ===\n\
                              ==========================================================\n\
                              === \x1b[38;5;15mUse the \x1b[38;5;152mdocs \x1b[38;5;15mcommand to view paging documentation \x1b[0m ===\n\
                              ==========================================================\n\
                              === \x1b[38;5;15mPress \x1b[38;5;152mCtrl + f\x1b[38;5;15m to toggle interactive file manager\x1b[0m  ===\n\
                              ==========================================================\n\
                              ======== \x1b[38;5;115m(c) 2022 QVLX LLC. All Rights Reserved.\x1b[0m =========\n\
                              ==========================================================";
  println!("{}", splash.white());
  terminal::flush();
}

mod main_info {
  use once_cell::sync::Lazy;
  use std::cell::RefCell;
  use std::sync::{atomic::{AtomicBool, AtomicUsize, Ordering::*}, Mutex};

  use crate::events;

  // BUILTINS
  thread_local!(pub static BACK_HISTORY: RefCell<Vec<usize>> = RefCell::new(Vec::new()));

  pub static BUILTIN_CMNDS: [&str; 37] = ["back",        "b",
                                          "parent",      "p",
                                          "menu",        "m",
                                          "jobs",        "j",
                                          "clear",       "c",
                                          "usage",       "u",
                                          "modules",
                                          "description", "desc", "d", "about",
                                          "documentation", "docs", "manual", "help",
                                          "author",      "a",
                                          "license",     "l",
                                          "note",        "n",
                                          "secure",      "s",
                                          "splash",
                                          "term",
                                          "test",        "t",
                                          "alias",
                                          "map",
                                          "exit",        "e",];

  // EVENTS
  static LAST_EVENT: AtomicUsize = AtomicUsize::new(0);
  static FILE_REDIRECT: AtomicBool = AtomicBool::new(false);
  thread_local!(pub static EVENTS_VEC: RefCell<Vec<Box<dyn events::Eventable + Send + Sync>>> = RefCell::new(Vec::new()));
  //pub static EVENTS_VEC: Mutex<Vec<Box<dyn events::Eventable + Send + Sync>>> = Mutex::new(Vec::new());

  pub fn get_last_event() -> usize {
    return LAST_EVENT.load(Acquire);
  }

  pub fn set_last_event(last_event: usize) {
    LAST_EVENT.store(last_event, Release);
  }

  pub fn get_file_redirect() -> bool {
    return FILE_REDIRECT.load(Acquire);
  }

  pub fn set_file_redirect(file_redirect: bool) {
    FILE_REDIRECT.store(file_redirect, Release);
  }

  pub fn set_events_vec(new_events: Vec<Box<dyn events::Eventable + Send + Sync>>) {
    EVENTS_VEC.with(|events_vec| {
      *events_vec.borrow_mut() = new_events;
    });
  }

  // BACKGROUND THREADS
  #[derive(Clone, PartialEq)]
  pub enum ThreadStatus {
    Waiting = 0,
    Running = 1,
    Finished = 2,
  }
  impl std::fmt::Display for ThreadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match &self {
        ThreadStatus::Waiting => write!(f, "Waiting"),
        ThreadStatus::Running => write!(f, "Running"),
        ThreadStatus::Finished => write!(f, "Finished"),
      }
    }
  }

  pub static BG_THREADS: Lazy<Mutex<Vec<(String, String, std::thread::ThreadId, ThreadStatus)>>> = Lazy::new(|| Mutex::new(Vec::new()));

  pub fn is_bg() -> bool {
    {
      let threads_guard = match BG_THREADS.lock() {
        Ok(lock) => lock,
        Err(err) => {
          print!("Thread currently holding this mutex panicked: {}\n\r", err);
          return false;
        }
      };
      for thread in &*threads_guard {
        let (_, _, tid, stat) = thread.clone();
        if tid == std::thread::current().id() && stat != ThreadStatus::Waiting {
          return true;
        }
      }
    }
    return get_file_redirect();
  }
}

/*
 * main
 * 
 * @brief highest level function
 * @param none
 * @return none
 */
fn main() {
  // SIGINT handle
  match ctrlc::set_handler(move || {
    println!("{}", "\nKilling child process".magenta());
  }) {
    Ok(_) => {}
    Err(err) => {
      println!("Error setting SIGINT handler. {}", err);
    }
  }

  // Ensure these directories exist
  for dir in &["log", "out", "srv", "usr"] {
    match fs::create_dir(dir) {
      Ok(_) => {}
      Err(_) => {}
    };
  }

  // Init functionalities
  log::init_logger();
  terminal::load_aliases();

  // Variables
  let events: Vec<Box<dyn events::Eventable + Send + Sync>> = events::get_events();
  main_info::set_last_event(events.len() - 1);
  main_info::set_events_vec(events);
  
  main_info::EVENTS_VEC.with(|events| {
    let events_borrow = &*events.borrow();
    let mut events_map: HashMap<String, usize> = HashMap::new();
    let mut input: String;
    let mut err: i32 = 0;

    // Add all of the event names and indices to the map from the vector
    let mut index = 0;
    for event in events_borrow {
      let name = event.get_event().name.clone();
      events_map.insert(name, index);
      index = index + 1;
    }

    // Pure CLI, doesn't fully enter salvum
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
      if handle_cli(args, events_borrow, &events_map) { return; }
    }

    print_splash(); // Print one-time branding splash

    events::servf_stop_all(); // Comes from events/netloaders.rs

    util::security::create_salvum_user(); 

    if events::get_clamav_status() {
      match events::stop_clamav() {
        Ok(_)=>(),
        Err(_)=>(),
      };
    }
    
    // Console loop, runs until receives 'quit' command
    loop {
      if terminal::get_array_mode() {
        input = interactive_menu(events_borrow, &events_map);
      } else {
        input = match terminal::get_input(terminal::create_prompt(constants::CONSOLE_PROMPT, ">")) {
          Ok(out) => out,
          Err(_) => String::from(""),
        };
      }
      
      handle_input(input, events_borrow, &events_map, &mut err);

      thread_popup();
      
      if err != 0 { errno::handle_err(err); }
    }
  });
}

/*
 * interactive_menu
 * 
 * @brief Gathers a list of all commands to display in a menu format
 * @param events: Vector of all included events
 * @return 
 */
fn interactive_menu(events: &Vec<Box<dyn events::Eventable + Send + Sync>>,
                    events_map: &HashMap<String, usize>) -> String {
  // Create the selection vector
  let current_event: &events::Event = events[main_info::get_last_event()].get_event();
  let mut options: Vec<String> = Vec::new();
  if current_event.name != "salvum" && current_event.name != "blue" && current_event.name != "red" {
    options.push("BACK".to_string());
  }
  let links = current_event.links.clone();
  let mut modified_links: Vec<String> = Vec::new();
  for i in 0..links.len() {
    let mut link = links[i].clone();
    // Core applets should be ignored
    if links[i] != "blue" && links[i] != "red" && current_event.name == "salvum" {
      continue;
    }
    if !events::is_module(events[events_map[&links[i]]].get_event()) {
      link.push_str("/");
    }
    if links[i].eq("blue") || links[i].eq("red") {
      link.insert(0, '_');
    }
    modified_links.push(link);
  }
  options.append(&mut modified_links);
  for builtin in main_info::BUILTIN_CMNDS {
    if builtin.len() > 1 && builtin != "term" {
      options.push(builtin.to_uppercase());
    }
  }

  // Grab the selection to use for input
  match terminal::get_selection(options.clone()) {
    Some(opt) => {
      let mut choice = options[opt].clone();
      let last = match choice.pop() {
        Some(c) => c,
        None => '0',
      };
      if last != '/' { choice.push(last); }
      let first = choice.remove(0);
      if first != '_' { choice.insert(0, first); }
      return choice.to_lowercase();
    }
    _ => {
      terminal::set_array_mode(false);
      return String::from("");
    }
  }
}

/*
 * print_menu
 * 
 * @brief Prints out the help menu for the last event executed
 * @param events: Vector of all included events
 * @param events_map: Map of events to ids
 * @param err: Reference to a value used for error checking
 */
fn print_menu(events: &Vec<Box<dyn events::Eventable + Send + Sync>>,
              events_map: &HashMap<String, usize>,
              err: &mut i32) {
  // Retrieve the last event using last_even: usize
  let current_event: &events::Event = events[main_info::get_last_event()].get_event();
  let mut output: String = "".to_string();

  // Print builtin commands
  output.push_str("-----------------------\n== Built-in Commands ==\n-----------------------\n");
  output.push_str("'(c)lear': Reset the terminal\n");
  output.push_str("'(u)sage [module]': Invoke usage helper or display usage for a module\n");
  output.push_str("'(s)ecure [module]': Print secure modules or if module is cryptographically secure\n");
  output.push_str("'(a)uthor <module>': Print the author of a module\n");
  output.push_str("'((d)esc)ription <module>': Print the author of a module\n");
  output.push_str("'(l)icense <module>': Print the license information for an app\n");
  output.push_str("'(b)ack': Go back to previous menu\n");
  output.push_str("'(p)arent': Reverse traverse the menu tree to direct parent\n");
  output.push_str("'(n)ote': Print Salvum legal disclaimer\n");
  output.push_str("'test <modules>: Run module integrity test'\n");
  output.push_str("'term': Run terminal compatibility test\n");
  output.push_str("'map [red|blue]': Display module tree for all, red, or blue\n");
  output.push_str("'modules': Print all modules associated with current menu location\n");
  output.push_str("'docs': Toggle the paging documentation viewer\n");
  output.push_str("'cd': Invoke Salvum file manager\n");
  output.push_str("'(j)obs': Print a list of currently incomplete background tasks\n");
  output.push_str("'(e)xit': Terminates Salvum. Can also use (q)uit\n");
  output.push_str("'splash': Print Salvum splash screen\n");
  output.push_str("'alias [alias]': Print all aliases or module that an alias references\n");

  if current_event.name != "salvum" && current_event.name != "blue" && current_event.name != "red" {
    output.push_str("'(p)arent': Go to ancestor\n");
  }
  output.push_str("'(e)xit' or '(q)uit': This will end the Salvum session\n");

  let dashes: Vec<u8> = vec!['-' as u8; 14 + &current_event.name.len()];
  
  // Generate menu title
  output.push_str(&String::from_utf8_lossy(&dashes));
  output.push_str("\n== ");
  output.push_str(&current_event.name[..].to_title_case());
  output.push_str(" Options ==\n");
  output.push_str(&String::from_utf8_lossy(&dashes));
  output.push_str("\n");

  // Iterate through the links vector to retrieve help descriptions for each linked event
  for event_name in current_event.links.iter() {
    // Core applets should be ignored
    if event_name != "blue" && event_name != "red" && current_event.name == "salvum" {
      continue;
    }
    match events_map.get(event_name) {
      Some(&index) => { // Get the description associated with the cur link
        let desc = match &events[index].get_event().desc.split_once(".") {
          Some((l, _)) => l,
          None => {
            let grab = &events[index].get_event().desc;
            if grab != "" {
              grab
            } else {
              "No description available"
            }
          }
        };
        output.push_str(&format!("'{}': {}\n", event_name, desc)[..]); // Append output with help description for link
      }
      _ => *err = errno::ERR_MISMATCHED_LINKS
    }
  }
  output.push_str(&String::from_utf8_lossy(&dashes));
  
  print::println(&output);
}

/*
 * print_modules_menu
 * 
 * @brief Will display a menu of all available menu options that have usages
 * @param events: Vector of all included events
 * @param events_map: Map of events to ids
 * @param err: Reference to a value used for error checking
 * @return none
 */
fn print_modules_menu(events: &Vec<Box<dyn events::Eventable + Send + Sync>>,
                    events_map: &HashMap<String, usize>,
                    builtin: &str,
                    err: &mut i32) -> String {
  let event: &events::Event = events[main_info::get_last_event()].get_event();
  let mut line = String::from("--------------");
  for _ in 0..builtin.len() {
    line.push('-');
  }
  let mut output: String = format!("{1}\n\
                                    == {} Options ==\n\
                                    {1}\n", builtin, line);

  let mut found_usage = false;
  let mut valid_events: Vec<String> = Vec::new();
  if event.name != "salvum" {
  // Iterate through the links vector to retrieve help descriptions for each linked event
    for event_name in event.links.iter() {
      match events_map.get(event_name) {
        Some(&index) => { // Get the usage associated with the cur link
          let usage = &events[index].get_event().usage;
          if usage != "" {
            output.push_str(&format!("'{}'\n", event_name)[..]);
            valid_events.push(events[index].get_event().name.clone());
            found_usage = true;
          }
        }
        _ => *err = errno::ERR_MISMATCHED_LINKS
      }
    }
    output.push_str(&line);
  }

  // If a valid command was found...
  if found_usage {
    //...and we're in array mode
    if terminal::get_array_mode() {
      // Grab the selection to use for input
      match terminal::get_selection(valid_events.clone()) {
        Some(opt) => return valid_events[opt].clone(),
        _ => {
          println!("You didn't select anything.");
          return String::from("");
        }
      }
    //...and we're in cli mode
    } else {
      print::println(&output);
      return match terminal::get_input(terminal::create_prompt(constants::CONSOLE_PROMPT, &format!(" {}> Please type an option:", builtin))) {
        Ok(out) => out,
        Err(_) => String::from(""),
      };
    }
  }
  return String::from("");
}

/*
 * print_modules
 * 
 * @brief Prints out the modules available
 * @param events: Vector of all included events
 * @param events_map: Map of events to ids
 * @param err: Reference to a value used for error checking
 */
fn print_modules(events: &Vec<Box<dyn events::Eventable + Send + Sync>>,
              events_map: &HashMap<String, usize>,
              err: &mut i32) {
  let mut output: String = "".to_string();
  let current_event: &events::Event = events[main_info::get_last_event()].get_event();
  let current_event_name = &events[main_info::get_last_event()].get_event().name;
  if current_event_name != "salvum" {
  output.push_str("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
  output.push_str("~~~~~~~~~~~ Apps ~~~~~~~~~~~~\n");
  output.push_str("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
  // Iterate through the links vector to retrieve help descriptions for each linked event
  for event_name in current_event.links.iter() {
    // Core applets should be ignored
    if event_name == "blue" || event_name == "red" || event_name == "salvum" { continue; }
    match events_map.get(event_name) {
      Some(&index) => { // Get the description associated with the cur link
        let desc = &events[index].get_event().desc;
        output.push_str(&format!("'{}': {}\n", event_name, desc)[..]); // Append output with help description for link
      }
      _ => *err = errno::ERR_MISMATCHED_LINKS
    }
  }
  }
  output.push_str("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
  output.push_str("~~~~~~~~~~~~ Applets ~~~~~~~~~~~~\n");
  output.push_str("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
  let mut salvum_event: &events::Event;
  for (idx,_event) in events.iter().enumerate() {
    if events[idx].get_event().name == "salvum" {
      salvum_event = events[idx].get_event();
			for event_name in salvum_event.links.iter() {
				if event_name == "blue" || event_name == "red" || event_name == "salvum" {
					continue;
				}
				match events_map.get(event_name) {
					Some(&index) => { // Get the description associated with the cur link
						let desc = &events[index].get_event().desc;
						output.push_str(&format!("'{}': {}\n", event_name, desc)[..]);
					}
					_ => *err = errno::ERR_MISMATCHED_LINKS
				}
			}
    }
  }
  output.push_str("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
  print::print(&output);
}

/*
 * print_usage
 * 
 * @brief Prints out the usage info for a given command
 * @param events: Vector of all included events
 * @param events_map: Map of events to ids
 * @param cmnd_arg: 
 * @param err: Reference to a value used for error checking
 */
fn print_usage(events: &Vec<Box<dyn events::Eventable + Send + Sync>>,
               events_map: &HashMap<String, usize>,
               cmnd_arg: String,
               err: &mut i32) {
  let mut cmnd = cmnd_arg.clone();
  // Display all available commands and prompt
  if cmnd_arg == "" {
    cmnd = print_modules_menu(events, events_map, "Usage", err);
  }

  // Check the map for the command
  match events_map.get(&cmnd) {
    // If it has a usage, print it out
    Some(&index) => {
      let usage = &events[index].get_event().usage;
      if usage == "" { print::print_custom("No usage available to print.\n","orange"); }
      else { print::print_custom(&format!("{}", usage),"lightorange"); }
    }
    // Command doesn't exist
    _ => {
      if cmnd_arg.len() > 0 {
        print::print_custom(&format!("The command: \"{}\", doesn't exist\n", cmnd_arg),"orange");
      }
      else {
        print::print_custom(&format!("To use usage helper, navigate to a direct parent of modules.\n"),"orange");
      }
    }
  }
}

/*
 * print_jobs
 * 
 * @brief Prints out the jobs currently running
 */
fn print_jobs() {
  let threads_guard = match main_info::BG_THREADS.lock() {
    Ok(lock) => lock,
    Err(err) => {
      print!("Thread currently holding this mutex panicked: {}\n\r", err);
      return;
    }
  };
  if threads_guard.len() > 0 {
    // Get the longest character length
    let mut longest_cmnd = 20;
    for thread in &*threads_guard {
      let (t_cmnd, _, _, _) = thread;
      if t_cmnd.len() > longest_cmnd { longest_cmnd = t_cmnd.len(); }
    }

    // Title
    println!("+{0:->width$}+{0:->15}+{0:->10}+", "", width=(longest_cmnd + 2));
    println!("|{:^width$}|{:^15}|{:^10}|", "Command", "TID", "Status", width=(longest_cmnd + 2));
    println!("+{0:->width$}+{0:->15}+{0:->10}+", "", width=(longest_cmnd + 2));

    for thread in &*threads_guard {
      let (t_cmnd, _, tid, stat) = thread;
      println!("| {:<width$} | {:<13} | {:<8} |", t_cmnd, format!("{:?}", tid), format!("{}", stat), width=(longest_cmnd));
    }
    
    println!("+{0:->width$}+{0:->15}+{0:->10}+", "", width=(longest_cmnd + 2));
  } else {
    println!("There are currently no running jobs");
  }
}

/*
 * print_alias
 * 
 * @brief Prints out the alias info for a given command
 * @param cmnd_arg: 
 */
fn print_alias(cmnd_arg: String) {
  terminal::aliases::ALIAS_MAP.with(|alias_map| {
    let refer = &*alias_map.borrow();

    // If no arg was given, print everything
    if cmnd_arg == "" {
      terminal::aliases::ALIAS_MAP.with(|alias_map| {
        for (alias, value) in &*alias_map.borrow() {
          print::print_custom(alias,"neongreen");
          print::print_custom(" --> ", "grey");
          print::print_custom(value,"rose");
          println!();
          //println!("{} => {}", alias, value);
        }
      });
    // If an arg was given, print only that alias
    } else {
      match refer.get(&cmnd_arg) {
        Some(value) => {
          print::print_custom(&cmnd_arg,"neongreen");
          print::print_custom(" --> ", "grey");
          print::print_custom(value,"rose");
          println!();
          //println!("{} => {}", cmnd, value);
        }
        None => {
          let mut tmp = String::from(&cmnd_arg);
          tmp.push_str(" isn't an alias.\n");
          print::print_custom(&tmp,"orange");
        }
      };
    }
  });
}

/*
 * print_author
 * 
 * @brief Prints out the author info for a given command
 * @param events: Vector of all included events
 * @param events_map: Map of events to ids
 * @param cmnd_arg: 
 * @param err: Reference to a value used for error checking
 */
fn print_description(events: &Vec<Box<dyn events::Eventable + Send + Sync>>,
                events_map: &HashMap<String, usize>,
                cmnd_arg: String,
                err: &mut i32) {
  let mut cmnd = cmnd_arg.clone();
  // Display all available commands and prompt
  if cmnd_arg == "" {
    cmnd = print_modules_menu(events, events_map, "Description", err);
  }

  // Check the map for the command
  match events_map.get(&cmnd) {
    // If it has a author, print it out
    Some(&index) => {
      let desc = &events[index].get_event().desc;
      if desc == "" { print::print_custom("No description information to show. This is not intentional.\n\nPlease contact security@qvlx.com so we can add this. We thank you for helping us improve Salvum.\n","orange"); }
      else { print::print_custom(&format!("{}\n", desc), "bluegreen"); }
    }
    // Command doesn't exist
    _ => print::print_custom(&format!("The command: \"{}\", doesn't exist\n", cmnd_arg),"orange")
  }
}

/*
 * print_author
 * 
 * @brief Prints out the author info for a given command
 * @param events: Vector of all included events
 * @param events_map: Map of events to ids
 * @param cmnd_arg: 
 * @param err: Reference to a value used for error checking
 */
 fn print_author(events: &Vec<Box<dyn events::Eventable + Send + Sync>>,
                 events_map: &HashMap<String, usize>,
                 cmnd_arg: String,
                 err: &mut i32) {
  let mut cmnd = cmnd_arg.clone();
  // Display all available commands and prompt
  if cmnd_arg == "" {
    cmnd = print_modules_menu(events, events_map, "Author", err);
  }

  // Check the map for the command
  match events_map.get(&cmnd) {
  // If it has a author, print it out
  Some(&index) => {
    let author = &events[index].get_event().author;
    if author == "" { print::print_custom("No author information to show.\n","orange"); }
    else { println!("{}", author.bright_green()); }
  }
  // Command doesn't exist
  _ => print::print_custom(&format!("The command: \"{}\", doesn't exist\n", cmnd_arg),"orange")
  }
}

/*
 * print_sec
 * 
 * @brief Prints out the secure info for a given command
 * @param events: Vector of all included events
 * @param events_map: Map of events to ids
 * @param cmnd_arg: 
 * @param err: Reference to a value used for error checking
 */
fn print_sec(events: &Vec<Box<dyn events::Eventable + Send + Sync>>,
             events_map: &HashMap<String, usize>,
             cmnd_arg: String,
             err: &mut i32) {
  let mut cmnd = cmnd_arg.clone();
  // Display all available commands and prompt
  if cmnd_arg == "" {
    cmnd = print_modules_menu(events, events_map, "Secure", err);
  }

  // Check the map for the command
  match events_map.get(&cmnd) {
    // If it has a sec, print it out
    Some(&index) => {
      let sec = &events[index].get_event().secure;
      if !sec { print::print_custom("This module isn't cryptographically secure.\n","rose"); }
      else { print::print_custom("This module is cryptographically secure.\n","neongreen"); }
    }
    // Command doesn't exist
    _ => print::print_custom(&format!("The command: \"{}\", doesn't exist\n", cmnd_arg), "grey")
  }
}

/*
 * print_license
 * 
 * @brief Prints out the license info for a given command
 * @param events: Vector of all included events
 * @param events_map: Map of events to ids
 * @param cmnd_arg: 
 * @param err: Reference to a value used for error checking
 */
fn print_license(events: &Vec<Box<dyn events::Eventable + Send + Sync>>,
                 events_map: &HashMap<String, usize>,
                 cmnd_arg: String,
                 err: &mut i32) {
  let mut cmnd = cmnd_arg.clone();
  // Display all available commands and prompt
  if cmnd_arg == "" {
    cmnd = print_modules_menu(events, events_map, "License", err);
  }

  // Check the map for the command
  match events_map.get(&cmnd) {
    // If it has a license, print it out
    Some(&index) => {
      let name = &events[index].get_event().name;
      let file_str = match fs::read_to_string(&format!("lic/{}.lic", name)) {
        Ok(out) => out,
        Err(_) => String::from(""),
      };
      if file_str == "" { print::print_custom("No license on file. Regardless, please respect original author contributions as intended.\n","orange"); }
      else { print::print_custom(&file_str, "gold"); }
    }
    // Command doesn't exist
    _ => print::print_custom(&format!("The command: \"{}\", doesn't exist\n", cmnd_arg),"orange")
  }
}

/*
 * update_mode
 *
 * @brief Climbs to the top of the tree to determine if the parent is red or blue
 * @param events: Vector of all included events
 * @param events_map: Map of events to ids
 * @param id: Needed to determine if an event exists
 */
fn update_mode(events: &Vec<Box<dyn events::Eventable + Send + Sync>>,
               events_map: &HashMap<String, usize>,
               id: usize) {
  // Changes mode every time an event is entered
  let mut climb: &events::Event = events[id].get_event();
  loop {
    if climb.name == "red" {
      terminal::set_color_mode(terminal::Mode::Red);
      break;
    } else if climb.name == "blue" {
      terminal::set_color_mode(terminal::Mode::Blue);
      break;
    }
    match events_map.get(&climb.parent) {
      Some(&parent_id) => {
        climb = events[parent_id].get_event();
      }
      None => {
        terminal::set_color_mode(terminal::Mode::Yellow);
        break;
      }
    }
  }
}

/*
 * handle_input
 *
 * @brief Takes the user input and determines if an event needs to be run
 * @param usr_input: Has the name of the event, and potentially args following
 * @param events: Vector of all included events
 * @param events_map: Determines if an event exists
 * @param err: Reference to a value used for error checking
 */
fn handle_input(usr_input: String,
                events: &Vec<Box<dyn events::Eventable + Send + Sync>>,
                events_map: &HashMap<String, usize>,
                err: &mut i32) {
  let next_id: usize;
  let current_event: &events::Event = events[main_info::get_last_event()].get_event();
  let input_args: Vec<&str> = usr_input.split(" ").collect();

  // Find the id of the event by accessing the map
  match events_map.get(input_args[0]) {
    // An id was found in the map
    Some(&index) => {
      match input_args[0] {
        "salvum" => { // Command exists, but is invalid
          print::print_custom("Invalid input.\n","orange");
          debug::print_debug(format!("'{}'", input_args[0]));
          return;
        },
        _ => {        // Proceed to the event lifecycle
          //...unless the command isn't linked
          // Uncomment this if we want to re-enable this feature again
          /*if !current_event.links.contains(&input_args[0].to_string()) {
            println!("Invalid input.");
            return;
          }*/
        }
      }
      next_id = index;
    }
    // No id was found in the map
    _ => {
      match input_args[0] {
        // If the user typed 'back', go to the previously typed parent event
        "back" | "b" => {
          // Pop the last id off the stack
          match main_info::BACK_HISTORY.with(|back_history| {
            back_history.borrow_mut().pop()
          }) {
            Some(back_id) => {
              main_info::set_last_event(back_id);
              update_mode(events, events_map, back_id);
              let msg = format!("Returned back to {}.", events[main_info::get_last_event()].get_event().name.to_title_case());
              print::print_custom(".","lightbluegreen");
              print::print_custom(".","gold");
              print::print_custom(".","purple");
              print::println(&msg);
              return;
            }
            _ => {
              print::print_custom("Nothing to go back to, the history is empty.\n","orange");
              return;
            }
          };
        }
        // If the user typed 'parent', go to the parent event
        "parent" | "p" | "ancestor" | "up" => {
          if &current_event.parent != "salvum" {
            match events_map.get(&current_event.parent) {
              Some(&back_id) => {
                main_info::set_last_event(back_id);
                let msg = format!("Stepped up to {}.", events[main_info::get_last_event()].get_event().name.to_title_case());
                print::print_custom(".","lightbluegreen");
                print::print_custom(".","gold");
                print::print_custom(".","purple");
                print::println(&msg);
                return;
              }
              _ => {
                print::print_custom("Can't go up from this menu.\n","orange");
                return;
              }
            }
          } else {
            print::print_custom("Can't go up from this menu.\n","orange");
            return;
          }
        }
        "clear" | "c" => {
          print!("{}{}", termion::cursor::Up(std::u16::MAX), termion::clear::All);
          return;
        }
        // If the user typed 'menu', send to print_menu
        "menu" | "m" => {
          print_menu(events, events_map, err);
          return;
        }
        // If the user typed 'splash', send to print_splash
        "splash" => {
          print_splash();
          return;
        }
        // If the user typed 'modules', send to print_modules
        "modules" => {
          print_modules(events, events_map, err);
          return;
        }
        // If the user typed 'menu', send to print_menu
        "map" => {
          if input_args.len() > 2 {
            return;
          }
          if input_args.len() == 1 {
						use std::process::Command;
						print::print_custom("Use a,w,s,d to nagivate the tree in the console. q to quit.\n","bluegreen");
						let output = match Command::new("ext/util/peep").args(vec!["-n","24","ext/util/map.txt"]).spawn() {
							Ok(o) => (o),
							Err(_) => { return; },
						};
						match output.wait_with_output() {
							Ok(_) => {}
							Err(_) => {}
						};
            print!("{}{}", termion::cursor::Up(std::u16::MAX), termion::clear::All);
            return;
          }
          if input_args[1].to_string().eq("blue") {
						use std::process::Command;
						print::print_custom("Use a,w,s,d to nagivate the tree in the console. q to quit.\n","bluegreen");
						let output = match Command::new("ext/util/peep").args(vec!["-n","24","ext/util/blue_map.txt"]).spawn() {
							Ok(o) => (o),
							Err(_) => { return; },
						};
						match output.wait_with_output() {
							Ok(_) => {}
							Err(_) => {}
						};
            print!("{}{}", termion::cursor::Up(std::u16::MAX), termion::clear::All);
            return;
          }
          if input_args[1].to_string().eq("red") {
						use std::process::Command;
						print::print_custom("Use a,w,s,d to nagivate the tree in the console. q to quit.\n","bluegreen");
						let output = match Command::new("ext/util/peep").args(vec!["-n","24","ext/util/red_map.txt"]).spawn() {
							Ok(o) => (o),
							Err(_) => { return; },
						};
						match output.wait_with_output() {
							Ok(_) => {}
							Err(_) => {}
						};
            print!("{}{}", termion::cursor::Up(std::u16::MAX), termion::clear::All);
            return;
          }
          return;
        }
        // If the user typed 'usage', send to print_usage
        "usage" | "u" => {
          if input_args.len() > 1 {
            print_usage(events, events_map, input_args[1].to_string(), err);
          } else {
            print_usage(events, events_map, "".to_string(), err);
          }
          return;
        }
        // If the user typed 'jobs', list all background processes
        "jobs" | "j" => {
          print_jobs();
          return;
        }
        // If the user typed 'alias', expand it
        "alias" => {
          if input_args.len() > 1 {
            print_alias(input_args[1].to_string());
          } else {
            print_alias("".to_string());
          }
          return;
        }
        // If the user typed 'sec', send to print_sec
        "secure" | "s" => {
          if input_args.len() > 1 {
            print_sec(events, events_map, input_args[1].to_string(), err);
          } else {
            print_sec(events, events_map, "".to_string(), err);
          }
          return;
        }
        // If the user typed 'author', send to print_author
        "author" | "a" => {
          if input_args.len() > 1 {
            print_author(events, events_map, input_args[1].to_string(), err);
          } else {
            print_author(events, events_map, "".to_string(), err);
          }
          return;
        }
        // If the user typed 'about', send to print_description
        "about" | "desc" | "description" | "d" => {
          if input_args.len() > 1 {
            print_description(events, events_map, input_args[1].to_string(), err);
          } else {
            print_description(events, events_map, "".to_string(), err);
          }
          return;
        }
        // If the user typed 'license', send to print_license
        "license" | "l" => {
          if input_args.len() > 1 {
            print_license(events, events_map, input_args[1].to_string(), err);
          } else {
            print_license(events, events_map, "".to_string(), err);
          }
          return;
        }
        // If the user typed 'note', print out the legal disclaimer
        "note" | "n" => {
          alerts::print_disclaimer();
          return;
        }
        // If the user typed 'term', print out the possible terminal colors
        "term" => {
          let child = match std::process::Command::new("ext/util/color256").spawn() {
            Ok(out) => out,
            Err(_) => { return; }
          };
        
          match child.wait_with_output() {
            Ok(_) => (),
            Err(e) => { println!("{}", e); }
          };
          return;
        }
        // If the user typed 'docs', show the paging documentation
        "documentation" | "docs" | "manual" | "help" => {
          let arguments = vec!["--paging=always","--color=always","-l","TOML","-p",
                              "-H","6","-H","16","-H","23",
                              "ext/util/man.txt"];
          let child = match std::process::Command::new("ext/batcat/batcat").args(arguments).spawn() {
            Ok(out) => out,
            Err(_) => { return; }
          };
          match child.wait_with_output() {
            Ok(_) => (),
            Err(e) => { println!("{}", e); }
          };
          print!("{}{}", termion::cursor::Up(std::u16::MAX), termion::clear::All);
          return;
        }
        "test" | "t" => {
          test::test(input_args[1..].to_vec(), events, events_map);
          return;
        }
        // If the user typed 'exit' or 'quit', make sure they wanted to
        "exit" | "e" | "quit" | "q" => {
          alerts::confirm_exit();
          return;
        }
        // If the user typed 'exit' or 'quit', make sure they wanted to
        "deprivilege" => {
          util::security::deprivilege();
          return;
        }
        // Exits without a confirmation
        "forcequit" => {
          alerts::print_exit();
          return;
        }
        // If the user typed some form of enter, do nothing
        "" | "\n" | "\r" => return,
        // Otherwise invalid
        _ => {
          println!("Invalid input.");
          debug::print_debug(format!("'{}'", input_args[0]));
          return;
        }
      }
    }
  }

  event_life_cycle(input_args, events, events_map, next_id);
}

/*
 * event_life_cycle
 * 
 * @brief
 * @param input_args: Has the name of the event, and potentially args following
 * @param events: Vector of all included events
 * @param events_map: Determines if an event exists
 * @param next_id: id of the event that the user typed
 */
fn event_life_cycle(mut input_args: Vec<&str>,
                    events: &Vec<Box<dyn events::Eventable + Send + Sync>>,
                    events_map: &HashMap<String, usize>,
                    next_id: usize) {
  let current_event: &events::Event = events[main_info::get_last_event()].get_event();
  let current_id = main_info::get_last_event();

  main_info::set_last_event(next_id);

  let old_mode = terminal::get_color_mode();
  update_mode(events, events_map, next_id);

  /*** Send this process to the background ***/
  if input_args.len() > 1 && input_args[input_args.len()-1] == "&" {
    input_args.pop();
    // Grab args with on_init
    if input_args.len() == 1
      && (!events[next_id].get_event().easyrun
        || (current_event.links.contains(&input_args[0].to_string()) && current_event.name != "salvum")) {
      let init_args = events[next_id].on_init();
      spawn_background(next_id, init_args);
    // Args provided
    } else {
      let args_remap: Vec<String> = input_args[1..].iter().map(|s| s.to_string()).collect();
      spawn_background(next_id, args_remap);
    }
  /*** Pipe output to file ***/
  } else if input_args.len() > 2 && input_args[input_args.len()-2] == ">" {
    let output_file = input_args[input_args.len()-1].to_string();
    input_args.pop();
    input_args.pop();
    // Grab args with on_init
    main_info::set_file_redirect(true);
    if input_args.len() == 1
      && (!events[next_id].get_event().easyrun
        || (current_event.links.contains(&input_args[0].to_string()) && current_event.name != "salvum")) {
      let init_args = events[next_id].on_init();
      let output_str = events[next_id].on_run(init_args);
      util::misc::write_file(output_str, output_file);
    // Args provided
    } else {
      let args_remap: Vec<String> = input_args[1..].iter().map(|s| s.to_string()).collect();
      let output_str = events[next_id].on_run(args_remap);
      util::misc::write_file(output_str, output_file);
    } 
    main_info::set_file_redirect(false);
  /*** Normal execution ***/
  } else {
    // Grab args with on_init
    if input_args.len() == 1
      && (!events[next_id].get_event().easyrun
        || (current_event.links.contains(&input_args[0].to_string()) && current_event.name != "salvum")) {
      let init_args = events[next_id].on_init();
      print!("{}", events[next_id].on_run(init_args));
    // Args provided
    } else {
      // Send to on_run with args
      if input_args.len() > 1 {
        print!("{}", events[next_id].on_run(input_args[1..].iter().map(|s| s.to_string()).collect()));
      // Send to on_run with no args
      } else {
        print!("{}", events[next_id].on_run(Vec::new()));
      }
    }
  }

  let next_event: &events::Event = events[next_id].get_event();
  // If the event is a module, don't fully enter it
  if events::is_module(next_event) {
    terminal::set_color_mode(old_mode);
    main_info::set_last_event(current_id);
  // If it's a parent, fully enter the event
  } else {
    // Don't add to history if 'back' was typed, and don't add salvum to the history
    if main_info::get_last_event() < events.len() - 1 {
      main_info::BACK_HISTORY.with(|back_history| {
        // Don't let the history exceed 20
        if back_history.borrow().len() == 20 {
          back_history.borrow_mut().remove(0);
        }
        back_history.borrow_mut().push(main_info::get_last_event());
      });
    }
  }
}

/*
 * spawn_background
 * 
 * @brief
 * @param next_id: 
 * @param full_args: 
 * @return 
 */
fn spawn_background(next_id: usize,
                    full_args: Vec<String>) -> std::thread::JoinHandle<()> {
  let thread_bg_ansi = match terminal::get_color_mode() {
    terminal::Mode::Yellow => "  \x1b[43m\x1b[30m",
    terminal::Mode::Blue => "  \x1b[46m\x1b[30m",
    terminal::Mode::Red => "  \x1b[41m\x1b[30m",
    terminal::Mode::Magenta => "  \x1b[45m\x1b[30m"
  };
  
  return std::thread::spawn(move || {
    let events = events::get_events();

    // Create the display name
    let mut args_str = events[next_id].get_event().name.clone() + " ";
    args_str.push_str(&full_args.join(" "));

    // Add this thread to invokers
    {
      let mut threads_guard = match main_info::BG_THREADS.lock() {
        Ok(lock) => lock,
        Err(err) => {
          print!("Thread currently holding this mutex panicked: {}\n\r", err);
          return;
        }
      };
      threads_guard.push((args_str.clone(), String::from(""), std::thread::current().id(), main_info::ThreadStatus::Running));
    }

    let (term_width, _) = match termion::terminal_size() {
      Ok(size) => size,
      Err(_) => (0, 0),
    };

    // Create the thread card
    let tid_card = format!("{0}\n{1}{2:<80}\n{0}\n",
      print::print_rainbow(
        format!("{:-^width$}", "", width=(term_width as usize))
      ),
      thread_bg_ansi,
      format!("TID {:?}: {}\x1b[0m", std::thread::current().id(), args_str.clone())
    );
    terminal::write_thread(format!("{}\x1b[0m", tid_card));

    // Run the event
    let out = events[next_id].on_run(full_args);
    terminal::write_thread(format!("{}\n", out));
    terminal::write_alt();

    // Let the main thread know we're done
    {
      let mut threads_guard = match main_info::BG_THREADS.lock() {
        Ok(lock) => lock,
        Err(err) => {
          print!("Thread currently holding this mutex panicked: {}\n\r", err);
          return;
        }
      };
      for i in 0..threads_guard.len() {
        let (_, t_buf, tid, _) = threads_guard[i].clone();
        if tid == std::thread::current().id() {
          threads_guard[i] = (args_str.clone(), t_buf, std::thread::current().id(), main_info::ThreadStatus::Finished);
          break;
        }
      }
    }
  });
}

/*
 * thread_popup
 * 
 * @brief
 */
fn thread_popup() {
  // Determine if any threads are done
  let mut finished_str = String::new();
  let mut finished_idx = Vec::new();
  let mut threads_guard = match main_info::BG_THREADS.lock() {
    Ok(lock) => lock,
    Err(err) => {
      print!("Thread currently holding this mutex panicked: {}\n\r", err);
      return;
    }
  };
  for i in 0..threads_guard.len() {
    let (t_cmnd, _, tid, stat) = threads_guard[i].clone();
    if stat == main_info::ThreadStatus::Finished {
      finished_str.push_str(&format!("{:?}: {}\n", tid, t_cmnd));
      finished_idx.push(i);
    }
  }

  // If any were found
  if finished_idx.len() > 0 {
    for _ in 0..finished_idx.len() {
      let idx = match finished_idx.pop() {
        Some(idx) => idx,
        None => break,
      };
      threads_guard.remove(idx);
    }

    // Display cursive pop-up window here
    let mut siv = cursive::termion();
    let mut theme = cursive::theme::Theme::default();
    theme.palette.set_color("Background", cursive::theme::Color::Dark(cursive::theme::BaseColor::Black));
    siv.set_theme(theme);
    {
      siv.add_layer(cursive::views::Dialog::text(
        format!("Completed Jobs:\n\
                 -----------------------------------------\n\
                 {}\
                 -----------------------------------------\n\n\
                 Hitting 'Ctrl + a' in the Salvum main console will\n\
                 allow you to view output from all background tasks.",
                 finished_str)).button("Close", |s| { s.quit(); }));
    }
    siv.run();
    
    // For some reason this prevents the prompt from double printing
    std::thread::sleep(std::time::Duration::from_millis(10));
  }
}

/*
 * handle_cli
 *
 * @brief Takes the environment args and runs either tests or a single module
 * @param args: User args from the environment
 * @param events: Vector of all included events
 * @param events_map: Determines if an event exists
 */
fn handle_cli(args: Vec<String>,
              events: &Vec<Box<dyn events::Eventable + Send + Sync>>,
              events_map: &HashMap<String, usize>) -> bool {
  match args[1].as_str() {
    /********** Testing component **********/
    "test" => {
      test::test(args, events, events_map);
    }
    /********** Run the cryptography secure checks **********/
    "secure" => {
      let longest_name = match events_map.keys().map(String::len).max() {
        Some(out) => out,
        None => 20
      };
  
      // Title
      println!("+{0:->width$}+", "", width=(longest_name + 2));
      println!("|{:^width$}|", "Module Name", width=(longest_name + 2));
      println!("+{0:->width$}+", "", width=(longest_name + 2));
  
      /********** Print all secure modules **********/
      for event in events {
        if event.get_event().secure {
          print!("| ");
          print::print_custom(&format!("{:>width$}", event.get_event().name, width=longest_name), "orange");
          println!(" |");
        }
      }
  
      // Closing line
      println!("+{0:->width$}+", "", width=(longest_name + 2));
    }
    /********** Run through the events map to print all modules **********/
    "list" => {
      let longest_name = match events_map.keys().map(String::len).max() {
        Some(out) => out,
        None => 20
      };
  
      // Title
      println!("+{0:->width$}+", "", width=(longest_name + 2));
      println!("|{:^width$}|", "Module Name", width=(longest_name + 2));
      println!("+{0:->width$}+", "", width=(longest_name + 2));
  
      /********** Print all modules **********/
      for event in events {
        if events::is_module(event.get_event()) {
          print!("| ");
          print::print_custom(&format!("{:>width$}", event.get_event().name, width=longest_name), "orange");
          println!(" |");
        }
      }
  
      // Closing line
      println!("+{0:->width$}+", "", width=(longest_name + 2));
    }
    /********** Generate the salvum config **********/
    "config" => {
      config::generate();
    }
    /********** Generate the report config **********/
    "report" => {
      reports_coder::generate();
    }
    #[cfg(debug_assertions)]
    "package" => {
      edition_packager::generate(args[2].clone());
    }
    /********** Assume it's a command and attempt the lifecycle **********/
    _ => {
      // Get the id of the module passed in
      let id = match events_map.get(&args[1]) {
        Some(&id) => id,
        None => {
          return true;
        }
      };
  
      if events::is_module(events[id].get_event()) {
        // Changes mode every time an event is entered
        update_mode(events, &events_map, id);
  
        // Run through the event lifecycle
        if args.len() > 2 {
          print!("{}", events[id].on_run(args[2..].to_vec()));
        } else {
          let init_args = events[id].on_init();
          print!("{}", events[id].on_run(init_args));
        }
      }
    }
  }

  return true;
}
