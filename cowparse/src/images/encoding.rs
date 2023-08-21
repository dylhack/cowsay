use crate::{parse, types::ANSIChar};
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

const VISIBLE: u8 = 255;
const BLOCK_CHAR: char = '█';
const ORIGINAL_CHAR: char = ' ';
const WHITE: Rgba<u8> = Rgba([255, 255, 255, VISIBLE]);

type Color = Rgba<u8>;
type Image = ImageBuffer<Color, Vec<u8>>;

// (w, h)
fn get_size(font_height: &u32, data: &Vec<Vec<ANSIChar>>) -> (u32, u32) {
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

fn get_color(color_opt: &Option<(u8, u8, u8)>) -> Color {
    match color_opt {
        Some(color) => Rgba([color.0, color.1, color.2, VISIBLE]),
        None => WHITE,
    }
}

/// ImageBuilder is a builder for creating an image from a cowsay string.
///
///  - It's recommended to use a bold font for the bubble font.
///
/// # Example
/// ```
/// use charasay::format_character;
///
/// let font = include_bytes!("./MyFont-Regular.ttf").to_vec();
/// let bold_font = include_bytes!("./MyFont-Bold.ttf").to_vec();
/// let cowsay = format_character("Hello world!", &Builtin(String::from("cow")), 80, BubbleType::Round).unwrap();
/// let image = ImageBuilder::from(&cowsay)
///     .set_font(font)
///     .set_bubble_font(bold_font)
///     .build()
///     .unwrap();
///
/// // Make sure to install image with png feature flag enabled.
/// image.save("./output.png").expect("Failed to save image.");
/// ```
pub struct ImageBuilder {
    // required
    data: Vec<Vec<ANSIChar>>,
    font: Option<Vec<u8>>,
    // optional
    bubble_font: Option<Vec<u8>>,
    font_size: i32,
}

impl ImageBuilder {
    pub fn from(cowsay: &str) -> ImageBuilder {
        let mut data = vec![];
        let mut line = vec![];
        for char in parse(cowsay) {
            if char.char == '\n' {
                data.push(line);
                line = vec![];
            } else {
                line.push(char);
            }
        }
        data.push(line);

        ImageBuilder {
            data,
            font: None,
            bubble_font: None,
            font_size: 20,
        }
    }

    pub fn set_font(mut self, font: Vec<u8>) -> ImageBuilder {
        self.font = Some(font);
        self
    }

    pub fn set_bubble_font(mut self, font: Vec<u8>) -> ImageBuilder {
        self.bubble_font = Some(font);
        self
    }

    pub fn set_font_size(mut self, font_size: i32) -> ImageBuilder {
        self.font_size = font_size;
        self
    }

    pub fn build(self) -> Result<Image, &'static str> {
        let (w, h) = get_size(&(self.font_size as u32), &self.data);
        let mut image = RgbaImage::new(w, h);
        let font = self.font.ok_or("Font Family not set.")?;
        let font = Font::try_from_vec(font).ok_or("Failed to set font data.")?;
        let bubble_font = if let Some(bubble_font) = self.bubble_font {
            Font::try_from_vec(bubble_font).ok_or("Failed to set bubble font data.")?
        } else {
            font.clone()
        };

        let scale = Scale {
            x: self.font_size as f32,
            y: self.font_size as f32,
        };

        let mut y = 0;
        for line in self.data {
            let mut x = 0;
            for block in line {
                // NOTE(dylhack): default is white for plain-text characters. (ie the cowsay bubble)
                let original_color = block.get_bg_color();
                let color = get_color(&original_color);
                let char = if block.char == ORIGINAL_CHAR && original_color.is_some() {
                    BLOCK_CHAR
                } else {
                    block.char
                };
                let font = if char == BLOCK_CHAR {
                    &font
                } else {
                    &bubble_font
                };
                draw_text_mut(&mut image, color, x, y, scale, &font, &String::from(char));
                x += (self.font_size / 2) - 1;
            }
            y += self.font_size;
        }

        Ok(image)
    }
}

#[cfg(test)]
mod test {
    use super::ImageBuilder;
    use charasay::{bubbles::BubbleType, format_character, Chara::Builtin};

    #[test]
    pub fn test_image() {
        let font = include_bytes!("../../test/fonts/JetBrainsMonoNerdFont-Regular.ttf").to_vec();
        let bold_font = include_bytes!("../../test/fonts/JetBrainsMonoNerdFont-Bold.ttf").to_vec();
        let cowsay = format_character(
            "Hello world!",
            &Builtin(String::from("cow")),
            80,
            BubbleType::Round,
        )
        .unwrap();
        let image = ImageBuilder::from(&cowsay)
            .set_font(font)
            .set_bubble_font(bold_font)
            .build()
            .unwrap();

        assert!(image.save("./test/test.png").is_ok());
    }
}
