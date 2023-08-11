use crate::bin_io::read::{BinRead, BinReader};
use byteorder::BigEndian;
use serde::Serialize;
use std::fs::File;
use std::path::Path;
use std::{fs, io};

pub fn file<T: BinRead + Serialize>(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
) -> io::Result<usize> {
    let mut file = File::open(input_filepath)?;

    let mut items = Vec::<T>::new();
    let mut count = 0;

    while let Ok(item) = file.read_bin::<BigEndian>() {
        items.push(item);
        count += 1;
    }

    serde_json::to_string_pretty(&items).map(|json| fs::write(output_filepath, json))??;

    Ok(count)
}
