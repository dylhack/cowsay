mod tmp;
use anyhow::{Result, anyhow};
use charasay::{Chara, format_character};
use image::{ImageBuffer, Rgba};
use std::path::PathBuf;
use base64::Engine;
use crate::{database::{cowfiles::get_cowfile, Client, previews::save_preview as save_preview_db}, proto::cowfiles::Cowfile};


pub type SavablePreview = ImageBuffer<Rgba<u8>, Vec<u8>>;
const MESSAGE: &str = "Hello, world!";

fn decode_base64(data: &String) -> Result<String> {
    let decoded = base64::engine::general_purpose::STANDARD.decode(data)?;
    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}

fn render(cowfile: &Cowfile) -> Result<SavablePreview> {
    let chara = Chara::Raw(decode_base64(&cowfile.data)?);
    let result = format_character(MESSAGE, &chara, 80, charasay::bubbles::BubbleType::Round).map_err(|err| anyhow!("{}", err.to_string()))?;
    let font = include_bytes!("../../../assets/font/JetBrainsMonoNerdFont-Regular.ttf").to_vec();
    let bold_font = include_bytes!("../../../assets/font/JetBrainsMonoNerdFont-Bold.ttf").to_vec();

    let image = cowparse::ImageBuilder::from(&result)
        .set_font(font)
        .set_bubble_font(bold_font)
        .build()
        .map_err(|err| anyhow!("{}", err.to_string()))?;

    Ok(image)
}

pub async fn get_preview(pool: &Client, cid: &str) -> Result<Vec<u8>> {
    let path = tmp::get_path(&format!("{}.webp", cid))?;
    if path.exists() {
        let bytes = std::fs::read(path)?;
        return Ok(bytes);
    }

    let cowfile = get_cowfile(pool, cid).await?;
    let image = render(&cowfile)?;
    save_preview(cid, &image).await?;
    Ok(image.to_vec())
}

pub async fn save_preview(cid: &str, image: &SavablePreview) -> Result<PathBuf> {
    let path = tmp::get_path(&format!("{}.webp", cid))?;
    image.save(&path)?;
    Ok(path)
}

pub async fn gen_preview(pool: &Client, cowfile: &Cowfile) -> Result<Vec<u8>> {
    let cid = cowfile.id.clone();
    let image = render(&cowfile).unwrap();
    let path = save_preview(&cid, &image).await?;
    save_preview_db(&pool, &cid, &path.to_str().ok_or(anyhow!("Failed to convert path to string."))?.to_string()).await?;
    Ok(image.to_vec())
}
