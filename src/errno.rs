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

#[allow(dead_code)]
pub const ERR_MISMATCHED_LINKS: i32 = -1; // err code for if events_map and events_vec are out of sync. 
                                          //a link in an event object was not found in the event map

#[allow(dead_code)]
pub const ERR_NOEVENT: i32 = -2; // err code for any time that that no events exist in the system.

pub fn handle_err(err: i32) {
  println!("ERROR {}.", err);
  std::process::exit(1);
  /*handle errors
  match err {
    errno::ERR_MISMATCHED_LINKS => {}, //handle mismatched links error
                                       //do we crash? do we restart? print out message?
    errno::ERR_NOEVENT => {}           //handle no event error
  }*/
}

pub fn print_err(error_caller: &str, error_msg: &str) {
  let msg = format!("{}: {}", error_caller, error_msg);
  println!("{}", msg.magenta());
}
