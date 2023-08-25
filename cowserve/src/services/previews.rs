mod tmp;
use anyhow::Result;
use image::{ImageBuffer, Rgba};


pub type SavablePreview = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub async fn get_preview(cid: &str) -> Result<Vec<u8>> {
    let path = tmp::get_path(&format!("{}.webp", cid))?;
    let bytes = std::fs::read(path)?;
    Ok(bytes)
}

pub async fn save_preview(cid: &str, image: &SavablePreview) -> Result<String> {
    let path = tmp::get_path(&format!("{}.webp", cid))?;
    image.save(&path)?;
    Ok(path)
}
