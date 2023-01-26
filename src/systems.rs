use crate::{Faction, SecurityLevel, System};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::ErrorKind;
use std::ops::Not;
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
    let mut systems = vec![];
    let mut count = 0;

    while let Ok(system) = read_one(&mut file, count) {
        systems.push(system);
        count += 1
    }

    serde_json::to_string_pretty(&systems).map(|string| fs::write(output_filepath, string))??;

    println!(
        "Unpacked {} systems to {}",
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

    println!("Repacking systems from {} ...", input_filepath.display());

    let json_string = fs::read_to_string(input_filepath)?;
    let mut systems = serde_json::from_str::<Vec<System>>(&json_string)?;

    systems.sort_unstable_by(|s1, s2| s1.index.cmp(&s2.index));

    let mut file = File::create(output_filepath)?;

    systems
        .iter()
        .try_for_each(|system| write_one(&mut file, system))?;

    println!(
        "Repacked {} systems to {}",
        systems.len(),
        output_filepath.display()
    );

    Ok(())
}

fn read_one(source: &mut impl ReadBytesExt, index: u32) -> io::Result<System> {
    let name_length = source.read_u16::<BigEndian>()?;
    let mut name = vec![0u8; name_length as usize];
    source.read_exact(&mut name)?;

    let security_level = source.read_u32::<BigEndian>()?;
    let starts_unlocked = source.read_u32::<BigEndian>()?;
    let faction = source.read_u32::<BigEndian>()?;

    let map_coords = [
        source.read_u32::<BigEndian>()?,
        source.read_u32::<BigEndian>()?,
        source.read_u32::<BigEndian>()?,
    ];

    let jumpgate_station_id = source.read_u32::<BigEndian>()?;
    let texture_index = source.read_u32::<BigEndian>()?;

    let unknown_bytes = read_u32_list(source)?;
    let station_ids = read_u32_list(source)?;
    let linked_system_ids = read_u32_list(source)?;
    let footer_bytes = read_u32_list(source)?;

    Ok(System {
        index,
        name: String::from_utf8(name).map_err(|_| ErrorKind::InvalidData)?,
        security_level: SecurityLevel::from_u32(security_level).ok_or(ErrorKind::InvalidData)?,
        faction: Faction::from_u32(faction).ok_or(ErrorKind::InvalidData)?,
        starts_unlocked: matches!(starts_unlocked, 0).not(),
        map_coords,
        jumpgate_station_id: match jumpgate_station_id {
            0xffffffff => None,
            _ => Some(jumpgate_station_id),
        },
        texture_index,
        unknown_bytes,
        station_ids,
        linked_system_ids,
        footer_bytes,
    })
}

fn write_one(destination: &mut impl WriteBytesExt, system: &System) -> io::Result<()> {
    destination.write_u16::<BigEndian>(system.name.as_bytes().len() as u16)?;
    destination.write_all(system.name.as_bytes())?;

    destination.write_u32::<BigEndian>(system.security_level.code())?;
    destination.write_u32::<BigEndian>(system.starts_unlocked.into())?;
    destination.write_u32::<BigEndian>(system.faction.code())?;

    destination.write_u32::<BigEndian>(system.map_coords[0])?;
    destination.write_u32::<BigEndian>(system.map_coords[1])?;
    destination.write_u32::<BigEndian>(system.map_coords[2])?;

    destination.write_u32::<BigEndian>(system.jumpgate_station_id.unwrap_or(0xffffffff))?;
    destination.write_u32::<BigEndian>(system.texture_index)?;

    write_u32_list(destination, &system.unknown_bytes)?;
    write_u32_list(destination, &system.station_ids)?;
    write_u32_list(destination, &system.linked_system_ids)?;
    write_u32_list(destination, &system.footer_bytes)?;

    Ok(())
}

fn read_u32_list(source: &mut impl ReadBytesExt) -> io::Result<Vec<u32>> {
    let count = source.read_u32::<BigEndian>()?;
    let mut list = Vec::with_capacity(count as usize);

    for _ in 0..count {
        list.push(source.read_u32::<BigEndian>()?);
    }

    Ok(list)
}

fn write_u32_list(destination: &mut impl WriteBytesExt, list: &[u32]) -> io::Result<()> {
    destination.write_u32::<BigEndian>(list.len() as u32)?;

    list.iter()
        .try_for_each(|x| destination.write_u32::<BigEndian>(*x))?;

    Ok(())
}
