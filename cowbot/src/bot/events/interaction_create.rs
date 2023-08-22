use super::super::commands;
use serenity::{model::prelude::interaction::Interaction, prelude::Context as SerenityContext};

pub async fn handle(ctx: &SerenityContext, interaction: &Interaction) {
    if let Interaction::ApplicationCommand(cmd) = &interaction {
        commands::handle(ctx, cmd).await;
    }
}
