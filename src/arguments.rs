use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum Action {
    UnpackSystems {
        input_filepath: PathBuf,
    },
    RepackSystems {
        input_filepath: PathBuf,
    },
    PatchSystems {
        json_filepath: PathBuf,
        so_filepath: PathBuf,
    },
    UnpackStations {
        input_filepath: PathBuf,
    },
    RepackStations {
        input_filepath: PathBuf,
    },
    PatchStations {
        json_filepath: PathBuf,
        so_filepath: PathBuf,
    },
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arguments {
    #[command(subcommand)]
    pub action: Action,
}
