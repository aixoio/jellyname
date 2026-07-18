use clap::Subcommand;
use std::process::ExitCode;

use owo_colors::OwoColorize;

use crate::{
    handle_error,
    jellyname::config::{self, Config, MediaType},
    print_error,
};

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

    if Config::check_config_exists() {
        print_error!("config already exists");
    }

    let config = Config::new(&kind);

    handle_error!(config.write_config());

    println!(
        "{} {}",
        "Config created in".bold(),
        config::CONFIG_FILENAME.bright_black()
    );

    ExitCode::SUCCESS
}
