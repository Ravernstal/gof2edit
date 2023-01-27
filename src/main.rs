use crate::arguments::{Action, Arguments};
use clap::Parser;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

mod arguments;
mod data;
mod lang;
mod patch;
mod stations;
mod systems;

fn main() {
    let arguments = Arguments::parse();

    let action = arguments.action;

    match parse_action(&action) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error: {}", error)
        }
    }
}

fn output_filepath(filepath: impl AsRef<Path>, extension: impl AsRef<str>) -> PathBuf {
    let filepath = filepath.as_ref();
    let extension = extension.as_ref();

    PathBuf::from(
        filepath
            .file_stem()
            .and_then(OsStr::to_str)
            .map(|filepath| format!("{}.{}", filepath, extension))
            .unwrap_or_else(|| format!("output.{}", extension)),
    )
}

fn parse_action(action: &Action) -> io::Result<()> {
    match action {
        Action::UnpackSystems { input_filepath } => {
            systems::unpack(input_filepath, output_filepath(input_filepath, "json"))
        }
        Action::RepackSystems { input_filepath } => {
            systems::repack(input_filepath, output_filepath(input_filepath, "bin"))
        }
        Action::PatchSystems {
            json_filepath,
            so_filepath,
        } => systems::patch(json_filepath, so_filepath),
        Action::UnpackStations { input_filepath } => {
            stations::unpack(input_filepath, output_filepath(input_filepath, "json"))
        }
        Action::RepackStations { input_filepath } => {
            stations::repack(input_filepath, output_filepath(input_filepath, "bin"))
        }
        Action::PatchStations {
            json_filepath,
            so_filepath,
        } => stations::patch(json_filepath, so_filepath),
        Action::UnpackLang { input_filepath } => {
            lang::unpack(input_filepath, output_filepath(input_filepath, "json"))
        }
        Action::RepackLang { input_filepath } => {
            lang::repack(input_filepath, output_filepath(input_filepath, "lang"))
        }
    }
}
