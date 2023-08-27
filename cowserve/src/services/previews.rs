mod tmp;
use crate::{
    cowsay::cowsay,
    database::{previews::save_preview as save_preview_db, Client},
    proto::cowfiles::{Cowsay, GetCowsayRequest},
};
use anyhow::{anyhow, Result};
use std::path::PathBuf;
use tokio::fs;

const MESSAGE: &str = "Hello, world!";
const FONT: &[u8; 2063520] =
    include_bytes!("../../../assets/font/JetBrainsMonoNerdFont-Regular.ttf");
const BOLD_FONT: &[u8; 2067288] =
    include_bytes!("../../../assets/font/JetBrainsMonoNerdFont-Bold.ttf");

async fn render(pool: &Client, cid: &str) -> Result<Cowsay> {
    Ok(cowsay(
        pool,
        GetCowsayRequest {
            chara_id: cid.to_string(),
            bubble_font: Some(BOLD_FONT.to_vec()),
            font: FONT.to_vec(),
            message: MESSAGE.to_string(),
            no_bubble: Some(true)
        },
    )
    .await?)
}

pub async fn get_preview(pool: &Client, cid: &str) -> Result<Vec<u8>> {
    let path = tmp::get_path(&format!("{}.webp", cid))?;
    if path.exists() {
        let bytes = std::fs::read(path)?;
        return Ok(bytes);
    }

    let image = render(pool, cid).await?;
    save_preview(cid, image.clone()).await?;

    Ok(image.data)
}

pub async fn save_preview(cid: &str, image: Cowsay) -> Result<PathBuf> {
    let path = tmp::get_path(&format!("{}.webp", cid))?;
    fs::write(&path, image.data).await?;

    Ok(path)
}

pub async fn gen_preview(pool: &Client, cid: &String) -> Result<Vec<u8>> {
    let image = render(pool, cid).await?;
    let path = save_preview(&cid, image.clone()).await?;
    save_preview_db(
        &pool,
        &cid,
        &path
            .to_str()
            .ok_or(anyhow!("Failed to convert path to string."))?
            .to_string(),
    )
    .await?;

    Ok(image.data)
}
