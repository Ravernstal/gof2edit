use crate::data::station::Station;
use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;
use std::{fs, io};

pub fn unpack(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
) -> io::Result<()> {
    let input_filepath = input_filepath.as_ref();
    let output_filepath = output_filepath.as_ref();

    println!("Unpacking stations from {} ...", input_filepath.display());

    let mut file = File::open(input_filepath)?;
    let mut stations = vec![];
    let mut count = 0;

    while let Ok(station) = read_one(&mut file) {
        stations.push(station);
        count += 1
    }

    serde_json::to_string_pretty(&stations).map(|string| fs::write(output_filepath, string))??;

    println!(
        "Unpacked {} stations into {}",
        count,
        output_filepath.display()
    );

    Ok(())
}

fn read_one(source: &mut impl ReadBytesExt) -> io::Result<Station> {
    let name_length = source.read_u16::<BigEndian>()?;
    let mut name = vec![0u8; name_length as usize];
    source.read_exact(&mut name)?;

    let index = source.read_u32::<BigEndian>()?;
    let system_index = source.read_u32::<BigEndian>()?;
    let tech_level = source.read_u32::<BigEndian>()?;
    let texture_index = source.read_u32::<BigEndian>()?;

    Ok(Station {
        index,
        name: String::from_utf8(name).map_err(|_| ErrorKind::InvalidData)?,
        system_index,
        tech_level,
        texture_index,
    })
}
