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

// Imports
use atomic::{Atomic};
use console::{style, Style};
use dialoguer::{Select, theme::ColorfulTheme};
use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use once_cell::sync::Lazy;
use online::sync::check;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::sync::{atomic::{AtomicBool, Ordering::*}, Mutex};
use std::process::Command;
use termion::{cursor::DetectCursorPos, event::Key, input::TermRead, raw::IntoRawMode};

// External Files
use crate::events;
use crate::main_info;
use crate::print;
use crate::util;

// Enums
#[derive(Copy, Clone)]
pub enum Mode {
  Yellow,
  Red,
  Blue,
  Magenta
}

pub mod aliases {
  use std::cell::RefCell;
  use std::collections::HashMap;

  thread_local!(pub static ALIAS_MAP: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new()));
  thread_local!(pub static CMND_HISTORY: RefCell<Vec<String>> = RefCell::new(Vec::new()));

  pub fn set_alias_map(new_alias: HashMap<String, String>) {
    ALIAS_MAP.with(|alias_vec| {
      *alias_vec.borrow_mut() = new_alias;
    });
  }
}

// Globals
static ALT_BUFFER: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
static COLOR_MODE: Atomic<Mode> = Atomic::<Mode>::new(Mode::Yellow);
static ARRAY_MODE: AtomicBool = AtomicBool::new(false);

/*
 * get_input
 *
 * @brief spin up console, waits to receive user input
 * @param none
 * @return a string the user typed into the console
 */
