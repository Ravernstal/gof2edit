use crate::data::station::Station;
use crate::utilities;
use std::fs::OpenOptions;
use std::path::Path;
use std::{fs, io};

const STATION_COUNT_ADDRESSES: &[u64] = &[0x00134DFE, 0x00135646, 0x00135B6A];

pub fn patch(json_filepath: impl AsRef<Path>, so_filepath: impl AsRef<Path>) -> io::Result<()> {
    let json_filepath = json_filepath.as_ref();
    let so_filepath = so_filepath.as_ref();

    println!("Reading stations from {} ...", json_filepath.display());

    let json_string = fs::read_to_string(json_filepath)?;
    let station_count = serde_json::from_str::<Vec<Station>>(&json_string)?.len() as u8;

    let mut file = OpenOptions::new().write(true).open(so_filepath)?;

    STATION_COUNT_ADDRESSES
        .iter()
        .try_for_each(|address| utilities::set_byte(&mut file, *address, station_count))?;

    println!(
        "Patched {} stations into {}",
        station_count,
        so_filepath.display()
    );

    Ok(())
}
