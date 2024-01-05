use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::BinWrite;
use crate::data::save::agent_mission::SaveAgentMission;
use crate::data::save::image_parts::ImageParts;
use crate::wide_string::WideString;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct SaveAgent {
    pub costs: i32,
    pub sell_system_index: i32,
    pub sell_blueprint_index: i32,
    pub event: i32,
    pub index: i32,
    pub offer: i32,
    pub race: i32,
    pub sell_item_index: i32,
    pub sell_item_price: i32,
    pub sell_item_quantity: i32,
    pub station_index: i32,
    pub system_index: i32,
    pub wingman_friends_count: i32,
    pub male: bool,
    pub has_reward: bool,
    pub has_accepted_offer: bool,
    pub unknown_bool_1: bool,
    pub unknown_bool_2: bool,
    pub image: Option<ImageParts>,
    pub sell_mod_index: Option<i32>,
    pub mission_string: String,
    pub name: String,
    pub station_name: String,
    pub system_name: String,
    pub unknown_string_1: String,
    pub unknown_string_2: String,
    pub mission: Option<SaveAgentMission>,
}

impl BinRead for Vec<SaveAgent> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        let count = source.read_i32::<O>()?;

        (0..count).map(|_| source.read_bin::<O>()).collect()
    }
}

impl BinWrite for Vec<SaveAgent> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        todo!()
    }
}

impl BinRead for Option<SaveAgent> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        let count = source.read_i32::<O>()?;

        if count == -1 {
            return Ok(None);
        }

        let agent = read_agent::<O>(source)?;

        Ok(Some(agent))
    }
}

impl BinWrite for Option<SaveAgent> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        todo!()
    }
}

impl BinRead for SaveAgent {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        read_agent::<O>(source)
    }
}

impl BinWrite for SaveAgent {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        todo!()
    }
}

fn read_agent<O: ByteOrder>(source: &mut impl Read) -> io::Result<SaveAgent> {
    let costs = source.read_i32::<O>()?;
    let sell_system_index = source.read_i32::<O>()?;
    let sell_blueprint_index = source.read_i32::<O>()?;
    let event = source.read_i32::<O>()?;
    let index = source.read_i32::<O>()?;

    Ok(SaveAgent {
        costs,
        sell_system_index,
        sell_blueprint_index,
        event,
        index,
        offer: source.read_i32::<O>()?,
        race: source.read_i32::<O>()?,
        sell_item_index: source.read_i32::<O>()?,
        sell_item_price: source.read_i32::<O>()?,
        sell_item_quantity: source.read_i32::<O>()?,
        station_index: source.read_i32::<O>()?,
        system_index: source.read_i32::<O>()?,
        wingman_friends_count: source.read_i32::<O>()?,
        male: source.read_u8()? != 0,
        has_reward: source.read_u8()? != 0,
        has_accepted_offer: source.read_u8()? != 0,
        unknown_bool_1: source.read_u8()? != 0,
        unknown_bool_2: source.read_u8()? != 0,
        image: source.read_bin::<O>()?,
        sell_mod_index: if index > 0x12 {
            Some(source.read_i32::<O>()?)
        } else {
            None
        },
        mission_string: WideString::read_bin::<O>(source)?.get(),
        name: WideString::read_bin::<O>(source)?.get(),
        station_name: WideString::read_bin::<O>(source)?.get(),
        system_name: WideString::read_bin::<O>(source)?.get(),
        unknown_string_1: WideString::read_bin::<O>(source)?.get(),
        unknown_string_2: WideString::read_bin::<O>(source)?.get(),
        mission: source.read_bin::<O>()?,
    })
}
