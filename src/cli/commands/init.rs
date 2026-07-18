use clap::Subcommand;
use std::process::ExitCode;

use crate::jellyname::config::MediaType;

#[derive(Subcommand)]
pub enum InitMediaType {
    Movie,
    Series,
}

impl InitMediaType {
    pub fn convert(self) -> MediaType {
        match self {
            InitMediaType::Movie => MediaType::Movie,
            InitMediaType::Series => MediaType::Series,
        }
    }
}

pub fn run(kind: InitMediaType) -> ExitCode {
    let kind = kind.convert();

    println!("Init command");
    println!("{kind:?}");

    ExitCode::SUCCESS
}
