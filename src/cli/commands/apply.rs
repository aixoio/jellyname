use std::process::ExitCode;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum ApplySubcommand {
    Series,
}

pub fn run(sub: ApplySubcommand) -> ExitCode {
    match sub {
        ApplySubcommand::Series => apply_series(),
    }
}

fn apply_series() -> ExitCode {
    println!("applying...");

    ExitCode::SUCCESS
}
