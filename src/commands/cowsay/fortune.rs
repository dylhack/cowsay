use serenity::{
    builder::CreateApplicationCommandOption,
    model::prelude::{application_command::ApplicationCommandInteraction, command::CommandOptionType},
    prelude::Context,
};
use crate::{
    cowsay::get_fortune,
    types::CommandResult,
    commands::cowsay::internal::respond,
};

pub fn register(grp: &mut CreateApplicationCommandOption) {
    grp.kind(CommandOptionType::SubCommand);
    grp.name("fortune").description("Let cowsay tell your fortune!");
}

pub async fn handle(ctx: &Context, cmd: &ApplicationCommandInteraction) -> CommandResult {
    let fortune = get_fortune();
    respond(ctx, cmd, &fortune).await
}
