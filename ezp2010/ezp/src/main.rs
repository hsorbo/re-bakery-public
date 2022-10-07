//use clap::{App, Arg};
//ctrl+option to show rust inlays in vscode
//https://docs.rs/byteorder/1.0.0/byteorder/trait.ByteOrder.html
//https://gill.net.in/posts/reverse-engineering-a-usb-device-with-rust/

use clap::{App, SubCommand};
use ezp::{programmer::UsbProgrammer, programming};
use itertools::Itertools;

fn open_usb() -> Result<bool, Box<dyn std::error::Error>> {
    let mut handle = rusb::open_device_with_vid_pid(0x10c4, 0xf5a0).ok_or("Not Found")?;
    let device = handle.device();
    let config = device.config_descriptor(0)?;
    let iface = config
        .interfaces()
        .exactly_one()
        .map_err(|_| "Interface not found")?;
    let ifdesc = iface.descriptors().exactly_one().map_err(|_| "not found")?;

    handle.set_auto_detach_kernel_driver(true)?;
    handle.set_active_configuration(config.number())?;
    handle.claim_interface(iface.number())?;
    //handle.reset()?;

    let p = UsbProgrammer::create_programmer(handle, &ifdesc);

    println!(
        "Programmer: {}\nS/N: {}\nStatus: {}",
        programming::get_version(&p)?,
        programming::get_serial(&p)?,
        programming::self_test(&p)?
    );
    //println!("{}", detect(&p, &ChipType::EE24)?);
    //let all = db::getall()?;
    //let chip = all.iter().find(|x| x.product_name == "EN25F80").unwrap();
    //let _ = read(&p, chip);

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
