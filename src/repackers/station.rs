use crate::bin_io::write::BinWriter;
use crate::data::station::Station;
use byteorder::BigEndian;
use std::fs::File;
use std::path::Path;
use std::{fs, io};

pub fn repack(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
) -> io::Result<()> {
    let input_filepath = input_filepath.as_ref();
    let output_filepath = output_filepath.as_ref();

    println!("Repacking stations from {} ...", input_filepath.display());

    let json_string = fs::read_to_string(input_filepath)?;
    let mut stations = serde_json::from_str::<Vec<Station>>(&json_string)?;

    stations.sort_unstable_by(|s1, s2| s1.index.cmp(&s2.index));

    let mut file = File::create(output_filepath)?;

    stations
        .iter()
        .try_for_each(|station| file.write_bin::<BigEndian>(station))?;

    println!(
        "Repacked {} stations into {}",
        stations.len(),
        output_filepath.display()
    );

    Ok(())
}
