use crate::arguments::Arguments;
use crate::command::Command;
use clap::Parser;

mod apply_patch;
mod arguments;
mod binary_patch;
mod binary_version;
mod command;
mod generate_patch;
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
            let output_filepath = input_filepath.with_extension("json");

            unpack::bin_to_json(input_filepath, output_filepath, *target, silent)
        }
        Command::Repack {
            target,
            input_filepath,
        } => {
            let output_filepath = input_filepath.with_extension("bin");

            repack::json_to_bin(input_filepath, output_filepath, *target, silent)
        }
        Command::Patch {
            target,
            json_filepath,
            binary_filepath,
            binary,
        } => patch::patch(json_filepath, binary_filepath, *target, *binary, silent),
        Command::GeneratePatch {
            binary1_filepath,
            binary2_filepath,
            binary,
        } => generate_patch::generate(binary1_filepath, binary2_filepath, *binary, silent),
        Command::ApplyPatch {
            patch_filepath,
            binary_filepath,
            binary,
        } => apply_patch::patch(patch_filepath, binary_filepath, *binary, silent),
    }
}
