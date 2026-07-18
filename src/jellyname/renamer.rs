use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use anyhow::anyhow;

use crate::jellyname::{
    config::{Episode, MovieData, SeriesData},
    core::{
        generate_episode_name, generate_movie_name, generate_season_name, generate_series_name,
    },
};

/// creates and moves movie into a new folder matching jellyfin spec
pub fn rename_movie(data: &MovieData, path: &str) -> anyhow::Result<()> {
    let name = generate_movie_name(data);
    let dest_folder = Path::new(&name);
    let source = Path::new(path);

    fs::create_dir_all(dest_folder)?;

    let Some(ext) = source.extension().and_then(|ext| ext.to_str()) else {
        return Err(anyhow!("source file is missing"));
    };

    let dest = format!("{name}.{ext}");
    let dest = dest_folder.join(dest);

    fs::rename(source, dest)?;

    Ok(())
}

/// creates and moves files into new folders matching jellyfin spec
pub fn rename_series(data: &SeriesData, episodes: &[Episode]) -> anyhow::Result<()> {
    let name = generate_series_name(data);
    let dest_folder = Path::new(&name);

    fs::create_dir_all(dest_folder)?;

    for season in episodes
        .iter()
        .filter(|e| !e.ignore)
        .map(|e| e.season)
        .collect::<HashSet<_>>()
        .into_iter()
    {
        let dest = generate_season_name(season);
        let dest = dest_folder.join(dest);

        fs::create_dir_all(dest)?;
    }

    for episode in episodes.iter().filter(|e| !e.ignore) {
        let season_dest = generate_season_name(episode.season);
        let season_dest = dest_folder.join(season_dest);

        let source_path = PathBuf::from(&episode.path);

        let Some(ext) = source_path.extension().and_then(|ext| ext.to_str()) else {
            return Err(anyhow!("source file is missing or has no valid extension"));
        };

        let name = generate_episode_name(data, episode);
        let edest = format!("{name}.{ext}");
        let edest = season_dest.join(edest);

        fs::rename(&episode.path, edest)?;
    }

    Ok(())
}
