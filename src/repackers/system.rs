use crate::data::system::System;
use crate::utilities;
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

    println!("Repacking systems from {} ...", input_filepath.display());

    let json_string = fs::read_to_string(input_filepath)?;
    let mut systems = serde_json::from_str::<Vec<System>>(&json_string)?;

    systems.sort_unstable_by(|s1, s2| s1.index.cmp(&s2.index));

    let mut file = File::create(output_filepath)?;

    systems
        .iter()
        .try_for_each(|system| write_one(&mut file, system))?;

    println!(
        "Repacked {} systems into {}",
        systems.len(),
        output_filepath.display()
    );

    Ok(())
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

    utilities::write_u32_list(destination, &system.unknown_bytes)?;
    utilities::write_u32_list(destination, &system.station_ids)?;
    utilities::write_u32_list(destination, &system.linked_system_ids)?;
    utilities::write_u32_list(destination, &system.footer_bytes)?;

    Ok(())
}
