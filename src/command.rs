use crate::binary_version::BinaryVersion;
use crate::targets::patch::PatchTarget;
use crate::targets::repack::RepackTarget;
use crate::targets::unpack::UnpackTarget;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Unpack BIN files into JSON format
    Unpack {
        /// Type of BIN file
        target: UnpackTarget,
        /// BIN file to unpack
        input_filepath: PathBuf,
    },
    /// Repack JSON files into BIN format
    Repack {
        /// Type of JSON file
        target: RepackTarget,
        /// JSON file to repack
        input_filepath: PathBuf,
    },
    /// Patch new objects into binary with JSON file
    Patch {
        /// Type of JSON file
        target: PatchTarget,
        /// JSON file used to patch
        json_filepath: PathBuf,
        /// Binary file to patch
        binary_filepath: PathBuf,
        /// Binary version to patch
        #[clap(short, long, value_enum, default_value_t = BinaryVersion::Android)]
        binary: BinaryVersion,
    },
}
