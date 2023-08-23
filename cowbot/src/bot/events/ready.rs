use super::super::commands::register_all;
use serenity::{model::gateway::Ready, prelude::Context as SerenityContext};

pub async fn handle(ctx: &SerenityContext, rdy: &Ready) {
    if let Err(why) = register_all(ctx).await {
        println!("Error registering commands: {}", why);
    }

    println!("Ready as {}!", rdy.user.tag());
}