pub fn get_input(prompt: String) -> Result<String, std::io::Error> {
  let mut stdout_main = match stdout().into_raw_mode() {
    Ok(raw) => raw,
    Err(err) => { return Err(err); }
  };

  // Print the prompt
  print::print(&prompt);
  flush();

  fn clear_line<W: std::io::Write>(sout: &mut termion::raw::RawTerminal<W>) {
    match write!(sout, "\n\r") {
      Ok(_) => {}
      Err(_) => {}
    };
  }

  let mut input: String = String::from("");
  let mut x_pos = 0;
  let mut history_pos = 0;
  let mut sim: Vec<String> = Vec::new();
  let mut tab_pos = 0;
  let mut prev_char = '0';

  // For every key press from stdin
  for raw in stdin().keys() {
    let (width, _) = match termion::terminal_size() {
      Ok(size) => size,
      Err(_) => (0, 0),
    };

    // Grab the character out of it
    let key = match raw {
      Ok(k) => k,
      Err(_) => { continue; }
    };

    let mut inc = 0;
    match key {
      /***** KEYS TO NEVER USE *****/
      /*
      CTRL+I MAPS TO TAB
      CTRL+J, CTRL+M MAPS TO ENTER
      CTRL+, CTRL+. MAPS TO SOMETHING?
      CTRL+[, MAPS TO ESC
      CTRL+Z, BAD HABIT
      CTRL+H, MAPS TO BACKSPACE BY ECLIPSE TM
      */
      /***** OK TO MAP *****/
      /*
        All possible keys have been mapped as of 11/17/2021
      */

      // Activate the array mode
      Key::Esc => {
        set_array_mode(true);
        clear_line(stdout_main.borrow_mut());

        input = String::from("");
        break;
      }
      // Display alternate screen
      Key::Ctrl('a') => {
        // Save the main screen cursor pos
        let (main_screen_x, main_screen_y) = match stdout().cursor_pos() {
          Ok(xy) => xy,
          Err(_) => { continue; }
        };

        // Setup cursor and display buffer
        {
          let guard = match ALT_BUFFER.lock() {
            Ok(lock) => lock,
            Err(err) => {
              print!("Thread currently holding this mutex panicked: {}\n\r", err);
              continue;
            }
          };
          match rp::run_pager(guard.to_string()) {
            Ok(_) => {}
            Err(err) => {
              print!("Failed to launch the background screen: {}\n\r", err);
              continue;
            }
          };
        }

        // Reset the main screen cursor pos
        print!("{}", termion::cursor::Goto(main_screen_x, main_screen_y));
      }
      // Handle a Ctrl+C
      Key::Ctrl('c') => {
        input = String::from("forcequit");
        clear_line(stdout_main.borrow_mut());
        break;
      }
      // Toggle filemanager
      Key::Ctrl('f') => {
        let file_path = load_scd();
        input.insert_str(x_pos, &file_path);
        x_pos += file_path.len();
      }
      // Toggle htop
      Key::Ctrl('d') => {
        match execute_cmnd(&mut Command::new("htop").args(vec!["-t","--sort-key=STATE"])) {
          Ok(_) => { },
          Err(_) => { }
        };
      }
      // Clear the terminal
      Key::Ctrl('k') => {
        input = String::from("clear");
        clear_line(stdout_main.borrow_mut());
        break;
      }
      // Toggle lynx
      Key::Ctrl('l') => {
        if !check(Some(5)).is_ok() {
          print::print_custom("Currently no internet connection.\n","orange");
          break;
        }
        let arguments;
        if input.clone().len() <1 {
          arguments = vec!["-cfg=ext/util/lynx/lynx.cfg","-ftp","-force_secure","www.google.com"];
        }
        else {
          arguments = vec!["-cfg=ext/util/lynx/lynx.cfg","-ftp","-force_secure",&input.trim()];
        }
        match execute_cmnd(&mut Command::new("lynx").args(&arguments)) {
          Ok(_) => { },
          Err(_) => { }
        };
        input = String::from("");
        x_pos = 0;
      }
      // Toggle elfedit
      Key::Ctrl('n') => {
        if input.clone().len() < 1 { 
          print::print_custom("Need path to a binary to analyze in the console.\n","orange");
          break;
        }
        match execute_cmnd(&mut Command::new("ext/util/elfedit").arg(&input.trim())) {
          Ok(_) => (),
          Err(e) => { print::print_custom(&e.to_string(), "orange"); println!(); break; }
        };
        input = String::from("");
        x_pos = 0;
      }
      // Toggle Stack Overflow
      Key::Ctrl('o') => {
        if !check(Some(5)).is_ok() {
          print::print_custom("Currently no internet connection.\n","orange");
          break;
        }
        if input.clone().len() < 1 {
          print::print_custom("Need a Stack Overflow query in the console.\n","orange");
          break;
        }
        match execute_cmnd(&mut Command::new("ext/util/so").arg(&input.trim_start().trim_end())) {
          Ok(_) => { },
          Err(_) => { }
        };
        input = String::from("");
        x_pos = 0;
      }
      // Toggle Tock
      Key::Ctrl('q') => {
        if !check(Some(5)).is_ok() {
          print::print_custom("Currently no internet connection.\n","orange");
          break;
        }
        match execute_cmnd(&mut Command::new("ext/util/tock").args(vec!["-msc","-h","2","-w","2"])) {
          Ok(_) => { },
          Err(_) => { }
        };
      }
      // Toggle radare2
      Key::Ctrl('r') => {
        load_radare2(stdout_main.borrow(), input.clone());

        input = String::from("");
        x_pos = 0;
      }
      // Toggle chat
      Key::Ctrl('s') => {
        match execute_cmnd(&mut Command::new("ext/util/stealthy")) {
          Ok(_) => { },
          Err(_) => { }
        };
      }
      // Toggle termshark
      Key::Ctrl('t') => {
        if input.clone().len() < 1 { 
          print::print_custom("Need the name of a network interface in the console to begin sniffing.\n","orange");
          break;
        }
        //let mut arguments;
        let arguments = vec!["-i", &input.trim()];
        match execute_cmnd(&mut Command::new("ext/tshark/termshark").args(arguments)) {
          Ok(_) => (),
          Err(e) => { println!("{}", e); break; }
        };
      }
      // Toggle Wikipedia
      Key::Ctrl('w') => {
        if !check(Some(5)).is_ok() {
          print::print_custom("Currently no internet connection.\n","orange");
          break;
        }
        match execute_cmnd(&mut Command::new("ext/util/wiki-tui")) {
          Ok(_) => { },
          Err(_) => { }
        };
        input = String::from("");
        x_pos = 0;
      }
      // Toggle calculator
      Key::Ctrl('x') => {
        match execute_cmnd(&mut Command::new("ext/util/pcalc")) {
          Ok(_) => { },
          Err(_) => { }
        };
      }
      // Toggle ugdb
      Key::Ctrl('u') => {
        if input.clone().len() < 1 { 
          print::print_custom("Need path to a binary to debug.\n","orange");
          break;
        }
        match execute_cmnd(&mut Command::new("ext/util/ugdb").arg(input.clone())) {
          Ok(_) => (),
          Err(e) => { print::print_custom(&e.to_string(), "orange"); println!(); break; }
        };
        input = String::from("");
        x_pos = 0;
      }
      // Toggle biodiff
      Key::Ctrl('y') => {
        if input.clone().len() < 1 { 
          print::print_custom("Need path to two binaries to diff in the console.\n","orange");
          break;
        }
        let files = input.trim().split(" ").collect::<Vec<&str>>();
        if files.len() < 2 {
          print::print_custom("Need path to two binaries to diff in the console.\n","orange");
          break;
        }
        match execute_cmnd(&mut Command::new("ext/util/biodiff").args(vec![&files[0],&files[1]])) {
          Ok(_) => (),
          Err(e) => { print::print_custom(&e.to_string(), "orange"); println!(); break; }
        };
        input = String::from("");
        x_pos = 0;
      }
      // Toggle bottom
      Key::Ctrl('p') => {
        print::print_custom("Due to Windows host compatibility issues, this feature is currently disabled on containerized releases.\n","orange");
        /*
        match execute_cmnd(&mut Command::new("ext/util/btm")) {
          Ok(_) => { },
          Err(_) => { }
        };
        */
      }
      // Auto-complete
      Key::Char('\t') => {
        let inp_clone = input.clone();
        // Split the vec on spaces
        let mut input_vec: Vec<String> = Vec::new();//inp_clone.split(" ").collect();
        let mut builder: String = String::new();
        let mut apos_counter = 0;
        for inp_c in inp_clone.chars() {
          if inp_c == '\'' {
            apos_counter += 1;
            builder.push(inp_c);
          } else if inp_c == ' ' && apos_counter % 2 == 0 {
            input_vec.push(builder.clone());
            builder = String::new();
          } else {
            builder.push(inp_c);
          }
        }
        input_vec.push(builder.clone());

        let last = input_vec.len() - 1;
        if prev_char != '\t' {
          sim = get_similar_strings(input_vec[last].to_string());
        }
        if sim.len() > 0 {
          if tab_pos >= sim.len() { tab_pos = sim.len() - 1; }
          //let sim_tab = ;
          input_vec[last] = sim[tab_pos].clone();
          input = input_vec[0].to_string();
          for inp in &input_vec[1..] {
            input += " ";
            input += inp;
          }

          //input = sim[tab_pos].clone();
          x_pos = input.len();
          tab_pos = (tab_pos + 1) % sim.len();
        }
        prev_char = '\t';
      }
      // Pop a character off the string and remove it from the screen
      Key::Backspace => {
        if input.len() > 0 && x_pos > 0 {
          let mut grapheme_cursor = strcursor::StrCursor::new_at_left_of_byte_pos(&input, x_pos);
          grapheme_cursor = match grapheme_cursor.prev() {
            Some((_, cur)) => cur,
            None => { continue; }
          };

          x_pos = grapheme_cursor.byte_pos();
          input.remove(x_pos);

          tab_pos = 0;
          prev_char = ' ';
        }
      }
      // Pop a character off the string and remove it from the screen
      Key::Delete => {
        if input.len() > 0 && x_pos < input.len() {
          input.remove(x_pos);

          tab_pos = 0;
          prev_char = ' ';
        }
      }
      // Get last typed command
      Key::Up => {
        input = aliases::CMND_HISTORY.with(|cmnd_history| {
          if cmnd_history.borrow().len() == 0 {
            return input;
          }
          let cmnd = cmnd_history.borrow()[history_pos].clone();
          if history_pos < cmnd_history.borrow().len() - 1 {
            history_pos += 1;
          }
          return cmnd;
        });
        x_pos = input.len();
      }
      // Clear the input
      Key::Down => {
        if history_pos == 0 {
          input = String::from("");
          x_pos = 0;
        } else {
          input = aliases::CMND_HISTORY.with(|cmnd_history| {
            if history_pos > 0 {
              history_pos -= 1;
            }
            return cmnd_history.borrow()[history_pos].clone();
          });
          x_pos = input.len();
        }
      }
      // Move the cursor left
      Key::Left => {
        if x_pos > 0 {
          let mut grapheme_cursor = strcursor::StrCursor::new_at_left_of_byte_pos(&input, x_pos);
          grapheme_cursor = match grapheme_cursor.at_prev() {
            Some(cur) => cur,
            None => { continue; }
          };
          x_pos = grapheme_cursor.byte_pos();
        }
      }
      // Move the cursor right
      Key::Right => {
        if x_pos < input.len() {
          let mut grapheme_cursor = strcursor::StrCursor::new_at_left_of_byte_pos(&input, x_pos);
          grapheme_cursor = match grapheme_cursor.at_next() {
            Some(cur) => cur,
            None => { continue; }
          };
          x_pos = grapheme_cursor.byte_pos();
        }
      }
      // Move to start
      Key::Home | Key::Ctrl('b') => {
        x_pos = 0;
      }
      // Move to back
      Key::End | Key::Ctrl('e') => {
        x_pos = input.len();
      }
      // Command is done, take the input
      Key::Char('\n') => {
        if input.starts_with("cd ") || input.eq("cd") {
          input = load_scd();
          x_pos = input.len();
        } else {
          aliases::CMND_HISTORY.with(|cmnd_history| {
            // Don't allow duplicates
            let len = cmnd_history.borrow().len();
            for i in 0..len {
              if cmnd_history.borrow()[i] == input.clone() {
                cmnd_history.borrow_mut().remove(i);
                break;
              }
            }

            // Don't let the history get above 10
            if cmnd_history.borrow().len() == 10 {
              cmnd_history.borrow_mut().remove(9);
            }
            // Hitting enter without any text shouldn't add to history
            if input != "" {
              cmnd_history.borrow_mut().insert(0, input.clone());
            }
          });
          clear_line(stdout_main.borrow_mut());
          break;
        }
      }
      // Store the character into the input
      Key::Char(c) => {
        input.insert(x_pos, c);

        let mut grapheme_cursor = strcursor::StrCursor::new_at_left_of_byte_pos(&input, x_pos);
        grapheme_cursor = match grapheme_cursor.at_next() {
          Some(cur) => cur,
          None => { continue; }
        };
        
        x_pos = grapheme_cursor.byte_pos();
        prev_char = c;
        inc += 2;
      }
      _ => {}
    }
    
    // Determine number of grapheme clusters
    let mut grapheme_cursor = strcursor::StrCursor::new_at_start(&input);
    let mut prev_byte_pos = 0;
    let mut clusters: usize = 0;
    if x_pos > 0 {
      let mut g_i: usize = 0;
      while g_i < x_pos {
        // Step once
        grapheme_cursor = match grapheme_cursor.at_next() {
          Some(cur) => cur,
          None => { continue; }
        };
        // Has the byte positioned move more than 1 byte?
        if grapheme_cursor.byte_pos() > prev_byte_pos + 1 {
          clusters += 1; // Cluster found
          g_i += grapheme_cursor.byte_pos() - prev_byte_pos - 1;
        }
        prev_byte_pos = grapheme_cursor.byte_pos();
        g_i += 1;
      }
    }

    // Gather the absolute positions of the cursor in the input
    let x_abs_pos = (prompt.len() + (x_pos - clusters)) as u16 % width;
    let y_abs_pos = (prompt.len() + (x_pos - clusters) - inc) as u16 / width;
    let lines = (prompt.len() + input.len() - inc) as u16 / width;

    print!("{}", termion::cursor::Left(0xFFFF));
    if y_abs_pos > 0 && key != Key::Right {
      print!("{}", termion::cursor::Up(y_abs_pos));
    }

    // Clear all the lines
    for _ in 0..lines {
      print!("{}{}", termion::clear::CurrentLine, termion::cursor::Down(1));
    }

    // Reset the cursor to the beginning of the console prompt
    print!("{}{}", termion::clear::CurrentLine, termion::cursor::Left(0xFFFF));
    if lines > 0 {
      print!("{}", termion::cursor::Up(lines));
    }

    // Print the prompt and input
    print::print(&prompt);
    print!("{}", input);
    
    // Reset the cursor
    print!("{}", termion::cursor::Left(0xFFFF));
    if lines > 0 {
      print!("{}", termion::cursor::Up(lines));
    }

    // Adjust the cursor
    if y_abs_pos > 0 {
      print!("{}", termion::cursor::Down(y_abs_pos));
    }
    if x_abs_pos > 0 {
      print!("{}", termion::cursor::Right(x_abs_pos));
    }
    flush();
  }
  print!("{}", termion::cursor::Left(0xFFFF));
  flush();

  // Get rid of tilde
  let input_vec: Vec<&str> = input.split(" ").collect();
  if input_vec.len() > 1 && (input_vec[input_vec.len()-1] == "~" || input_vec[input_vec.len()-1].starts_with("~/")) {
    let home = match std::env::var("HOME") {
      Ok(dir) => dir,
      Err(_) => String::from(""),
    };
    input = input.replace("~", &home);
  }

  input = replace_aliases(input.clone());

  // Check for path safety
  match check_input_safety(input.clone()) {
    Ok(_) => {}
    Err(err) => {
      return Err(err);
    }
  };

  // Replace input with aliases
  return Ok(input);
}

