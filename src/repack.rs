use crate::targets::repack::RepackTarget;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use gof2edit::bin_io::write::BinWrite;
use gof2edit::data::save::Save;
use gof2edit::data::{
    Agent, Item, LangString, NewsItem, SavePreview, Ship, ShipPosition, Station, System, Wanted,
};
use gof2edit::index::Index;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Not;
use std::path::Path;

pub fn json_to_bin(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
    target: RepackTarget,
    silent: bool,
) -> gof2edit::Result<()> {
    let input_filepath = input_filepath.as_ref();
    let output_filepath = output_filepath.as_ref();

    let mut source = File::open(input_filepath)?;
    let mut destination = File::create(output_filepath)?;

    if silent.not() {
        println!(
            "Repacking {target} from \"{}\" ...",
            input_filepath.display()
        );
    }

    let count = match target {
        RepackTarget::Agents => {
            serialise_objects::<Agent, BigEndian>(&mut source, &mut destination)?
        }
        RepackTarget::Items => serialise_objects::<Item, BigEndian>(&mut source, &mut destination)?,
        RepackTarget::Lang => {
            serialise_objects::<LangString, BigEndian>(&mut source, &mut destination)?
        }
        RepackTarget::NewsItems => {
            serialise_objects::<NewsItem, BigEndian>(&mut source, &mut destination)?
        }
        RepackTarget::Save => {
            println!("Warning: Save game repacking currently only works for Android saves");
            serialise_object::<Save, LittleEndian>(&mut source, &mut destination)?
        }
        RepackTarget::SavePreview => {
            serialise_object::<SavePreview, LittleEndian>(&mut source, &mut destination)?
        }
        RepackTarget::Ships => serialise_objects::<Ship, BigEndian>(&mut source, &mut destination)?,
        RepackTarget::ShipPositions => {
            serialise_objects::<ShipPosition, LittleEndian>(&mut source, &mut destination)?
        }
        RepackTarget::Stations => {
            serialise_objects::<Station, BigEndian>(&mut source, &mut destination)?
        }
        RepackTarget::Systems => {
            serialise_objects::<System, BigEndian>(&mut source, &mut destination)?
        }
        RepackTarget::Wanted => {
            serialise_objects::<Wanted, BigEndian>(&mut source, &mut destination)?
        }
    };

    if silent.not() {
        println!(
            "Repacked {count} {target} into \"{}\"",
            output_filepath.display()
        );
    }

    Ok(())
}

fn serialise_object<T: BinWrite + DeserializeOwned, O: ByteOrder>(
    source: &mut impl Read,
    destination: &mut impl Write,
) -> gof2edit::Result<usize> {
    let object = serde_json::from_reader(source)?;

    gof2edit::write_object::<T, O>(destination, &object)?;

    Ok(1)
}

fn serialise_objects<T: BinWrite + DeserializeOwned + Index, O: ByteOrder>(
    source: &mut impl Read,
    destination: &mut impl Write,
) -> gof2edit::Result<usize> {
    let objects = serde_json::from_reader(source)?;

    let count = gof2edit::write_object_list::<T, O>(destination, objects)?;

    Ok(count)
}
