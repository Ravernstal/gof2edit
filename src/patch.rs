use crate::patch_addresses::binary_version::BinaryVersion;
use crate::patch_addresses::{station, system, WriteValueFn};
use crate::targets::patch::PatchTarget;
use gof2edit::data::{Station, System};
use serde::de::DeserializeOwned;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::Not;
use std::path::Path;

pub fn patch(
    json_filepath: impl AsRef<Path>,
    binary_filepath: impl AsRef<Path>,
    target: PatchTarget,
    binary: BinaryVersion,
    silent: bool,
) -> gof2edit::Result<()> {
    let json_filepath = json_filepath.as_ref();
    let binary_filepath = binary_filepath.as_ref();

    let mut source = File::open(json_filepath)?;
    let mut destination = OpenOptions::new().write(true).open(binary_filepath)?;

    if silent.not() {
        println!("Reading {target} from \"{}\" ...", json_filepath.display());
    }

    let count = match target {
        PatchTarget::Stations => patch_objects::<Station, File>(
            &mut source,
            &mut destination,
            station::addresses(binary),
        ),
        PatchTarget::Systems => {
            patch_objects::<System, File>(&mut source, &mut destination, system::addresses(binary))
        }
    }?;

    if silent.not() {
        println!(
            "Patched {count} {target} into \"{}\"",
            binary_filepath.display()
        );
    }

    Ok(())
}

fn patch_objects<T: DeserializeOwned, D: Write + Seek>(
    source: &mut impl Read,
    destination: &mut D,
    address_list_modifiers: &[(u64, &WriteValueFn)],
) -> gof2edit::Result<usize> {
    let objects: Vec<T> = serde_json::from_reader(source)?;
    let count = objects.len().try_into()?;

    patch_address_list_modifiers(destination, address_list_modifiers, count)?;

    Ok(objects.len())
}

fn patch_address_list_modifiers<T: Write + Seek>(
    destination: &mut T,
    address_modifiers: &[(u64, &WriteValueFn)],
    byte: u8,
) -> gof2edit::Result<()> {
    address_modifiers
        .iter()
        .try_for_each(|&(address, function)| {
            destination.seek(SeekFrom::Start(address))?;
            function(destination, byte)
        })
}
