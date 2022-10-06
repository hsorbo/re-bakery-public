//use clap::{App, Arg};
//ctrl+option to show rust inlays in vscode
//https://docs.rs/byteorder/1.0.0/byteorder/trait.ByteOrder.html
//https://gill.net.in/posts/reverse-engineering-a-usb-device-with-rust/

use std::time::Duration;

use ezp::db;
use ezp::db::ChipDbEntry;
use ezp::ezp_commands;
use ezp::ezp_common::ChipType;
use itertools::Itertools;
use rusb::DeviceHandle;
use rusb::EndpointDescriptor;
use rusb::GlobalContext;
use rusb::InterfaceDescriptor;

pub trait Programmer {
    fn read(&self, buf: &mut [u8]) -> Result<usize, rusb::Error>;
    fn write(&self, buf: &[u8]) -> Result<usize, rusb::Error>;
}

struct UsbProgrammer<'a> {
    handle: DeviceHandle<GlobalContext>,
    fin: EndpointDescriptor<'a>,
    fout: EndpointDescriptor<'a>,
}
impl UsbProgrammer<'_> {
    fn create_programmer<'a>(
        handle: DeviceHandle<GlobalContext>,
        ifdesc: &'a InterfaceDescriptor,
    ) -> UsbProgrammer<'a> {
        return UsbProgrammer {
            handle: handle,
            fout: ifdesc
                .endpoint_descriptors()
                .find(|x| x.direction() == rusb::Direction::Out)
                .unwrap(),
            fin: ifdesc
                .endpoint_descriptors()
                .find(|x| x.direction() == rusb::Direction::In)
                .unwrap(),
        };
    }
}
impl Programmer for UsbProgrammer<'_> {
    fn write(&self, buf: &[u8]) -> Result<usize, rusb::Error> {
        let timeout: Duration = core::time::Duration::from_millis(10000);
        return self.handle.write_bulk(self.fout.address(), buf, timeout);
    }
    fn read(&self, buf: &mut [u8]) -> Result<usize, rusb::Error> {
        let timeout: Duration = core::time::Duration::from_millis(10000);
        return self.handle.read_bulk(self.fin.address(), buf, timeout);
    }
}

fn get_serial(p: &UsbProgrammer) -> Result<String, Box<dyn std::error::Error>> {
    let mut data: [u8; 512] = [0x00; 512];
    let _ = p.write(&ezp_commands::create_serial_cmd());
    let read = p.read(&mut data)?;
    return Ok(ezp_commands::process_serial(&data[..read].to_vec())?);
}

fn get_version(p: &UsbProgrammer) -> Result<String, Box<dyn std::error::Error>> {
    let mut data: [u8; 512] = [0x00; 512];
    let _ = p.write(&ezp_commands::create_version_cmd());
    let read = p.read(&mut data)?;
    return Ok(ezp_commands::process_version(&data[..read].to_vec())?);
}

fn self_test(p: &UsbProgrammer) -> Result<String, Box<dyn std::error::Error>> {
    let mut data: [u8; 512] = [0x00; 512];
    let _ = p.write(&ezp_commands::create_self_test_cmd());
    let _ = p.read(&mut data)?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    let read = p.read(&mut data)?;
    return Ok(String::from_utf8(data[..read].to_vec())?);
}

fn detect(p: &UsbProgrammer, chip_type: &ChipType) -> Result<String, Box<dyn std::error::Error>> {
    let mut data: [u8; 5] = [0x00; 5];
    let cmd = &ezp_commands::create_detect_cmd(chip_type);
    let _ = p.write(cmd);
    std::thread::sleep(std::time::Duration::from_millis(100));
    let read = p.read(&mut data)?;
    return Ok(hex::encode(&data[..read]));
}

fn read(p: &UsbProgrammer, chip: &ChipDbEntry) -> Result<(), Box<dyn std::error::Error>> {
    let mut data: [u8; 4096] = [0x00; 4096];
    let cmd = &ezp_commands::create_read_cmd(&chip.chip_type, chip.size, chip.flags(), chip.is5v());
    let _ = p.write(cmd);
    std::thread::sleep(std::time::Duration::from_millis(100));
    let read = p.read(&mut data)?;
    println!("{}", hex::encode(&data[..read]));
    //asert 110100
    loop {
        let read = p.read(&mut data)?;
        println!("{}", read);
        if read < 4096 {
            break;
        }
    }
    return Ok(());
}

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
    //println!("Programmer: {}\nS/N: {}", get_version(&p)?, get_serial(&p)?);
    println!("Programmer reported: {}", self_test(&p)?);
    //println!("{}", detect(&p, &ChipType::EE24)?);
    //let all = db::getall()?;
    //let chip = all.iter().find(|x| x.product_name == "EN25F80").unwrap();
    //let _ = read(&p, chip);

    return Ok(true);
}

fn main() {
    open_usb().unwrap();
}
