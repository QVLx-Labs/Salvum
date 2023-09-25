/*
 * QVLx Salvum 
 *
 * log.rs -> outputs information to a log file
 *
 * authors: $t@$h, r00r00, n3wmAn
 */

// Imports
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering::*};
use std::time::SystemTime;

// External files
use crate::errno;

static START_TIME: AtomicU64 = AtomicU64::new(0);

pub fn init_logger() {
  // Get system time in secs
  let secs = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(duration) => duration.as_secs(),
    Err(err) => {
      let err_msg = format!("Failed to get system time. {}", err);
      errno::print_err("Logger", &err_msg);
      return;
    }
  };
  START_TIME.store(secs, Release);
}

/*
 * log
 * 
 * @brief
 * @param logger: The name of the function calling this
 * @param msg: What to have logged
 */
pub fn log(logger: &str, msg: &str) {
  if msg == "" {
    return;
  }

  let filename = format!("log/log_{}.txt", START_TIME.load(Acquire));

  // Create the file
  let mut file = match OpenOptions::new().write(true).create(true).open(filename.clone()) {
    Ok(f) => f,
    Err(err) => {
      let err_msg = format!("Can't create file {}. {}", filename, err);
      errno::print_err("Logger", &err_msg);
      return;
    }
  };

  // Construct the log String
  let mut log: String = "----- ".to_string();
  log.push_str(logger);
  log.push_str(" -----\n");
  log.push_str(msg);

  // Write the log to the file
  match file.write_all(log.as_bytes()) {
    Ok(_) => {}
    Err(err) => {
      let err_msg = format!("Can't write to file. {}", err);
      errno::print_err("Logger", &err_msg);
      return;
    }
  }
}
