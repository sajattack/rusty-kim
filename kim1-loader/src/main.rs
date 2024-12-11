#![feature(trait_upcasting)]

use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Write;

use serialport::SerialPort;

#[derive(Debug)]
pub enum Error {
    WrongExtension,
    InvalidFile,
    IoError(std::io::Error),
    SerialPortError(serialport::Error),
    Utf8Error(std::str::Utf8Error),
    InvalidArgs,
}

fn main() -> Result<(), Error> {
    let mut port = serialport::new("/dev/ttyUSB0", 300).open().expect("Failed to open port");
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(Error::InvalidArgs)
    }
    run_on_serialport(port.as_mut(), &args[0], &args[1])
}

fn run_on_serialport<P: AsRef<Path>>(port: &mut dyn SerialPort, program_path: P, input_path: P) -> Result<(), Error>
{
    load_bin_file_as_papertape(port, program_path.as_ref(), 0x200)?;
    feed_inputs_from_file(port, input_path.as_ref())
}

fn run_on_emulator() -> Result<(), Error> {
    todo!()
}

fn load_papertape_from_file<P: AsRef<Path>>(port: &mut dyn Write, file: P) -> Result<(), Error> {
    if file.as_ref().extension().unwrap_or_default() != "ptp" {
        return Err(Error::WrongExtension);
    } else {
        let mut file = File::open(file.as_ref()).map_err(|e| Error::IoError(e))?;
        let mut data = Vec::new();
        file.read_to_end(&mut data).map_err(|e| Error::IoError(e))?;
        load_papertape(port, &data)?;
    }
    Ok(())
}

fn load_papertape(port: &mut dyn Write, data: &[u8]) -> Result<(), Error> {
    port.write(b"L").map_err(|e| Error::IoError(e))?;
    port.write_all(&data).map_err(|e| Error::IoError(e))?;
    Ok(())
}

fn load_bin_file_as_papertape<P: AsRef<Path>> (port: &mut dyn Write, file: P, start_address: u16) -> Result<(), Error> {
    load_papertape(port,  convert_binary_file_to_papertape(file, start_address)?.as_bytes())
}

fn load_bin_as_papertape(port: &mut dyn Write, data: &[u8], start_address: u16) -> Result<(), Error> {
    load_papertape(port,  convert_binary_to_papertape(data, start_address)?.as_bytes())
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
    for (i, data_slice) in data.windows(record_length as usize).enumerate() {
        let address = start_address + i as u16 * record_length;
        let address_string = format!("{:04X}", address);
        let address_bytes = address.to_be_bytes();
        let mut checksum = record_length + address_bytes[0] as u16 + address_bytes[1] as u16;
        let mut record = String::from(";18") + address_string.as_str();
            for byte in data_slice {
                checksum += *byte as u16;
                record += format!("{:02X}", *byte).as_str();
            }
        record += format!("{:04X}", checksum).as_str();
        out += record.as_str();
        out += "\r";
    } 
    Ok(out)
}

fn feed_inputs_from_file<P: AsRef<Path>>(port: &mut dyn Write, file: P) -> Result<(), Error> {
    todo!()
}
