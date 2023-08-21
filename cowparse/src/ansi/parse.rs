use std::sync::Arc;

use super::{
    sgr,
    types::{ANSIChar, ANSIString, ControlFunction, ControlSequence},
};

const ESCAPE_START: u8 = 0x1B;
const INTRODUCER_START: u8 = 0x5B;
const PARAM_RULE: [u8; 2] = [0x30, 0x3F];

pub fn parse_control(ctrl: &str, last_control: &ControlSequence) -> ControlSequence {
    let mut result = last_control.clone();
    let mut clean = vec![];
    let mut temp = String::new();

    ctrl.chars().for_each(|c| {
        let char_code = c as u8;
        if char_code >= PARAM_RULE[0] || char_code <= PARAM_RULE[1] {
            if c.is_digit(10) {
                temp.push(c);
            } else if char_code == sgr::SEPERATOR || char_code == sgr::FINAL_BYTE {
                let param = temp.parse::<u8>().unwrap_or(0);
                clean.push(param);
                temp.clear();
            }
        }
    });

    if clean.len() == 0 {
        // X3.64: If no parameters are given, then the default value is NORMAL.
        clean.push(sgr::NORMAL);
    }

    let mut color_escape = false;
    let mut rgb_escape = false;
    let mut ansi_escape = false;
    let mut temp: [u8; 5] = [0; 5];
    let mut i = 0;
    macro_rules! finish {
        () => {
            result.push(ControlFunction {
                escape: temp[0],
                params: [temp[1], temp[2], temp[3], temp[4]],
            });
            temp = [0; 5];
            i = 0;
        };
    }

    clean.iter().for_each(|byte| {
        if color_escape {
            if byte == &sgr::T416_RGB {
                rgb_escape = true;
            } else if byte == &sgr::T416_256 {
                ansi_escape = true;
            }
            temp[i] = *byte;
            color_escape = false;
            i += 1;
        } else if ansi_escape {
            temp[i] = *byte;
            ansi_escape = false;
            finish!();
        } else if rgb_escape {
            temp[i] = *byte;
            if i == 4 {
                rgb_escape = false;
                finish!();
            } else {
                i += 1;
            }
        } else {
            match byte {
                &sgr::FOREGROUND | &sgr::BACKGROUND => {
                    color_escape = true;
                    temp[i] = *byte;
                    i += 1;
                }
                &sgr::NORMAL => result.clear(),
                other => {
                    temp[0] = *other;
                    finish!();
                }
            }
        }
    });

    result
}

