mod tmp;
use super::cowsay::cowsay;
use crate::{
    cowfiles::{CowsayData, GetCowsayRequest},
    database::{previews::save_preview as save_preview_db, Client},
};
use anyhow::Result;
use uriparse::{URI, URIBuilder, Path};
use tokio::fs;

const FONT: &[u8; 2063520] =
    include_bytes!("../../../assets/font/JetBrainsMonoNerdFont-Regular.ttf");
const BOLD_FONT: &[u8; 2067288] =
    include_bytes!("../../../assets/font/JetBrainsMonoNerdFont-Bold.ttf");

async fn render(pool: &Client, cid: &str) -> Result<CowsayData> {
    Ok(cowsay(
        pool,
        GetCowsayRequest {
            chara_id: cid.to_string(),
            bubble_font: Some(BOLD_FONT.to_vec()),
            font: FONT.to_vec(),
            message: "".to_string(),
            no_bubble: Some(true),
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
    save_preview(cid, &image).await?;

    Ok(image.data)
}

async fn save_preview_fs<'uri>(cid: &str, image: &CowsayData) -> Result<String> {
    let path = tmp::get_path(&format!("{}.webp", cid))?;
    fs::write(&path, &image.data).await?;
    let path = Path::try_from(path.to_str().unwrap())?;
    let path = URIBuilder::new()
        .with_scheme(uriparse::Scheme::File)
        .with_path(path)
        .build()?;
    
    Ok(path.to_string())
}

pub async fn save_preview<'uri>(cid: &str, image: &CowsayData) -> Result<String> {
    save_preview_fs(cid, image).await
}

pub async fn gen_preview(pool: &Client, cid: &String) -> Result<Vec<u8>> {
    let image = render(pool, cid).await?;
    let path = save_preview(&cid, &image).await?;

    save_preview_db(
        &pool,
        &cid,
        &path
    )
    .await?;

    Ok(image.data)
}
