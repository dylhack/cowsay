use super::respond;
use crate::{fortune::get_fortune, types::CommandResult};
use charasay::BUILTIN_CHARA;
use serenity::{
    builder::CreateApplicationCommandOption,
    model::prelude::{
        application_command::{ApplicationCommandInteraction, CommandDataOption}, command::CommandOptionType,
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

pub async fn handle(ctx: &Context, cmd: &ApplicationCommandInteraction, subcmd: &CommandDataOption) -> CommandResult {
    let fortune = get_fortune();
    let mut chara = "cow";
    if let Some(chara_arg) = subcmd.options.get(1) {
        let chara_val = chara_arg.value.as_ref();
        if let Some(chara_str) = chara_val {
            chara = chara_str.as_str().unwrap_or("cow");
        }
    }

    respond(ctx, cmd, chara, &fortune).await
}
