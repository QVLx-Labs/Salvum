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
 * ecctools.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
 use crate::events::*;

/*********************************** YELLOW ***********************************/
struct Yellow { event: Event }
impl Eventable for Yellow {
  fn on_run(&self, _args: Vec<String>) -> String {
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn yellow(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Yellow {
    event: Event {
      name:   name,
      desc:   "yellow desc".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
 
/*********************************** BLUE ***********************************/
struct Blue { event: Event }
impl Eventable for Blue {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Defensive mode engaged. New menu options available.");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn blue(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Blue {
    event: Event {
      name:   name,
      desc:   "Defensive security applications".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
 
/*********************************** RED ***********************************/
struct Red { event: Event }
impl Eventable for Red {
  fn on_run(&self, _args: Vec<String>) -> String {
    print::println("Offensive mode engaged. New menu options available.");
    return String::from("");
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn red(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Red {
    event: Event {
      name:   name,
      desc:   "Offensive security applications".to_string(),
      usage:  "".to_string(),
      author: "".to_string(),
      easyrun: false,
      secure: false,
      parent: parent,
      links:  links
    }
  })
}
