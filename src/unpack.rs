use crate::bin_io::read::{BinRead, BinReader};
use crate::data::item::Item;
use crate::data::lang_string::LangString;
use crate::data::ship::Ship;
use crate::data::station::Station;
use crate::data::system::System;
use crate::index::Index;
use crate::targets::unpack::UnpackTarget;
use byteorder::BigEndian;
use serde::Serialize;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::ops::Not;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn unpack(
    target: UnpackTarget,
    input_filepath: impl AsRef<Path>,
    silent: bool,
) -> io::Result<()> {
    let input_filepath = input_filepath.as_ref();
    let output_filepath = &output_filepath(input_filepath, "json");

    if silent.not() {
        println!("Unpacking {target} from {} ...", input_filepath.display());
    }

    let count = match target {
        UnpackTarget::Items => unpack_file::<Item>(input_filepath, output_filepath, target),
        UnpackTarget::Lang => unpack_file::<LangString>(input_filepath, output_filepath, target),
        UnpackTarget::Ships => unpack_file::<Ship>(input_filepath, output_filepath, target),
        UnpackTarget::Stations => unpack_file::<Station>(input_filepath, output_filepath, target),
        UnpackTarget::Systems => unpack_file::<System>(input_filepath, output_filepath, target),
    }?;

    if silent.not() {
        println!(
            "Unpacked {count} {target} into {}",
            output_filepath.display()
        );
    }

    Ok(())
}

fn unpack_file<T: Index + BinRead + Serialize>(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
    target: UnpackTarget,
) -> io::Result<usize> {
    let mut file = File::open(input_filepath)?;

    let objects = bin_to_json::<T>(&mut file, target)?;

    let json = serde_json::to_string_pretty(&objects)?;

    fs::write(output_filepath, json)?;

    Ok(objects.len())
}

fn bin_to_json<T: Index + BinRead>(
    source: &mut impl Read,
    target: UnpackTarget,
) -> io::Result<Vec<T>> {
    let mut objects = vec![];
    let mut count = 0;

    while let Ok(object) = source.read_bin::<BigEndian>() {
        let mut object: T = object;

        match target {
            UnpackTarget::Systems | UnpackTarget::Lang => object.set_index(count),
            _ => {}
        }

        objects.push(object);
        count += 1;
    }

    Ok(objects)
}

// TODO: Move to dedicated module
fn output_filepath(filepath: impl AsRef<Path>, extension: impl AsRef<OsStr>) -> PathBuf {
    let mut filepath = filepath.as_ref().to_owned();
    filepath.set_extension(extension);

    filepath
}
