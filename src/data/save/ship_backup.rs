use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::data::save::inventory_item::SaveInventoryItem;
use crate::data::save::ship::SaveShip;
use crate::data::save::ship_equipment::SaveShipEquipment;
use crate::result::Result;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct ShipBackup {
    pub ship: SaveShip,
    pub ship_equipment: Vec<Option<SaveShipEquipment>>,
    pub ship_cargo: Vec<SaveInventoryItem>,
}

impl BinRead for Option<ShipBackup> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let count = source.read_i32::<O>()?;

        if count == 0 {
            return Ok(None);
        }

        Ok(Some(ShipBackup {
            ship: source.read_bin::<O>()?,
            ship_equipment: source.read_bin::<O>()?,
            ship_cargo: source.read_bin::<O>()?,
        }))
    }
}

impl BinWrite for Option<ShipBackup> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        match self {
            Some(unknown) => {
                destination.write_i32::<O>(1)?;
                destination.write_bin::<O>(&unknown.ship)?;
                destination.write_bin::<O>(&unknown.ship_equipment)?;
                destination.write_bin::<O>(&unknown.ship_cargo)?
            }
            None => destination.write_i32::<O>(0)?,
        }

        Ok(())
    }
}
