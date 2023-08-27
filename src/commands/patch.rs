use crate::patch_addresses::binary_version::BinaryVersion;
use crate::targets::patch::PatchTarget;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct PatchCommand {
    /// Type of JSON file
    pub target: PatchTarget,
    /// JSON file used to patch
    pub json_filepath: PathBuf,
    /// Binary file to patch
    pub binary_filepath: PathBuf,
    /// Binary version to patch
    #[clap(short, long, value_enum, default_value_t = BinaryVersion::AndroidKiritoJpk)]
    pub binary: BinaryVersion,
}