/*
 * write_alt
 * 
 * @brief
 * @param w
 */
pub fn write_alt() {
  // Grab the alt screen buffer mutex
  {
    let mut guard = match ALT_BUFFER.lock() {
      Ok(lock) => lock,
      Err(err) => {
        print!("Thread currently holding this mutex panicked: {}\n\r", err);
        return;
      }
    };
    // Regex for replacing reset codes (for some reason the pager doesn't recognize them)
    let re = match regex::Regex::new(r"\[0[;a-zA-Z0-9]*m") {
      Ok(re) => re,
      Err(err) => {
        print!("Bad regex expression: {}", err);
        return;
      }
    };

    // Grab the bg threads mutex
    {
      let threads_guard = match main_info::BG_THREADS.lock() {
        Ok(lock) => lock,
        Err(err) => {
          print!("Thread currently holding this mutex panicked: {}\n\r", err);
          return;
        }
      };
      if threads_guard.len() > 0 {
        // Find the bg we're on
        for i in 0..threads_guard.len() {
          let (_, t_buf, tid, _) = threads_guard[i].clone();
          // Write to the alt screen buffer
          if tid == std::thread::current().id() {
            guard.push_str(&format!("{}", re.replace_all(&t_buf, "[37m\x1b[49m\x1b[0m")));
            break;
          }
        }
      }
    }
  }
}

