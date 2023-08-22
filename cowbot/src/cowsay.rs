use crate::types::Result;
use charasay::{format_character, Chara::File};
use cowparse::ImageBuilder;
use image::RgbaImage;

pub fn cowsay(character: &str, msg: &str) -> Result<String> {
    let cow = Builtin(String::from(character));
    let result = format_character(msg, &cow, 80, charasay::bubbles::BubbleType::Round);

    if let Err(why) = result {
        Err(format!("Failed to create cowsay, {}", why))
    } else {
        Ok(result.unwrap())
    }
}

pub fn cowsay_to_image(cowsay: &str) -> Result<RgbaImage> {
    let font = include_bytes!("../assets/font/JetBrainsMonoNerdFont-Regular.ttf").to_vec();
    let bold_font = include_bytes!("../assets/font/JetBrainsMonoNerdFont-Bold.ttf").to_vec();
    let image = ImageBuilder::from(cowsay)
        .set_font(font)
        .set_bubble_font(bold_font)
        .build()?;

    Ok(image)
}
