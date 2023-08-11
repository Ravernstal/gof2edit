use crate::data::station::Station;
use crate::patch;
use crate::patch_addresses::binary_version::BinaryVersion;
use crate::patch_addresses::station;
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
        println!("Reading stations from {} ...", json_filepath.display());
    }

    let json_string = fs::read_to_string(json_filepath)?;
    let station_count = serde_json::from_str::<Vec<Station>>(&json_string)?.len();
    let station_count = station_count
        .try_into()
        .map_err(|_| ErrorKind::InvalidData)?;

    let mut file = OpenOptions::new().write(true).open(so_filepath)?;
    let addresses = station::addresses(binary);

    patch::address_list_modifiers(&mut file, addresses, station_count)?;

    if silent.not() {
        println!(
            "Patched {} stations into {}",
            station_count,
            so_filepath.display()
        );
    }

    Ok(())
}
