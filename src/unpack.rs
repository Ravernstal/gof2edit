use crate::targets::unpack::UnpackTarget;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use gof2edit::bin_io::read::BinRead;
use gof2edit::data::save::Save;
use gof2edit::data::{
    Agent, Item, LangString, NewsItem, SavePreview, Ship, ShipPosition, Station, System, Wanted,
};
use gof2edit::index::Index;
use serde::Serialize;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Not;
use std::path::Path;

pub fn bin_to_json(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
    target: UnpackTarget,
    silent: bool,
) -> gof2edit::Result<()> {
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
        UnpackTarget::Agents => {
            deserialise_objects::<Agent, BigEndian>(&mut source, &mut destination)?
        }
        UnpackTarget::Items => {
            deserialise_objects::<Item, BigEndian>(&mut source, &mut destination)?
        }
        UnpackTarget::Lang => {
            deserialise_objects_indexed::<LangString, BigEndian>(&mut source, &mut destination)?
        }
        UnpackTarget::NewsItems => {
            deserialise_objects_indexed::<NewsItem, BigEndian>(&mut source, &mut destination)?
        }
        UnpackTarget::Save => {
            deserialise_object::<Save, LittleEndian>(&mut source, &mut destination)?
        }
        UnpackTarget::SavePreview => {
            deserialise_object::<SavePreview, LittleEndian>(&mut source, &mut destination)?
        }
        UnpackTarget::Ships => {
            deserialise_objects::<Ship, BigEndian>(&mut source, &mut destination)?
        }
        UnpackTarget::ShipPositions => {
            deserialise_objects::<ShipPosition, LittleEndian>(&mut source, &mut destination)?
        }
        UnpackTarget::Stations => {
            deserialise_objects::<Station, BigEndian>(&mut source, &mut destination)?
        }
        UnpackTarget::Systems => {
            deserialise_objects_indexed::<System, BigEndian>(&mut source, &mut destination)?
        }
        UnpackTarget::Wanted => {
            deserialise_objects_indexed::<Wanted, BigEndian>(&mut source, &mut destination)?
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

fn deserialise_object<T: BinRead + Serialize, O: ByteOrder>(
    source: &mut impl Read,
    destination: &mut impl Write,
) -> gof2edit::Result<usize> {
    let object = gof2edit::read_object::<T, O>(source)?;

    serde_json::to_writer_pretty(destination, &object)?;

    Ok(1)
}

fn deserialise_objects<T: BinRead + Serialize, O: ByteOrder>(
    source: &mut impl Read,
    destination: &mut impl Write,
) -> gof2edit::Result<usize> {
    let objects = gof2edit::read_object_list::<T, O>(source)?;

    serde_json::to_writer_pretty(destination, &objects)?;

    Ok(objects.len())
}

fn deserialise_objects_indexed<T: BinRead + Serialize + Index, O: ByteOrder>(
    source: &mut impl Read,
    destination: &mut impl Write,
) -> gof2edit::Result<usize> {
    let objects = gof2edit::read_object_list_indexed::<T, O>(source)?;

    serde_json::to_writer_pretty(destination, &objects)?;

    Ok(objects.len())
}
