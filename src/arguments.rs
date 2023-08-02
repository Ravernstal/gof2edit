use crate::commands::Command;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,
}
