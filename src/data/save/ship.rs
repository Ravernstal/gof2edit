use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::BinWrite;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct SaveShip {
    pub index: i32,
    pub race: i32,
}

impl BinRead for Vec<SaveShip> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        let count = source.read_i32::<O>()?;

        (0..count).map(|_| source.read_bin::<O>()).collect()
    }
}

impl BinWrite for Vec<SaveShip> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        todo!()
    }
}

impl BinRead for SaveShip {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        Ok(Self {
            index: source.read_i32::<O>()?,
            race: source.read_i32::<O>()?,
        })
    }
}

impl BinWrite for SaveShip {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        todo!()
    }
}
