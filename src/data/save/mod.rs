use crate::bin_io::read::BinRead;
use crate::bin_io::write::BinWrite;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Save {
    unknown_bytes: Vec<u8>,
}

impl BinRead for Save {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        Ok(Self {
            unknown_bytes: read_remaining(source)?,
        })
    }
}

impl BinWrite for Save {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        destination.write_all(&self.unknown_bytes)
    }
}

fn read_remaining(source: &mut impl Read) -> io::Result<Vec<u8>> {
    let mut buffer = vec![];
    source.read_to_end(&mut buffer)?;

    Ok(buffer)
}
