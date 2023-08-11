use crate::data::lang_string::LangString;
use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::ErrorKind;
use std::ops::Not;
use std::path::Path;
use std::{fs, io};

pub fn unpack(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
    silent: bool,
) -> io::Result<()> {
    let input_filepath = input_filepath.as_ref();
    let output_filepath = output_filepath.as_ref();

    if silent.not() {
        println!(
            "Unpacking lang strings from {} ...",
            input_filepath.display()
        );
    }

    let mut file = File::open(input_filepath)?;
    let mut lang_strings = vec![];
    let mut count = 0;

    while let Ok(lang_string) = read_one(&mut file, count) {
        lang_strings.push(lang_string);
        count += 1
    }

    serde_json::to_string_pretty(&lang_strings)
        .map(|string| fs::write(output_filepath, string))??;

    if silent.not() {
        println!(
            "Unpacked {} lang strings into {}",
            count,
            output_filepath.display()
        );
    }

    Ok(())
}

fn read_one(source: &mut impl ReadBytesExt, index: u32) -> io::Result<LangString> {
    let string_length = source.read_u16::<BigEndian>()?;
    let mut string = vec![0u8; string_length as usize];
    source.read_exact(&mut string)?;

    Ok(LangString {
        index,
        length: string_length,
        string: String::from_utf8(string).map_err(|_| ErrorKind::InvalidData)?,
    })
}
