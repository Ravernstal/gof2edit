use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::BinWrite;
use crate::wide_string::WideString;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct SaveWanted {
    pub active: bool,
    pub terminated: bool,
    pub current_location: i32,
    pub travels_to: i32,
    pub last_seen: i32,
    pub name: String,
    pub index: i32,
    pub board: i32,
    pub race: i32,
    pub male: bool,
    pub ship: i32,
    pub weapon: i32,
    pub hitpoints: i32,
    pub loot: i32,
    pub loot_amount: i32,
    pub reward: i32,
    pub required_bounties: i32,
    pub required_mission: i32,
    pub wingmen: i32,
    pub image_parts: Vec<i32>,
}

impl BinRead for Vec<SaveWanted> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        let count = source.read_i32::<O>()?;

        (0..count).map(|_| source.read_bin::<O>()).collect()
    }
}

impl BinWrite for Vec<SaveWanted> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        todo!()
    }
}

impl BinRead for SaveWanted {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        Ok(Self {
            active: source.read_u8()? != 0,
            terminated: source.read_u8()? != 0,
            current_location: source.read_i32::<O>()?,
            travels_to: source.read_i32::<O>()?,
            last_seen: source.read_i32::<O>()?,
            name: WideString::read_bin::<O>(source)?.get(),
            index: source.read_i32::<O>()?,
            board: source.read_i32::<O>()?,
            race: source.read_i32::<O>()?,
            male: source.read_u8()? != 0,
            ship: source.read_i32::<O>()?,
            weapon: source.read_i32::<O>()?,
            hitpoints: source.read_i32::<O>()?,
            loot: source.read_i32::<O>()?,
            loot_amount: source.read_i32::<O>()?,
            reward: source.read_i32::<O>()?,
            required_bounties: source.read_i32::<O>()?,
            required_mission: source.read_i32::<O>()?,
            wingmen: source.read_i32::<O>()?,
            image_parts: vec![
                source.read_i32::<O>()?,
                source.read_i32::<O>()?,
                source.read_i32::<O>()?,
                source.read_i32::<O>()?,
                source.read_i32::<O>()?,
            ],
        })
    }
}

impl BinWrite for SaveWanted {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        todo!()
    }
}
