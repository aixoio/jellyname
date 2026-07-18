use std::{fs, path::Path, vec};

use serde::{Deserialize, Serialize};

use crate::getter;

pub const CONFIG_FILENAME: &str = "jellyname.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub targets: Vec<String>,
    pub data: ConfigData,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigData {
    Movie(MovieData),
    Series(SeriesData),
}

#[derive(Debug)]
pub enum MediaType {
    Movie,
    Series,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieData {
    pub name: String,
    pub year: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesData {
    pub name: String,
    pub year: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub filename: String,
    pub season: u16,
    pub episode: u16,
    pub ignore: bool,
}

impl Config {
    getter!(targets, Vec<String>);
    getter!(data, ConfigData);

    pub fn new(kind: &MediaType) -> Self {
        let data = match kind {
            MediaType::Movie => ConfigData::Movie(MovieData::default()),
            MediaType::Series => ConfigData::Series(SeriesData::default()),
        };

        Self {
            targets: ["mkv", "mp4", "mov"].map(String::from).to_vec(),
            data,
        }
    }

    pub fn check_config_exists() -> bool {
        Path::new(CONFIG_FILENAME).exists()
    }

    pub fn read_config() -> anyhow::Result<Self> {
        let contents = fs::read_to_string(CONFIG_FILENAME)?;

        Ok(toml::from_str(&contents)?)
    }

    pub fn write_config(&self) -> anyhow::Result<()> {
        let contents = toml::to_string_pretty(self)?;

        fs::write(CONFIG_FILENAME, contents)?;

        Ok(())
    }
}

impl MovieData {
    getter!(name, String);
    getter!(year, u16);

    pub fn new(name: impl Into<String>, year: u16) -> Self {
        Self {
            name: name.into(),
            year,
        }
    }
}

impl SeriesData {
    getter!(name, String);
    getter!(year, u16);

    pub fn new(name: impl Into<String>, year: u16) -> Self {
        Self {
            name: name.into(),
            year,
        }
    }
}

impl Default for MovieData {
    fn default() -> Self {
        Self {
            name: "[DEFAULT]".to_string(),
            year: 0,
        }
    }
}

impl Default for SeriesData {
    fn default() -> Self {
        Self {
            name: "[DEFAULT]".to_string(),
            year: 0,
        }
    }
}
