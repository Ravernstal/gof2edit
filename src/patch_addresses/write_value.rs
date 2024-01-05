use byteorder::WriteBytesExt;
use std::io::Write;

pub fn default(destination: &mut dyn Write, value: u8) -> gof2edit::Result<()> {
    destination.write_u8(value).map_err(|error| error.into())
}

pub fn minus_one(destination: &mut dyn Write, value: u8) -> gof2edit::Result<()> {
    destination
        .write_u8(value.saturating_sub(1))
        .map_err(|error| error.into())
}
