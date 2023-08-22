use serenity::model::id::GuildId;
use std::env::var;

pub fn get_dev_server() -> Option<GuildId> {
    let val = var("DEV_SERVER_ID");
    match val.unwrap_or("".to_string()).trim().parse::<u64>() {
        Ok(val_int) => Some(GuildId(val_int)),
        Err(_) => None,
    }
}

pub fn get_bot_token() -> String {
    var("BOT_TOKEN").expect("Expected a bot token in the environment")
}

pub fn get_contact_url() -> String {
    var("CONTACT_URL").unwrap_or(String::from("https://github.com/dylhack/cowsay-bot"))
}

pub fn get_server_url() -> String {
    var("SERVER_URL").unwrap_or(String::from("5005"))
}
