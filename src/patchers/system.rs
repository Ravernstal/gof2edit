use crate::data::system::System;
use crate::patch;
use crate::patch_addresses::system;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::path::Path;
use std::{fs, io};

pub fn patch(json_filepath: impl AsRef<Path>, so_filepath: impl AsRef<Path>) -> io::Result<()> {
    let json_filepath = json_filepath.as_ref();
    let so_filepath = so_filepath.as_ref();

    println!("Reading systems from {} ...", json_filepath.display());

    let json_string = fs::read_to_string(json_filepath)?;
    let system_count = serde_json::from_str::<Vec<System>>(&json_string)?.len();
    let system_count = system_count
        .try_into()
        .map_err(|_| ErrorKind::InvalidData)?;

    let mut file = OpenOptions::new().write(true).open(so_filepath)?;

    patch::address_list_modifiers(&mut file, system::ADDRESSES, system_count)?;

    println!(
        "Patched {} systems into {}",
        system_count,
        so_filepath.display()
    );

    Ok(())
}
