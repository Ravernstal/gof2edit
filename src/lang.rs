use crate::data::lang_string::LangString;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;
use std::{fs, io};

pub fn unpack(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
) -> io::Result<()> {
    let input_filepath = input_filepath.as_ref();
    let output_filepath = output_filepath.as_ref();

    println!(
        "Unpacking lang strings from {} ...",
        input_filepath.display()
    );

    let mut file = File::open(input_filepath)?;
    let mut lang_strings = vec![];
    let mut count = 0;

    while let Ok(lang_string) = read_one(&mut file, count) {
        lang_strings.push(lang_string);
        count += 1
    }

    serde_json::to_string_pretty(&lang_strings)
        .map(|string| fs::write(output_filepath, string))??;

    println!(
        "Unpacked {} lang strings into {}",
        count,
        output_filepath.display()
    );

    Ok(())
}

pub fn repack(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
) -> io::Result<()> {
    let input_filepath = input_filepath.as_ref();
    let output_filepath = output_filepath.as_ref();

    println!(
        "Repacking lang strings from {} ...",
        input_filepath.display()
    );

    let json_string = fs::read_to_string(input_filepath)?;
    let mut lang_strings = serde_json::from_str::<Vec<LangString>>(&json_string)?;

    lang_strings.sort_unstable_by(|s1, s2| s1.index.cmp(&s2.index));

    let mut file = File::create(output_filepath)?;

    lang_strings
        .iter()
        .try_for_each(|system| write_one(&mut file, system))?;

    println!(
        "Repacked {} lang strings into {}",
        lang_strings.len(),
        output_filepath.display()
    );

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

fn write_one(destination: &mut impl WriteBytesExt, lang_string: &LangString) -> io::Result<()> {
    destination.write_u16::<BigEndian>(lang_string.string.as_bytes().len() as u16)?;
    destination.write_all(lang_string.string.as_bytes())?;

    Ok(())
}