/*
 * write_thread
 * 
 * @brief
 * @param w
 */
pub fn write_thread<D: std::fmt::Display>(w: D) {
  let write = w.to_string();
  // Grab the alt screen buffer mutex
  {
    let mut threads_guard = match main_info::BG_THREADS.lock() {
      Ok(lock) => lock,
      Err(err) => {
        print!("Thread currently holding this mutex panicked: {}\n\r", err);
        return;
      }
    };
    if threads_guard.len() > 0 {
      // Find the bg we're on
      for i in 0..threads_guard.len() {
        let (t_cmnd, mut t_buf, tid, stat) = threads_guard[i].clone();
        // Write to the bg's buffer
        if tid == std::thread::current().id() {
          t_buf.push_str(&write);
          threads_guard[i] = (t_cmnd, t_buf, tid, stat);
          break;
        }
      }
    }
  }
}

/*
 * check_input_safety
 * 
 * @brief
 * @param input:
 * @return
 */
fn check_input_safety(input: String) -> Result<(), std::io::Error> {
  const UNSAFE_DIRS: [&str; 6] = ["/boot", "/dev", "/etc", "/root", "/sys", "/usr"];
  let input_vec: Vec<&str> = input.split(" ").collect();

  // Check if the event we're in, or about to enter has unsafe access
  let event_name = main_info::EVENTS_VEC.with(|events| {
    events.borrow()[main_info::get_last_event()].get_event().name.clone()
  });
  let next_event_name = input_vec[0].to_string();
  if events::has_dir_access(event_name.clone()) || events::has_dir_access(next_event_name.clone()) {
    return Ok(());
  }

  // Look at every word in the user input
  for iter in input_vec {
    // If the word is a path
    if std::path::Path::new(&iter).exists() {
      // Get file metadata
      match std::fs::symlink_metadata(iter.clone()) {
        Ok(out) => {
          // Is it a symlink?
          if out.file_type().is_symlink() {
            print::print_custom_uncapped("","orange");
            println!("Restricted operation: file is a symlink.{}", termion::cursor::Left(0xFFFF));
            print::print_custom("","reset");
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "FileIsSymlinkError"));
          }
        }
        Err(_) => {}
      }

      // Get the full path of the file
      match std::fs::canonicalize(iter.clone()) {
        Ok(out) => {
          // Does it start with an unsafe top level directory?
          for dir in &UNSAFE_DIRS {
            if out.starts_with(dir) {
              print::print_custom_uncapped("","orange");
              println!("Restricted operation: top-level directory access prohibited for this command.{}", termion::cursor::Left(0xFFFF));
              print::print_custom("","reset");
              return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "DangerousDirError"));
            }
          }
        }
        Err(_) => {}
      }
    // Check for local IPv4 addresses
    } else if iter.matches(".").count() == 3 {
      let sections: Vec<&str> = iter.split(".").collect();
      let is_local = match sections[0] {
        "10" => sections[1] == "0" && sections[2] == "0" && sections[3] == "0",
        "127" => sections[1] == "0" && sections[2] == "0" && sections[3] == "1",
        "172" => {
          let id1 = match sections[1].parse::<u8>() {
            Ok(id) => id,
            Err(_) => 0,
          };
          id1 >= 16 && id1 <= 31
        }
        "192" => sections[1] == "168",
        "169" => sections[1] == "254",
        _ => false,
      };
      if !is_local
        && (!events::has_ip_access(event_name.clone()) && !events::has_ip_access(next_event_name.clone())) {
        print::print_custom_uncapped("","orange");
        println!("Restricted operation: non-local IPv4 addresses are prohibited for this command.{}", termion::cursor::Left(0xFFFF));
        print::print_custom("","reset");
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "LocalAddressOnlyError"));
      }
    } 
    else if iter.matches(":").count() > 2 && !iter.eq("0:0:0:0:0:0:0:1") && !iter.eq("::1") {
      let sections: Vec<&str> = iter.split(":").collect();
      if &sections[0].len() < &4 && (!events::has_ip_access(event_name.clone()) && !events::has_ip_access(next_event_name.clone())) {
        print::print_custom_uncapped("","orange");
        println!("Restricted operation: non-local IPv6 addresses are prohibited for this command.{}", termion::cursor::Left(0xFFFF));
        print::print_custom("","reset");
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "LocalAddressOnlyError"));
      }
      let mut is_local = false;
      if sections[0].len() == 4 {
				is_local = match &sections[0][..2].to_lowercase()[..] {
					"fc" => { true },
					"fd" => { true },
					"fe" => &sections[0][..4].to_lowercase()[..] == "fe80",
					_ => false,
				};
      }
      if !is_local
        && (!events::has_ip_access(event_name.clone()) && !events::has_ip_access(next_event_name.clone())) {
        print::print_custom_uncapped("","orange");
        println!("Restricted operation: non-local IPv6 addresses are prohibited for this command.{}", termion::cursor::Left(0xFFFF));
        print::print_custom("","reset");
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "LocalAddressOnlyError"));
      }
    }
  }
  // No problems found
  return Ok(());
}

