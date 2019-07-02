extern crate clap;
use clap::{App, Arg};
use regex::Regex;
use std::process::Command;

fn main() {
    let matches = App::new("myapp")
        .version("1.0")
        .about("Mac Changer")
        .author("Prometeo")
        .arg(
            Arg::with_name("interface")
                .short("i")
                .long("interface")
                .takes_value(true)
                .help("Interface to change its MAC address"),
        )
        .arg(
            Arg::with_name("mac")
                .short("m")
                .long("mac")
                .takes_value(true)
                .help("New MAC address"),
        )
        .get_matches();

    let interface = String::from(matches.value_of("interface").unwrap_or("none"));
    let mac_address = String::from(matches.value_of("mac").unwrap_or("none"));

    let current_mac = get_current_mac(&interface);
    println!("[+] current MAC for given interface: {}", current_mac);
    let valid_mac_address = validate_mac_address(&mac_address);
    if valid_mac_address {
        println!("[+] Changing MAC address");
        let result = change_mac_address(&interface, &mac_address);
        let result_mac_address = get_current_mac(&interface);
        println!("[+] MAC address changed to: {}", result_mac_address);
    } else {
        println!("[-] Please enter a valid MAC address");
    }
}

fn change_mac_address(interface: &String, mac_address: &String) -> bool {
    let mut mac_down = Command::new("ifconfig");
    mac_down.arg(&interface);
    mac_down.arg("down");

    // let mut mac_change = Command::new("ifconfig");
    // mac_change.arg(&interface);
    // mac_change.arg("hw ether");
    // mac_change.arg(&mac_address);

    // let mut mac_up = Command::new("ifconfig");
    // mac_up.arg(&interface);
    // mac_up.arg("up");

    return true;
}

fn validate_mac_address(mac_address: &String) -> bool {
    let re = Regex::new(r"\w\w:\w\w:\w\w:\w\w:\w\w:\w\w").unwrap();
    let validation: bool;
    match re.captures(&mac_address) {
        Some(_caps) => validation = true,
        None => validation = false,
    }
    return validation;
}

fn get_current_mac(interface: &String) -> String {
    let re = Regex::new(r"\w\w:\w\w:\w\w:\w\w:\w\w:\w\w").unwrap();
    let mut cmd = Command::new("ifconfig");
    let mut output = String::new();
    let mut current_mac = String::new();
    cmd.arg(interface);
    match cmd.output() {
        Ok(o) => unsafe {
            output = String::from_utf8_unchecked(o.stdout);
        },
        Err(e) => {
            println!("There was an error {}", e);
        }
    }
    match re.captures(&output) {
        Some(caps) => current_mac = String::from(caps.get(0).unwrap().as_str()),
        None => current_mac = String::from("MAC not found"),
    }
    return current_mac;
}

//example cargo run -- -help
