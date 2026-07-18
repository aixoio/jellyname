use std::process::ExitCode;

use clap::Subcommand;
use owo_colors::OwoColorize;

use crate::{
    jellyname::config::{self, Config, ConfigData, Episode},
    match_error, return_error,
};

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
    println!(
        "Loading config from {}",
        config::CONFIG_FILENAME.italic().bold()
    );
    let config = match_error!(Config::read_config());
    let ConfigData::Series(data) = config.data() else {
        return_error!("config is not for series");
    };

    println!(
        "Loading data from {}",
        config::SERIES_FILENAME.italic().bold()
    );
    let mut rdr = match_error!(csv::Reader::from_path(config::SERIES_FILENAME));
    let episodes = rdr
        .deserialize()
        .collect::<Result<Vec<Episode>, csv::Error>>();
    let episodes = match_error!(episodes);

    println!("{episodes:#?}");

    ExitCode::SUCCESS
}
