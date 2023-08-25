use std::{
    fmt::Display,
    fs::{create_dir, read_dir}
};
use anyhow::{anyhow, Result};
use crate::config::get_previews_path;


pub fn get_path<T: Display>(file_name: &T) -> Result<String> {
    let root = get_previews_path();
    println!("Root: {}", root);
    if read_dir(&root).is_err()  {
        if let Err(why) = create_dir(&root) {
            return Err(anyhow!("Failed to create root dir {}", why));
        }
    }

    Ok(format!("{root}/{file_name}"))
}
