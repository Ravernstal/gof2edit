use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::data::position::Position;
use crate::data::Engine;
use crate::index::Index;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Read, Write};

const PRIMARY_WEAPON_CODE: u16 = 0;
const SECONDARY_WEAPON_CODE: u16 = 1;
const TURRET_CODE: u16 = 2;
const ENGINE_CODE: u16 = 3;

#[derive(Debug, Deserialize, Serialize)]
pub struct ShipPosition {
    ship_index: u16,
    primary_weapons: Vec<Position>,
    secondary_weapons: Vec<Position>,
    turrets: Vec<Position>,
    engines: Vec<Engine>,
}

impl Index for ShipPosition {
    fn index(&self) -> u32 {
        self.ship_index.into()
    }

    fn set_index(&mut self, index: u32) {
        // TODO: Find way of removing panic
        self.ship_index = index
            .try_into()
            .expect("index was too large to convert to u16");
    }
}

impl BinRead for ShipPosition {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> std::io::Result<Self> {
        let ship_index = source.read_u16::<O>()?;
        let position_count = source.read_u16::<O>()?;

        let mut primary_weapons = vec![];
        let mut secondary_weapons = vec![];
        let mut turrets = vec![];
        let mut engines = vec![];

        for _ in 0..position_count {
            match source.read_u16::<O>()? {
                0 => primary_weapons.push(source.read_bin::<O>()?),
                1 => secondary_weapons.push(source.read_bin::<O>()?),
                2 => turrets.push(source.read_bin::<O>()?),
                3 => engines.push(source.read_bin::<O>()?),
                _ => return Err(Error::from(ErrorKind::InvalidData)),
            }
        }

        Ok(Self {
            ship_index,
            primary_weapons,
            secondary_weapons,
            turrets,
            engines,
        })
    }
}

impl BinWrite for ShipPosition {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> std::io::Result<()> {
        destination.write_u16::<O>(self.ship_index)?;

        let position_count = self.primary_weapons.len()
            + self.secondary_weapons.len()
            + self.turrets.len()
            + self.engines.len();

        let position_count = position_count
            .try_into()
            .map_err(|_| ErrorKind::InvalidData)?;

        destination.write_u16::<O>(position_count)?;

        self.primary_weapons.iter().try_for_each(|primary_weapon| {
            destination.write_u16::<O>(PRIMARY_WEAPON_CODE)?;
            destination.write_bin::<O>(primary_weapon)
        })?;

        self.secondary_weapons
            .iter()
            .try_for_each(|secondary_weapon| {
                destination.write_u16::<O>(SECONDARY_WEAPON_CODE)?;
                destination.write_bin::<O>(secondary_weapon)
            })?;

        self.turrets.iter().try_for_each(|turret| {
            destination.write_u16::<O>(TURRET_CODE)?;
            destination.write_bin::<O>(turret)
        })?;

        self.engines.iter().try_for_each(|engines| {
            destination.write_u16::<O>(ENGINE_CODE)?;
            destination.write_bin::<O>(engines)
        })
    }
}
