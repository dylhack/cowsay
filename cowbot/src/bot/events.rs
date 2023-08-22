mod interaction_create;
mod ready;

use std::sync::Arc;

use serenity::model::application::interaction::Interaction;
use serenity::{
    model::prelude::Ready,
    prelude::{Context as SerenityContext, EventHandler},
};

use crate::{bot::Context, client::CowClient};

pub struct Handler {
    pub client: Arc<CowClient>,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: SerenityContext, ready: Ready) {
        ready::handle(
            &Context {
                client: Arc::clone(&self.client),
                ctx,
            },
            &ready,
        )
        .await;
    }

    async fn interaction_create(&self, ctx: SerenityContext, interaction: Interaction) {
        interaction_create::handle(
            &Context {
                client: Arc::clone(&self.client),
                ctx,
            },
            &interaction,
        )
        .await;
    }
}
