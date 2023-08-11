use crate::patchers::{station, system};
use crate::targets::patch::PatchTarget;
use clap::Args;
use std::io;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct PatchCommand {
    /// Type of JSON file
    pub target: PatchTarget,
    /// JSON file used to patch
    pub json_filepath: PathBuf,
    /// SO file to patch
    pub so_filepath: PathBuf,
}

impl PatchCommand {
    pub fn execute(&self, silent: bool) -> io::Result<()> {
        match self.target {
            PatchTarget::Stations => station::patch(&self.json_filepath, &self.so_filepath, silent),
            PatchTarget::Systems => system::patch(&self.json_filepath, &self.so_filepath, silent),
        }
    }
}
