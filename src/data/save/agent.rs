use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::data::save::image_parts::ImageParts;
use crate::data::save::mission::SaveMission;
use crate::data::Faction;
use crate::result::Result;
use crate::wide_string::WideString;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SaveAgent {
    pub costs: i32,
    pub sell_system_index: i32,
    pub sell_blueprint_index: i32,
    pub event: i32,
    pub index: i32,
    pub offer: i32,
    pub race: Faction,
    pub sell_item_index: i32,
    pub sell_item_price: i32,
    pub sell_item_quantity: i32,
    pub station_index: i32,
    pub system_index: i32,
    pub wingman_friends_count: i32,
    pub male: bool,
    pub has_reward: bool,
    pub has_accepted_offer: bool,
    pub asked_about_difficulty: bool,
    pub asked_about_location: bool,
    pub image: Option<ImageParts>,
    pub sell_mod_index: Option<i32>,
    pub mission_string: String,
    pub name: String,
    pub station_name: String,
    pub system_name: String,
    pub wingman_friend_1: String,
    pub wingman_friend_2: String,
    pub mission: Option<Box<SaveMission>>,
}

impl BinRead for Vec<SaveAgent> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let count = source.read_i32::<O>()?;

        (0..count).map(|_| source.read_bin::<O>()).collect()
    }
}

impl BinWrite for Vec<SaveAgent> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        let length = self.len().try_into()?;
        destination.write_u32::<O>(length)?;

        self.iter()
            .try_for_each(|agent| destination.write_bin::<O>(agent))
    }
}

impl BinRead for Option<SaveAgent> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let count = source.read_i32::<O>()?;

        if count == -1 {
            return Ok(None);
        }

        let agent = read_agent::<O>(source)?;

        Ok(Some(agent))
    }
}

impl BinWrite for Option<SaveAgent> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        match self {
            Some(agent) => {
                destination.write_i32::<O>(1)?;
                write_agent::<O>(destination, agent)?
            }
            None => destination.write_i32::<O>(-1)?,
        }

        Ok(())
    }
}

impl BinRead for SaveAgent {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        read_agent::<O>(source)
    }
}

impl BinWrite for SaveAgent {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        write_agent::<O>(destination, self)
    }
}

fn read_agent<O: ByteOrder>(source: &mut impl Read) -> Result<SaveAgent> {
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
        race: source.read_bin::<O>()?,
        sell_item_index: source.read_i32::<O>()?,
        sell_item_price: source.read_i32::<O>()?,
        sell_item_quantity: source.read_i32::<O>()?,
        station_index: source.read_i32::<O>()?,
        system_index: source.read_i32::<O>()?,
        wingman_friends_count: source.read_i32::<O>()?,
        male: source.read_u8()? != 0,
        has_reward: source.read_u8()? != 0,
        has_accepted_offer: source.read_u8()? != 0,
        asked_about_difficulty: source.read_u8()? != 0,
        asked_about_location: source.read_u8()? != 0,
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
        wingman_friend_1: WideString::read_bin::<O>(source)?.get(),
        wingman_friend_2: WideString::read_bin::<O>(source)?.get(),
        mission: {
            if source.read_i32::<O>()? == -1 {
                None
            } else {
                let mission = Option::<SaveMission>::read_bin::<O>(source)?;

                mission.map(Box::new)
            }
        },
    })
}

fn write_agent<O: ByteOrder>(destination: &mut impl Write, agent: &SaveAgent) -> Result<()> {
    destination.write_i32::<O>(agent.costs)?;
    destination.write_i32::<O>(agent.sell_system_index)?;
    destination.write_i32::<O>(agent.sell_blueprint_index)?;
    destination.write_i32::<O>(agent.event)?;
    destination.write_i32::<O>(agent.index)?;
    destination.write_i32::<O>(agent.offer)?;
    destination.write_bin::<O>(&agent.race)?;
    destination.write_i32::<O>(agent.sell_item_index)?;
    destination.write_i32::<O>(agent.sell_item_price)?;
    destination.write_i32::<O>(agent.sell_item_quantity)?;
    destination.write_i32::<O>(agent.station_index)?;
    destination.write_i32::<O>(agent.system_index)?;
    destination.write_i32::<O>(agent.wingman_friends_count)?;
    destination.write_u8(agent.male.into())?;
    destination.write_u8(agent.has_reward.into())?;
    destination.write_u8(agent.has_accepted_offer.into())?;
    destination.write_u8(agent.asked_about_difficulty.into())?;
    destination.write_u8(agent.asked_about_location.into())?;
    destination.write_bin::<O>(&agent.image)?;

    if agent.index > 0x12 {
        // TODO: Fix potential crash
        destination.write_i32::<O>(agent.sell_mod_index.unwrap())?;
    }

    destination.write_bin::<O>(&WideString::new(agent.mission_string.clone()))?;
    destination.write_bin::<O>(&WideString::new(agent.name.clone()))?;
    destination.write_bin::<O>(&WideString::new(agent.station_name.clone()))?;
    destination.write_bin::<O>(&WideString::new(agent.system_name.clone()))?;
    destination.write_bin::<O>(&WideString::new(agent.wingman_friend_1.clone()))?;
    destination.write_bin::<O>(&WideString::new(agent.wingman_friend_2.clone()))?;

    match &agent.mission {
        Some(mission) => {
            let mission = *mission.clone();
            destination.write_i32::<O>(1)?;
            destination.write_bin::<O>(&Some(mission))?
        }
        None => Option::<SaveMission>::write_bin::<O>(&None, destination)?,
    }

    Ok(())
}
