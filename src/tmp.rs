use std::{
    fs::{create_dir, read_dir},
    fmt::Display,
};
use crate::types::Result;

const ROOT: &str = "./tmp";

fn init() -> std::io::Result<()> {
    create_dir(ROOT)
}

fn exists() -> bool {
    if let Err(_) = read_dir(ROOT) {
        return false;
    }
    return true;
}

pub fn get_path<T : Display>(file_name: &T) -> Result<String> {
    if !exists() {
        if let Err(why) = init() {
            return Err(format!("Failed to create root dir {}", why));
        }
    }

    Ok(format!("{ROOT}/{file_name}"))
}
