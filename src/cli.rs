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
/// Prepare and rename media files using Jellyfin-compatible names.
pub struct Cli {
    /// Media operation to perform.
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
/// Operations for configuring and renaming a media library.
pub enum Commands {
    /// Create a `jellyname.toml` configuration template.
    Init {
        /// Type of media library to configure.
        #[command(subcommand)]
        kind: InitMediaType,
    },
    /// Prepare or perform a media rename operation.
    Rename {
        /// Type of media to process.
        #[command(subcommand)]
        subcomnmand: RenameSubcommand,
    },
    /// Apply a prepared rename plan.
    Apply {
        /// Type of media rename plan to apply.
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
