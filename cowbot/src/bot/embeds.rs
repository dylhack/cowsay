use crate::config;
use serenity::{builder::CreateEmbed, utils::Color};

fn create_embed() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.color(Color::ORANGE);
    embed
}

pub fn create_error_embed(why: &String) -> CreateEmbed {
    let mut embed = create_embed();
    let contact = config::get_contact_url();
    let message = format!(
    "**An internal error has occurred**, this has been reported to the developers. Please contact us [here]({}) for further discussion.\n```\n{}\n```",
    contact,
    why
  );
    embed.title("Error").description(message).color(Color::RED);
    embed
}
