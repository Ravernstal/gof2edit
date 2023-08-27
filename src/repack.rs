use crate::bin_io::write::{BinWrite, BinWriter};
use crate::data::item::Item;
use crate::data::lang_string::LangString;
use crate::data::ship::Ship;
use crate::data::station::Station;
use crate::data::system::System;
use crate::index::Index;
use crate::targets::repack::RepackTarget;
use byteorder::BigEndian;
use serde::Deserialize;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::ops::Not;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn repack(
    target: RepackTarget,
    input_filepath: impl AsRef<Path>,
    silent: bool,
) -> io::Result<()> {
    let input_filepath = input_filepath.as_ref();
    let output_filepath = &output_filepath(input_filepath, "bin");

    if silent.not() {
        println!("Repacking {target} from {} ...", input_filepath.display());
    }

    let count = match target {
        RepackTarget::Items => repack_file::<Item>(input_filepath, output_filepath),
        RepackTarget::Lang => repack_file::<LangString>(input_filepath, output_filepath),
        RepackTarget::Ships => repack_file::<Ship>(input_filepath, output_filepath),
        RepackTarget::Stations => repack_file::<Station>(input_filepath, output_filepath),
        RepackTarget::Systems => repack_file::<System>(input_filepath, output_filepath),
    }?;

    if silent.not() {
        println!(
            "Repacked {count} {target} into {}",
            output_filepath.display()
        );
    }

    Ok(())
}

fn repack_file<T: Index + BinWrite + for<'de> Deserialize<'de>>(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
) -> io::Result<usize> {
    let json = fs::read_to_string(input_filepath)?;

    let objects = serde_json::from_str::<Vec<T>>(&json)?;

    let mut file = File::create(output_filepath)?;

    json_to_bin(&mut file, objects)
}

fn json_to_bin<T: Index + BinWrite>(
    destination: &mut impl Write,
    mut objects: Vec<T>,
) -> io::Result<usize> {
    objects.sort_unstable_by_key(|object| object.index());

    objects
        .iter()
        .try_for_each(|object| destination.write_bin::<BigEndian>(object))?;

    Ok(objects.len())
}

// TODO: Move to dedicated module
fn output_filepath(filepath: impl AsRef<Path>, extension: impl AsRef<OsStr>) -> PathBuf {
    let mut filepath = filepath.as_ref().to_owned();
    filepath.set_extension(extension);

    filepath
}
