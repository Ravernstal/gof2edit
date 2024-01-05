use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::BinWrite;
use crate::data::save::agent::SaveAgent;
use crate::data::save::image_parts::ImageParts;
use crate::result::Result;
use crate::wide_string::WideString;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct SaveMission {
    pub mission_type: i32,
    pub client_name: String,
    pub target_name: String,
    pub target_station_name: String,
    pub target_system_name: String,
    pub campaign_mission: bool,
    pub client_image: Option<ImageParts>,
    pub client_race: i32,
    pub costs: i32,
    pub bonus: i32,
    pub reward: i32,
    pub target_station_index: i32,
    pub difficulty: i32,
    pub production_good_index: i32,
    pub production_good_amount: i32,
    pub status_value: i32,
    pub visible: bool,
    pub agent: Option<SaveAgent>,
}

impl BinRead for Option<SaveMission> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let mission_type = source.read_i32::<O>()?;

        if mission_type == -1 {
            return Ok(None);
        }

        Ok(Some(SaveMission {
            mission_type,
            client_name: WideString::read_bin::<O>(source)?.get(),
            target_name: WideString::read_bin::<O>(source)?.get(),
            target_station_name: WideString::read_bin::<O>(source)?.get(),
            target_system_name: WideString::read_bin::<O>(source)?.get(),
            campaign_mission: source.read_u8()? != 0,
            client_image: source.read_bin::<O>()?,
            client_race: source.read_i32::<O>()?,
            costs: source.read_i32::<O>()?,
            bonus: source.read_i32::<O>()?,
            reward: source.read_i32::<O>()?,
            target_station_index: source.read_i32::<O>()?,
            difficulty: source.read_i32::<O>()?,
            production_good_index: source.read_i32::<O>()?,
            production_good_amount: source.read_i32::<O>()?,
            status_value: source.read_i32::<O>()?,
            visible: source.read_u8()? != 0,
            agent: source.read_bin::<O>()?,
        }))
    }
}

impl BinWrite for Option<SaveMission> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        todo!()
    }
}
