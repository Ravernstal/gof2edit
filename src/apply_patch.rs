use crate::binary_version::BinaryVersion;
use std::path::Path;

pub fn patch(
    patch_filepath: impl AsRef<Path>,
    binary_filepath: impl AsRef<Path>,
    binary: BinaryVersion,
    silent: bool,
) -> gof2edit::Result<()> {
    todo!()
}
