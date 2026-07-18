use std::process::ExitCode;

use clap::{Parser, Subcommand};

use crate::cli::commands::init;

mod commands;

#[derive(Parser)]
#[command(version, name = "jellyname")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
}

impl Commands {
    pub fn run(self) -> ExitCode {
        match self {
            Commands::Init => init::run(),
        }
    }
}
