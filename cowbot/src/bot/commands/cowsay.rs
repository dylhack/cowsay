use std::{borrow::Cow, io::Cursor};

use crate::{
    client,
    cowsay::{cowsay, cowsay_to_image},
    fortune::get_fortune,
    types::{CommandResult, Response},
};
use anyhow::Result;
use charasay::Chara;
use image::ImageFormat;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        application_command::ApplicationCommandInteraction, autocomplete::AutocompleteInteraction, AttachmentType,
    },
    prelude::Context as SerenityContext,
};

const CHAR_OPTION_ID: &str = "character";
const MSG_OPTION_ID: &str = "message";

pub fn register() -> CreateApplicationCommand {
    let mut cmd = CreateApplicationCommand::default();
    cmd.name("cowsay")
        .description("The cowsay command!")
        .create_option(|opt| {
            opt.name(CHAR_OPTION_ID)
                .description("The character to use.")
                .required(false)
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
    let server_id = cmd.guild_id.and_then(|id| Some(id.to_string()));
    let mut chara_name = "cow";
    let mut message = String::new();
    cmd.data.options.iter().for_each(|opt| {
        if opt.name == CHAR_OPTION_ID {
            if let Some(name) = opt.value.as_ref().unwrap().as_str() {
                chara_name = name;
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

    let chara = if chara_name == "cow" {
        Chara::Builtin("cow".to_string())
    } else {
        Chara::Raw(client::get_cowfile_by_name(server_id, chara_name).await?)
    };

    if message.len() == 0 {
        message = get_fortune();
    }

    let mut bytes: Vec<u8> = Vec::new();
    let image = cowsay_to_image(&cowsay(&chara, &message)?)?;
    image.write_to(&mut Cursor::new(&mut bytes), ImageFormat::WebP)?;

    if let Err(why) = cmd
        .create_interaction_response(&ctx.http, |f| {
            f.interaction_response_data(|f| {
                f.add_file(AttachmentType::Bytes { data: Cow::Owned(bytes), filename: format!("{}.webp", chara_name) });
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
            resp.add_string_choice(cowfile.name.clone(), cowfile.name.clone());
        }
        resp
    })
    .await?;

    Ok(())
}
