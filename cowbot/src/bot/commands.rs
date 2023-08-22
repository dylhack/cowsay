mod cowsay;

use crate::{
    bot::embeds::create_error_embed,
    config::get_dev_server,
    types::{CommandResult, Response},
};
use anyhow::anyhow;
use serenity::{
    builder::CreateApplicationCommand,
    futures::future::{join, join_all},
    model::{
        id::GuildId,
        prelude::{
            application_command::ApplicationCommandInteraction,
            autocomplete::AutocompleteInteraction, command::Command,
        },
    },
    prelude::Context as SerenityContext,
};
use std::io;

pub async fn register_all(ctx: &SerenityContext) -> io::Result<()> {
    let cmds = vec![cowsay::register()];
    let dev_server = get_dev_server();

    if let Some(guild_id) = dev_server {
        println!("registering commands to dev server.");
        register_to_dev(ctx, cmds, &guild_id).await?;
        return Ok(());
    }

    register_to_global(ctx, cmds).await?;
    Ok(())
}

async fn clear_dev(ctx: &SerenityContext, guild_id: &GuildId) {
    let cmds = guild_id
        .get_application_commands(&ctx.http)
        .await
        .unwrap_or(vec![]);
    let mut jobs = vec![];

    for cmd in cmds {
        let job = guild_id.delete_application_command(&ctx.http, cmd.id);
        jobs.push(job);
    }

    // explicitly not caring about the errors
    join_all(jobs).await;
}

async fn clear_global(ctx: &SerenityContext) {
    let cmds = ctx
        .http
        .get_global_application_commands()
        .await
        .unwrap_or(vec![]);
    let mut jobs = vec![];

    for cmd in cmds {
        let job = ctx.http.delete_global_application_command(cmd.id.0);
        jobs.push(job);
    }

    // explicitly not caring about the errors
    join_all(jobs).await;
}

async fn register_to_dev(
    ctx: &SerenityContext,
    cmds: Vec<CreateApplicationCommand>,
    guild_id: &GuildId,
) -> std::io::Result<()> {
    join(clear_dev(ctx, guild_id), clear_global(ctx)).await;

    let result = guild_id
        .set_application_commands(&ctx.http, |f| {
            for cmd in cmds {
                f.add_application_command(cmd);
            }
            f
        })
        .await;

    if let Err(why) = result {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            why.to_string(),
        ));
    }

    Ok(())
}

async fn register_to_global(
    ctx: &SerenityContext,
    cmds: Vec<CreateApplicationCommand>,
) -> std::io::Result<()> {
    let result = Command::set_global_application_commands(&ctx.http, |f| {
        for cmd in cmds {
            f.add_application_command(cmd);
        }
        f
    })
    .await;

    if let Err(why) = result {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            why.to_string(),
        ));
    }

    Ok(())
}

pub async fn handle(ctx: &SerenityContext, cmd: &ApplicationCommandInteraction) {
    ///////////////////////////////////////////////////////////////////////////
    // Command Handler                                                       //
    ///////////////////////////////////////////////////////////////////////////
    let result = match cmd.data.name.as_str() {
        "cowsay" => cowsay::handle(ctx, cmd).await,
        _ => Err(anyhow!("Command not found `{}`", cmd.data.name)),
    };

    respond(ctx, cmd, &result).await;
}

pub async fn handle_autocomplete(ctx: &SerenityContext, auto: &AutocompleteInteraction) {
    let result = match auto.data.name.as_str() {
        "cowsay" => cowsay::autocomplete(ctx, auto).await,
        _ => Err(anyhow!("Command not found `{}`", auto.data.name)),
    };

    if let Err(why) = result {
        println!("Failed to create autocomplete response: {:?}", why);
    }
}

async fn respond(
    ctx: &SerenityContext,
    cmd: &ApplicationCommandInteraction,
    result: &CommandResult,
) {
    const DEFAULT_EPHEMERAL: bool = true;

    if let Ok(resp) = result {
        if let Response::Ignore = resp {
            return;
        }
    }

    let response = cmd
        .create_interaction_response(&ctx.http, |f| {
            f.interaction_response_data(|f| {
                match result {
                    Err(why) => {
                        let embed = create_error_embed(&why.to_string());
                        f.add_embed(embed);
                        f.ephemeral(DEFAULT_EPHEMERAL);
                        // TODO(dylhack): so it worky
                        // log::error(format!("Error: {:?}", why));
                    }
                    Ok(resp) => {
                        match resp {
                            Response::Success => {
                                f.content("Done.");
                                f.ephemeral(DEFAULT_EPHEMERAL);
                            }
                            Response::Ok(msg, is_eph) => {
                                f.content(msg);
                                f.ephemeral(*is_eph);
                            }
                            // NOTE(dylhack): this will be the same as Ok for now
                            Response::Warning(why) => {
                                f.content(why);
                                f.ephemeral(DEFAULT_EPHEMERAL);
                            }
                            Response::Ignore => {}
                        };
                    }
                }
                f
            })
        })
        .await;

    if let Err(why) = response {
        println!("Error sending response: {:?}", why);
    }
}
