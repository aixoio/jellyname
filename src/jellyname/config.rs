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
