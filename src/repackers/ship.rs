use crate::data::ship::Ship;
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

    println!("Repacking ships from {} ...", input_filepath.display());

    let json_string = fs::read_to_string(input_filepath)?;
    let mut ships = serde_json::from_str::<Vec<Ship>>(&json_string)?;

    ships.sort_unstable_by(|s1, s2| s1.index.cmp(&s2.index));

    let mut file = File::create(output_filepath)?;

    ships
        .iter()
        .try_for_each(|ship| write_one(&mut file, ship))?;

    println!(
        "Repacked {} ships into {}",
        ships.len(),
        output_filepath.display()
    );

    Ok(())
}

fn write_one(destination: &mut impl WriteBytesExt, ship: &Ship) -> io::Result<()> {
    destination.write_u32::<BigEndian>(ship.index)?;
    destination.write_u32::<BigEndian>(ship.armour)?;
    destination.write_u32::<BigEndian>(ship.cargo_capacity)?;
    destination.write_u32::<BigEndian>(ship.price)?;
    destination.write_u32::<BigEndian>(ship.primary_weapon_count)?;
    destination.write_u32::<BigEndian>(ship.secondary_weapon_count)?;
    destination.write_u32::<BigEndian>(ship.turret_count)?;
    destination.write_u32::<BigEndian>(ship.equipment_slot_count)?;
    destination.write_u32::<BigEndian>(ship.handling)?;

    Ok(())
}
