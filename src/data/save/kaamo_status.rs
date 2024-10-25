use crate::bin_io::read::BinRead;
use crate::bin_io::write::BinWrite;
use crate::result::Result;
use crate::Error;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

const PIRATES_PRESENT_CODE: u32 = 0;
const PIRATES_DESTROYED_CODE: u32 = 1;
const SECURED_CODE: u32 = 2;
const OWNED_CODE: u32 = 3;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum KaamoStatus {
    PiratesPresent,
    PiratesDestroyed,
    Secured,
    Owned,
}

impl BinRead for KaamoStatus {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        match source.read_u32::<O>()? {
            PIRATES_PRESENT_CODE => Ok(Self::PiratesPresent),
            PIRATES_DESTROYED_CODE => Ok(Self::PiratesDestroyed),
            SECURED_CODE => Ok(Self::Secured),
            OWNED_CODE => Ok(Self::Owned),
            value => Err(Error::KaamoStatusParse(value)),
        }
    }
}

impl BinWrite for KaamoStatus {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        destination.write_u32::<O>(match self {
            Self::PiratesPresent => PIRATES_PRESENT_CODE,
            Self::PiratesDestroyed => PIRATES_DESTROYED_CODE,
            Self::Secured => SECURED_CODE,
            Self::Owned => OWNED_CODE,
        })?;

        Ok(())
    }
}
