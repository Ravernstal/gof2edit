use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::BinWrite;
use crate::data::save::agent::SaveAgent;
use crate::data::save::inventory_item::SaveInventoryItem;
use crate::data::save::ship::SaveShip;
use crate::result::Result;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct SaveStation {
    pub index: i32,
    pub items: Vec<SaveInventoryItem>,
    pub ships: Vec<SaveShip>,
    pub agents: Vec<SaveAgent>,
    pub has_attacked_friends: bool,
}

impl BinRead for Vec<Option<SaveStation>> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let count = source.read_i32::<O>()? + 1;

        (0..count).map(|_| source.read_bin::<O>()).collect()
    }
}

impl BinWrite for Vec<Option<SaveStation>> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        todo!()
    }
}

impl BinRead for Option<SaveStation> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let index = source.read_i32::<O>()?;

        if index == -1 {
            return Ok(None);
        }

        Ok(Some(SaveStation {
            index,
            items: source.read_bin::<O>()?,
            ships: source.read_bin::<O>()?,
            agents: source.read_bin::<O>()?,
            has_attacked_friends: source.read_u8()? != 0,
        }))
    }
}

impl BinWrite for Option<SaveStation> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        todo!()
    }
}
