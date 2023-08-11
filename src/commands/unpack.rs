use crate::targets::unpack::UnpackTarget;
use crate::unpackers::{item, lang, ship, station, system};
use clap::Args;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Args, Debug)]
pub struct UnpackCommand {
    /// Type of BIN file
    pub target: UnpackTarget,
    /// BIN file to unpack
    pub filepath: PathBuf,
}

impl UnpackCommand {
    pub fn execute(&self, silent: bool) -> io::Result<()> {
        let input_filepath = &self.filepath;
        let output_filepath = output_filepath(input_filepath, "json");

        match self.target {
            UnpackTarget::Items => item::unpack(input_filepath, output_filepath, silent),
            UnpackTarget::Lang => lang::unpack(input_filepath, output_filepath, silent),
            UnpackTarget::Ships => ship::unpack(input_filepath, output_filepath, silent),
            UnpackTarget::Stations => station::unpack(input_filepath, output_filepath, silent),
            UnpackTarget::Systems => system::unpack(input_filepath, output_filepath, silent),
        }
    }
}

// TODO: Move to dedicated module
fn output_filepath(filepath: impl AsRef<Path>, extension: impl AsRef<OsStr>) -> PathBuf {
    let mut filepath = filepath.as_ref().to_owned();
    filepath.set_extension(extension);

    filepath
}
