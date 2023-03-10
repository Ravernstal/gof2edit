use crate::data::station::Station;
use crate::patch;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fs::{File, OpenOptions};
use std::io::ErrorKind;
use std::path::Path;
use std::{fs, io};

const STATION_COUNT_ADDRESSES: &[u64] = &[0x00134DFE, 0x00135646, 0x00135B6A];

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
        .try_for_each(|system| write_one(&mut file, system))?;

    println!(
        "Repacked {} stations into {}",
        stations.len(),
        output_filepath.display()
    );

    Ok(())
}

pub fn patch(json_filepath: impl AsRef<Path>, so_filepath: impl AsRef<Path>) -> io::Result<()> {
    let json_filepath = json_filepath.as_ref();
    let so_filepath = so_filepath.as_ref();

    println!("Reading stations from {} ...", json_filepath.display());

    let json_string = fs::read_to_string(json_filepath)?;
    let station_count = serde_json::from_str::<Vec<Station>>(&json_string)?.len() as u8;

    let mut file = OpenOptions::new().write(true).open(so_filepath)?;

    STATION_COUNT_ADDRESSES
        .iter()
        .try_for_each(|address| patch::set_byte(&mut file, *address, station_count))?;

    println!(
        "Patched {} stations into {}",
        station_count,
        so_filepath.display()
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

fn write_one(destination: &mut impl WriteBytesExt, station: &Station) -> io::Result<()> {
    destination.write_u16::<BigEndian>(station.name.as_bytes().len() as u16)?;
    destination.write_all(station.name.as_bytes())?;

    destination.write_u32::<BigEndian>(station.index)?;
    destination.write_u32::<BigEndian>(station.system_index)?;
    destination.write_u32::<BigEndian>(station.tech_level)?;
    destination.write_u32::<BigEndian>(station.texture_index)?;

    Ok(())
}
