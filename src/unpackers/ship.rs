use crate::data::ship::Ship;
use crate::unpack;
use std::io;
use std::ops::Not;
use std::path::Path;

pub fn unpack(
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
    silent: bool,
) -> io::Result<()> {
    let input_filepath = input_filepath.as_ref();
    let output_filepath = output_filepath.as_ref();

    if silent.not() {
        println!("Unpacking ships from {} ...", input_filepath.display());
    }

    let count = unpack::file::<Ship>(input_filepath, output_filepath)?;

    if silent.not() {
        println!("Unpacked {count} ships into {}", output_filepath.display());
    }

    Ok(())
}
