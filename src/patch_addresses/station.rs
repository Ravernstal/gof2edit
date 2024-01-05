use crate::patch_addresses::binary_version::BinaryVersion;
use crate::patch_addresses::{write_value, WriteValueFn};
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Write;

pub fn addresses(binary: BinaryVersion) -> &'static [(u64, &'static WriteValueFn)] {
    match binary {
        BinaryVersion::AndroidKiritoJpk => android_kirito_jpk_addresses(),
        BinaryVersion::Ios => ios_addresses(),
    }
}

fn android_kirito_jpk_addresses() -> &'static [(u64, &'static WriteValueFn)] {
    &[
        (0x91ED8, &write_value::default),  // Generator::generateSystemIndex
        (0xCF7C4, &write_value::default),  // RecordHandler::recordStoreWrite
        (0xCF7D6, &write_value::default),  // RecordHandler::recordStoreWrite
        (0xE915A, &write_value::default),  // Globals::getRandomStation
        (0x134DFE, &write_value::default), // FileRead::loadStationsBinary
        (0x135646, &write_value::default), // FileRead::loadStationsBinary
        (0x135B6A, &write_value::default), // FileRead::loadStationsBinary
        (0x170CFE, &write_value::default), // GameRecord::GameRecord
        (0x194CE0, &write_value::default), // Galaxy::Galaxy
        (0x194CE6, &write_value::default), // Galaxy::Galaxy
        (0x194D58, &write_value::default), // Galaxy::reset
        (0x194DE0, &write_value::minus_one), // Galaxy::setVisited
        (0x194DEA, &write_value::default), // Galaxy::setVisited
    ]
}

fn ios_addresses() -> &'static [(u64, &'static WriteValueFn)] {
    &[
        (0x18B81, &write_transmute1),  // FileRead::loadStationsBinary
        (0x19381, &write_transmute1),  // FileRead::loadStationsBinary
        (0x1C7D8, &write_transmute2),  // Galaxy::Galaxy
        (0x1DC9C, &write_transmute2),  // GameRecord::GameRecord
        (0x6F8AC, &write_transmute3),  // Generator::generateSystemIndex
        (0x7308C, &write_transmute3),  // Globals::getRandomStation
        (0x11AC94, &write_transmute2), // RecordHandler::recordStoreWrite
        (0x11ACB1, &write_transmute1), // RecordHandler::recordStoreWrite
        (0x1CA29, &write_transmute4),  // Galaxy::setVisited
        (0x1CA38, &write_transmute5),  // Galaxy::setVisited
    ]
}

pub fn write_transmute1(destination: &mut dyn Write, value: u8) -> gof2edit::Result<()> {
    let value = (u16::from(value) * 4) + 2;
    destination
        .write_u16::<LittleEndian>(value)
        .map_err(|error| error.into())
}

pub fn write_transmute2(destination: &mut dyn Write, value: u8) -> gof2edit::Result<()> {
    let value = u16::from(value) * 32;
    destination
        .write_u16::<LittleEndian>(value)
        .map_err(|error| error.into())
}

pub fn write_transmute3(destination: &mut dyn Write, value: u8) -> gof2edit::Result<()> {
    let value = (u16::from(value) * 32) + 1;
    destination
        .write_u16::<LittleEndian>(value)
        .map_err(|error| error.into())
}

pub fn write_transmute4(destination: &mut dyn Write, value: u8) -> gof2edit::Result<()> {
    let value = u16::from(value - 1) * 4;
    destination
        .write_u16::<LittleEndian>(value)
        .map_err(|error| error.into())
}

pub fn write_transmute5(destination: &mut dyn Write, value: u8) -> gof2edit::Result<()> {
    let value = (u16::from(value - 1) * 32) + 8;
    destination
        .write_u16::<LittleEndian>(value)
        .map_err(|error| error.into())
}
