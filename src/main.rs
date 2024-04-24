use crate::arguments::Arguments;
use crate::command::Command;
use clap::Parser;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

mod arguments;
mod binary_patch;
mod binary_version;
mod command;
mod patch;
mod patch_addresses;
mod repack;
mod targets;
mod unpack;

fn main() {
    let arguments = Arguments::parse();

    match execute_command(&arguments.command, arguments.silent) {
        Ok(_) => {}
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn execute_command(command: &Command, silent: bool) -> gof2edit::Result<()> {
    match command {
        Command::Unpack {
            target,
            input_filepath,
        } => {
            let output_filepath = output_filepath(input_filepath, "json");
            unpack::bin_to_json(input_filepath, output_filepath, *target, silent)
        }
        Command::Repack {
            target,
            input_filepath,
        } => {
            let output_filepath = output_filepath(input_filepath, "bin");
            repack::json_to_bin(input_filepath, output_filepath, *target, silent)
        }

        Command::Patch {
            target,
            json_filepath,
            binary_filepath,
            binary,
        } => patch::patch(json_filepath, binary_filepath, *target, *binary, silent),
    }
}

// TODO: Move to dedicated module
fn output_filepath(filepath: impl AsRef<Path>, extension: impl AsRef<OsStr>) -> PathBuf {
    let mut filepath = filepath.as_ref().to_owned();
    filepath.set_extension(extension);

    filepath
}