/*
 * replace_aliases
 * 
 * @brief
 * @param input:
 * @return
 */
pub fn replace_aliases(input: String) -> String {
  return aliases::ALIAS_MAP.with(|alias_map| {
    // Split all input on a space
    let mut input_vec: Vec<&str> = input.split(" ").collect();
    if input_vec.len() == 0 {
      return String::from("");
    }

    if aliases::CMND_HISTORY.with(|cmnd_history| {
      return input_vec[0] == "alias" || (cmnd_history.borrow().len() > 1 && &cmnd_history.borrow()[1] == "alias");
    }) { return input; }
    
    // Perform the replaces
    let refer = &alias_map.borrow();
    //let check = if events::is_builtin(input.clone()) { 2 } else { 1 };
    //let max = if input_vec.len() < check { input_vec.len() } else { check };
    let max = if events::is_builtin(String::from(input_vec[0])) { 
      //println!("input vec 0 : {}",input_vec[0]);
      input_vec.len() 
    } 
    else { 
      1 
    };

    for i in 0..max {
      match refer.get(input_vec[i]) {
        Some(alias_full) => {
          //println!("i : {}\ninput : {}\nalias : {}", i, input_vec[i], alias_full);
          input_vec[i] = alias_full;
        }
        None => {}
      };
    }

    // Join the vec into a String
    let mut modified_input = input_vec[0].to_string();
    for inp in &input_vec[1..] {
      modified_input += " ";
      modified_input += inp;
    }

    return modified_input;
  });
}

