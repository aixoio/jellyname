use std::{path::PathBuf, process::ExitCode};

use clap::Subcommand;

#[derive(Subcommand)]
pub enum ApplySubcommand {
    Series { path: PathBuf },
}

pub fn run(sub: ApplySubcommand) -> ExitCode {
    match sub {
        ApplySubcommand::Series { path } => apply_series(path),
    }
}

fn apply_series(path: PathBuf) -> ExitCode {
    println!("{:?}", path);

    ExitCode::SUCCESS
}
