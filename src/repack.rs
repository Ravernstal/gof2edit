use crate::targets::repack::RepackTarget;
use gof2edit::bin_io::write::BinWrite;
use gof2edit::data::{Item, LangString, Ship, Station, System};
use gof2edit::index::Index;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::ops::Not;
use std::path::Path;

pub fn json_to_bin(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
    target: RepackTarget,
    silent: bool,
) -> io::Result<()> {
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
        RepackTarget::Items => serialise_objects::<Item>(&mut source, &mut destination)?,
        RepackTarget::Lang => serialise_objects::<LangString>(&mut source, &mut destination)?,
        RepackTarget::Ships => serialise_objects::<Ship>(&mut source, &mut destination)?,
        RepackTarget::Stations => serialise_objects::<Station>(&mut source, &mut destination)?,
        RepackTarget::Systems => serialise_objects::<System>(&mut source, &mut destination)?,
    };

    if silent.not() {
        println!(
            "Repacked {count} {target} into \"{}\"",
            output_filepath.display()
        );
    }

    Ok(())
}

fn serialise_objects<T: BinWrite + DeserializeOwned + Index>(
    source: &mut impl Read,
    destination: &mut impl Write,
) -> io::Result<usize> {
    let objects: Vec<T> = serde_json::from_reader(source)?;

    let count = gof2edit::write_object_list(destination, objects)?;

    Ok(count)
}
