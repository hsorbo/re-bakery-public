//use clap::{App, Arg};
//ctrl+option to show rust inlays in vscode
//https://docs.rs/byteorder/1.0.0/byteorder/trait.ByteOrder.html
//https://gill.net.in/posts/reverse-engineering-a-usb-device-with-rust/

mod arguments;

use clap::Parser;
use ezp::{db, programmer::UsbProgrammer, programming};
use itertools::Itertools;
use rusb::{ConfigDescriptor, Interface};

pub fn only_interface(c: &ConfigDescriptor) -> Interface {
    return c
        .interfaces()
        .exactly_one()
        .map_err(|_| "Interface not found")
        .unwrap();
}

fn mein(arg: arguments::EzpArgs) -> Result<(), Box<dyn std::error::Error>> {
    match arg.command {
        arguments::Command::Read(_) | arguments::Command::Write(_) | arguments::Command::Info => {
            let usb = ezp::programmer::UsbProgrammerContext::open()?;
            let ifdesc = only_interface(&usb.config)
                .descriptors()
                .exactly_one()
                .map_err(|_| "not found")?;
            let p = UsbProgrammer::create_programmer(usb.handle, &ifdesc);
            match arg.command {
                arguments::Command::Info => {
                    println!(
                        "Programmer: {}\nS/N: {}\nStatus: {}",
                        programming::get_version(&p)?,
                        programming::get_serial(&p)?,
                        programming::self_test(&p)?
                    );
                }
                arguments::Command::Read(x) => {
                    let chip = db::get_by_product_name(&x.chip_type);
                    match chip {
                        None => println!("Chip not found: {}", x.chip_type),
                        Some(chip) => {
                            println!("Reading....");
                            let mut f = std::fs::File::create(x.file)?;
                            programming::read(&p, &chip, &mut f)?;
                        }
                    }
                }
                arguments::Command::Write(x) => {
                    let chip = db::get_by_product_name(&x.chip_type);
                    match chip {
                        None => println!("Chip not found: {}", x.chip_type),
                        Some(chip) => {
                            println!("writing.... {:?}", chip);
                            
                            
                        }
                    }
                }
                _ => println!("noop"),
            }
        }
        arguments::Command::List => {
            for x in ezp::db::getall() {
                let size_s = human_format::Formatter::new()
                    .with_separator("")
                    .with_decimals(0)
                    .with_units("B")
                    .format(x.size as f64);
                println!(
                    "{: <10} {: <24} {: <5}\t--type='{}' ",
                    x.vendor_name, x.product_name, size_s, x.product_name
                );
            }
        }
    }
    return Ok(());
}

fn main() {
    let args = arguments::EzpArgs::parse();
    match mein(args) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
