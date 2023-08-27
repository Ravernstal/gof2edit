use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::index::Index;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Station {
    pub index: u32,
    pub name: String,
    pub system_index: u32,
    pub tech_level: u32,
    pub texture_index: u32,
}

impl Index for Station {
    fn index(&self) -> u32 {
        self.index
    }

    fn set_index(&mut self, index: u32) {
        self.index = index;
    }
}

impl BinRead for Station {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        let name = source.read_bin::<O>()?;
        let index = source.read_u32::<O>()?;
        let system_index = source.read_u32::<O>()?;
        let tech_level = source.read_u32::<O>()?;
        let texture_index = source.read_u32::<O>()?;

        Ok(Self {
            index,
            name,
            system_index,
            tech_level,
            texture_index,
        })
    }
}

impl BinWrite for Station {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        destination.write_bin::<O>(&self.name)?;
        destination.write_u32::<O>(self.index)?;
        destination.write_u32::<O>(self.system_index)?;
        destination.write_u32::<O>(self.tech_level)?;
        destination.write_u32::<O>(self.texture_index)?;

        Ok(())
    }
}
