mod commands;
mod embeds;
mod events;
use crate::config::get_bot_token;
use events::Handler;
use serenity::{client::ClientBuilder, prelude::GatewayIntents};

pub async fn start() {
    let token = get_bot_token();
    let mut client = ClientBuilder::new(token, GatewayIntents::empty())
        .event_handler(Handler {})
        .await
        .expect("Failed to init bot.");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
