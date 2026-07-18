use serde::{Deserialize, Serialize};

use crate::getter;

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
