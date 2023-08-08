use crate::types::Result;
use ansi_colours::rgb_from_ansi256;
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::fmt::Debug;

const START: char = '\x1B';
const END: char = 'm';
const SEP: char = ';';
const VISIBLE: u8 = 255;
const CHAR: char = '█';
const ORIGINAL_CHAR: char = ' ';
const WHITE: Rgba<u8> = Rgba([255, 255, 255, VISIBLE]);

type Blocks = Vec<Vec<Block>>;
type Color = Rgba<u8>;
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
    let clean = String::from(x).replace("\x1B[", "").replace("m", "");

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
    Some(Rgba([result.0, result.1, result.2, VISIBLE]))
}

fn is_cancel(x: &RawColor) -> bool {
    return x[0] == 49 && x[1] == 0 && x[2] == 0;
}

fn parse(data: &str, last_color: &Option<Color>) -> Vec<Block> {
    let mut tmp = String::new();
    let mut blocks = Vec::new();
    let mut color = last_color.clone();

    data.chars().for_each(|c| {
        if c == START {
            tmp.push(c);
        } else if tmp.len() > 0 {
            if c == END {
                color = to_rgb(&chop(tmp.as_str()));
                tmp.clear();
            } else {
                tmp.push(c);
            }
        } else {
            blocks.push(Block {
                char: c,
                color: color.clone(),
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
    let mut image = RgbaImage::new(w, h);
    let font = Vec::from(include_bytes!("font/JetBrainsMonoNerdFont-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font).ok_or("Failed to get font family, how did this happen?")?;

    let bold_font = Vec::from(include_bytes!("font/JetBrainsMonoNerdFont-Bold.ttf") as &[u8]);
    let bold_font =
        Font::try_from_vec(bold_font).ok_or("Failed to get font family, how did this happen?")?;

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
            let color = block.color.unwrap_or(WHITE);
            let char = if block.char == ORIGINAL_CHAR && block.color.is_some() {
                CHAR
            } else {
                block.char
            };
            let font = if char == CHAR { &font } else { &bold_font };
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

mod tests {
    use super::*;
    use crate::tmp::get_path;

    #[test]
    fn test_chop() {
        let input = "\x1B[25;12;223m";
        let result = chop(input);
        assert_eq!(result, [25, 12, 223]);
    }

    #[test]
    fn test_chop_2() {
        let input = "\x1B[25;223m";
        let result = chop(input);
        assert_eq!(result, [25, 223, 0]);
    }

    #[test]
    fn test_chop_3() {
        let input = "\x1B[223m";
        let result = chop(input);
        assert_eq!(result, [223, 0, 0]);
    }

    #[test]
    fn test_ansi_to_blocks() {
        let input = String::from("\x1B[25;223mhell\x1B[49;0;0mo\n\x1B[223mworld");
        let result = ansi_to_blocks(&input);
        let answer_color = Rgba([255, 215, 175, VISIBLE]);
        let answer = vec![
            vec![
                Block {
                    char: 'h',
                    color: Some(answer_color.clone()),
                },
                Block {
                    char: 'e',
                    color: Some(answer_color.clone()),
                },
                Block {
                    char: 'l',
                    color: Some(answer_color.clone()),
                },
                Block {
                    char: 'l',
                    color: Some(answer_color.clone()),
                },
                Block {
                    char: 'o',
                    color: None,
                },
            ],
            vec![
                Block {
                    char: 'w',
                    color: Some(answer_color.clone()),
                },
                Block {
                    char: 'o',
                    color: Some(answer_color.clone()),
                },
                Block {
                    char: 'r',
                    color: Some(answer_color.clone()),
                },
                Block {
                    char: 'l',
                    color: Some(answer_color.clone()),
                },
                Block {
                    char: 'd',
                    color: Some(answer_color.clone()),
                },
            ],
        ];

        for (i, line) in result.iter().enumerate() {
            for (j, block) in line.iter().enumerate() {
                let answer = &answer[i][j];
                println!("{:?} {:?}", block, answer);
                assert_eq!(block.char, answer.char);
                // assert color
                if let Some(acolor) = answer.color {
                    let result_color = block.color.unwrap();
                    assert_eq!(result_color[0], acolor[0]);
                    assert_eq!(result_color[1], acolor[1]);
                    assert_eq!(result_color[2], acolor[2]);
                    assert_eq!(result_color[3], acolor[3]);
                } else {
                    assert!(block.color.is_none() && answer.color.is_none());
                }
            }
        }
    }

    #[test]
    fn test_cowsay_to_image() {
        let input = String::from("\x1B[25;223mhell\x1B[49;0;0mo\n\x1B[223mworld");
        let result = cowsay_to_image(&input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_saving() {
        let input = String::from("\x1B[25;223mhell\x1B[49;0;0mo\n\x1B[223mworld");
        let image = cowsay_to_image(&input).unwrap();
        let file_path = get_path(&format!("{}.webp", "test")).unwrap();
        assert!(image.save(file_path).is_ok());
    }
}
