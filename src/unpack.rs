use crate::targets::unpack::UnpackTarget;
use gof2edit::bin_io::read::BinRead;
use gof2edit::data::{Item, LangString, Ship, Station, System};
use gof2edit::index::Index;
use serde::Serialize;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::ops::Not;
use std::path::Path;

pub fn bin_to_json(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
    target: UnpackTarget,
    silent: bool,
) -> io::Result<()> {
    let input_filepath = input_filepath.as_ref();
    let output_filepath = output_filepath.as_ref();

    let mut source = File::open(input_filepath)?;
    let mut destination = File::create(output_filepath)?;

    if silent.not() {
        println!(
            "Unpacking {target} from \"{}\" ...",
            input_filepath.display()
        );
    }

    let count = match target {
        UnpackTarget::Items => deserialise_objects::<Item>(&mut source, &mut destination)?,
        UnpackTarget::Lang => {
            deserialise_objects_indexed::<LangString>(&mut source, &mut destination)?
        }
        UnpackTarget::Ships => deserialise_objects::<Ship>(&mut source, &mut destination)?,
        UnpackTarget::Stations => deserialise_objects::<Station>(&mut source, &mut destination)?,
        UnpackTarget::Systems => {
            deserialise_objects_indexed::<System>(&mut source, &mut destination)?
        }
    };

    if silent.not() {
        println!(
            "Unpacked {count} {target} into \"{}\"",
            output_filepath.display()
        );
    }

    Ok(())
}

fn deserialise_objects<T: BinRead + Serialize>(
    source: &mut impl Read,
    destination: &mut impl Write,
) -> io::Result<usize> {
    let objects = gof2edit::read_object_list::<T>(source)?;

    serde_json::to_writer_pretty(destination, &objects)?;

    Ok(objects.len())
}

fn deserialise_objects_indexed<T: BinRead + Serialize + Index>(
    source: &mut impl Read,
    destination: &mut impl Write,
) -> io::Result<usize> {
    let objects = gof2edit::read_object_list_indexed::<T>(source)?;

    serde_json::to_writer_pretty(destination, &objects)?;

    Ok(objects.len())
}
