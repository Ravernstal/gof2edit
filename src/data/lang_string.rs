use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::index::Index;
use crate::result::Result;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct LangString {
    pub index: u32,
    pub string: String,
}

impl Index for LangString {
    fn index(&self) -> u32 {
        self.index
    }

    fn set_index(&mut self, index: u32) {
        self.index = index;
    }
}

impl BinRead for LangString {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let string = source.read_bin::<O>()?;

        Ok(Self { index: 0, string })
    }
}

impl BinWrite for LangString {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        destination.write_bin::<O>(&self.string)
    }
}
