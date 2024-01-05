use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::BinWrite;
use crate::result::Result;
use crate::wide_string::WideString;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct UnknownStructure1 {
    pub unknown_string_list_1: Vec<String>,
    pub unknown_int_1: i32,
    pub unknown_int_2: i32,
    pub unknown_int_list_1: Vec<i32>,
}

impl BinRead for Option<UnknownStructure1> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let count = source.read_i32::<O>()?;

        if count == -1 {
            return Ok(None);
        }

        let unknown_string_list_1 = (0..count)
            .map(|_| WideString::read_bin::<O>(source))
            .collect::<Result<Vec<_>>>()?;
        let unknown_string_list_1 = unknown_string_list_1
            .into_iter()
            .map(|string| string.get())
            .collect();

        Ok(Some(UnknownStructure1 {
            unknown_string_list_1,
            unknown_int_1: source.read_i32::<O>()?,
            unknown_int_2: source.read_i32::<O>()?,
            unknown_int_list_1: source.read_bin::<O>()?,
        }))
    }
}

impl BinWrite for Option<UnknownStructure1> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        todo!()
    }
}
