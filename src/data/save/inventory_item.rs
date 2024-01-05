use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::result::Result;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct SaveInventoryItem {
    pub index: i32,
    pub amount: i32,
    pub single_price: i32,
    pub unsaleable: bool,
}

impl BinRead for Vec<Option<SaveInventoryItem>> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let length = source.read_u32::<O>()?;
        let length = length.try_into()?;

        (0..length)
            .map(|_| source.read_bin::<O>())
            .collect::<Result<Vec<_>>>()
    }
}

impl BinWrite for Vec<Option<SaveInventoryItem>> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        let length = self.len().try_into()?;
        destination.write_u32::<O>(length)?;

        self.iter().try_for_each(|x| destination.write_bin::<O>(x))
    }
}

impl BinRead for Vec<SaveInventoryItem> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let count = source.read_i32::<O>()?;

        (0..count).map(|_| source.read_bin::<O>()).collect()
    }
}

impl BinWrite for Vec<SaveInventoryItem> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        todo!()
    }
}

impl BinRead for Option<SaveInventoryItem> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let index = source.read_i32::<O>()?;

        if index == -1 {
            return Ok(None);
        }

        let item = SaveInventoryItem {
            index,
            amount: source.read_i32::<O>()?,
            single_price: source.read_i32::<O>()?,
            unsaleable: source.read_u8()? != 0,
        };

        Ok(Some(item))
    }
}

impl BinWrite for Option<SaveInventoryItem> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        match self {
            Some(equipment) => {
                destination.write_i32::<O>(equipment.index)?;
                destination.write_i32::<O>(equipment.amount)?;
                destination.write_i32::<O>(equipment.single_price)?;
                destination.write_u8(equipment.unsaleable.into())?
            }
            None => destination.write_i32::<O>(-1)?,
        }

        Ok(())
    }
}

impl BinRead for SaveInventoryItem {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        Ok(SaveInventoryItem {
            index: source.read_i32::<O>()?,
            amount: source.read_i32::<O>()?,
            single_price: source.read_i32::<O>()?,
            unsaleable: source.read_u8()? != 0,
        })
    }
}

impl BinWrite for SaveInventoryItem {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        destination.write_i32::<O>(self.index)?;
        destination.write_i32::<O>(self.amount)?;
        destination.write_i32::<O>(self.single_price)?;
        destination.write_u8(self.unsaleable.into())?;

        Ok(())
    }
}
