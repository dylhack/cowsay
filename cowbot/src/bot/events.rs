mod interaction_create;
mod ready;
use serenity::model::application::interaction::Interaction;
use serenity::{
    model::prelude::Ready,
    prelude::{Context as SerenityContext, EventHandler},
};

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: SerenityContext, ready: Ready) {
        ready::handle(&ctx, &ready).await;
    }

    async fn interaction_create(&self, ctx: SerenityContext, interaction: Interaction) {
        interaction_create::handle(&ctx, &interaction).await;
    }
}
