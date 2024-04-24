use crate::binary_patch::BinaryPatch;
use crate::binary_version::BinaryVersion;
use byteorder::WriteBytesExt;
use gof2edit::Error;
use std::collections::hash_map::Entry;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};
use std::ops::Not;
use std::path::Path;

pub fn patch(
    patch_filepath: impl AsRef<Path>,
    binary_filepath: impl AsRef<Path>,
    binary: BinaryVersion,
    silent: bool,
) -> gof2edit::Result<()> {
    let patch_filepath = patch_filepath.as_ref();
    let binary_filepath = binary_filepath.as_ref();

    let source = File::open(patch_filepath)?;
    let mut destination = OpenOptions::new().write(true).open(binary_filepath)?;

    if silent.not() {
        println!("Loading patch from \"{}\" ...", patch_filepath.display());
    }

    let mut patch: BinaryPatch = serde_json::from_reader(source)?;

    match patch.addresses.entry(binary) {
        Entry::Occupied(entry) => entry.get().iter().try_for_each(|(address, value)| {
            destination.seek(SeekFrom::Start(*address))?;
            destination.write_u8(*value)
        })?,
        Entry::Vacant(_) => return Err(Error::PatchNotAvailableForBinaryVersion),
    }

    if silent.not() {
        println!(
            "Applied patch \"{}\" into \"{}\"",
            patch.name,
            binary_filepath.display()
        );
    }

    Ok(())
}
