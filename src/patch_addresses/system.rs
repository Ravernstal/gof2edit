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
        (0xA5DB2, &write_value::default),  // Status::Status
        (0xA5DDA, &write_value::default),  // Status::Status
        (0xC69A8, &write_value::default),  // StarMap::StarMap
        (0xC69BA, &write_value::default),  // StarMap::StarMap
        (0x135C66, &write_value::default), // FileRead::loadSystemsBinary
        (0x135E80, &write_value::default), // FileRead::loadSystemsBinary
    ]
}

fn ios_addresses() -> &'static [(u64, &'static WriteValueFn)] {
    &[
        (0x19ADC, &write_transmute1),  // FileRead::loadSystemsBinary
        (0x19DBD, &write_transmute2),  // FileRead::loadSystemsBinary
        (0x12EF1C, &write_transmute1), // StarMap::StarMap
        (0x12EF2C, &write_transmute1), // StarMap::StarMap
        (0x137BE8, &write_transmute1), // Status::Status
        (0x137C18, &write_transmute1), // Status::Status
    ]
}

pub fn write_transmute1(destination: &mut dyn Write, value: u8) -> gof2edit::Result<()> {
    let value = u16::from(value) * 32;
    destination
        .write_u16::<LittleEndian>(value)
        .map_err(|error| error.into())
}

pub fn write_transmute2(destination: &mut dyn Write, value: u8) -> gof2edit::Result<()> {
    let value = (u16::from(value) * 4) + 2;
    destination
        .write_u16::<LittleEndian>(value)
        .map_err(|error| error.into())
}
