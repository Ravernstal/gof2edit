use crate::bin_io::read::BinRead;
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::wide_string::WideString;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct SavePreview {
    pub credits: u32,
    pub ship_index: u32,
    pub campaign_mission: u32,
    pub level: u32,
    pub difficulty: f32,
    pub system_name: String,
    pub station_name: String,
    pub play_time_ms: u64,
}

impl BinRead for SavePreview {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> std::io::Result<Self> {
        Ok(Self {
            play_time_ms: source.read_u64::<O>()?,
            credits: source.read_u32::<O>()?,
            station_name: WideString::read_bin::<O>(source)?.get(),
            system_name: WideString::read_bin::<O>(source)?.get(),
            campaign_mission: source.read_u32::<O>()?,
            level: source.read_u32::<O>()?,
            difficulty: source.read_f32::<O>()?,
            ship_index: source.read_u32::<O>()?,
        })
    }
}

impl BinWrite for SavePreview {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> std::io::Result<()> {
        destination.write_u64::<O>(self.play_time_ms)?;
        destination.write_u32::<O>(self.credits)?;
        destination.write_bin::<O>(&WideString::new(self.station_name.clone()))?;
        destination.write_bin::<O>(&WideString::new(self.system_name.clone()))?;
        destination.write_u32::<O>(self.campaign_mission)?;
        destination.write_u32::<O>(self.level)?;
        destination.write_f32::<O>(self.difficulty)?;
        destination.write_u32::<O>(self.ship_index)
    }
}
