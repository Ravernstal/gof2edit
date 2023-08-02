use crate::repackers::{lang, ship, station, system};
use crate::targets::repack::RepackTarget;
use clap::Args;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Args, Debug)]
pub struct RepackCommand {
    /// Type of JSON file
    pub target: RepackTarget,
    /// JSON file to repack
    pub filepath: PathBuf,
}

impl RepackCommand {
    pub fn execute(&self) -> io::Result<()> {
        let input_filepath = &self.filepath;
        let output_filepath = output_filepath(input_filepath, "bin");

        match self.target {
            RepackTarget::Lang => lang::repack(input_filepath, output_filepath),
            RepackTarget::Ships => ship::repack(input_filepath, output_filepath),
            RepackTarget::Stations => station::repack(input_filepath, output_filepath),
            RepackTarget::Systems => system::repack(input_filepath, output_filepath),
        }
    }
}

// TODO: Move to dedicated module
fn output_filepath(filepath: impl AsRef<Path>, extension: impl AsRef<OsStr>) -> PathBuf {
    let mut filepath = filepath.as_ref().to_owned();
    filepath.set_extension(extension);

    filepath
}
