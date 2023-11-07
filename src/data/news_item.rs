use crate::bin_io::read::BinRead;
use crate::bin_io::write::BinWrite;
use crate::index::Index;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewsItem {
    index: u32,
    unknown: bool,
    terran_systems: bool,
    vossk_systems: bool,
    nivelian_systems: bool,
    midorian_systems: bool,
    start_mission_id: u32,
    end_mission_id: u32,
}

impl Index for NewsItem {
    fn index(&self) -> u32 {
        self.index
    }

    fn set_index(&mut self, index: u32) {
        self.index = index;
    }
}

impl BinRead for NewsItem {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> std::io::Result<Self> {
        Ok(Self {
            index: 0,
            unknown: source.read_u32::<O>()? != 0,
            terran_systems: source.read_u32::<O>()? != 0,
            vossk_systems: source.read_u32::<O>()? != 0,
            nivelian_systems: source.read_u32::<O>()? != 0,
            midorian_systems: source.read_u32::<O>()? != 0,
            start_mission_id: source.read_u32::<O>()?,
            end_mission_id: source.read_u32::<O>()?,
        })
    }
}

impl BinWrite for NewsItem {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> std::io::Result<()> {
        destination.write_u32::<O>(self.unknown.into())?;
        destination.write_u32::<O>(self.terran_systems.into())?;
        destination.write_u32::<O>(self.vossk_systems.into())?;
        destination.write_u32::<O>(self.nivelian_systems.into())?;
        destination.write_u32::<O>(self.midorian_systems.into())?;
        destination.write_u32::<O>(self.start_mission_id)?;
        destination.write_u32::<O>(self.end_mission_id)
    }
}
