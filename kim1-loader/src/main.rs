#![feature(trait_upcasting)]
#![feature(ascii_char)]

use std::fs::File;
use std::num::ParseIntError;
use std::path::Path;
use std::io::Read;
use std::io::Write;
use std::string::FromUtf8Error;

use serialport::{StopBits, SerialPort};

#[derive(Debug)]
pub enum Error {
    WrongExtension,
    InvalidFile,
    IoError(std::io::Error),
    SerialPortError(serialport::Error),
    Utf8Error(std::str::Utf8Error),
    FromUtf8Error(FromUtf8Error),
    ParseIntError(ParseIntError),
    InvalidArgs,
}

#[repr(u8)]
pub enum Command {
    Space = 0x20, // switch from address to data after entering an address
    CarriageReturnNext = 0x0D, // step to the next address
    ConfirmData = 0x6D,
    LFPrev = 0x0A, // step to the previous address
    Execute = b'G',
}

fn main() -> Result<(), Error> {
    let mut port = serialport::new("/dev/ttyUSB0", 300).open().expect("Failed to open port");
    port.set_stop_bits(StopBits::Two);
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    if args.len() != 3 {
        return Err(Error::InvalidArgs)
    }
    run_on_serialport(port.as_mut(), &args[1], &args[2])?;
    //feed_inputs_from_file(port.as_mut(), "../aoc2024d1p1/test_input_not_mine.txt")?;
    Ok(())
}

fn run_on_serialport<P: AsRef<Path>>(port: &mut dyn SerialPort, program_path: P, input_path: P) -> Result<(), Error>
{
    load_bin_file_as_papertape(port, program_path.as_ref(), 0x200)?;
    //feed_inputs_from_file(port, input_path.as_ref())?;
    Ok(())
}

fn run_on_emulator() -> Result<(), Error> {
    todo!()
}

fn load_papertape_from_file<PATH: AsRef<Path>, PORT>(port: &mut PORT, file: PATH) -> Result<(), Error> 
where PORT: Read+Write+?Sized
{
    if file.as_ref().extension().unwrap_or_default() != "ptp" {
        return Err(Error::WrongExtension);
    } else {
        let mut file = File::open(file.as_ref()).map_err(|e| Error::IoError(e))?;
        let mut data = Vec::new();
        file.read_to_end(&mut data).map_err(|e| Error::IoError(e))?;
        load_papertape(port, std::str::from_utf8(&data).map_err(|e| Error::Utf8Error(e))?.to_string())?
    }
    Ok(())
}

fn load_papertape<PORT>(port: &mut PORT, data: String) -> Result<(), Error> 
where PORT: Read+Write+?Sized
{
    port.write(b"L\r\n").map_err(|e| Error::IoError(e))?;
    for record in data.lines() {
        println!("{}", record);
        //port.write(record.as_bytes());
        record.as_bytes().iter().for_each(|b| {
            port.write(&[*b]);
            port.flush();
            std::thread::sleep_ms(20);
        });
        port.write(b"\r\n");
        port.flush();
        std::thread::sleep_ms(200);
    }
    port.write(&[0x13]);
    port.write(b"\r\n");
    port.flush();
    Ok(())
}

fn load_bin_file_as_papertape<P: AsRef<Path>, PORT> (port: &mut PORT, file: P, start_address: u16) -> Result<(), Error>
where PORT: Read+Write+?Sized
{
    load_papertape(port,  convert_binary_file_to_papertape(file, start_address)?)
}

fn load_bin_as_papertape<PORT>(port: &mut PORT, data: &[u8], start_address: u16) -> Result<(), Error> 
where PORT: Read+Write+?Sized
{
    load_papertape(port,  convert_binary_to_papertape(data, start_address)?)
}

fn convert_binary_file_to_papertape<P: AsRef<Path>>(file: P, start_address: u16) -> Result<String, Error> {
    if file.as_ref().extension().unwrap_or_default() != "bin" {
        return Err(Error::WrongExtension);
    } else {
        let mut file = File::open(file.as_ref()).map_err(|e| Error::IoError(e))?;
        let mut data = Vec::new();
        file.read_to_end(&mut data).map_err(|e| Error::IoError(e))?;
        convert_binary_to_papertape(&data, start_address)
    }
}

fn convert_binary_to_papertape(data: &[u8], start_address: u16) -> Result<String, Error> {
    let mut out = String::new();
    let record_length: u16 = 0x18;
    for (i, data_slice) in data.chunks(record_length as usize).enumerate() {
        let address = start_address + (i as u16 * record_length);
        let address_bytes = address.to_be_bytes();
        let mut checksum = data_slice.len() as u16 + address_bytes[0] as u16 + address_bytes[1] as u16;
        let mut record = format!(";{:02X}{:04X}", data_slice.len() as u16, address);
            for byte in data_slice {
                checksum += *byte as u16;
                record += format!("{:02X}", *byte).as_str();
            }
        record += format!("{:04X}", checksum).as_str();
        out += record.as_str();
        out += "\n";
    } 
    let num_records = (data.len() as f32 / record_length as f32).ceil() as u16;
    let checksum: u16 = num_records.to_be_bytes().iter().map(|rb| *rb as u16).sum();
    let record = format!(";00{:04X}{:04X}\n", num_records, checksum);
    out += record.as_str();
    Ok(out)
}

fn feed_inputs_from_file<P: AsRef<Path>>(port: &mut dyn Write, path: P) -> Result<(), Error> {
    // parse the file ✅
    // feed the data as 32bit pairs
    // execute the subroutine for every pair
    let mut file = File::open(path.as_ref()).map_err(|e| Error::IoError(e))?;
    let mut data = String::new();
    file.read_to_string(&mut data).map_err(|e| Error::IoError(e))?;
    let pairs: Result<Vec<Vec<u32>>, Error> = data
        .lines()
        .map(|l| l.split_whitespace()
            .map(|s| s.parse::<u32>().map_err(|e| Error::ParseIntError(e)))
            .collect()
        ).collect();
    //println!("{:?}", pairs?);

    for pair in pairs?.iter() {
        port.write(b"0020 ").map_err(|e| Error::IoError(e))?;
        for value in pair {
            for byte in value.to_be_bytes().into_iter() {
                let hex_str = format!("{:02X}", byte);
                port.write(hex_str.as_bytes()).map_err(|e| Error::IoError(e))?;
                port.write(&[Command::ConfirmData as u8]).map_err(|e| Error::IoError(e))?;
                port.write(&[Command::CarriageReturnNext as u8]).map_err(|e| Error::IoError(e))?;
            }
        }
        port.write(b"0204G").map_err(|e| Error::IoError(e))?;
    };

    Ok(())
}
