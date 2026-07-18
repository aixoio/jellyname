use std::{path::PathBuf, process::ExitCode};

use clap::Subcommand;

#[derive(Subcommand)]
pub enum RenameSubcommand {
    Movie { path: PathBuf },
}

pub fn run(sub: RenameSubcommand) -> ExitCode {
    match sub {
        RenameSubcommand::Movie { path } => rename_movie(path),
    }
}

fn rename_movie(path: PathBuf) -> ExitCode {
    println!("{:?}", path.extension());

    ExitCode::SUCCESS
}
