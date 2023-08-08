pub mod images;
use crate::types::Result;
use charasay::{format_character, Chara::Builtin};

// TODO(dylhack): The following characters are not supported because the 
//                cowsay_bot::cowsay::images::chop function doesn't work 
//                with colors greater than xterm / ansi 256
pub const BUILTIN_CHARA: [&str; 20] = [
    // "aya",
    // "cirno",
    "clefairy",
    "cow",
    "eevee",
    "ferris",
    "ferris1",
    "flareon",
    "goldeen",
    "growlithe",
    "kirby",
    "kitten",
    "mario",
    "mew",
    "nemo",
    "pikachu",
    "piplup",
    "psyduck",
    // "remilia-scarlet",
    "seaking",
    "togepi",
    "tux",
    "wartortle",
];

pub fn cowsay(character: &str, msg: &str) -> Result<String> {
    let cow = Builtin(String::from(character));
    let result = format_character(msg, &cow, 80, charasay::bubbles::BubbleType::Round);

    if let Err(why) = result {
        Err(format!("Failed to create cowsay, {}", why))
    } else {
        Ok(result.unwrap())
    }
}
