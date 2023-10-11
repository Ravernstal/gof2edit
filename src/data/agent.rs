use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::data::Faction;
use crate::index::Index;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Agent {
    pub index: u32,
    pub name: String,
    pub station_id: u32,
    pub system_id: u32,
    pub race: Faction,
    pub is_male: bool,
    pub sell_system_id: i32,
    pub sell_blueprint_id: i32,
    pub sell_mod_id: i32,
    pub sell_price: i32,
    pub image_parts: Vec<u8>,
}

impl Index for Agent {
    fn index(&self) -> u32 {
        self.index
    }

    fn set_index(&mut self, index: u32) {
        self.index = index;
    }
}

impl BinRead for Agent {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> std::io::Result<Self> {
        Ok(Self {
            name: source.read_bin::<O>()?,
            index: source.read_u32::<O>()?,
            station_id: source.read_u32::<O>()?,
            system_id: source.read_u32::<O>()?,
            race: source.read_bin::<O>()?,
            is_male: source.read_u32::<O>()? != 0,
            sell_system_id: source.read_i32::<O>()?,
            sell_blueprint_id: source.read_i32::<O>()?,
            sell_mod_id: source.read_i32::<O>()?,
            sell_price: source.read_i32::<O>()?,
            image_parts: source.read_bin::<O>()?,
        })
    }
}

impl BinWrite for Agent {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> std::io::Result<()> {
        destination.write_bin::<O>(&self.name)?;
        destination.write_u32::<O>(self.index)?;
        destination.write_u32::<O>(self.station_id)?;
        destination.write_u32::<O>(self.system_id)?;
        destination.write_bin::<O>(&self.race)?;
        destination.write_u32::<O>(self.is_male.into())?;
        destination.write_i32::<O>(self.sell_system_id)?;
        destination.write_i32::<O>(self.sell_blueprint_id)?;
        destination.write_i32::<O>(self.sell_mod_id)?;
        destination.write_i32::<O>(self.sell_price)?;
        destination.write_bin::<O>(&self.image_parts)
    }
}
