use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Unpack systems.bin into JSON format
    UnpackSystems { input_filepath: PathBuf },
    /// Repack systems.json into BIN format
    RepackSystems { input_filepath: PathBuf },
    /// Patch binary with systems.json
    PatchSystems {
        json_filepath: PathBuf,
        so_filepath: PathBuf,
    },
    /// Unpack stations.bin into JSON format
    UnpackStations { input_filepath: PathBuf },
    /// Repack stations.json into BIN format
    RepackStations { input_filepath: PathBuf },
    /// Patch binary with stations.json
    PatchStations {
        json_filepath: PathBuf,
        so_filepath: PathBuf,
    },
    /// Unpack LANG file into JSON format
    UnpackLang { input_filepath: PathBuf },
    /// Repack JSON file into LANG format
    RepackLang { input_filepath: PathBuf },
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arguments {
    #[command(subcommand)]
    pub action: Action,
}
