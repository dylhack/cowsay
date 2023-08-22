use super::respond;
use crate::{client::cowfiles::Cowfile, fortune::get_fortune, types::CommandResult};
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
    grp.name("fortune")
        .description("Let cowsay tell your fortune!")
        .create_sub_option(|opt| {
            opt.name("character")
                .description("Character to display.")
                .kind(CommandOptionType::String)
                .required(false);

            for character in cowfiles {
                opt.add_string_choice(character.name.clone(), character.id.clone());
            }

            opt
        });
}

pub async fn handle(
    ctx: &SerenityContext,
    cmd: &ApplicationCommandInteraction,
    subcmd: &CommandDataOption,
) -> CommandResult {
    let fortune = get_fortune();
    let chara = subcmd
        .options
        .get(0)
        .and_then(|opt| opt.value.as_ref())
        .and_then(|val| val.as_str());

    respond(ctx, cmd, &chara, &fortune).await
}
