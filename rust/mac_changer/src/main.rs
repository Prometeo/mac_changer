extern crate clap;
use clap::{App, Arg};

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

    let interface = matches.value_of("interface").unwrap_or("none");
    println!("{:?}", interface)
}

//example cargo run -- -help
