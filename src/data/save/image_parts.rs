use crate::bin_io::read::BinRead;
use crate::bin_io::write::BinWrite;
use crate::result::Result;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageParts {
    pub parts: Vec<i32>,
}

impl BinRead for Option<ImageParts> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let count = source.read_i32::<O>()?;

        if count == -1 {
            return Ok(None);
        }

        Ok(Some(ImageParts {
            parts: vec![
                source.read_i32::<O>()?,
                source.read_i32::<O>()?,
                source.read_i32::<O>()?,
                source.read_i32::<O>()?,
                source.read_i32::<O>()?,
            ],
        }))
    }
}

impl BinWrite for Option<ImageParts> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        todo!()
    }
}
