use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::data::Position;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Engine {
    pub position: Position,
    pub intensity: (f32, f32, f32),
}

impl BinRead for Engine {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> std::io::Result<Self> {
        Ok(Self {
            position: source.read_bin::<O>()?,
            intensity: (
                source.read_f32::<O>()?,
                source.read_f32::<O>()?,
                source.read_f32::<O>()?,
            ),
        })
    }
}

impl BinWrite for Engine {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> std::io::Result<()> {
        destination.write_bin::<O>(&self.position)?;
        destination.write_f32::<O>(self.intensity.0)?;
        destination.write_f32::<O>(self.intensity.1)?;
        destination.write_f32::<O>(self.intensity.2)
    }
}
