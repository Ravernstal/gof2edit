use crate::data::station::Station;
use crate::data::system::System;
use crate::patch_addresses::binary_version::BinaryVersion;
use crate::patch_addresses::{station, system};
use crate::targets::patch::PatchTarget;
use byteorder::WriteBytesExt;
use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Seek, SeekFrom};
use std::ops::Not;
use std::path::Path;
use std::{fs, io};

pub fn patch(
    target: PatchTarget,
    json_filepath: impl AsRef<Path>,
    binary_filepath: impl AsRef<Path>,
    binary: BinaryVersion,
    silent: bool,
) -> io::Result<()> {
    let json_filepath = json_filepath.as_ref();
    let binary_filepath = binary_filepath.as_ref();

    if silent.not() {
        println!("Reading {target} from {} ...", json_filepath.display());
    }

    let count = match target {
        PatchTarget::Stations => {
            file_with_counts::<Station>(json_filepath, binary_filepath, station::addresses(binary))
        }
        PatchTarget::Systems => {
            file_with_counts::<System>(json_filepath, binary_filepath, system::addresses(binary))
        }
    }?;

    if silent.not() {
        println!(
            "Patched {count} {target} into {}",
            binary_filepath.display()
        );
    }

    Ok(())
}

pub fn file_with_counts<T: for<'de> Deserialize<'de>>(
    json_filepath: impl AsRef<Path>,
    binary_filepath: impl AsRef<Path>,
    address_modifiers: &[(u64, i8)],
) -> io::Result<usize> {
    let json = fs::read_to_string(json_filepath)?;
    let data = serde_json::from_str::<Vec<T>>(&json)?;
    let count = data.len().try_into().map_err(|_| ErrorKind::InvalidData)?;

    let mut file = OpenOptions::new().write(true).open(binary_filepath)?;

    address_list_modifiers(&mut file, address_modifiers, count)?;

    Ok(data.len())
}

pub fn address_list_modifiers(
    file: &mut File,
    address_modifiers: &[(u64, i8)],
    byte: u8,
) -> io::Result<()> {
    address_modifiers
        .iter()
        .try_for_each(|&(address, modifier)| {
            let byte = i16::from(byte) + i16::from(modifier);
            let byte = byte.try_into().map_err(|_| ErrorKind::InvalidData)?;

            set_byte(file, address, byte)
        })
}

pub fn set_byte(file: &mut File, address: u64, byte: u8) -> io::Result<()> {
    file.seek(SeekFrom::Start(address))?;
    file.write_u8(byte)
}
