use crate::types::Result;
use std::{
    fmt::Display,
    fs::{create_dir, read_dir},
};

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

pub fn get_path<T: Display>(file_name: &T) -> Result<String> {
    if !exists() {
        if let Err(why) = init() {
            return Err(format!("Failed to create root dir {}", why));
        }
    }

    Ok(format!("{ROOT}/{file_name}"))
}

mod test {
    use super::*;

    #[test]
    fn test_get_path() {
        let file_name = "test.txt";
        let path = get_path(&file_name).unwrap();
        assert_eq!(path, format!("{ROOT}/{file_name}"));
    }
}
