use super::super::commands::register_all;
use serenity::{model::gateway::Ready, prelude::Context as SerenityContext};

pub async fn handle(ctx: &SerenityContext, _ready: &Ready) {
    if let Err(why) = register_all(ctx).await {
        println!("Error registering commands: {}", why);
    }

    println!("Ready!")
}
