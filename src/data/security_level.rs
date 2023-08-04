use crate::bin_io::read::BinRead;
use crate::bin_io::write::BinWrite;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Error, ErrorKind, Read, Write};

const DANGEROUS_CODE: u32 = 0;
const RISKY_CODE: u32 = 1;
const AVERAGE_CODE: u32 = 2;
const SECURE_CODE: u32 = 3;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum SecurityLevel {
    Dangerous,
    Risky,
    Average,
    Secure,
}

impl BinRead for SecurityLevel {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        match source.read_u32::<O>()? {
            DANGEROUS_CODE => Ok(Self::Dangerous),
            RISKY_CODE => Ok(Self::Risky),
            AVERAGE_CODE => Ok(Self::Average),
            SECURE_CODE => Ok(Self::Secure),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "invalid security level code",
            )),
        }
    }
}

impl BinWrite for SecurityLevel {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        destination.write_u32::<O>(match self {
            Self::Dangerous => DANGEROUS_CODE,
            Self::Risky => RISKY_CODE,
            Self::Average => AVERAGE_CODE,
            Self::Secure => SECURE_CODE,
        })
    }
}
