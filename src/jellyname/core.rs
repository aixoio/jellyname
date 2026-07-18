use std::path::PathBuf;

use regex::regex;

use crate::jellyname::config::{self, MovieData};

pub fn generate_movie_name(data: &MovieData) -> String {
    format!("{} ({})", data.name(), data.year())
}

#[derive(Debug, PartialEq, Eq)]
pub struct EpisodeData {
    pub season: u16,
    pub episode: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Episode {
    pub filename: String,
    pub data: EpisodeData,
}

#[inline]
pub fn convert_episode_to_config(e: Episode) -> config::Episode {
    config::Episode {
        filename: e.filename,
        season: e.data.season,
        episode: e.data.episode,
        ignore: false,
    }
}

pub fn extract_episodes(paths: &[PathBuf]) -> impl Iterator<Item = Episode> {
    paths.iter().filter_map(|path| {
        let data = extract_episode(&path.to_string_lossy())?;
        let filename = path.file_name()?.to_string_lossy().to_string();

        Some(Episode { filename, data })
    })
}

pub fn extract_episode(filename: &str) -> Option<EpisodeData> {
    let re = regex!(r"(?i)S[0-9]+E[0-9]+"); // matches `S01E01` from a string
    let cap = re.find(filename)?;
    if cap.is_empty() {
        return None;
    }

    let cap = cap.as_str().to_uppercase();
    let cap = cap.split_once('E')?;
    let Ok(season) = cap.0[1..].parse::<u16>() else {
        return None;
    };
    let Ok(episode) = cap.1.parse::<u16>() else {
        return None;
    };

    Some(EpisodeData { season, episode })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn episode(filename: &str, season: u16, episode: u16) -> Episode {
        Episode {
            filename: filename.to_string(),
            data: EpisodeData { season, episode },
        }
    }

    #[test]
    fn extracts_episodes_from_valid_paths() {
        let paths = vec![
            PathBuf::from("random.series.S01E04.mkv"),
            PathBuf::from("another.series.S02E09.mp4"),
        ];

        let episodes: Vec<Episode> = extract_episodes(&paths).collect();

        assert_eq!(
            episodes,
            vec![
                episode("random.series.S01E04.mkv", 1, 4),
                episode("another.series.S02E09.mp4", 2, 9),
            ]
        );
    }

    #[test]
    fn skips_paths_without_episode_information() {
        let paths = vec![
            PathBuf::from("random.series.S01E04.mkv"),
            PathBuf::from("movie.2024.1080p.mkv"),
            PathBuf::from("another.series.S03E12.mp4"),
        ];

        let episodes: Vec<Episode> = extract_episodes(&paths).collect();

        assert_eq!(
            episodes,
            vec![
                episode("random.series.S01E04.mkv", 1, 4),
                episode("another.series.S03E12.mp4", 3, 12),
            ]
        );
    }

    #[test]
    fn returns_empty_iterator_when_no_paths_match() {
        let paths = vec![
            PathBuf::from("movie.mkv"),
            PathBuf::from("random.series.mkv"),
            PathBuf::from("video.2024.mp4"),
        ];

        let episodes: Vec<Episode> = extract_episodes(&paths).collect();

        assert!(episodes.is_empty());
    }

    #[test]
    fn returns_empty_iterator_for_empty_input() {
        let paths: Vec<PathBuf> = Vec::new();

        let episodes: Vec<Episode> = extract_episodes(&paths).collect();

        assert!(episodes.is_empty());
    }

    #[test]
    fn preserves_the_order_of_matching_paths() {
        let paths = vec![
            PathBuf::from("series.S04E10.mkv"),
            PathBuf::from("not-an-episode.mkv"),
            PathBuf::from("series.S01E02.mkv"),
            PathBuf::from("series.S03E07.mkv"),
        ];

        let episodes: Vec<Episode> = extract_episodes(&paths).collect();

        assert_eq!(
            episodes,
            vec![
                episode("series.S04E10.mkv", 4, 10),
                episode("series.S01E02.mkv", 1, 2),
                episode("series.S03E07.mkv", 3, 7),
            ]
        );
    }

    #[test]
    fn extracts_episodes_from_nested_paths() {
        let paths = vec![
            PathBuf::from("/media/tv/Random Series/Season 01/random.series.S01E04.mkv"),
            PathBuf::from("/media/tv/Another Series/Season 12/another.series.S12E103.mkv"),
        ];

        let episodes: Vec<Episode> = extract_episodes(&paths).collect();

        assert_eq!(
            episodes,
            vec![
                episode("random.series.S01E04.mkv", 1, 4),
                episode("another.series.S12E103.mkv", 12, 103),
            ]
        );
    }

    #[test]
    fn can_consume_the_iterator_lazily() {
        let paths = vec![
            PathBuf::from("invalid.mkv"),
            PathBuf::from("series.S01E04.mkv"),
            PathBuf::from("series.S01E05.mkv"),
        ];

        let mut episodes = extract_episodes(&paths);

        assert_eq!(episodes.next(), Some(episode("series.S01E04.mkv", 1, 4)));

        assert_eq!(episodes.next(), Some(episode("series.S01E05.mkv", 1, 5)));

        assert_eq!(episodes.next(), None);
    }

    #[test]
    fn test_generate_movie_name_empty() {
        let data = MovieData::new("", 0);

        assert_eq!(generate_movie_name(&data), " (0)");
        assert_ne!(generate_movie_name(&data), "");
    }

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
