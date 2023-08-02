use crate::commands::patch::PatchCommand;
use crate::commands::repack::RepackCommand;
use crate::commands::unpack::UnpackCommand;
use clap::Subcommand;

mod patch;
mod repack;
mod unpack;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Unpack BIN files into JSON format
    Unpack(UnpackCommand),
    /// Repack JSON files into BIN format
    Repack(RepackCommand),
    /// Patch binary with JSON files
    Patch(PatchCommand),
}
