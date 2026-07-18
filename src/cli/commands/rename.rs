use std::{path::PathBuf, process::ExitCode};

use clap::Subcommand;
use owo_colors::OwoColorize;

use crate::{
    handle_error,
    jellyname::{
        config::{self, Config, ConfigData},
        core::{convert_episode_to_config, extract_episodes},
        descover::descover_series,
        renamer,
    },
    match_error, return_error,
};

#[derive(Subcommand)]
pub enum RenameSubcommand {
    Movie { path: PathBuf },
    Series { path: PathBuf },
}

pub fn run(sub: RenameSubcommand) -> ExitCode {
    match sub {
        RenameSubcommand::Movie { path } => rename_movie(path),
        RenameSubcommand::Series { path } => rename_series(path),
    }
}

fn rename_movie(path: PathBuf) -> ExitCode {
    println!(
        "Loading config from {}",
        config::CONFIG_FILENAME.italic().bold()
    );
    let config = match_error!(Config::read_config());
    let ConfigData::Movie(data) = config.data() else {
        return_error!("config is not for movies");
    };
    println!();

    println!("{}", "Movie".bold().blue());
    println!("  {}: {}", "Name".cyan(), data.name());
    println!("  {}: {}", "Year".cyan(), data.year());

    println!();

    println!("Renaming...");

    handle_error!(renamer::rename_movie(data, &path.to_string_lossy()));

    println!("{}", "Done!".bright_green().bold());

    ExitCode::SUCCESS
}

fn rename_series(path: PathBuf) -> ExitCode {
    println!(
        "Loading config from {}",
        config::CONFIG_FILENAME.italic().bold()
    );
    let config = match_error!(Config::read_config());
    let ConfigData::Series(data) = config.data() else {
        return_error!("config is not for series");
    };
    println!();

    println!("{}", "Series".bold().blue());
    println!("  {}: {}", "Name".cyan(), data.name());
    println!("  {}: {}", "Year".cyan(), data.year());

    println!();

    println!("Walking...");

    let Some(path) = path.to_str() else {
        return_error!("missing path");
    };
    let paths = match_error!(descover_series(path, &config));

    let episodes: Vec<_> = extract_episodes(&paths)
        .map(convert_episode_to_config)
        .collect();

    println!("{episodes:#?}");

    ExitCode::SUCCESS
}
