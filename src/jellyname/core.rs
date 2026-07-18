use std::path::PathBuf;

use crate::jellyname::config::MovieData;

pub fn generate_movie_name(data: &MovieData) -> String {
    format!("{} ({})", data.name(), data.year())
}

#[derive(Debug, PartialEq, Eq)]
pub struct EpisodeData {
    season: u16,
    episode: u16,
}

pub fn extract_episodes(paths: &[PathBuf]) -> Vec<EpisodeData> {
    paths.iter().filter_map(|path| unimplemented!()).collect()
}

pub fn extract_episode(filename: &str) -> Option<EpisodeData> {
    None
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

    #[test]
    fn extracts_episode_from_standard_filename() {
        let result = extract_episode("random.series.S01E04.mkv");

        assert_eq!(
            result,
            Some(EpisodeData {
                season: 1,
                episode: 4,
            })
        );
    }

    #[test]
    fn extracts_episode_case_insensitively() {
        let result = extract_episode("Random.Series.s02e09.mp4");

        assert_eq!(
            result,
            Some(EpisodeData {
                season: 2,
                episode: 9,
            })
        );
    }

    #[test]
    fn extracts_episode_from_full_path() {
        let result = extract_episode("/media/tv/Random Series/Season 03/random.series.S03E12.mkv");

        assert_eq!(
            result,
            Some(EpisodeData {
                season: 3,
                episode: 12,
            })
        );
    }

    #[test]
    fn extracts_episode_when_surrounded_by_extra_metadata() {
        let result = extract_episode("random.series.2024.S04E07.1080p.WEB-DL.x264.mkv");

        assert_eq!(
            result,
            Some(EpisodeData {
                season: 4,
                episode: 7,
            })
        );
    }

    #[test]
    fn supports_more_than_two_digits() {
        let result = extract_episode("long.running.series.S12E103.mkv");

        assert_eq!(
            result,
            Some(EpisodeData {
                season: 12,
                episode: 103,
            })
        );
    }

    #[test]
    fn returns_none_when_episode_marker_is_missing() {
        assert_eq!(extract_episode("random.series.mkv"), None);
    }

    #[test]
    fn returns_none_when_season_number_is_missing() {
        assert_eq!(extract_episode("random.series.SE04.mkv"), None);
    }

    #[test]
    fn returns_none_when_episode_number_is_missing() {
        assert_eq!(extract_episode("random.series.S01E.mkv"), None);
    }

    #[test]
    fn returns_none_for_reversed_episode_format() {
        assert_eq!(extract_episode("random.series.E04S01.mkv"), None);
    }

    #[test]
    fn returns_none_for_unrelated_numbers() {
        assert_eq!(extract_episode("random.series.2024.1080p.mkv"), None);
    }

    #[test]
    fn returns_none_for_empty_filename() {
        assert_eq!(extract_episode(""), None);
    }

    #[test]
    fn extracts_first_episode_when_multiple_markers_exist() {
        let result = extract_episode("random.series.S01E04.S01E05.mkv");

        assert_eq!(
            result,
            Some(EpisodeData {
                season: 1,
                episode: 4,
            })
        );
    }
}
