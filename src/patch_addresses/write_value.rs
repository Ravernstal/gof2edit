use byteorder::WriteBytesExt;
use std::io;
use std::io::Write;

pub fn default(destination: &mut dyn Write, value: u8) -> io::Result<()> {
    destination.write_u8(value)
}

pub fn minus_one(destination: &mut dyn Write, value: u8) -> io::Result<()> {
    destination.write_u8(value.saturating_sub(1))
}
