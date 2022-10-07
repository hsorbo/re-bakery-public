//use clap::{App, Arg};
//ctrl+option to show rust inlays in vscode
//https://docs.rs/byteorder/1.0.0/byteorder/trait.ByteOrder.html
//https://gill.net.in/posts/reverse-engineering-a-usb-device-with-rust/

use clap::{App, ArgMatches, SubCommand};
use ezp::{programmer::UsbProgrammer, programming};
use itertools::Itertools;
use rusb::{ConfigDescriptor, Interface};

pub fn only_interface(c: &ConfigDescriptor) -> Interface {
    return c
        .interfaces()
        .exactly_one()
        .map_err(|_| "Interface not found")
        .unwrap();
}

fn mein(matches: ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("read", _)) | Some(("write", _)) | Some(("info", _)) => {
            let usb = ezp::programmer::UsbProgrammerContext::open()?;
            let ifdesc = only_interface(&usb.config)
                .descriptors()
                .exactly_one()
                .map_err(|_| "not found")?;
            let p = UsbProgrammer::create_programmer(usb.handle, &ifdesc);
            match matches.subcommand() 
            {
                Some(("info", _)) => {
                    println!(
                        "Programmer: {}\nS/N: {}\nStatus: {}",
                        programming::get_version(&p)?,
                        programming::get_serial(&p)?,
                        programming::self_test(&p)?
                    );
                }
                _ => println!("noop")
            }

            
        }
        _ => {
            println!("no io");
        }
    }
    return Ok(());
}

fn main() {
    let matches = App::new("ezp2010")
        .version("0.1")
        .author("Håvard Sørbø <havard@hsorbo.no>")
        .about("Read and write flash-roms using ezp2010")
        .subcommand(
            SubCommand::with_name("read")
                .about("Shows information about connected programmer")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("write")
                .about("Shows information about connected programmer")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .subcommand(
            SubCommand::with_name("info")
                .about("Shows information about connected programmer")
                .arg_from_usage("-d, --debug 'Print debug information'"),
        )
        .get_matches();

    match mein(matches) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
