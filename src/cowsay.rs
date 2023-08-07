pub mod images;
use charasay::{format_character, Chara::Builtin};
use crate::types::Result;


pub fn cowsay(character: &str, msg: &str) -> Result<String> {
    let cow = Builtin(String::from(character));
    let result = format_character(msg, &cow, 30, charasay::bubbles::BubbleType::Round);

    if let Err(why) = result {
        Err(format!("Failed to create cowsay, {}", why))
    } else {
        Ok(result.unwrap())
    }
}

pub fn get_fortune() -> String {
    // TODO(dylhack): Implement fortune
    String::from("Not implemented yet.")
}
