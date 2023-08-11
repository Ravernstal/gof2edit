use crate::data::system::System;
use crate::patch;
use crate::patch_addresses::binary_version::BinaryVersion;
use crate::patch_addresses::system;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::ops::Not;
use std::path::Path;
use std::{fs, io};

pub fn patch(
    json_filepath: impl AsRef<Path>,
    so_filepath: impl AsRef<Path>,
    binary: BinaryVersion,
    silent: bool,
) -> io::Result<()> {
    let json_filepath = json_filepath.as_ref();
    let so_filepath = so_filepath.as_ref();

    if silent.not() {
        println!("Reading systems from {} ...", json_filepath.display());
    }

    let json_string = fs::read_to_string(json_filepath)?;
    let system_count = serde_json::from_str::<Vec<System>>(&json_string)?.len();
    let system_count = system_count
        .try_into()
        .map_err(|_| ErrorKind::InvalidData)?;

    let mut file = OpenOptions::new().write(true).open(so_filepath)?;
    let addresses = system::addresses(binary);

    patch::address_list_modifiers(&mut file, addresses, system_count)?;

    if silent.not() {
        println!(
            "Patched {} systems into {}",
            system_count,
            so_filepath.display()
        );
    }

    Ok(())
}
