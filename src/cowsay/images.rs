use std::fmt::Debug;
use crate::types::Result;
use image::{Rgb, RgbImage, ImageBuffer};
use rusttype::{Font, Scale};
use imageproc::drawing::draw_text_mut;
use ansi_colours::rgb_from_ansi256;


const BREAK: char = '\x1B';
const END: char = 'm';
const SEP: char = ';';

type Blocks = Vec<Vec<Block>>;
type Color = Rgb<u8>;
type RawColor = [u8; 3];
type Image = ImageBuffer<Color, Vec<u8>>;

pub struct Block {
    pub color: Option<Color>,
    pub char: char,
}


impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Block")
            .field("color", &self.color)
            .field("char", &self.char)
            .finish()
    }
}

// \x1B[25;12;223m -> [25, 12, 223]
// \x1B[25;223m -> [25, 223, 0]
// \x1B[223m -> [223, 0, 0]
fn chop(x: &str) -> RawColor {
    let mut result = [0, 0, 0];
    let mut i = 0;
    let clean = String::from(x)
        .replace("\x1B[", "")
        .replace("m", "");

    let split = clean.split(SEP);
    split.for_each(|num| {
        if let Ok(parsed) = num.parse::<u8>() {
            result[i] = parsed;
        }
        i += 1;
    });

    result
}

fn to_rgb(x: &RawColor) -> Option<Color> {
    let mut color_ansi = 0;
    if is_cancel(x) {
        // None = transparent / reset
        return None;
    }

    // get last non-zero
    for int in x {
        if int > &0 {
            color_ansi = *int;
        }
    }

    let result = rgb_from_ansi256(color_ansi);
    Some(Rgb([result.0, result.1, result.2]))
}

fn is_cancel(x: &RawColor) -> bool {
    return x[0] == 49 && x[1] == 0 && x[2] == 0;
}

fn parse(data: &str, last_color: &Option<Color>) -> Vec<Block> {
    let mut tmp = String::new();
    let mut blocks = Vec::new();
    let mut color = last_color.clone();

    data.chars().for_each(|c| {
        if c == BREAK {
            tmp.push(c);
        } else if tmp.len() > 0 {
            if c == END {
                color = to_rgb(&chop(tmp.as_str()));
                tmp.clear();
            } else {
                tmp.push(c);
            }
        } else {
            blocks.push(Block{
                char: c,
                color: color.clone()
            });
        }
    });

    blocks
}

fn ansi_to_blocks(x: &String) -> Blocks {
    let mut result = Vec::<Vec<Block>>::new();
    let mut last_color = None;

    x.lines().for_each(|ln| {
        let row = parse(ln, &last_color);
        let last = row.last();
        if let Some(block) = last {
            last_color = block.color.clone();
        }
        result.push(row);
    });

    result
}

// (w, h)
fn get_size(font_height: &u32, data: &Blocks) -> (u32, u32) {
    let h = data.len() as u32;
    let mut longest_line: u32 = 0;
    for line in data {
        let len = line.len() as u32;
        if len > longest_line {
            longest_line = len;
        }
    }
    let height = (h * font_height) as u32;
    let width = (longest_line * (font_height / 2) - 1) as u32; 

    return (width, height);
}

fn to_image(data: &Blocks) -> Result<Image> {
    let font_height: i32 = 20;
    let (w, h) = get_size(&(font_height as u32), data);
    let mut image = RgbImage::new(w, h);
    let font = Vec::from(include_bytes!("JetBrainsMono.ttf") as &[u8]);
    let font = Font::try_from_vec(font).ok_or("Failed to get font family, how did this happen?")?;

    let scale = Scale {
        x: font_height as f32,
        y: font_height as f32,
    };

    let mut y = 1;
    for line in data {
        let mut x = 0;
        if y % 2 == 0 {
            y += 1;
            continue;
        }
        for block in line {
            // NOTE(dylhack): default is white for plain-text characters. (ie the cowsay bubble)
            let color = block.color.unwrap_or(Rgb([255, 255, 255]));
            let mut char = block.char;
            if char == ' ' && block.color.is_some() {
                char = '█';
            }
            draw_text_mut(&mut image, color, x, y, scale, &font, &String::from(char));
            x += (font_height / 2) - 1;
        }
        y += font_height;
    }

    Ok(image)
}

pub fn cowsay_to_image(data: &String) -> Result<Image> {
    let res = ansi_to_blocks(&data);
    to_image(&res)
}