/*
 * print_console
 *
 * @brief print console prompt 
 * @param mode: enum used to determine print color
 * @return none
 */
pub fn create_prompt<D: std::fmt::Display>(prompt: D, symbol: D) -> String {
  return format!("{}{} ", prompt, symbol);
}

/*
 * flush
 * 
 * @brief
 * @return none
 */
pub fn flush() {
  match std::io::stdout().flush() {
    Ok(_) => {}
    Err(err) => {
      println!("Failed to flush output. {}", err);
    }
  };
}

/*
 * set_color_mode
 * 
 * @brief Updates the color that gets printed
 */
pub fn set_color_mode(color_mode: Mode) {
  COLOR_MODE.store(color_mode, Release);
}

/*
 * get_color_mode
 * 
 * @return the current color mode
 */
pub fn get_color_mode() -> Mode {
  return COLOR_MODE.load(Acquire);
}

/*
 * set_array_mode
 * 
 * @brief Updates the array mode to determine how users interact with salvum
 */
pub fn set_array_mode(array_mode: bool) {
  ARRAY_MODE.store(array_mode, Release);
}

/*
 * get_array_mode
 * 
 * @return the current array mode
 */
pub fn get_array_mode() -> bool {
  return ARRAY_MODE.load(Acquire);
}

/*
 * get_style_color
 * 
 * @brief Get a style that is appropriate to the current color
 * @param style: The ColorfulTheme style to color
 * @param bg: Is a background color
 * @return the style with an appropriate color
 */
pub fn get_style_color(style: Style, bg: bool) -> Style {
  if bg {
    match get_color_mode() {
      Mode::Yellow => style.on_yellow(),
      Mode::Red => style.on_red(),
      Mode::Blue => style.on_cyan(),
      Mode::Magenta => style.on_magenta()
    }
  } else {
    match get_color_mode() {
      Mode::Yellow => style.yellow(),
      Mode::Red => style.red(),
      Mode::Blue => style.cyan(),
      Mode::Magenta => style.magenta()
    }
  }
}

/*
 * get_style_color
 * 
 * @brief Get a style that is appropriate to the current color
 * @param style: The ColorfulTheme style to color
 * @param bg: Is a background color
 * @return the style with an appropriate color
 */
pub fn get_style_color_custom(style: Style, bg: bool, color: &str) -> Style {
  if bg {
    match color {
      "white" => style.on_white(),
      "green" => style.on_green(),
             _=> style.on_white()
    }
  } else {
    match color {
      "white" => style.white(),
      "green" => style.green(),
             _=> style.white()
    }
  }
}
/*
 * get_menu_theme
 * 
 * @return a customized theme
 */
