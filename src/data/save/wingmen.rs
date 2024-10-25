use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::data::Faction;
use crate::result::Result;
use crate::wide_string::WideString;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Wingmen {
    pub names: Vec<String>,
    pub race: Faction,
    pub time_remaining: i32,
    pub image_parts: Vec<i32>,
}

impl BinRead for Option<Wingmen> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let count = source.read_i32::<O>()?;

        if count == -1 {
            return Ok(None);
        }

        let names = (0..count)
            .map(|_| WideString::read_bin::<O>(source))
            .collect::<Result<Vec<_>>>()?;
        let names = names.into_iter().map(|string| string.get()).collect();

        Ok(Some(Wingmen {
            names,
            race: source.read_bin::<O>()?,
            time_remaining: source.read_i32::<O>()?,
            image_parts: source.read_bin::<O>()?,
        }))
    }
}

impl BinWrite for Option<Wingmen> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        match self {
            Some(wingmen) => {
                destination.write_i32::<O>(wingmen.names.len().try_into()?)?;
                wingmen.names.iter().try_for_each(|string| {
                    destination.write_bin::<O>(&WideString::new(string.clone()))
                })?;

                destination.write_bin::<O>(&wingmen.race)?;
                destination.write_i32::<O>(wingmen.time_remaining)?;
                destination.write_bin::<O>(&wingmen.image_parts)?
            }
            None => destination.write_i32::<O>(-1)?,
        }

        Ok(())
    }
}
