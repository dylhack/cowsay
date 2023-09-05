use std::io::Cursor;

use anyhow::{anyhow, Result};
use base64::Engine;
use charasay::{format_character, Chara};
use image::ImageFormat;

use crate::{
    database::{cowdata::get_cowdata, Client},
    proto::cowfiles::{CowsayData, GetCowsayRequest},
};

fn decode_base64(data: &String) -> Result<String> {
    let decoded = base64::engine::general_purpose::STANDARD.decode(data)?;
    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}

pub async fn cowsay(pool: &Client, opt: GetCowsayRequest) -> Result<CowsayData> {
    let cowdata = get_cowdata(pool, &opt.chara_id).await?;
    let chara = Chara::Raw(decode_base64(&cowdata)?);
    let result = format_character(
        &opt.message,
        &chara,
        80,
        charasay::bubbles::BubbleType::Round,
    )
    .map_err(|err| anyhow!("{}", err.to_string()))?;

    let mut builder = cowparse::ImageBuilder::from(&result).set_font(opt.font);

    if let Some(bubble_font) = opt.bubble_font {
        builder = builder.set_bubble_font(bubble_font);
    }

    if let Some(no_bubble) = opt.no_bubble {
        builder = builder.set_no_bubble(no_bubble);
    }

    let image = builder
        .build()
        .map_err(|err| anyhow!("{}", err.to_string()))?;

    let mut data: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut data), ImageFormat::WebP)?;

    Ok(CowsayData { data })
}
