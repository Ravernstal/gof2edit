use crate::data::station::Station;
use byteorder::{BigEndian, WriteBytesExt};
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
        .try_for_each(|station| write_one(&mut file, station))?;

    println!(
        "Repacked {} stations into {}",
        stations.len(),
        output_filepath.display()
    );

    Ok(())
}

fn write_one(destination: &mut impl WriteBytesExt, station: &Station) -> io::Result<()> {
    destination.write_u16::<BigEndian>(station.name.as_bytes().len() as u16)?;
    destination.write_all(station.name.as_bytes())?;

    destination.write_u32::<BigEndian>(station.index)?;
    destination.write_u32::<BigEndian>(station.system_index)?;
    destination.write_u32::<BigEndian>(station.tech_level)?;
    destination.write_u32::<BigEndian>(station.texture_index)?;

    Ok(())
}
