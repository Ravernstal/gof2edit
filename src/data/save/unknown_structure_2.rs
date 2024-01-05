use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::BinWrite;
use crate::data::save::inventory_item::SaveInventoryItem;
use crate::data::save::ship::SaveShip;
use crate::data::save::ship_equipment::SaveShipEquipment;
use crate::result::Result;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct UnknownStructure2 {
    pub ship: SaveShip,
    pub ship_equipment: Vec<Option<SaveShipEquipment>>,
    pub ship_cargo: Vec<SaveInventoryItem>,
}

impl BinRead for Option<UnknownStructure2> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let count = source.read_i32::<O>()?;

        if count == 0 {
            return Ok(None);
        }

        Ok(Some(UnknownStructure2 {
            ship: source.read_bin::<O>()?,
            ship_equipment: source.read_bin::<O>()?,
            ship_cargo: source.read_bin::<O>()?,
        }))
    }
}

impl BinWrite for Option<UnknownStructure2> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        todo!()
    }
}
