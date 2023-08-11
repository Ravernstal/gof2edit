use crate::commands::Command;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,
    /// Disable standard output
    #[arg(short, long)]
    pub silent: bool,
}
