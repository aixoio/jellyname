use std::path::PathBuf;

use walkdir::WalkDir;

use crate::jellyname::config::Config;

pub fn descover_series(path: &str, config: &Config) -> anyhow::Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = vec![];

    for entry in WalkDir::new(path) {
        let entry = entry?;

        if !entry.file_type().is_file() {
            continue;
        }

        let Some(ext) = entry.path().extension().and_then(|ext| ext.to_str()) else {
            continue;
        };
        let ext = ext.to_lowercase();

        if !config.targets().iter().any(|t| t.to_lowercase() == ext) {
            continue;
        }

        paths.push(entry.into_path());
    }

    Ok(paths)
}
