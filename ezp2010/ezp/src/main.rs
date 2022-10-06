//use clap::{App, Arg};

//const USB_ID: (u16, u16) = (0x10c4, 0xf5a0);
// //type MathResult = Result<Vec<rusb::Device<rusb::GlobalContext>>, std::io::Error>;
// fn get_device() -> Result<Device<GlobalContext>, rusb::Error> {
//     let devices = rusb::devices()?
//         .iter()
//         .filter(|x| {
//             let device_desc = x.device_descriptor().unwrap();
//             return USB_ID == (device_desc.vendor_id(), device_desc.product_id());
//         })
//         .collect::<Vec<_>>();
//     //let a = rusb::devices().unwrap();
//     //let devices = a.iter().nth(0);
//     //return devices.map_or(Err(rusb::Error::NotFound), Ok);
//     return match &devices[..] {
//         [x] => Ok(x.clone()),
//         _ => Err(rusb::Error::NotFound),
//     };

//}
//https://gill.net.in/posts/reverse-engineering-a-usb-device-with-rust/
//ctrl+option to show rust inlays in vscode
//https://docs.rs/byteorder/1.0.0/byteorder/trait.ByteOrder.html
//https://gill.net.in/posts/reverse-engineering-a-usb-device-with-rust/

// fn get_descr(
//     device: Device<GlobalContext>,
// ) -> Result<(EndpointDescriptor, EndpointDescriptor), Box<dyn std::error::Error>> {
// }

use ezp::ezp_commands;
use ezp::db;

fn open_usb() -> Result<bool, Box<dyn std::error::Error>> {
    let mut handle = rusb::open_device_with_vid_pid(0x10c4, 0xf5a0).ok_or("Not Found")?;
    //handle.reset()?;
    let device = handle.device();
    let descr = device.device_descriptor()?;
    //println!("{:#?}", descr);
    assert_eq!(1, descr.num_configurations());
    let config = device.config_descriptor(0)?;
    //println!("{:#?}", config);
    assert_eq!(1, config.num_interfaces());
    let iface = config.interfaces().nth(0).ok_or("Interface not found")?;
    assert_eq!(1, iface.descriptors().count());
    let ifdesc = iface
        .descriptors()
        .nth(0)
        .ok_or("Interface descriptor not found")?;
    //println!("{:#?}", ifdesc);
    let fout = ifdesc
        .endpoint_descriptors()
        .find(|x| x.direction() == rusb::Direction::Out)
        .ok_or("meh")?;

    let fin = ifdesc
        .endpoint_descriptors()
        .find(|x| x.direction() == rusb::Direction::In)
        .ok_or("meh")?;

    handle.set_auto_detach_kernel_driver(true)?;
    handle.set_active_configuration(config.number())?;
    handle.claim_interface(iface.number())?;
    //handle.reset()?;

    let timeout = core::time::Duration::from_millis(10000);
    println!("fin {} fout {}", fin.address(), fout.address());

    let mut data: [u8; 4096] = [0x00; 4096];
    // {
    //     handle.write_bulk(fout.address(), &ezp_commands::create_serial_cmd(), timeout)?;
    //     let read = handle.read_bulk(fin.address(), &mut data, timeout)?;
    //     println!("{:#?}", ezp_commands::process_serial(&data[..read].to_vec())?);
    // }

    // {
    //     handle.write_bulk(fout.address(), &ezp_commands::create_version_cmd(), timeout)?;
    //     let read = handle.read_bulk(fin.address(), &mut data, timeout)?;
    //     println!("{:#?}", ezp_commands::process_version(&data[..read].to_vec())?);
    // }

    // {
    //     handle.write_bulk(fout.address(), &ezp_commands::create_self_test_cmd(), timeout)?;
    //     handle.read_bulk(fin.address(), &mut data, timeout)?;
    //     std::thread::sleep(std::time::Duration::from_millis(200));
    //     let read = handle.read_bulk(fin.address(), &mut data, timeout)?;
    //     println!("{:#?}",  String::from_utf8(data[..read].to_vec())?);
    // }
    // {
    //     let cmd = &ezp_commands::create_detect_cmd(&ezp::ezp_common::ChipType::EE25);
    //     handle.write_bulk(fout.address(), cmd, timeout)?;
    //     std::thread::sleep(std::time::Duration::from_millis(200));
    //     let read = handle.read_bulk(fin.address(), &mut data, timeout)?;
    //     println!("{}",hex::encode(&data[..read]));
    // }
    {
        let all = db::getall().unwrap();
        let chip = all.iter().find(|x| x.product_name == "EN25F80").unwrap();
        let cmd = &ezp_commands::create_read_cmd(&chip.chip_type, chip.size, chip.flags(), chip.is5v());
        handle.write_bulk(fout.address(), cmd, timeout)?;
        std::thread::sleep(std::time::Duration::from_millis(200));
        let read = handle.read_bulk(fin.address(), &mut data, timeout)?;
        println!("{}",hex::encode(&data[..read]));
        //asert 110100.
        loop {
            let read = handle.read_bulk(fin.address(), &mut data, timeout)?;
            println!("{}", read);
            if read < 4096{
                break;
            }
        }

    }

    return Ok(true);
}

fn main() {
    open_usb().unwrap();
}
