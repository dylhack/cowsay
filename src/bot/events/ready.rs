use super::super::commands::register_all;
use serenity::{client::Context, model::gateway::Ready};

pub async fn handle(ctx: &Context, _ready: &Ready) {
    if let Err(why) = register_all(ctx).await {
        println!("Error registering commands: {}", why);
    }

    println!("Ready!")
}
