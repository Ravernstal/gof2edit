use crate::bin_io::read::BinRead;
use crate::bin_io::write::BinWrite;
use crate::index::Index;
use crate::result::Result;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ship {
    pub index: u32,
    pub armour: u32,
    pub cargo_capacity: u32,
    pub price: u32,
    pub primary_weapon_count: u32,
    pub secondary_weapon_count: u32,
    pub turret_count: u32,
    pub equipment_slot_count: u32,
    pub handling: u32,
}

impl Index for Ship {
    fn index(&self) -> u32 {
        self.index
    }

    fn set_index(&mut self, index: u32) {
        self.index = index;
    }
}

impl BinRead for Ship {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        Ok(Self {
            index: source.read_u32::<O>()?,
            armour: source.read_u32::<O>()?,
            cargo_capacity: source.read_u32::<O>()?,
            price: source.read_u32::<O>()?,
            primary_weapon_count: source.read_u32::<O>()?,
            secondary_weapon_count: source.read_u32::<O>()?,
            turret_count: source.read_u32::<O>()?,
            equipment_slot_count: source.read_u32::<O>()?,
            handling: source.read_u32::<O>()?,
        })
    }
}

impl BinWrite for Ship {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        destination.write_u32::<O>(self.index)?;
        destination.write_u32::<O>(self.armour)?;
        destination.write_u32::<O>(self.cargo_capacity)?;
        destination.write_u32::<O>(self.price)?;
        destination.write_u32::<O>(self.primary_weapon_count)?;
        destination.write_u32::<O>(self.secondary_weapon_count)?;
        destination.write_u32::<O>(self.turret_count)?;
        destination.write_u32::<O>(self.equipment_slot_count)?;
        destination.write_u32::<O>(self.handling)?;

        Ok(())
    }
}
