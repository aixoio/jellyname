use clap::Subcommand;
use std::process::ExitCode;

use owo_colors::OwoColorize;

use crate::{
    handle_error,
    jellyname::config::{self, Config, MediaType},
    return_error,
};

#[derive(Subcommand)]
/// Media library types supported by configuration templates.
pub enum InitMediaType {
    /// Create a configuration template for a movie.
    Movie,
    /// Create a configuration template for a television series.
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
        return_error!("config already exists");
    }

    let config = Config::new(&kind);

    handle_error!(config.write_config());

    println!(
        "Config created in {}",
        config::CONFIG_FILENAME.italic().bold()
    );

    ExitCode::SUCCESS
}
