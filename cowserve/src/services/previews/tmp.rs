use std::{
    fs::create_dir,
    path::{Path, PathBuf},
};
use anyhow::{anyhow, Result};
use crate::config::get_previews_path;


pub fn get_path(filename: &str) -> Result<PathBuf> {
    let env_dir = get_previews_path();
    let root = Path::new(&env_dir);
    if !root.exists() {
        if let Err(why) = create_dir(&root) {
            return Err(anyhow!("Failed to create root dir {}", why));
        }
    }

   Ok(root.join(Path::new(filename)))
}
