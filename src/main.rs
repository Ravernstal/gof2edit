use crate::arguments::Arguments;
use crate::commands::Command;
use clap::Parser;
use std::io;

mod arguments;
mod bin_io;
mod commands;
mod data;
mod patch;
mod patch_addresses;
mod patchers;
mod repackers;
mod targets;
mod unpackers;

fn main() {
    let arguments = Arguments::parse();

    match execute_command(&arguments.command, arguments.silent) {
        Ok(_) => {}
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn execute_command(command: &Command, silent: bool) -> io::Result<()> {
    match command {
        Command::Unpack(command) => command.execute(silent),
        Command::Repack(command) => command.execute(silent),
        Command::Patch(command) => command.execute(silent),
    }
}
