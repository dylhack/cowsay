mod fortune;
mod say;
use anyhow::{anyhow, Result};
use base64::Engine;
use charasay::Chara;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::application_command::ApplicationCommandInteraction,
    prelude::Context as SerenityContext,
};

use crate::{
    client::{
        self,
        cowfiles::{GetCowfileRequest, GetCowfilesRequest},
    },
    cowsay::{cowsay, cowsay_to_image},
    tmp::get_path,
    types::{CommandResult, Response},
};

fn decode_base64(data: &str) -> Result<String> {
    let decoded = base64::engine::general_purpose::STANDARD.decode(data)?;
    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}

async fn respond(
    ctx: &SerenityContext,
    cmd: &ApplicationCommandInteraction,
    character: &Option<&str>,
    text: &str,
) -> CommandResult {
    let mut client = client::connect().await?;
    let cowfile = if let Some(id) = character {
        client
            .get_cowfile(GetCowfileRequest { id: id.to_string() })
            .await
            .and_then(|res| Ok(decode_base64(&res.get_ref().data)))
            .and_then(|data| Ok(Chara::Raw(data.unwrap())))
            .unwrap_or(Chara::Builtin("cow".to_string()))
    } else {
        Chara::Builtin("cow".to_string())
    };

    let sample = cowsay(&cowfile, text)?;
    let image = cowsay_to_image(&sample)?;
    let file_path = get_path(&format!("{}.webp", cmd.id))?;

    if let Err(_) = image.save(file_path.clone()) {
        return Response::err("Failed to save image.");
    }

    if let Err(why) = cmd
        .create_interaction_response(&ctx.http, |f| {
            f.interaction_response_data(|f| {
                f.add_file(file_path.as_str());
                f
            })
        })
        .await
    {
        return Response::err(format!("Failed to send cowsay. {}", why));
    }

    Response::ignore()
}

pub async fn register() -> CreateApplicationCommand {
    let mut cmd = CreateApplicationCommand::default();
    let data = client::connect()
        .await
        .expect("Failed to connect to cowfiles.")
        .get_cowfiles(GetCowfilesRequest {
            // TODO(dylhack): add server-specific cowfiles
            server_id: None,
        })
        .await
        .expect("Failed to get cowfiles from cowserve.");

    let cowfiles = &data.get_ref().cowfiles;

    cmd.name("cowsay")
        .description("The cowsay command!")
        .create_option(|opt| {
            fortune::register(cowfiles, opt);
            opt
        })
        .create_option(|opt| {
            say::register(cowfiles, opt);
            opt
        });
    cmd
}

pub async fn handle(ctx: &SerenityContext, cmd: &ApplicationCommandInteraction) -> CommandResult {
    let subcmd = cmd
        .data
        .options
        .get(0)
        .ok_or(anyhow!("Subcommand not found"))?;

    match subcmd.name.as_str() {
        "fortune" => fortune::handle(ctx, cmd, subcmd).await,
        "say" => say::handle(ctx, cmd, subcmd).await,
        other => Err(anyhow!("Command not found `{}`", other)),
    }
}
