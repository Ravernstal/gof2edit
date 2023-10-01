use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::data::faction::Faction;
use crate::data::security_level::SecurityLevel;
use crate::index::Index;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct System {
    pub index: u32,
    pub name: String,
    pub security_level: SecurityLevel,
    pub faction: Faction,
    pub starts_unlocked: bool,
    pub map_coords: [u32; 3],
    pub jumpgate_station_id: Option<u32>,
    pub texture_index: u32,
    pub unknown_bytes: Vec<u32>,
    pub station_ids: Vec<u32>,
    pub linked_system_ids: Vec<u32>,
    pub footer_bytes: Vec<u32>,
}

impl Index for System {
    fn index(&self) -> u32 {
        self.index
    }

    fn set_index(&mut self, index: u32) {
        self.index = index;
    }
}

impl BinRead for System {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        let name = source.read_bin::<O>()?;
        let security_level = source.read_bin::<O>()?;
        let starts_unlocked = source.read_u32::<O>()? != 0;
        let faction = source.read_bin::<O>()?;
        let map_coords = [
            source.read_u32::<O>()?,
            source.read_u32::<O>()?,
            source.read_u32::<O>()?,
        ];
        let jumpgate_station_id = match source.read_u32::<O>()? {
            u32::MAX => None,
            jumpgate_station_id => Some(jumpgate_station_id),
        };
        let texture_index = source.read_u32::<O>()?;
        let unknown_bytes = source.read_bin::<O>()?;
        let station_ids = source.read_bin::<O>()?;
        let linked_system_ids = source.read_bin::<O>()?;
        let footer_bytes = source.read_bin::<O>()?;

        Ok(Self {
            index: 0,
            name,
            security_level,
            faction,
            starts_unlocked,
            map_coords,
            jumpgate_station_id,
            texture_index,
            unknown_bytes,
            station_ids,
            linked_system_ids,
            footer_bytes,
        })
    }
}

impl BinWrite for System {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        destination.write_bin::<O>(&self.name)?;
        destination.write_bin::<O>(&self.security_level)?;
        destination.write_u32::<O>(self.starts_unlocked.into())?;
        destination.write_bin::<O>(&self.faction)?;
        destination.write_u32::<O>(self.map_coords[0])?;
        destination.write_u32::<O>(self.map_coords[1])?;
        destination.write_u32::<O>(self.map_coords[2])?;
        destination.write_u32::<O>(self.jumpgate_station_id.unwrap_or(0xFFFFFFFF))?;
        destination.write_u32::<O>(self.texture_index)?;
        destination.write_bin::<O>(&self.unknown_bytes)?;
        destination.write_bin::<O>(&self.station_ids)?;
        destination.write_bin::<O>(&self.linked_system_ids)?;
        destination.write_bin::<O>(&self.footer_bytes)
    }
}
