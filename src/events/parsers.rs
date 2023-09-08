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
 * snoopingtools.rs
 *
 * authors: $t@$h, r00r00, n3wmAn
 */
use crate::events::*;
  
/*********************************** PARSERS ***********************************/
pub struct Parsers { event: Event }
impl Eventable for Parsers {
  fn on_init(&self) -> Vec<String> {
    print::println("Parsers");
    return Vec::new();
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn parsers(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Parsers {
    event: Event {
      name:   name,
      desc:   "Parser implementations".to_string(),
      usage:  "".to_string(),
      parent: parent,
      author: "".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** Space_Packet_Parse ***********************************/
pub struct SpacePacketParse { event: Event }
impl Eventable for SpacePacketParse {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/spacepacketparse/space_packet_parse").args(args)));

    log::log("space_packet_parse", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/ananas/atf/atf.txt";

    // Run command
    let output = match run_bounded_command(Command::new("ext/ananas/ananas").arg("N"), false, 2) {
      Ok(o) => o,
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Read artifact file
    let atf_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("ananas::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&output.stdout) != atf_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn spacepacketparse(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SpacePacketParse {
    event: Event {
      name:   name,
      desc:   "Parses CCSDS files for key information.".to_string(),
      usage:  "spacepacketparse <file_path>".to_string(),
      parent: parent,
      author: "Matzr3lla & KubOS Preservation Group".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** DHCP_Parse ***********************************/
pub struct DHCPparse { event: Event }
impl Eventable for DHCPparse {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/dhcpparse/DHCPparse").args(args)));
    
    log::log("dhcpparse", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let inp_path = "tst/dhcpparse/inp/dhcp-discover.bin";
    let atf_path = "tst/dhcpparse/atf/atf.txt";
    let out_path = "out/dhcpparse/parsed_dhcp.txt";

    // Run command
    let mut child = simple_test_match!(Command::new("ext/dhcpparse/DHCPparse")
      .stdin(Stdio::piped())
      .stdout(Stdio::null())
      .spawn());

    let child_stdin = child.stdin.as_mut().unwrap();
    simple_test_match!(child_stdin.write_all(inp_path.as_bytes()));

    simple_test_match!(child.wait());

    // Read file
    let out_str = simple_test_match!(fs::read_to_string(out_path));
    let file_str = simple_test_match!(fs::read_to_string(atf_path));

    // Compare
    if out_str != file_str {
      util::misc::cleanup(out_path);
      return TestStatus::Failed;
    }
    util::misc::cleanup(out_path);
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn dhcpparse(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(DHCPparse {
    event: Event {
      name:   name,
      desc:   "Parses DHCP files for key information.".to_string(),
      usage:  "dhcpparse <file_path>".to_string(),
      parent: parent,
      author: "Matzr3lla & Paul Woolcock (github.com/pwoolcoc)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** IPSEC_Parse ***********************************/
pub struct IPSecparse { event: Event }
impl Eventable for IPSecparse {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/ipsecparse/IPSECparse").args(args)));

    log::log("ipsecparse", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let inp_path = "tst/ipsecparse/inp/ike-sa-init-req.bin";
    let atf_path = "tst/ipsecparse/atf/atf.txt";
    let out_path = "out/ipsecparse/parse_ikev2_mes.txt";

    // Run command
    let mut child = simple_test_match!(Command::new("ext/ipsecparse/IPSECparse")
      .stdin(Stdio::piped())
      .stdout(Stdio::null())
      .spawn());

    let child_stdin = child.stdin.as_mut().unwrap();
    simple_test_match!(child_stdin.write_all(inp_path.as_bytes()));

    simple_test_match!(child.wait());

    // Read file
    let out_str = simple_test_match!(fs::read_to_string(out_path));
    let file_str = simple_test_match!(fs::read_to_string(atf_path));

    // Compare
    if out_str != file_str {
      util::misc::cleanup(out_path);
      return TestStatus::Failed;
    }
    util::misc::cleanup(out_path);
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ipsecparse(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(IPSecparse {
    event: Event {
      name:   name,
      desc:   "Parses IPSEC files for key information.".to_string(),
      usage:  "ipsecparse <file_path>".to_string(),
      parent: parent,
      author: "Matzr3lla & Pierre Chifflier (github.com/rusticata)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** NTP_Parse ***********************************/
pub struct NTPparse { event: Event }
impl Eventable for NTPparse {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/ntpparse/NTPparse").args(args)));

    log::log("ntpparse", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let inp_path = "tst/ntpparse/inp/ntpv3_req.txt";
    let atf_path = "tst/ntpparse/atf/atf.txt";
    let out_path = "out/ntpparse/parse_ntp.txt";

    // Run command
    let mut child = simple_test_match!(Command::new("ext/ntpparse/NTPparse")
      .stdin(Stdio::piped())
      .stdout(Stdio::null())
      .spawn());

    let child_stdin = child.stdin.as_mut().unwrap();
    simple_test_match!(child_stdin.write_all(inp_path.as_bytes()));

    simple_test_match!(child.wait());

    // Read file
    let out_str = simple_test_match!(fs::read_to_string(out_path));
    let file_str = simple_test_match!(fs::read_to_string(atf_path));

    // Compare
    if out_str != file_str {
      util::misc::cleanup(out_path);
      return TestStatus::Failed;
    }
    util::misc::cleanup(out_path);
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn ntpparse(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(NTPparse {
    event: Event {
      name:   name,
      desc:   "Parses NTP files for key information.".to_string(),
      usage:  "ntpparse <file_path>".to_string(),
      parent: parent,
      author: "Matzr3lla & Pierre Chiflier (github.com/rusticata)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** SNMP_Parse ***********************************/
pub struct SNMPparse { event: Event }
impl Eventable for SNMPparse {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/snmpparse/SNMPparse").args(args)));

    log::log("snmpparse", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let inp_path = "tst/snmpparse/inp/snmpv3_req.bin";
    let atf_path = "tst/snmpparse/atf/atf.txt";
    let out_path = "out/snmpparse/parsed_snmp.txt";

    // Run command
    let mut child = simple_test_match!(Command::new("ext/snmpparse/SNMPparse")
      .stdin(Stdio::piped())
      .stdout(Stdio::null())
      .spawn());

    let child_stdin = child.stdin.as_mut().unwrap();
    simple_test_match!(child_stdin.write_all(inp_path.as_bytes()));

    simple_test_match!(child.wait());

    // Read file
    let out_str = simple_test_match!(fs::read_to_string(out_path));
    let file_str = simple_test_match!(fs::read_to_string(atf_path));

    // Compare
    if out_str != file_str {
      util::misc::cleanup(out_path);
      return TestStatus::Failed;
    }
    util::misc::cleanup(out_path);
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn snmpparse(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SNMPparse {
    event: Event {
      name:   name,
      desc:   "Parses SNMP files for key information.".to_string(),
      usage:  "snmpparse <file_path>".to_string(),
      parent: parent,
      author: "Matzr3lla & Pierre Chifflier (github.com/rusticata)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** SSH_Parse ***********************************/
pub struct SSHparse { event: Event }
impl Eventable for SSHparse {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/sshparse/SSHparse").args(args)));

    log::log("sshparse", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let inp_path = "tst/sshparse/inp/new_keys.raw";
    let atf_path = "tst/sshparse/atf/atf.txt";
    let out_path = "out/sshparse/parsed_ssh.txt";

    // Run command
    let mut child = simple_test_match!(Command::new("ext/sshparse/SSHparse")
      .stdin(Stdio::piped())
      .stdout(Stdio::null())
      .spawn());

    let child_stdin = child.stdin.as_mut().unwrap();
    simple_test_match!(child_stdin.write_all(inp_path.as_bytes()));

    simple_test_match!(child.wait());

    // Read file
    let out_str = simple_test_match!(fs::read_to_string(out_path));
    let file_str = simple_test_match!(fs::read_to_string(atf_path));

    // Compare
    if out_str != file_str {
      util::misc::cleanup(out_path);
      return TestStatus::Failed;
    }
    util::misc::cleanup(out_path);
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn sshparse(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(SSHparse {
    event: Event {
      name:   name,
      desc:   "Parses SSH files for key information.".to_string(),
      usage:  "sshparse <file_path>".to_string(),
      parent: parent,
      author: "Matzr3lla & Pierre Chifflier (github.com/rusticata)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** TLS_Parse ***********************************/
pub struct TLSparse { event: Event }
impl Eventable for TLSparse {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/tlsparse/TLSparse").args(args)));

    log::log("tlsparse", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let inp_path = "tst/tlsparse/inp/plain_test.txt";
    let atf_path = "tst/tlsparse/atf/atf.txt";
    let out_path = "out/tlsparse/parsed_tls.txt";

    // Run command
    let mut child = simple_test_match!(Command::new("ext/tlsparse/TLSparse")
      .stdin(Stdio::piped())
      .stdout(Stdio::null())
      .spawn());

    let child_stdin = child.stdin.as_mut().unwrap();
    simple_test_match!(child_stdin.write_all(inp_path.as_bytes()));

    simple_test_match!(child.wait());

    // Read file
    let out_str = simple_test_match!(fs::read_to_string(out_path));
    let file_str = simple_test_match!(fs::read_to_string(atf_path));

    // Compare
    if out_str != file_str {
      util::misc::cleanup(out_path);
      return TestStatus::Failed;
    }
    util::misc::cleanup(out_path);
    return TestStatus::Passed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn tlsparse(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(TLSparse {
    event: Event {
      name:   name,
      desc:   "Parses TLS files for key information.".to_string(),
      usage:  "tlsparse <file_path>".to_string(),
      parent: parent,
      author: "Matzr3lla & Pierre Chifflier (github.com/rusticata)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** FITS_Parse ***********************************/
pub struct FITSparse { event: Event }
impl Eventable for FITSparse {
  fn on_init(&self) -> Vec<String> {
    return Vec::new();
  }
  fn on_run(&self, args: Vec<String>) -> String {
    // Run command
    let output = simple_match!(run_console_command(Command::new("ext/tlsparse/TLSparse").args(args)));

    log::log("tlsparse", &String::from_utf8_lossy(&output.stderr));
    return String::from_utf8_lossy(&output.stdout).to_string();
  }
  fn on_test(&self) -> TestStatus {
    let artifact_path = "tst/ananas/atf/atf.txt";

    // Run command
    let output = match run_bounded_command(Command::new("ext/ananas/ananas").arg("N"), false, 2) {
      Ok(o) => o,
      Err(err) => {
        debug::print_debug(format!("Error: {}", err));
        return TestStatus::Failed;
      }
    };

    // Read artifact file
    let atf_str = match fs::read_to_string(artifact_path) {
      Ok(out) => out,
      Err(err) => {
        debug::print_debug(format!("ananas::on_test: Failed to open the test file. {}", err));
        return TestStatus::Failed;
      }
    };

    // Compare
    if String::from_utf8_lossy(&output.stdout) != atf_str {
      return TestStatus::Failed;
    }
    return TestStatus::Failed;
  }
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn fitsparse(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(FITSparse {
    event: Event {
      name:   name,
      desc:   "Parses FITS files for key information.".to_string(),
      usage:  "fitsparse <file_path>".to_string(),
      parent: parent,
      author: "Matzr3lla & Matthew Stadelman (github.com/stadelmanma)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

/*********************************** jindex ***********************************/
pub struct Jindex { event: Event }
impl Eventable for Jindex {
	fn on_init(&self) -> Vec<String> {
    return Vec::new();
	}
	fn on_run(&self, args: Vec<String>) -> String {
		// Run command
		if args.len() != 1 { return self.event.usage.clone(); }
    let output = match Command::new("ext/jindex/jindex").args(vec!["-s",";;;",&args[0]]).output() {
			Ok(o) => o,
			Err(e) => {
        print::print_custom_uncapped("Error running jindex","orange");
        return format!("{}\n",e);
      }  
		};
    let out = String::from_utf8_lossy(&output.stdout);
    let out_nl = out.split("\n").collect::<Vec<&str>>();
    for i in 0..out_nl.len() - 1 {
      let out_space = out_nl[i].split(";;;").collect::<Vec<&str>>();
      print::print_custom("/","gold");
      let first_tkn = out_space[0].split("/").collect::<Vec<&str>>();
      if first_tkn.len() > 1 {
        print::print_custom(first_tkn[1],"rose");
        print::print_custom("/","gold");
      }
      if first_tkn.len() > 2 {
        print::print_custom(first_tkn[2],"orange");
        print::print_custom(" --> ","grey");
      }
      if first_tkn.len() > 1 {
        print::print_custom(out_space[1],"purple");
        print::print_custom("\n","white");
      }
    }
    return String::from("");
	}
	fn on_test(&self) -> TestStatus {
    let inp_path = "tst/jindex/inp/test.json";
    let atf_path = "tst/jindex/atf/atf.txt";

    // Run command
    let output = simple_test_match!(Command::new("ext/jindex/jindex").arg(inp_path).output());

    // Read file
    let file_str = simple_test_match!(fs::read_to_string(atf_path));

    // Compare
    if String::from_utf8_lossy(&output.stdout).to_string() != file_str {
      return TestStatus::Failed;
    }
    return TestStatus::Passed;
	}
  fn get_event(&self) -> &Event { return &self.event; }
}
pub fn jindex(links: Vec<String>, name: String, parent: String) -> Box<dyn Eventable + Send + Sync> {
  Box::new(Jindex {
    event: Event {
      name:   name,
      desc:   "Parses JavaScript files for key information.".to_string(),
      usage:  "jindex <path_to_JSON_file>\n".to_string(),
      parent: parent,
      author: "Clark Kampfe (github.com/ckampfe)".to_string(),
      easyrun: false,
      secure: false,
      links:  links
    }
  })
}

