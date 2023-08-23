use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::mem::size_of;
use std::mem::transmute;

const ENTRY_LEN: usize = 84;
const ENTRY_PAD: i64 = 24;

#[derive(Debug)]
#[repr(C)]
//#[repr(packed(2))]
struct ChipDbEntry {
    chip_type: i32,
    product_name: [u8; 40],
    vendor_name: [u8; 20],
    _unknown1: u8,
    voltage: u8,
    size: i32,
    write_1: i32,
    write_2: i16,
    _unknown2: u8,
    mystisk: u8,
    ee93_unk: u8,
    ee93_bits: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const db_entry_len: usize = size_of::<ChipDbEntry>();
    assert_eq!(ENTRY_LEN, db_entry_len);
    let mut f = File::open("../database/DateBase.bin")?;
    f.seek(SeekFrom::Start(0x64))?;
    let mut buf = [0x00; db_entry_len];
    while f.read(&mut buf)? == db_entry_len {
        let foobar: ChipDbEntry;
        unsafe {
            foobar = transmute::<[u8; db_entry_len], ChipDbEntry>(buf);
        }
        f.seek(SeekFrom::Current(ENTRY_PAD))?;
        //CStr::from_bytes_with_nul(&foobar.product_name) <- unstable
        let prod = std::str::from_utf8(&foobar.product_name)?;
        println!("{:#?}", prod);
        //println!("{:#?}", foobar);
    }
    Ok(())
}
