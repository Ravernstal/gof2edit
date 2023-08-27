use crate::arguments::Arguments;
use crate::commands::Command;
use clap::Parser;
use std::io;

mod arguments;
mod bin_io;
mod commands;
mod data;
mod index;
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

fn execute_command(command: &Command, silent: bool) -> io::Result<()> {
    match command {
        Command::Unpack(command) => unpack::unpack(command.target, &command.filepath, silent),
        Command::Repack(command) => repack::repack(command.target, &command.filepath, silent),
        Command::Patch(command) => patch::patch(
            command.target,
            &command.json_filepath,
            &command.binary_filepath,
            command.binary,
            silent,
        ),
    }
}
