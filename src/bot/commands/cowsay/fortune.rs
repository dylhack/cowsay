use crate::{cowsay::get_fortune, types::CommandResult};
use super::respond;
use serenity::{
    builder::CreateApplicationCommandOption,
    model::prelude::{
        application_command::ApplicationCommandInteraction, command::CommandOptionType,
    },
    prelude::Context,
};

pub fn register(grp: &mut CreateApplicationCommandOption) {
    grp.kind(CommandOptionType::SubCommand);
    grp.name("fortune")
        .description("Let cowsay tell your fortune!");
}

pub async fn handle(ctx: &Context, cmd: &ApplicationCommandInteraction) -> CommandResult {
    let fortune = get_fortune();
    respond(ctx, cmd, "cow", &fortune).await
}
