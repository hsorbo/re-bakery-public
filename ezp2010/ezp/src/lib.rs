pub mod ezp_commands {
    use std::{error::Error, fmt};

    use byteorder::*;

    #[derive(Debug)]
    struct MyError {
        details: String,
    }

    impl MyError {
        fn new(msg: &str) -> MyError {
            MyError {
                details: msg.to_string(),
            }
        }
    }
    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.details)
        }
    }

    impl Error for MyError {
        fn description(&self) -> &str {
            &self.details
        }
    }

    pub enum ChipType {
        Spi,
        EE24,
        EE25,
        EE93,
    }
    fn chip_to_u8(t: ChipType) -> u8 {
        return match t {
            ChipType::Spi => 0x01,
            ChipType::EE24 => 0x02,
            ChipType::EE25 => 0x03,
            ChipType::EE93 => 0x04,
        };
    }

    pub fn create_read_cmd(chip_type: ChipType, size: u32, flags: u16, is5v: bool) -> Vec<u8> {
        //'> H B 4x i H B x'
        let mut data = vec![0x11, 0x0a, chip_to_u8(chip_type), 0x00, 0x00, 0x00, 0x00];
        data.write_u32::<NetworkEndian>(size).unwrap();
        data.write_u16::<NetworkEndian>(flags).unwrap();
        data.push(if is5v { 0x01 } else { 0x00 });
        return data;
    }

    pub fn create_write_cmd(chip_type: ChipType, size: u32, flags: u16, is5v: bool) -> Vec<u8> {
        //'> H B 4x i H H B x',
        let mut data = vec![0x12, 0x0c, chip_to_u8(chip_type), 0x00, 0x00, 0x00, 0x00];
        data.write_u32::<NetworkEndian>(size).unwrap();
        ////0x01 if db_entry['unk_write_1'] == 0 else db_entry['unk_write_2'],
        data.write_u16::<NetworkEndian>(flags).unwrap();
        data.push(if is5v { 0x01 } else { 0x00 });
        return data;
    }

    pub fn create_detect_cmd(chip_type: ChipType) -> Vec<u8> {
        return vec![0x15, 0x00, chip_to_u8(chip_type)];
    }

    pub fn create_version_cmd() -> Vec<u8> {
        return vec![0x17, 0x00];
    }

    pub fn process_version(data: &Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
        if data[..2] != [0x17, 0x1e] {
            return Err(Box::new(MyError::new("No")));
        }
        //core::slice::ascii::escape_ascii
        let end = data[2..].iter().position(|x| x == &0x00).unwrap_or(0);
        return Ok(String::from_utf8(data[2..end].to_vec())?);
    }

    pub fn create_serial_cmd() -> Vec<u8> {
        return vec![0x18, 0x00];
    }

    pub fn process_serial(data: &Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
        if data[..2] != [0x18, 0x10] {
            return Err(Box::new(MyError::new("No")));
        }
        //TODO: Why 2x 0xff at end?
        return Ok(String::from_utf8(data[2..16].to_vec())?);
    }

    pub fn create_self_test_cmd() -> Vec<u8> {
        return vec![0xf3, 0x00];
    }
}