use crate::data::ship::Ship;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::path::Path;
use std::{fs, io};

pub fn unpack(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
) -> io::Result<()> {
    let input_filepath = input_filepath.as_ref();
    let output_filepath = output_filepath.as_ref();

    println!("Unpacking ships from {} ...", input_filepath.display());

    let mut file = File::open(input_filepath)?;
    let mut ships = vec![];
    let mut count = 0;

    while let Ok(ship) = read_one(&mut file) {
        ships.push(ship);
        count += 1
    }

    serde_json::to_string_pretty(&ships).map(|string| fs::write(output_filepath, string))??;

    println!(
        "Unpacked {} ships into {}",
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

    println!("Repacking ships from {} ...", input_filepath.display());

    let json_string = fs::read_to_string(input_filepath)?;
    let mut ships = serde_json::from_str::<Vec<Ship>>(&json_string)?;

    ships.sort_unstable_by(|s1, s2| s1.index.cmp(&s2.index));

    let mut file = File::create(output_filepath)?;

    ships
        .iter()
        .try_for_each(|system| write_one(&mut file, system))?;

    println!(
        "Repacked {} ships into {}",
        ships.len(),
        output_filepath.display()
    );

    Ok(())
}

fn read_one(source: &mut impl ReadBytesExt) -> io::Result<Ship> {
    let index = source.read_u32::<BigEndian>()?;
    let armour = source.read_u32::<BigEndian>()?;
    let cargo_capacity = source.read_u32::<BigEndian>()?;
    let price = source.read_u32::<BigEndian>()?;
    let primary_weapon_count = source.read_u32::<BigEndian>()?;
    let secondary_weapon_count = source.read_u32::<BigEndian>()?;
    let turret_count = source.read_u32::<BigEndian>()?;
    let equipment_slot_count = source.read_u32::<BigEndian>()?;
    let handling = source.read_u32::<BigEndian>()?;

    Ok(Ship {
        index,
        armour,
        cargo_capacity,
        price,
        primary_weapon_count,
        secondary_weapon_count,
        turret_count,
        equipment_slot_count,
        handling,
    })
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
