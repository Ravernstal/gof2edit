use crate::data::system::System;
use crate::utilities;
use std::fs::OpenOptions;
use std::path::Path;
use std::{fs, io};

const SYSTEM_COUNT_ADDRESSES: &[u64] = &[0x000C69A8, 0x000C69BA, 0x00135C66, 0x00135E80];

pub fn patch(json_filepath: impl AsRef<Path>, so_filepath: impl AsRef<Path>) -> io::Result<()> {
    let json_filepath = json_filepath.as_ref();
    let so_filepath = so_filepath.as_ref();

    println!("Reading systems from {} ...", json_filepath.display());

    let json_string = fs::read_to_string(json_filepath)?;
    let system_count = serde_json::from_str::<Vec<System>>(&json_string)?.len() as u8;

    let mut file = OpenOptions::new().write(true).open(so_filepath)?;

    SYSTEM_COUNT_ADDRESSES
        .iter()
        .try_for_each(|address| utilities::set_byte(&mut file, *address, system_count))?;

    println!(
        "Patched {} systems into {}",
        system_count,
        so_filepath.display()
    );

    Ok(())
}
