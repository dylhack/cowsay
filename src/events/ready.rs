use crate::commands;
use serenity::{client::Context, model::gateway::Ready};

pub async fn handle(ctx: &Context, _ready: &Ready) {
    if let Err(why) = commands::register_all(ctx).await {
        println!("Error registering commands: {}", why);
    }

    println!("Ready!")
}
