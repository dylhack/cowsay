use crate::{
    types::CommandResult,
    commands::cowsay::internal::respond,
};
use serenity::{
    builder::CreateApplicationCommandOption,
    model::prelude::{
        application_command::{ApplicationCommandInteraction, CommandDataOption},
        command::CommandOptionType,
    },
    prelude::Context,
};

// 510 = Four lines of wrapped text
static MAX_LENGTH: u16 = 510;

pub fn register(grp: &mut CreateApplicationCommandOption) {
    grp.kind(CommandOptionType::SubCommand);
    grp.name("say").description("Cow say something!");
    grp.create_sub_option(|opt| {
        opt.name("text")
            .description("The text to say")
            .kind(CommandOptionType::String)
            .max_length(MAX_LENGTH)
            .required(true);
        opt
    });
}

pub async fn handle(
    ctx: &Context,
    cmd: &ApplicationCommandInteraction,
    subcmd: &CommandDataOption,
) -> CommandResult {
    let text_arg = subcmd
        .options
        .get(0)
        .ok_or("Text to say not provided")?
        .value
        .as_ref()
        .ok_or("Text to say not found")?;
    let say = text_arg.as_str().ok_or("Failed to parse text as string")?;

    respond(ctx, cmd, &say).await
}
