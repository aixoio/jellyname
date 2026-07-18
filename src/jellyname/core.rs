use std::path::PathBuf;

use crate::jellyname::config::MovieData;

pub fn generate_movie_name(data: &MovieData) -> String {
    format!("{} ({})", data.name(), data.year())
}

pub struct EpisodeData {
    season: u16,
    number: u16,
}

pub fn extract_episodes(paths: &[PathBuf]) -> Vec<EpisodeData> {
    paths.iter().filter_map(|path| unimplemented!()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_movie_name() {
        let data = MovieData::new("movie name", 2011);

        assert_eq!(generate_movie_name(&data), "movie name (2011)");
        assert_ne!(generate_movie_name(&data), "movie name 2011");
    }
}
