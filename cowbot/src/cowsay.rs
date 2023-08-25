use anyhow::{anyhow, Result};
use charasay::{format_character, Chara};
use cowparse::ImageBuilder;
use image::RgbaImage;

pub fn cowsay(cowfile: &Chara, msg: &str) -> Result<String> {
    let result = format_character(msg, &cowfile, 80, charasay::bubbles::BubbleType::Round);

    if let Err(why) = result {
        Err(anyhow!("Failed to create cowsay, {}", why))
    } else {
        Ok(result.unwrap())
    }
}

pub fn cowsay_to_image(cowsay: &str) -> Result<RgbaImage> {
    let font = include_bytes!("../../assets/font/JetBrainsMonoNerdFont-Regular.ttf").to_vec();
    let bold_font = include_bytes!("../../assets/font/JetBrainsMonoNerdFont-Bold.ttf").to_vec();
    let image = ImageBuilder::from(cowsay)
        .set_font(font)
        .set_bubble_font(bold_font)
        .build()
        .expect("Failed to create cowsay image. How did this happen?");

    Ok(image)
}
