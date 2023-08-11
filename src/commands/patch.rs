use crate::patch_addresses::binary_version::BinaryVersion;
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
    /// Binary version to patch
    #[clap(short, long, value_enum, default_value_t = BinaryVersion::AndroidKiritoJpk)]
    pub binary: BinaryVersion,
}

impl PatchCommand {
    pub fn execute(&self, silent: bool) -> io::Result<()> {
        match self.target {
            PatchTarget::Stations => {
                station::patch(&self.json_filepath, &self.so_filepath, self.binary, silent)
            }
            PatchTarget::Systems => {
                system::patch(&self.json_filepath, &self.so_filepath, self.binary, silent)
            }
        }
    }
}
