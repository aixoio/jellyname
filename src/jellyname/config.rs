use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::getter;

pub const CONFIG_FILENAME: &str = "jellyname.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    applied: bool,
    data: ConfigData,
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
    name: String,
    year: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesData {
    name: String,
    year: u16,
}

impl Config {
    getter!(applied, bool);
    getter!(data, ConfigData);

    pub fn new(kind: &MediaType) -> Self {
        match kind {
            MediaType::Movie => Config {
                applied: false,
                data: ConfigData::Movie(MovieData::default()),
            },
            MediaType::Series => Config {
                applied: false,
                data: ConfigData::Series(SeriesData::default()),
            },
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
}

impl SeriesData {
    getter!(name, String);
    getter!(year, u16);
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
