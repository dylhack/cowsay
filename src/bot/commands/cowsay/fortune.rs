use super::respond;
use crate::{fortune::get_fortune, types::CommandResult};
use charasay::BUILTIN_CHARA;
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
        .description("Let cowsay tell your fortune!")
        .create_sub_option(|opt| {
            opt.name("character")
                .description("Character to display.")
                .kind(CommandOptionType::String)
                .required(false);

            for character in BUILTIN_CHARA {
                opt.add_string_choice(character, character);
            }

            opt
        });
}

pub async fn handle(ctx: &Context, cmd: &ApplicationCommandInteraction) -> CommandResult {
    let fortune = get_fortune();
    respond(ctx, cmd, "cow", &fortune).await
}