pub fn get_menu_theme() -> ColorfulTheme {
  ColorfulTheme {
    defaults_style: get_style_color(Style::new(), false),
    prompt_style: Style::new(),
    prompt_prefix: style("".to_string()),
    prompt_suffix: style("".to_string()),
    success_prefix: style("".to_string()),
    success_suffix: style("".to_string()),
    error_prefix: style("".to_string()),
    error_style: Style::new(),
    hint_style: Style::new(),
    values_style: Style::new(),
    active_item_style: get_style_color(Style::new(), true).black(),
    inactive_item_style: get_style_color(Style::new(), false),
    active_item_prefix: match get_color_mode() {
      Mode::Yellow => style(">".to_string()).yellow().bright().blink(),
      Mode::Red => style(">".to_string()).red().bright().blink(),
      Mode::Blue => style(">".to_string()).cyan().bright().blink(),
      Mode::Magenta => style(">".to_string()).magenta().bright().blink()
    },
    inactive_item_prefix: style("".to_string()),
    checked_item_prefix: style("".to_string()),
    unchecked_item_prefix: style("".to_string()),
    picked_item_prefix: style("".to_string()),
    unpicked_item_prefix: style("".to_string()),
    inline_selections: true,
  }
}

/*
 * get_selection
 * 
 * @brief Handles the error output of interact_opt to return an option
 * @param options: The available choices for the user
 * @return the option selected
 */
pub fn get_selection<D: std::fmt::Display>(options: Vec<D>) -> std::option::Option<usize> {
  match Select::with_theme(&get_menu_theme()).default(0).items(&options).interact_opt() {
    Ok(sel) => return sel,
    Err(err) => {
      println!("Issue handling your selection. {}", err);
      return std::option::Option::None;
    }
  }
}

/*
 * get_menu_theme
 * 
 * @return a customized theme
 */
pub fn get_menu_theme_custom(color: &str) -> ColorfulTheme {
  ColorfulTheme {
    defaults_style: get_style_color_custom(Style::new(), false, color),
    prompt_style: Style::new(),
    prompt_prefix: style("".to_string()),
    prompt_suffix: style("".to_string()),
    success_prefix: style("".to_string()),
    success_suffix: style("".to_string()),
    error_prefix: style("".to_string()),
    error_style: Style::new(),
    hint_style: Style::new(),
    values_style: Style::new(),
    active_item_style: get_style_color_custom(Style::new(), true, color).black(),
    inactive_item_style: get_style_color_custom(Style::new(), false, color),
    active_item_prefix: match color {
      "white" => style(">".to_string()).white().bright().blink(),
      "green" => style(">".to_string()).green().bright().blink(),
             _=> style(">".to_string()).white().bright().blink()
    },
    inactive_item_prefix: style("".to_string()),
    checked_item_prefix: style("".to_string()),
    unchecked_item_prefix: style("".to_string()),
    picked_item_prefix: style("".to_string()),
    unpicked_item_prefix: style("".to_string()),
    inline_selections: true,
  }
}

/*
 * get_selection_custom
 * 
 * @brief Handles the error output of interact_opt to return an option
 * @param options: The available choices for the user
 * @return the option selected
 */
pub fn get_selection_custom<D: std::fmt::Display>(options: Vec<D>, color: &str) -> std::option::Option<usize> {
  match Select::with_theme(&get_menu_theme_custom(color)).default(0).items(&options).interact_opt() {
    Ok(sel) => return sel,
    Err(err) => {
      println!("Issue handling your selection. {}", err);
      return std::option::Option::None;
    }
  }
}

/*
 * get_similar_strings
 * 
 * @brief Given a string, will check for valid events that start with that string
 * @param input: Is what an event could start with
 * @return the vector of strings that starts with input
 */
 fn get_similar_strings(input: String) -> Vec<String> {
  let mut similar = Vec::new();
  //if "back".starts_with(&input) {
  //  similar.push("back".to_string());
  //}

  // Check for a event command
  main_info::EVENTS_VEC.with(|events| {
    let events_borrow = &*events.borrow();
    let current_event = events_borrow[main_info::get_last_event()].get_event();
    for event in events_borrow {
      let event_name = event.get_event().name.clone();
      if current_event.links.contains(&event_name) && event_name.starts_with(&input) && event_name != "salvum" {
        similar.push(event_name);
      }
    }
  });
  
  // Check for a builtin command
  for builtin in main_info::BUILTIN_CMNDS {
    if builtin.starts_with(&input) {
      similar.push(builtin.to_string());
    }
  }

  // Check for a file to autocomplete
  let mut input_path = String::from("./");
  let mut input_file = String::from("");
  // Split the path from file
  if input != "" {
    let (mut p, f) = match input.rsplit_once('/') {
      Some((p, f)) => (p.to_string(), f.to_string()),
      None => (String::from("."), input.clone()),
    };
    let home = match std::env::var("HOME") {
      Ok(dir) => dir,
      Err(_) => String::from(""),
    };
    p = p.replace("~", &home);
    p.push('/');
    input_path = p;
    input_file = f;
  }

  // Run ls and get the files from the directory
  let ls_vec = util::misc::get_files_in_path(input_path.clone());

  // Add the proper files to the similar vec
  for ls_file in ls_vec {
    if ls_file.starts_with(&input_file) {
      let mut full_path = input_path.to_string() + &ls_file;
      if std::path::Path::new(&full_path).is_dir() {
        full_path += "/";
      }
      similar.push(full_path);
    }
  }
  
  return similar;
}

