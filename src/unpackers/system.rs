use crate::bin_io::read::BinReader;
use crate::data::system::System;
use byteorder::BigEndian;
use std::fs::File;
use std::path::Path;
use std::{fs, io};

pub fn unpack(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
) -> io::Result<()> {
    let input_filepath = input_filepath.as_ref();
    let output_filepath = output_filepath.as_ref();

    println!("Unpacking systems from {} ...", input_filepath.display());

    let mut file = File::open(input_filepath)?;
    let mut systems: Vec<System> = vec![];
    let mut count = 0;

    while let Ok(system) = file.read_bin::<BigEndian>() {
        let mut system: System = system;
        system.index = count;
        systems.push(system);
        count += 1;
    }

    serde_json::to_string_pretty(&systems).map(|string| fs::write(output_filepath, string))??;

    println!(
        "Unpacked {} systems into {}",
        count,
        output_filepath.display()
    );

    Ok(())
}
