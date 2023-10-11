use crate::bin_io::read::BinRead;
use crate::bin_io::write::BinWrite;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl BinRead for Position {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> std::io::Result<Self> {
        Ok(Self {
            x: source.read_i16::<O>()?,
            y: source.read_i16::<O>()?,
            z: source.read_i16::<O>()?,
        })
    }
}

impl BinWrite for Position {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> std::io::Result<()> {
        destination.write_i16::<O>(self.x)?;
        destination.write_i16::<O>(self.y)?;
        destination.write_i16::<O>(self.z)
    }
}
