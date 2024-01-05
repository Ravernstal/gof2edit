use crate::bin_io::read::BinRead;
use crate::bin_io::write::BinWrite;
use crate::result::Result;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
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
        match self {
            Some(image_parts) => {
                destination.write_i32::<O>(image_parts.parts.len().try_into()?)?;

                image_parts
                    .parts
                    .iter()
                    .try_for_each(|part| destination.write_i32::<O>(*part))?
            }
            None => destination.write_i32::<O>(-1)?,
        }

        Ok(())
    }
}
