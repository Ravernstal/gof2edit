use crate::bin_io::read::BinReader;
use crate::data::ship::Ship;
use byteorder::BigEndian;
use std::fs::File;
use std::ops::Not;
use std::path::Path;
use std::{fs, io};

pub fn unpack(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
    silent: bool,
) -> io::Result<()> {
    let input_filepath = input_filepath.as_ref();
    let output_filepath = output_filepath.as_ref();

    if silent.not() {
        println!("Unpacking ships from {} ...", input_filepath.display());
    }

    let mut file = File::open(input_filepath)?;
    let mut ships: Vec<Ship> = vec![];
    let mut count = 0;

    while let Ok(ship) = file.read_bin::<BigEndian>() {
        ships.push(ship);
        count += 1
    }

    serde_json::to_string_pretty(&ships).map(|string| fs::write(output_filepath, string))??;

    if silent.not() {
        println!(
            "Unpacked {} ships into {}",
            count,
            output_filepath.display()
        );
    }

    Ok(())
}
