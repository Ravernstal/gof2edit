use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::data::save::agent::SaveAgent;
use crate::data::save::inventory_item::SaveInventoryItem;
use crate::data::save::ship::SaveShip;
use crate::result::Result;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
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
        let length = u32::try_from(self.len())?;
        destination.write_u32::<O>(length - 1)?;

        self.iter()
            .try_for_each(|station| destination.write_bin::<O>(station))
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
        match self {
            Some(station) => {
                destination.write_i32::<O>(station.index)?;
                destination.write_bin::<O>(&station.items)?;
                destination.write_bin::<O>(&station.ships)?;
                destination.write_bin::<O>(&station.agents)?;
                destination.write_u8(station.has_attacked_friends.into())?
            }
            None => destination.write_i32::<O>(-1)?,
        }

        Ok(())
    }
}
