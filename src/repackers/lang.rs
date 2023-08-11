use crate::data::lang_string::LangString;
use byteorder::{BigEndian, WriteBytesExt};
use std::fs::File;
use std::ops::Not;
use std::path::Path;
use std::{fs, io};

pub fn repack(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
    silent: bool,
) -> io::Result<()> {
    let input_filepath = input_filepath.as_ref();
    let output_filepath = output_filepath.as_ref();

    if silent.not() {
        println!(
            "Repacking lang strings from {} ...",
            input_filepath.display()
        );
    }

    let json_string = fs::read_to_string(input_filepath)?;
    let mut lang_strings = serde_json::from_str::<Vec<LangString>>(&json_string)?;

    lang_strings.sort_unstable_by(|s1, s2| s1.index.cmp(&s2.index));

    let mut file = File::create(output_filepath)?;

    lang_strings
        .iter()
        .try_for_each(|system| write_one(&mut file, system))?;

    if silent.not() {
        println!(
            "Repacked {} lang strings into {}",
            lang_strings.len(),
            output_filepath.display()
        );
    }

    Ok(())
}

fn write_one(destination: &mut impl WriteBytesExt, lang_string: &LangString) -> io::Result<()> {
    destination.write_u16::<BigEndian>(lang_string.string.as_bytes().len() as u16)?;
    destination.write_all(lang_string.string.as_bytes())?;

    Ok(())
}
