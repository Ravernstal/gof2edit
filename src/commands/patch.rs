use crate::patchers::{station, system};
use crate::targets::patch::PatchTarget;
use clap::Args;
use std::io;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct PatchCommand {
    pub target: PatchTarget,
    pub json_filepath: PathBuf,
    pub so_filepath: PathBuf,
}

impl PatchCommand {
    pub fn execute(&self) -> io::Result<()> {
        match self.target {
            PatchTarget::Stations => station::patch(&self.json_filepath, &self.so_filepath),
            PatchTarget::Systems => system::patch(&self.json_filepath, &self.so_filepath),
        }
    }
}
