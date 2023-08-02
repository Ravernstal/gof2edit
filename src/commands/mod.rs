use crate::commands::patch::PatchCommand;
use crate::commands::repack::RepackCommand;
use crate::commands::unpack::UnpackCommand;
use clap::Subcommand;

mod patch;
mod repack;
mod unpack;

#[derive(Debug, Subcommand)]
pub enum Command {
    Unpack(UnpackCommand),
    Repack(RepackCommand),
    Patch(PatchCommand),
}
