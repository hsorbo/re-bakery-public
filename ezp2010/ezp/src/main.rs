//use clap::{App, Arg};
//ctrl+option to show rust inlays in vscode
//https://docs.rs/byteorder/1.0.0/byteorder/trait.ByteOrder.html
//https://gill.net.in/posts/reverse-engineering-a-usb-device-with-rust/

use clap::{App, SubCommand};
use ezp::{programmer::UsbProgrammer, programming};
use itertools::Itertools;
use rusb::{ConfigDescriptor, Device, DeviceHandle, GlobalContext, Interface, InterfaceDescriptor};



pub fn only_interface(c: &ConfigDescriptor) -> Interface {
    return c
        .interfaces()
        .exactly_one()
        .map_err(|_| "Interface not found")
        .unwrap();
}


fn open_usb() -> Result<bool, Box<dyn std::error::Error>> {
    let usb = ezp::programmer::UsbProgrammerContext::open()?;
    let iface = only_interface(&usb.config);
    let ifdesc = iface.descriptors().exactly_one().map_err(|_| "not found")?;
    let p = UsbProgrammer::create_programmer(usb.handle, &ifdesc);
    return Ok(true);
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

    if let Some(_) = matches.subcommand_matches("info") {
        match open_usb() {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }
}
