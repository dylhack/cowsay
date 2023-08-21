use super::{sgr, types::ANSIChar};
use ansi_colours::rgb_from_ansi256;

type Color = (u8, u8, u8);

const BLACK: Color = (0, 0, 0);
const RED: Color = (255, 0, 0);
const GREEN: Color = (0, 255, 0);
const YELLOW: Color = (255, 255, 0);
const CYAN: Color = (0, 255, 255);
const BLUE: Color = (0, 0, 255);
const MAGENTA: Color = (255, 0, 255);
const WHITE: Color = (255, 255, 255);

macro_rules! is_method {
    ($name:ident, $control:ident) => {
        pub fn $name(&self) -> bool {
            self.is(&sgr::$control)
        }
    };
}

impl ANSIChar {
    // [38, 2, 255, 255, 255] -> (255, 255, 255)
    // [38, 5, 15] -> (255, 255, 255)
    fn parse_color(slice: &[u8]) -> Option<Color> {
        match slice.get(1) {
            Some(color_type) => match color_type {
                &sgr::T416_RGB => {
                    let r = slice.get(2)?.clone();
                    let g = slice.get(3)?.clone();
                    let b = slice.get(4)?.clone();
                    Some((r, g, b))
                }
                &sgr::T416_256 => {
                    let color = slice.get(2)?;
                    Some(rgb_from_ansi256(*color))
                }
                _ => None,
            },
            None => None,
        }
    }

    fn get_color(&self, is_fg: bool) -> Option<Color> {
        let mut result: Option<Color> = None;
        self.for_each(|escape, args| {
            if is_fg {
                result = match escape {
                    &sgr::NORMAL => None,
                    &sgr::FOREGROUND => {
                        ANSIChar::parse_color(&[38, args[0], args[1], args[2], args[3]])
                    }
                    &sgr::RED_FG => Some(RED),
                    &sgr::GREEN_FG => Some(GREEN),
                    &sgr::YELLOW_FG => Some(YELLOW),
                    &sgr::BLUE_FG => Some(BLUE),
                    &sgr::CYAN_FG => Some(CYAN),
                    &sgr::WHITE_FG => Some(WHITE),
                    &sgr::BLACK_FG => Some(BLACK),
                    &sgr::MAGENTA_FG => Some(MAGENTA),
                    _ => result,
                };
            } else {
                result = match escape {
                    &sgr::NORMAL => None,
                    &sgr::BACKGROUND => {
                        ANSIChar::parse_color(&[48, args[0], args[1], args[2], args[3]])
                    }
                    &sgr::RED_BG => Some(RED),
                    &sgr::GREEN_BG => Some(GREEN),
                    &sgr::YELLOW_BG => Some(YELLOW),
                    &sgr::BLUE_BG => Some(BLUE),
                    &sgr::MAGENTA_BG => Some(MAGENTA),
                    &sgr::CYAN_BG => Some(CYAN),
                    &sgr::WHITE_BG => Some(WHITE),
                    &sgr::BLACK_BG => Some(BLACK),
                    &sgr::DEFAULT_BG => None,
                    _ => result,
                };
            }
        });

        result
    }

    pub fn get_fg_color(&self) -> Option<Color> {
        self.get_color(true)
    }

    pub fn get_bg_color(&self) -> Option<Color> {
        self.get_color(false)
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&u8, [u8; 4]),
    {
        self.control.iter().for_each(|ctrl| {
            f(&ctrl.escape, ctrl.params);
        });
    }

    // is(&sgr::NORMAL)
    pub fn is(&self, control: &u8) -> bool {
        let mut result = false;
        self.for_each(|escape, _| {
            if escape == control {
                result = true;
            } else if escape == &sgr::NORMAL {
                result = false;
            }
        });
        result
    }

    is_method!(is_normal, NORMAL);
    is_method!(is_bold, BOLD);
    is_method!(is_faint, FAINT);
    is_method!(is_italic, ITALIC);
    is_method!(is_underline, UNDERSCORE);
    is_method!(is_reverse, REVERSE);
    is_method!(is_conceal, CONCEAL);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ansi::parse;

    macro_rules! create_result {
        ($data:expr) => {
            &parse($data)[0]
        };
    }

    #[test]
    fn test_is_method() {
        let answer = true;
        let result = create_result!("\x1B[3;1mHello world");
        assert_eq!(result.is_bold(), answer);
    }

    #[test]
    fn test_is_method_2() {
        let answer = false;
        let result = create_result!("\x1B[1;0;3mHello world");
        assert_eq!(result.is_bold(), answer);
    }

    // Color
    #[test]
    pub fn test_color() {
        let answer = Some(GREEN);
        let result = create_result!("\x1B[38;2;197;108;53;32;3mHello world");
        assert_eq!(result.get_fg_color(), answer);
    }

    #[test]
    pub fn test_color_2() {
        let answer = None;
        let result = create_result!("\x1B[42;3mHello world");
        assert_eq!(result.get_fg_color(), answer);
    }

    #[test]
    pub fn test_color_3() {
        let answer = None;
        let result = create_result!("\x1B[38;2;197;108;53;32;0mHello world");
        assert_eq!(result.get_fg_color(), answer);
    }

    #[test]
    pub fn test_color_rgb() {
        let answer: Option<Color> = Some((197, 108, 53));
        let result = create_result!("\x1B[38;2;197;108;53mHello world");
        assert_eq!(result.get_fg_color(), answer);
    }

    #[test]
    pub fn test_color_rgb_2() {
        let answer: Option<Color> = Some((197, 108, 53));
        let result = create_result!("\x1B[31;38;2;197;108;53mHello world");
        assert_eq!(result.get_fg_color(), answer);
    }

    #[test]
    pub fn test_color_rgb_3() {
        let answer: Option<Color> = None;
        let result = create_result!("\x1B[31;38;2;197;108;53;0mHello world");
        assert_eq!(result.get_fg_color(), answer);
    }

    #[test]
    pub fn test_color_ansi() {
        let answer: Option<Color> = Some((215, 175, 0));
        let result = create_result!("\x1B[31;38;5;178;1mHello world");
        assert_eq!(result.get_fg_color(), answer);
    }

    #[test]
    pub fn test_color_ansi_2() {
        let answer: Option<Color> = None;
        let result = create_result!("\x1B[31;38;5;178;0mHello world");
        assert_eq!(result.get_fg_color(), answer);
    }
}
