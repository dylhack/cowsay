use crate::bot::Context;

use super::super::commands;
use serenity::model::prelude::interaction::Interaction;

pub async fn handle(ctx: &Context, interaction: &Interaction) {
    if let Interaction::ApplicationCommand(cmd) = interaction {
        commands::handle(&ctx, &cmd).await;
    }
}
