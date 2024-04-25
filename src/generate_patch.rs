use crate::binary_patch::BinaryPatch;
use crate::binary_version::BinaryVersion;
use gof2edit::Error;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::ops::Not;
use std::path::Path;

const PATCH_FILEPATH: &str = "patch.json";

pub fn generate(
    binary1_filepath: impl AsRef<Path>,
    binary2_filepath: impl AsRef<Path>,
    binary: BinaryVersion,
    silent: bool,
) -> gof2edit::Result<()> {
    let binary1_filepath = binary1_filepath.as_ref();
    let binary2_filepath = binary2_filepath.as_ref();

    let binary1 = File::open(binary1_filepath)?;
    let binary2 = File::open(binary2_filepath)?;

    if binary1.metadata()?.len() != binary2.metadata()?.len() {
        return Err(Error::BinarySizesDiffer);
    }

    if silent.not() {
        println!(
            "Reading binaries \"{}\" and \"{}\" ...",
            binary1_filepath.display(),
            binary2_filepath.display()
        );
    }

    let mut patch = BinaryPatch::default();
    let mut addresses = BTreeMap::new();

    for (address, (value1, value2)) in binary1.bytes().zip(binary2.bytes()).enumerate() {
        let address = address.try_into()?;
        let value1 = value1?;
        let value2 = value2?;

        if value1 != value2 {
            addresses.insert(address, value2);
        }
    }

    patch.addresses.insert(binary, addresses);

    let output_filepath = PATCH_FILEPATH;
    let destination = File::create(&output_filepath)?;

    serde_json::to_writer_pretty(destination, &patch)?;

    if silent.not() {
        println!("Generated patch \"{}\"", output_filepath);
    }

    Ok(())
}
