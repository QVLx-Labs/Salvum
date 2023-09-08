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
 * main.rs -> orchestrating program for Salvum
 *
 * authors: $t@$h, r00r00, n3wmAn
 */

use colored::Colorize;
use rand::Rng;

use crate::main_info;
use crate::terminal;

/*
 * print
 *
 * @brief uses the mode to determine what color to print in
 * @param msg: What is displayed
 * @param mode: What color to display it in
 */
pub fn print(msg: &str) {
  let color_msg = match terminal::get_color_mode() {
    terminal::Mode::Yellow => msg.yellow(),
    terminal::Mode::Red => msg.red(),
    terminal::Mode::Blue => msg.cyan(),
    terminal::Mode::Magenta => msg.magenta()
  };

  if main_info::is_bg() {
    terminal::write_thread(color_msg);
  } else {
    print!("{}", color_msg);
    terminal::flush();
  }
}

/*
 * println
 *
 * @brief uses the mode to determine what color to println in
 * @param msg: What is displayed
 * @param mode: What color to display it in
 */
pub fn println(msg: &str) {
  let color_msg = format!("{}\n", match terminal::get_color_mode() {
    terminal::Mode::Yellow => msg.yellow(),
    terminal::Mode::Red => msg.red(),
    terminal::Mode::Blue => msg.cyan(),
    terminal::Mode::Magenta => msg.magenta()
  });

  if main_info::is_bg() {
    terminal::write_thread(color_msg);
  } else {
    print!("{}", color_msg);
    terminal::flush();
  }
}

/*
 * print
 *
 * @brief uses the mode to determine what color to print in
 * @param msg: What is displayed
 */
pub fn print_neutral(msg: &str) {
  if main_info::is_bg() {
    terminal::write_thread(msg);
  } else {
    print!("{}", msg);
    terminal::flush();
  }
}

/*
 * println
 *
 * @brief uses the mode to determine what color to println in
 * @param msg: What is displayed
 */
pub fn println_neutral(msg: &str) {
  let fmt_msg = format!("{}\n", msg);
  if main_info::is_bg() {
    terminal::write_thread(fmt_msg);
  } else {
    print!("{}", fmt_msg);
    terminal::flush();
  }
}

/*
 * print_custom
 * 
 * @brief
 * @param msg:
 * @param color:
 * @return none
 */
pub fn print_custom(msg: &str, color: &str) {
  let color_msg = match color {
    "green" => format!("{}", msg.green()),
    "cyan" => format!("{}", msg.cyan()),
    "red" => format!("{}", msg.red()),
    "magenta" => format!("{}", msg.magenta()),
    "grey" => format!("\x1b[38;5;110m{}\x1b[0m", msg),
    "brightgreen" => format!("{}", msg.bright_green()),
    "orange" => format!("\x1b[38;5;208m{}\x1b[0m", msg),
    "lightorange" => format!("\x1b[38;5;214m{}\x1b[0m", msg),
    "purple" => format!("\x1b[38;5;141m{}\x1b[0m", msg),
    "white" => format!("{}", msg.bright_white()),
    "gold" => format!("\x1b[38;5;11m{}\x1b[0m", msg),
    "rose" => format!("\x1b[38;5;203m{}\x1b[0m", msg),
    "neongreen" => format!("\x1b[38;5;154m{}\x1b[0m", msg),
    "bluegreen" => format!("\x1b[38;5;85m{}\x1b[0m", msg),
    "lightbluegreen" => format!("\x1b[38;5;83m{}\x1b[0m", msg),
    "brightblue" => format!("\x1b[38;5;12m{}\x1b[0m", msg),
    "reset" => format!("\x1b[0m"),
    _ => format!("{}", msg)
  };

  if main_info::is_bg() {
    terminal::write_thread(color_msg);
  } else {
    print!("{}", color_msg);
    terminal::flush();
  }
}

/*
 * print_custom_uncapped
 * 
 * @brief
 * @param msg:
 * @param color:
 * @return none
 */
pub fn print_custom_uncapped(msg: &str, color: &str) {
  let color_msg = match color {
    "grey" => format!("\x1b[38;5;110m{}", msg),
    "orange" => format!("\x1b[38;5;208m{}", msg),
    "lightorange" => format!("\x1b[38;5;214m{}", msg),
    "purple" => format!("\x1b[38;5;141m{}", msg),
    "gold" => format!("\x1b[38;5;11m{}", msg),
    "rose" => format!("\x1b[38;5;203m{}", msg),
    "lightbluegreen" => format!("\x1b[38;5;83m{}", msg),
    "brightblue" => format!("\x1b[38;5;12m{}", msg),
    "neongreen" => format!("\x1b[38;5;154m{}", msg),
    _ => format!("{}", msg)
  };

  if main_info::is_bg() {
    terminal::write_thread(color_msg);
  } else {
    print!("{}", color_msg);
    terminal::flush();
  }
}

pub fn print_rainbow(s: String) -> String {
  let ansi_code = String::from("\x1b[38;5;");
  let mut colored_string = String::new();
  let raw = s.chars();
  let mut count = 0;
  let mut code = 0;
  for (_i,character) in raw.enumerate() {
    if count == 28 { count = 0; } 
    colored_string.push_str(&ansi_code);
    if count == 0 || count == 1 { code = 33 }; // darker blue
    if count == 2 || count == 3 { code = 37 }; // lighter blue
    if count == 4 || count == 5 { code = 51 }; // blue green
    if count == 6 || count == 7 { code = 83 }; // lighter bluegreen
    if count == 8 || count == 9 { code = 154 }; // neon green
    if count == 10 || count == 11 { code = 226 }; // bright yellow
    if count == 12 || count == 13 { code = 214 }; // light orange
    if count == 14 || count == 15 { code = 208 }; // darker orange
    if count == 16 || count == 17 { code = 203 }; // rose
    if count == 18 || count == 19 { code = 162 }; // pink
    if count == 20 || count == 21 { code = 165 }; // magenta
    if count == 22 || count == 23 { code = 129 }; // light purple
    if count == 24 || count == 25 { code = 55 }; // darker purple
    if count == 26 || count == 27 { code = 62 }; // darker purple
    colored_string.push_str(&code.to_string()[..]);
    colored_string.push_str("m");
    colored_string.push_str(&character.to_string());
    colored_string.push_str("\x1b[0m");
    if character != ' ' &&
       character != '\n' &&
       character != '\r' &&
       character != '\t' {
         count = count + 1;
    }   
  }
  //println!("{}", colored_string);
  return colored_string;
}

#[allow(dead_code)]
pub fn print_colorful(s: String) -> String {
  let ansi_code = String::from("\x1b[38;5;");
  let mut colored_string = String::new();
  let raw = s.chars();
  for (_i,character) in raw.enumerate() {
    let r = rand::thread_rng().gen_range(1..6);
    let g = rand::thread_rng().gen_range(1..6);
    let b = rand::thread_rng().gen_range(1..6);
    let non_system = 16 + (r * 36) + (g * 6) + b;
    let system_color = rand::thread_rng().gen_range(1..9);
    let shuffler = rand::thread_rng().gen_range(0..7);
    let mut code = non_system;
    if shuffler > 5 { code = system_color; }
    colored_string.push_str(&ansi_code);
    colored_string.push_str(&code.to_string()[..]);
    colored_string.push_str("m");
    colored_string.push_str(&character.to_string());
    colored_string.push_str("\x1b[0m"); 
  }
  //println!("{}", colored_string);
  return colored_string;
}
