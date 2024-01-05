use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::result::Result;
use crate::wide_string::WideString;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct SavePendingProduct {
    pub unknown_int_1: i32,
    pub unknown_int_2: i32,
    pub station_index: i32,
    pub station_name: String,
}

impl BinRead for Vec<SavePendingProduct> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let count = source.read_i32::<O>()?;

        (0..count).map(|_| source.read_bin::<O>()).collect()
    }
}

impl BinWrite for Vec<SavePendingProduct> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        let length = self.len().try_into()?;
        destination.write_u32::<O>(length)?;

        self.iter()
            .try_for_each(|product| destination.write_bin::<O>(product))
    }
}

impl BinRead for SavePendingProduct {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        Ok(Self {
            unknown_int_1: source.read_i32::<O>()?,
            unknown_int_2: source.read_i32::<O>()?,
            station_index: source.read_i32::<O>()?,
            station_name: WideString::read_bin::<O>(source)?.get(),
        })
    }
}

impl BinWrite for SavePendingProduct {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        destination.write_i32::<O>(self.unknown_int_1)?;
        destination.write_i32::<O>(self.unknown_int_2)?;
        destination.write_i32::<O>(self.station_index)?;
        destination.write_bin::<O>(&WideString::new(self.station_name.clone()))
    }
}
