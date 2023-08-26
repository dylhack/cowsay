use crate::{
    client,
    fortune::get_fortune,
    types::{CommandResult, Response},
};
use anyhow::Result;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        application_command::ApplicationCommandInteraction, autocomplete::AutocompleteInteraction,
        AttachmentType,
    },
    prelude::Context as SerenityContext,
};
use std::borrow::Cow;

const CHAR_OPTION_ID: &str = "character";
const MSG_OPTION_ID: &str = "message";

pub fn register() -> CreateApplicationCommand {
    let mut cmd = CreateApplicationCommand::default();
    cmd.name("cowsay")
        .description("The cowsay command!")
        .create_option(|opt| {
            opt.name(CHAR_OPTION_ID)
                .description("The character to use.")
                .required(true)
                .set_autocomplete(true)
                .kind(serenity::model::prelude::command::CommandOptionType::String)
        })
        .create_option(|opt| {
            opt.name(MSG_OPTION_ID)
                .description("Message to say. (Leave blank for a fortune)")
                .required(false)
                .kind(serenity::model::prelude::command::CommandOptionType::String)
        });
    cmd
}

pub async fn handle(ctx: &SerenityContext, cmd: &ApplicationCommandInteraction) -> CommandResult {
    let mut chara_id = None;
    let mut message = String::new();
    cmd.data.options.iter().for_each(|opt| {
        if opt.name == CHAR_OPTION_ID {
            if let Some(id) = opt.value.as_ref().unwrap().as_str() {
                chara_id = Some(id);
            }
        }
        if opt.name == MSG_OPTION_ID {
            message = opt
                .value
                .as_ref()
                .and_then(|v| v.as_str())
                .and_then(|s| Some(s.to_string()))
                .unwrap_or(String::new());
        }
    });

    if chara_id.is_none() {
        return Response::err("No character specified.");
    }

    let chara = chara_id.unwrap();
    if message.len() == 0 {
        message = get_fortune();
    }

    let bytes = client::cowsay(chara, &message).await?;
    if let Err(why) = cmd
        .create_interaction_response(&ctx.http, |f| {
            f.interaction_response_data(|f| {
                f.add_file(AttachmentType::Bytes {
                    data: Cow::Owned(bytes),
                    filename: format!("{}.webp", chara),
                });
                f
            })
        })
        .await
    {
        return Response::err(format!("Failed to send cowsay. {}", why));
    }

    Response::ignore()
}

pub async fn autocomplete(ctx: &SerenityContext, auto: &AutocompleteInteraction) -> Result<()> {
    let cowfiles = client::get_cowfiles(auto.guild_id.and_then(|id| Some(id.to_string()))).await?;
    let matcher = auto
        .data
        .options
        .get(0)
        .and_then(|opt| opt.value.as_ref())
        .and_then(|value| value.as_str());

    auto.create_autocomplete_response(&ctx.http, |resp| {
        for cowfile in cowfiles {
            if let Some(matcher) = matcher {
                if !cowfile.name.contains(matcher) {
                    continue;
                }
            }
            resp.add_string_choice(cowfile.name.clone(), cowfile.id.clone());
        }
        resp
    })
    .await?;

    Ok(())
}
