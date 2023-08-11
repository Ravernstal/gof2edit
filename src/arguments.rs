use crate::commands::Command;
use clap::Parser;

const SILENT_HELP_TEXT: &str = "Disable standard output";

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,
    #[arg(short, long, help = SILENT_HELP_TEXT)]
    pub silent: bool,
}
