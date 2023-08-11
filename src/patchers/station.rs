use crate::data::station::Station;
use crate::patch;
use crate::patch_addresses::binary_version::BinaryVersion;
use crate::patch_addresses::station;
use std::io;
use std::ops::Not;
use std::path::Path;

pub fn patch(
    json_filepath: impl AsRef<Path>,
    binary_filepath: impl AsRef<Path>,
    binary: BinaryVersion,
    silent: bool,
) -> io::Result<()> {
    let json_filepath = json_filepath.as_ref();
    let binary_filepath = binary_filepath.as_ref();

    if silent.not() {
        println!("Reading stations from {} ...", json_filepath.display());
    }

    let address_modifiers = station::addresses(binary);

    let count =
        patch::file_with_counts::<Station>(json_filepath, binary_filepath, address_modifiers)?;

    if silent.not() {
        println!(
            "Patched {count} stations into {}",
            binary_filepath.display()
        );
    }

    Ok(())
}
