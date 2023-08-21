pub const SEPERATOR: u8 = 0x3B;
pub const FINAL_BYTE: u8 = 0x6D;

// ANSI X3.64 - SGR
pub const NORMAL: u8 = 0;
pub const BOLD: u8 = 1;
pub const FAINT: u8 = 2;
pub const ITALIC: u8 = 3;
pub const UNDERSCORE: u8 = 4;
pub const SLOW_BLINK: u8 = 5;
pub const RAPID_BLINK: u8 = 6;
pub const REVERSE: u8 = 7;
// ISO 6429
pub const CONCEAL: u8 = 8;
// ANSI X3.64 - FNT
pub const PRIMARY_FONT: u8 = 10;
pub const ALTERNATE_FONT_1: u8 = 11;
pub const ALTERNATE_FONT_2: u8 = 12;
pub const ALTERNATE_FONT_3: u8 = 13;
pub const ALTERNATE_FONT_4: u8 = 14;
pub const ALTERNATE_FONT_5: u8 = 15;
pub const ALTERNATE_FONT_6: u8 = 16;
pub const ALTERNATE_FONT_7: u8 = 17;
pub const ALTERNATE_FONT_8: u8 = 18;
pub const ALTERNATE_FONT_9: u8 = 19;
// ISO 6429
pub const BLACK_FG: u8 = 30;
pub const RED_FG: u8 = 31;
pub const GREEN_FG: u8 = 32;
pub const YELLOW_FG: u8 = 33;
pub const BLUE_FG: u8 = 34;
pub const MAGENTA_FG: u8 = 35;
pub const CYAN_FG: u8 = 36;
pub const WHITE_FG: u8 = 37;
// T.416 - SGR
pub const FOREGROUND: u8 = 38;
// ISO 6429
pub const BLACK_BG: u8 = 40;
pub const RED_BG: u8 = 41;
pub const GREEN_BG: u8 = 42;
pub const YELLOW_BG: u8 = 43;
pub const BLUE_BG: u8 = 44;
pub const MAGENTA_BG: u8 = 45;
pub const CYAN_BG: u8 = 46;
pub const WHITE_BG: u8 = 47;
// T.416 - SGR
pub const BACKGROUND: u8 = 48;
// ECMA-48 - SGR
// NOTE(dylhack): Not supporting 56-64.
pub const DEFAULT_BG: u8 = 49;
pub const FRAMED: u8 = 51;
pub const ENCIRCLED: u8 = 52;
pub const OVERLINED: u8 = 53;
pub const NOT_FRAMED_OR_ENCIRCLED: u8 = 54;
pub const NOT_OVERLINED: u8 = 55;

// T.416 - Color
pub const T416_RGB: u8 = 2;
pub const T416_256: u8 = 5;