/// Parse a string with ANSI X3.64 escape sequences.
///
/// # Examples
/// ```
/// use cowparse::parse;
///
/// parse("\x1B[38;2;197;108;59mHello World!\x1B[m\n");
/// ```
pub fn parse(text: &str) -> ANSIString {
    let mut result = Vec::new();
    let mut control = Arc::new(vec![]);
    let mut escaped = false;
    let mut escaped_control = false;
    let mut control_tmp = String::new();

    text.chars().for_each(|c| {
        let char_code = c as u8;
        if escaped {
            if char_code == INTRODUCER_START {
                escaped_control = true;
                escaped = false;
            } else {
                escaped = false;
            }
        } else if escaped_control {
            if char_code == sgr::FINAL_BYTE {
                control_tmp.push(c);
                control = Arc::new(parse_control(&control_tmp, &control.clone()));
                control_tmp.clear();
                escaped_control = false;
            } else {
                control_tmp.push(c);
            }
        } else if char_code == ESCAPE_START {
            escaped = true;
        } else {
            result.push(ANSIChar {
                control: Arc::clone(&control),
                char: c,
            });
        }
    });

    result
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::ansi::types::ControlFunction;

    use super::*;

    macro_rules! null_params {
        () => {
            [0, 0, 0, 0]
        };
    }

    #[test]
    fn test_parse_control() {
        let answer = vec![sgr::BOLD, sgr::RED_FG, sgr::GREEN_BG];
        let result = parse_control("\x1B[1;31;42m", &vec![]);

        for (i, answer_byte) in answer.iter().enumerate() {
            assert_eq!(answer_byte, &result[i].escape);
            assert_eq!([0, 0, 0, 0], result[i].params);
        }
    }

    #[test]
    fn test_parse_line() {
        let result = parse_control("\x1B[38;2;197;108;59;1;3m", &vec![]);
        let answer = vec![
            ControlFunction {
                escape: 38,
                params: [2, 197, 108, 59],
            },
            ControlFunction {
                escape: 1,
                params: null_params!(),
            },
            ControlFunction {
                escape: 3,
                params: null_params!(),
            },
        ];

        for (i, answer_byte) in answer.iter().enumerate() {
            assert_eq!(answer_byte.escape, result[i].escape);
            assert_eq!(answer_byte.params, result[i].params);
        }
    }

    #[test]
    fn test_parse() {
        let result = parse("\x1B[38;2;197;108;59mHello\x1B[m\nWorld!");
        let color = Arc::new(vec![ControlFunction {
            escape: 38,
            params: [2, 197, 108, 59],
        }]);
        let null_ctrl = Arc::new(vec![]);
        let answer = vec![
            ANSIChar {
                control: Arc::clone(&color),
                char: 'H',
            },
            ANSIChar {
                control: Arc::clone(&color),
                char: 'e',
            },
            ANSIChar {
                control: Arc::clone(&color),
                char: 'l',
            },
            ANSIChar {
                control: Arc::clone(&color),
                char: 'l',
            },
            ANSIChar {
                control: Arc::clone(&color),
                char: 'o',
            },
            ANSIChar {
                control: Arc::clone(&null_ctrl),
                char: '\n',
            },
            ANSIChar {
                control: Arc::clone(&null_ctrl),
                char: 'W',
            },
            ANSIChar {
                control: Arc::clone(&null_ctrl),
                char: 'o',
            },
            ANSIChar {
                control: Arc::clone(&null_ctrl),
                char: 'r',
            },
            ANSIChar {
                control: Arc::clone(&null_ctrl),
                char: 'l',
            },
            ANSIChar {
                control: Arc::clone(&null_ctrl),
                char: 'd',
            },
            ANSIChar {
                control: Arc::clone(&null_ctrl),
                char: '!',
            },
        ];

        for (i, char) in answer.iter().enumerate() {
            assert_eq!(char, &result[i]);
        }
    }

    #[test]
    fn test_parse_2() {
        let result = parse("\x1B[38;2;197;108;59mHello\x1B[1m World!");
        let color = Arc::new(vec![ControlFunction {
            escape: 38,
            params: [2, 197, 108, 59],
        }]);
        let color_2 = Arc::new(vec![
            ControlFunction {
                escape: 38,
                params: [2, 197, 108, 59],
            },
            ControlFunction {
                escape: 1,
                params: null_params!(),
            },
        ]);
        let answer = vec![
            ANSIChar {
                control: Arc::clone(&color),
                char: 'H',
            },
            ANSIChar {
                control: Arc::clone(&color),
                char: 'e',
            },
            ANSIChar {
                control: Arc::clone(&color),
                char: 'l',
            },
            ANSIChar {
                control: Arc::clone(&color),
                char: 'l',
            },
            ANSIChar {
                control: Arc::clone(&color),
                char: 'o',
            },
            ANSIChar {
                control: Arc::clone(&color_2),
                char: ' ',
            },
            ANSIChar {
                control: Arc::clone(&color_2),
                char: 'W',
            },
            ANSIChar {
                control: Arc::clone(&color_2),
                char: 'o',
            },
            ANSIChar {
                control: Arc::clone(&color_2),
                char: 'r',
            },
            ANSIChar {
                control: Arc::clone(&color_2),
                char: 'l',
            },
            ANSIChar {
                control: Arc::clone(&color_2),
                char: 'd',
            },
            ANSIChar {
                control: Arc::clone(&color_2),
                char: '!',
            },
        ];

        for (i, char) in answer.iter().enumerate() {
            assert_eq!(char, &result[i]);
        }
    }
}
