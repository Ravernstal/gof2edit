use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::result::Result;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct SaveShipEquipment {
    pub item_index: i32,
    pub amount: i32,
    pub unsaleable: bool,
}

impl BinRead for Vec<Option<SaveShipEquipment>> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let length = source.read_u32::<O>()?;
        let length = length.try_into()?;

        (0..length)
            .map(|_| source.read_bin::<O>())
            .collect::<Result<Vec<_>>>()
    }
}

impl BinWrite for Vec<Option<SaveShipEquipment>> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        let length = self.len().try_into()?;
        destination.write_u32::<O>(length)?;

        self.iter().try_for_each(|x| destination.write_bin::<O>(x))
    }
}

impl BinRead for Option<SaveShipEquipment> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let item_index = source.read_i32::<O>()?;

        if item_index == -1 {
            return Ok(None);
        }

        let equipment = SaveShipEquipment {
            item_index,
            amount: source.read_i32::<O>()?,
            unsaleable: source.read_u8()? != 0,
        };

        Ok(Some(equipment))
    }
}

impl BinWrite for Option<SaveShipEquipment> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        match self {
            Some(equipment) => {
                destination.write_i32::<O>(equipment.item_index)?;
                destination.write_i32::<O>(equipment.amount)?;
                destination.write_u8(equipment.unsaleable.into())?
            }
            None => destination.write_i32::<O>(-1)?,
        }

        Ok(())
    }
}
