use crate::jellyname::config::MovieData;

pub fn generate_movie_name(data: &MovieData) -> String {
    format!("{} ({})", data.name(), data.year())
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
