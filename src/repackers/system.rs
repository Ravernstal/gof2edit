use crate::bin_io::write::BinWriter;
use crate::data::system::System;
use byteorder::BigEndian;
use std::fs::File;
use std::ops::Not;
use std::path::Path;
use std::{fs, io};

pub fn repack(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
    silent: bool,
) -> io::Result<()> {
    let input_filepath = input_filepath.as_ref();
    let output_filepath = output_filepath.as_ref();

    if silent.not() {
        println!("Repacking systems from {} ...", input_filepath.display());
    }

    let json_string = fs::read_to_string(input_filepath)?;
    let mut systems = serde_json::from_str::<Vec<System>>(&json_string)?;

    systems.sort_unstable_by(|s1, s2| s1.index.cmp(&s2.index));

    let mut file = File::create(output_filepath)?;

    systems
        .iter()
        .try_for_each(|system| file.write_bin::<BigEndian>(system))?;

    if silent.not() {
        println!(
            "Repacked {} systems into {}",
            systems.len(),
            output_filepath.display()
        );
    }

    Ok(())
}
