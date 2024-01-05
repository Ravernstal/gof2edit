use crate::bin_io::read::BinRead;
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::error::Error;
use crate::result::Result;
use crate::wide_string::WideString;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Read, Write};

const BLUEPRINT_INGREDIENT_COUNTS: &[i32] = &[
    6, 6, 5, 5, 3, 4, 6, 5, 5, 4, 8, 5, 6, 6, 8, 8, 9, 5, 4, 4, 5, 9, 5, 4, 2,
];

#[derive(Debug, Deserialize, Serialize)]
pub struct SaveBlueprint {
    pub ingredient_counts: Vec<i32>,
    pub unknown_int_1: i32,
    pub unlocked: bool,
    pub unknown_int_2: i32,
    pub station_index: i32,
    pub station_name: String,
}

impl BinRead for Vec<SaveBlueprint> {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let count = source.read_i32::<O>()?;

        (0..count).map(|i| read_blueprint::<O>(source, i)).collect()
    }
}

impl BinWrite for Vec<SaveBlueprint> {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        let length = self.len().try_into()?;
        destination.write_u32::<O>(length)?;

        self.iter()
            .try_for_each(|blueprint| write_blueprint::<O>(destination, blueprint))
    }
}

fn read_blueprint<O: ByteOrder>(source: &mut impl Read, index: i32) -> Result<SaveBlueprint> {
    let index = usize::try_from(index)?;

    let ingredient_count = *BLUEPRINT_INGREDIENT_COUNTS
        .get(index)
        .ok_or(Error::BlueprintIngredientListIndex(index))?;
    let ingredient_counts = (0..ingredient_count)
        .map(|_| source.read_i32::<O>())
        .collect::<io::Result<Vec<_>>>()?;

    Ok(SaveBlueprint {
        ingredient_counts,
        unknown_int_1: source.read_i32::<O>()?,
        unlocked: source.read_u8()? != 0,
        unknown_int_2: source.read_i32::<O>()?,
        station_index: source.read_i32::<O>()?,
        station_name: WideString::read_bin::<O>(source)?.get(),
    })
}

fn write_blueprint<O: ByteOrder>(
    destination: &mut impl Write,
    blueprint: &SaveBlueprint,
) -> Result<()> {
    blueprint
        .ingredient_counts
        .iter()
        .try_for_each(|count| destination.write_i32::<O>(*count))?;
    destination.write_i32::<O>(blueprint.unknown_int_1)?;
    destination.write_u8(blueprint.unlocked.into())?;
    destination.write_i32::<O>(blueprint.unknown_int_2)?;
    destination.write_i32::<O>(blueprint.station_index)?;
    destination.write_bin::<O>(&WideString::new(blueprint.station_name.clone()))
}
