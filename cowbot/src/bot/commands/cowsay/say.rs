use super::respond;
use crate::{client::cowfiles::Cowfile, types::CommandResult};
use anyhow::anyhow;
use serenity::{
    builder::CreateApplicationCommandOption,
    model::prelude::{
        application_command::{ApplicationCommandInteraction, CommandDataOption},
        command::CommandOptionType,
    },
    prelude::Context as SerenityContext,
};

pub fn register(cowfiles: &Vec<Cowfile>, grp: &mut CreateApplicationCommandOption) {
    grp.kind(CommandOptionType::SubCommand);
    grp.name("say").description("Cow say something!");
    grp.create_sub_option(|opt| {
        opt.name("text")
            .description("The text to say")
            .kind(CommandOptionType::String)
            .required(true);
        opt
    });
    grp.create_sub_option(|opt| {
        opt.name("character")
            .description("Character to display.")
            .kind(CommandOptionType::String)
            .required(false);

        for cowfile in cowfiles {
            opt.add_string_choice(cowfile.name.clone(), cowfile.id.clone());
        }

        opt
    });
}

pub async fn handle(
    ctx: &SerenityContext,
    cmd: &ApplicationCommandInteraction,
    subcmd: &CommandDataOption,
) -> CommandResult {
    let text_arg = subcmd
        .options
        .get(0)
        .ok_or(anyhow!("Text to say not provided"))?
        .value
        .as_ref()
        .ok_or(anyhow!("Text to say not found"))?;
    let chara = subcmd
        .options
        .get(1)
        .and_then(|opt| opt.value.as_ref())
        .and_then(|val| val.as_str());

    let say = text_arg
        .as_str()
        .ok_or(anyhow!("Failed to parse text as string"))?;

    respond(ctx, cmd, &chara, &say).await
}
