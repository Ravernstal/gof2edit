use byteorder::WriteBytesExt;
use std::fs::File;
use std::io;
use std::io::{Seek, SeekFrom};

pub fn set_byte(file: &mut File, address: u64, byte: u8) -> io::Result<()> {
    file.seek(SeekFrom::Start(address))?;
    file.write_u8(byte)
}
