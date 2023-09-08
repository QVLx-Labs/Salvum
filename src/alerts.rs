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
 * alerts.rs -> Group of functions for easily calling when a user needs to be notified
 *
 * authors: $t@$h, r00r00, n3wmAn
 */

// External files
use crate::constants;
use crate::events;
use crate::main_info;
use crate::print;
use crate::terminal;
use crate::util;
/*
 * confirm_task
 * 
 * @brief Will display a question to the user that requires a yes/no response.
 * Background processes will force a NO
 * @param msg: Prompt to print out
 * @return if the user's input started with (y|Y)
 */
pub fn confirm_task(msg: &str) -> bool {
  // Don't allow confirm tasks in background processes
  if main_info::is_bg() {
    return constants::UNCONFIRMED;
  }

  let old_mode = terminal::get_color_mode();
  terminal::set_color_mode(terminal::Mode::Magenta);
  // When in array mode, use the interface
  let options = vec!["Yes", "No"];
  if terminal::get_array_mode() {
    print::println(msg);
    terminal::set_color_mode(old_mode);
    match terminal::get_selection(options.clone()) {
      Some(opt) => return if options[opt].to_lowercase() == "y" || options[opt].to_lowercase() == "yes" { constants::CONFIRMED } else { constants::UNCONFIRMED },
      _ => return constants::UNCONFIRMED,
    }
  // Otherwise do a CLI confirmation
  } else {
    let selection = match terminal::get_input(terminal::create_prompt(constants::CONSOLE_PROMPT, &format!("> {} (Y)es/(N)o:", msg))) {
      Ok(out) => out,
      Err(_) => String::from(""),
    };
    terminal::set_color_mode(old_mode);
    if selection.is_empty() {
      return constants::UNCONFIRMED;
    }
    return if selection.to_lowercase() == "y" || selection.to_lowercase() == "yes" { constants::CONFIRMED } else { constants::UNCONFIRMED };
  }
}

// This is a wrapper macro to force all alert functions to act the same
// by setting the proper mode, and then resetting once done.
macro_rules! alert_mode {
  (pub fn $func:ident() = $body:expr) => {
    pub fn $func() {
      let _old_mode = terminal::get_color_mode();
      terminal::set_color_mode(terminal::Mode::Magenta);
      $body;
      #[allow(unreachable_code)] { terminal::set_color_mode(_old_mode); }
    }
  };
  (pub fn $func:ident($arg:ident: $argt:ty) = $body:expr) => {
    pub fn $func($arg: $argt) {
      let _old_mode = terminal::get_color_mode();
      terminal::set_color_mode(terminal::Mode::Magenta);
      $body;
      #[allow(unreachable_code)] { terminal::set_color_mode(_old_mode); }
    }
  };
}

alert_mode! {
pub fn print_generic_warning(warning: &str) = {
  print::println(&format!("WARNING: {}", warning));
}}

alert_mode! {
pub fn print_disclaimer() = {
  print::println("----------------\n== Disclaimer ==\n----------------\n\
                     We don't take credit for all of the many applications integrated\n\
                     into Salvum that are provided by our beloved security communities.\n\
                     Brilliant people work hard to build and maintain these awesome tools.\n\
                     Please use the 'author' command for author information.\n----------------");
}}

alert_mode! {
pub fn print_warning() = {
  print::println("WARNING: QVLx does not support the use of any tools for\n\
                     malicious purposes nor do we take any responsibility for\n\
                     the misuse of software that we or our partners provide.\n\
                     Please use your skills to protect, not to inflict harm.\n\
                     Thank you.");
}}

alert_mode! {
pub fn print_notice() = {
  print::println("NOTICE: Salvum does not write any passwords to files or history.");
}}

alert_mode! {
pub fn print_advisory() = {
  print::println("ADVISORY: Secure file editing means no auto-backup (swap) files.\n\
                     Please save frequently to not avoiding losing your hard work.");
}}

alert_mode! {
pub fn confirm_exit() = {
  if confirm_task("Are you sure you'd like to quit?") == constants::CONFIRMED {
    print_exit();
  } else {
    print::print_custom("Exit cancelled.\n","orange");
  }
}}

alert_mode! {
pub fn print_exit() = {
  print::print_custom("Exiting...\n","orange");
  events::servf_stop_all(); // Comes from events/netloaders.rs
  util::security::deprivilege();
  std::process::exit(1);
}}
