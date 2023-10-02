use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::data::Faction;
use crate::index::Index;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Wanted {
    index: u32,
    name: String,
    board: Faction,
    race: Faction,
    unknown: bool,
    ship_id: u32,
    weapon: u32,
    hitpoints: u32,
    loot_item_id: u32,
    loot_amount: u32,
    reward: u32,
    required_bounties: u32,
    required_mission: u32,
    num_wingmen: u32,
    image_parts: Vec<u8>,
}

impl Index for Wanted {
    fn index(&self) -> u32 {
        self.index
    }

    fn set_index(&mut self, index: u32) {
        self.index = index;
    }
}

impl BinRead for Wanted {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> std::io::Result<Self> {
        let name = source.read_bin::<O>()?;

        Ok(Self {
            index: source.read_u32::<O>()?,
            name,
            board: source.read_bin::<O>()?,
            race: source.read_bin::<O>()?,
            unknown: source.read_u32::<O>()? != 0,
            ship_id: source.read_u32::<O>()?,
            weapon: source.read_u32::<O>()?,
            hitpoints: source.read_u32::<O>()?,
            loot_item_id: source.read_u32::<O>()?,
            loot_amount: source.read_u32::<O>()?,
            reward: source.read_u32::<O>()?,
            required_bounties: source.read_u32::<O>()?,
            required_mission: source.read_u32::<O>()?,
            num_wingmen: source.read_u32::<O>()?,
            image_parts: source.read_bin::<O>()?,
        })
    }
}

impl BinWrite for Wanted {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> std::io::Result<()> {
        destination.write_bin::<O>(&self.name)?;
        destination.write_u32::<O>(self.index)?;
        destination.write_bin::<O>(&self.board)?;
        destination.write_bin::<O>(&self.race)?;
        destination.write_u32::<O>(self.unknown.into())?;
        destination.write_u32::<O>(self.ship_id)?;
        destination.write_u32::<O>(self.weapon)?;
        destination.write_u32::<O>(self.hitpoints)?;
        destination.write_u32::<O>(self.loot_item_id)?;
        destination.write_u32::<O>(self.loot_amount)?;
        destination.write_u32::<O>(self.reward)?;
        destination.write_u32::<O>(self.required_bounties)?;
        destination.write_u32::<O>(self.required_mission)?;
        destination.write_u32::<O>(self.num_wingmen)?;
        destination.write_bin::<O>(&self.image_parts)
    }
}
