use crate::targets::repack::RepackTarget;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct RepackCommand {
    /// Type of JSON file
    pub target: RepackTarget,
    /// JSON file to repack
    pub filepath: PathBuf,
}
