use std::{fs, path::Path};

use anyhow::anyhow;

use crate::jellyname::{config::MovieData, core::generate_movie_name};

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
