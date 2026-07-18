use std::process::ExitCode;

use clap::{Parser, Subcommand};

use crate::cli::commands::{
    apply::{self, ApplySubcommand},
    init::{self, InitMediaType},
    rename::{self, RenameSubcommand},
};

mod commands;

#[derive(Parser)]
#[command(version, name = "jellyname")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[command(subcommand)]
        kind: InitMediaType,
    },
    Rename {
        #[command(subcommand)]
        subcomnmand: RenameSubcommand,
    },
    Apply {
        #[command(subcommand)]
        sub: ApplySubcommand,
    },
}

impl Commands {
    pub fn run(self) -> ExitCode {
        match self {
            Commands::Init { kind } => init::run(kind),
            Commands::Rename { subcomnmand } => rename::run(subcomnmand),
            Commands::Apply { sub } => apply::run(sub),
        }
    }
}
