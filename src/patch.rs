use byteorder::WriteBytesExt;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Seek, SeekFrom};

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
