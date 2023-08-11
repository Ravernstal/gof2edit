use byteorder::WriteBytesExt;
use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Seek, SeekFrom};
use std::path::Path;
use std::{fs, io};

pub fn file_with_counts<T: for<'de> Deserialize<'de>>(
    json_filepath: impl AsRef<Path>,
    binary_filepath: impl AsRef<Path>,
    address_modifiers: &[(u64, i8)],
) -> io::Result<usize> {
    let json = fs::read_to_string(json_filepath)?;
    let data = serde_json::from_str::<Vec<T>>(&json)?;
    let count = data.len().try_into().map_err(|_| ErrorKind::InvalidData)?;

    let mut file = OpenOptions::new().write(true).open(binary_filepath)?;

    address_list_modifiers(&mut file, address_modifiers, count)?;

    Ok(data.len())
}

pub fn address_list_modifiers(
    file: &mut File,
    address_modifiers: &[(u64, i8)],
    byte: u8,
) -> io::Result<()> {
    address_modifiers
        .iter()
        .try_for_each(|&(address, modifier)| {
            let byte = i16::from(byte) + i16::from(modifier);
            let byte = byte.try_into().map_err(|_| ErrorKind::InvalidData)?;

            set_byte(file, address, byte)
        })
}

pub fn set_byte(file: &mut File, address: u64, byte: u8) -> io::Result<()> {
    file.seek(SeekFrom::Start(address))?;
    file.write_u8(byte)
}
