use crate::targets::unpack::UnpackTarget;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct UnpackCommand {
    /// Type of BIN file
    pub target: UnpackTarget,
    /// BIN file to unpack
    pub filepath: PathBuf,
}
