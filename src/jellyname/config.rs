use crate::getter;

#[derive(Debug)]
pub struct Config {
    applied: bool,
    data: ConfigData,
}

#[derive(Debug)]
pub enum ConfigData {
    Movie(MovieData),
    Series(SeriesData),
}

#[derive(Debug)]
pub struct MovieData {
    name: String,
    year: u16,
}

#[derive(Debug)]
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
