use crate::bin_io::read::BinRead;
use crate::bin_io::write::BinWrite;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Error, ErrorKind, Read, Write};

const TERRAN_CODE: u32 = 0;
const VOSSK_CODE: u32 = 1;
const NIVELIAN_CODE: u32 = 2;
const MIDORIAN_CODE: u32 = 3;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Faction {
    Terran,
    Vossk,
    Nivelian,
    Midorian,
}

impl BinRead for Faction {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        match source.read_u32::<O>()? {
            TERRAN_CODE => Ok(Self::Terran),
            VOSSK_CODE => Ok(Self::Vossk),
            NIVELIAN_CODE => Ok(Self::Nivelian),
            MIDORIAN_CODE => Ok(Self::Midorian),
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid faction code")),
        }
    }
}

impl BinWrite for Faction {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        destination.write_u32::<O>(match self {
            Self::Terran => TERRAN_CODE,
            Self::Vossk => VOSSK_CODE,
            Self::Nivelian => NIVELIAN_CODE,
            Self::Midorian => MIDORIAN_CODE,
        })
    }
}