/*
 * execute_cmnd
 *
 * @brief Launch a process and wait for it to finish
 * @param: cmnd The command to execute
 * @return A result of the process output
 */
fn execute_cmnd(cmnd: &mut std::process::Command) -> std::io::Result<std::process::Output> {
  let child = match cmnd.spawn() {
    Ok(out) => out,
    Err(err) => return Err(err)
  };

  return child.wait_with_output();
}

#[allow(dead_code)]
fn create_scd_sock() {
	use nix::sys::stat;
	use tempfile::tempdir;
	let tmp_dir = match tempdir() {
    Ok(o) => o,
   Err(err) => { println!("Error creating temp dir: {}", err); return; },
  };
	let fifo_path = tmp_dir.path().join("slm_scd.sock");

	// create new fifo and give read, write and execute rights to the owner
	match nix::unistd::mkfifo(&fifo_path, stat::Mode::S_IRWXU) {
		 Ok(_) => (),
		 Err(err) => println!("Error creating fifo: {}", err),
	}
}

/*
 * load_scd
 *
 * @brief Launch the scd file manager and wait for it to exit
 * @return The string representation of the path selected
 */
fn load_scd() -> String {
  // Error hook
  fn handle_error(connection: std::io::Result::<LocalSocketStream>) -> std::io::Result::<LocalSocketStream> {
    match connection {
      Ok(val) => Ok(val),
      Err(err) => Err(err),
    }
  }

  util::misc::cleanup("slm_scd.sock");
  create_scd_sock();

  // Bind the listener to slm_scd.sock
  let listener = match LocalSocketListener::bind("slm_scd.sock") {
    Ok(out) => out,
    Err(err) => {
      println!("Failed to set up the listening server. {}", err);
      util::misc::cleanup("slm_scd.sock");
      return String::from("");
    }
  };
  let mut writer: Vec<u8> = Vec::new();

  // Launch scd
  match execute_cmnd(&mut Command::new("ext/util/scd")) {
    Ok(_) => {}
    Err(err) => {
      println!("Failed to launch the file manager. {}", err);
      util::misc::cleanup("slm_scd.sock");
      return String::from("");
    }
  }

  // Wait for a single connection from scd
  let mut iter = listener.incoming().map(handle_error);
  let mut conn = match iter.next() {
    Some(c) => {
      match c {
        Ok(o) => o,
        Err(err) => {
          println!("Failed to accept incoming connection to file manager. {}", err);
          util::misc::cleanup("slm_scd.sock");
          return String::from("");
        }
      }
    },
    None => {
      util::misc::cleanup("slm_scd.sock");
      return String::from("");
    }
  };

  // Wait for scd to send a message
  match std::io::copy(&mut conn, &mut writer) {
    Ok(_) => {
      util::misc::cleanup("slm_scd.sock");
      return String::from_utf8_lossy(&writer).to_string();
    }
    Err(err) => {
      util::misc::cleanup("slm_scd.sock");
      println!("Failed to copy from sock to stdout. {}", err);
      return String::from("");
    }
  }
}

fn load_radare2<W: std::io::Write>(sout: &termion::raw::RawTerminal<W>, input: String) {
  // Disable raw mode
  match sout.suspend_raw_mode() {
    Ok(_) => {}
    Err(err) => {
      println!("Failed to suspend raw mode. {}", err);
    }
  };

  // Execute radare2
  let out = match Command::new("ext/radare/radare2").args(vec!["-cv".to_string(),"-e".to_string(),"bin.cache=true".to_string(),input]).env("LD_LIBRARY_PATH", "ext/radare/").spawn() {
    Ok(o) => o,
    Err(e) => { print::print_custom(&e.to_string(), "orange"); println!(); return; }
  };
  print::print_custom_uncapped("", "orange");
  match out.wait_with_output(){
    Ok(_)=>(),
    Err(e)=> {
      print::print_custom(&e.to_string(), "orange"); println!(); return;
    }
  }
  print::print_custom("", "reset");
  // Enable raw mode
  match sout.activate_raw_mode() {
    Ok(_) => {}
    Err(err) => {
      println!("Failed to activate raw mode. {}", err);
    }
  };
}

pub fn load_aliases() {
  let file = match std::fs::read_to_string("cfg/aliases.cfg") {
    Ok(out) => out,
    Err(err) => {
      println!("Failed to read alias file. {}", err);
      return;
    }
  };
  let lines: Vec<&str> = file.split("\n").collect();

  let mut aliases: HashMap<String, String> = HashMap::new();
  for line in lines {
    if line != "" {
      let alias_pair: Vec<&str> = line.split("=").collect();
      aliases.insert(alias_pair[0].to_string(), alias_pair[1].to_string());
    }
  }

  aliases::set_alias_map(aliases);
}
