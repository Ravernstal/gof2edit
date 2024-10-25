use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::result::Result;
use crate::Error;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

const UNOBTAINED_CODE: u32 = 0;
const GOLD_CODE: u32 = 1;
const SILVER_CODE: u32 = 2;
const BRONZE_CODE: u32 = 3;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Medal {
    Unobtained,
    Gold,
    Silver,
    Bronze,
}

impl BinRead for Vec<Medal> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let count = source.read_i32::<O>()?;

        (0..count).map(|_| source.read_bin::<O>()).collect()
    }
}

impl BinWrite for Vec<Medal> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        let length = self.len().try_into()?;
        destination.write_u32::<O>(length)?;

        self.iter()
            .try_for_each(|wanted| destination.write_bin::<O>(wanted))
    }
}

impl BinRead for Medal {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        match source.read_u32::<O>()? {
            UNOBTAINED_CODE => Ok(Self::Unobtained),
            GOLD_CODE => Ok(Self::Gold),
            SILVER_CODE => Ok(Self::Silver),
            BRONZE_CODE => Ok(Self::Bronze),
            value => Err(Error::KaamoStatusParse(value)),
        }
    }
}

impl BinWrite for Medal {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        destination.write_u32::<O>(match self {
            Self::Unobtained => UNOBTAINED_CODE,
            Self::Gold => GOLD_CODE,
            Self::Silver => SILVER_CODE,
            Self::Bronze => BRONZE_CODE,
        })?;

        Ok(())
    }
}
