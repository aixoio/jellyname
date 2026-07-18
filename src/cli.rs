use clap::{Parser, Subcommand};

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
