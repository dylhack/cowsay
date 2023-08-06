use std::env::var;
use serenity::model::id::GuildId;


pub fn get_dev_server() -> Option<GuildId> {
    var("DEV_SERVER_ID")
        .ok()
        .and_then(|id| id.parse::<u64>().ok())
        .map(|id| GuildId(id))
}

pub fn get_bot_token() -> String {
    var("BOT_TOKEN").expect("Expected a bot token in the environment")
}

pub fn get_contact_url() -> String {
    var("CONTACT_URL").unwrap_or(String::from("https://github.com/dylhack/cowsay-bot"))
}
