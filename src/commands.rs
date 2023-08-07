mod cowsay;

use crate::{
    config::get_dev_server,
    embeds::create_error_embed,
    types::{CommandResult, Response},
};
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        id::GuildId,
        prelude::{application_command::ApplicationCommandInteraction, command::Command},
    },
    prelude::Context, futures::future::join_all,
};
use std::io;

pub async fn register_all(ctx: &Context) -> io::Result<()> {
    let cmds = vec![cowsay::register()];
    let dev_server = get_dev_server();

    if let Some(guild_id) = dev_server {
        register_to_dev(ctx, cmds, &guild_id).await?;
        return Ok(());
    }

    register_to_global(ctx, cmds).await?;
    Ok(())
}

async fn clear_dev(ctx: &Context, guild_id: &GuildId) {
    let cmds = guild_id.get_application_commands(&ctx.http).await.unwrap_or(vec![]);
    let mut jobs = vec![];

    for cmd in cmds {
        let job = guild_id.delete_application_command(&ctx.http, cmd.id);
        jobs.push(job);
    }

    // explicitly not caring about the errors
    join_all(jobs).await;
}

async fn register_to_dev(
    ctx: &Context,
    cmds: Vec<CreateApplicationCommand>,
    guild_id: &GuildId,
) -> std::io::Result<()> {
    clear_dev(ctx, guild_id).await;
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
    ctx: &Context,
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

pub async fn handle(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    ///////////////////////////////////////////////////////////////////////////
    // Command Handler                                                       //
    ///////////////////////////////////////////////////////////////////////////
    let result = match cmd.data.name.as_str() {
        "cowsay" => cowsay::handle(ctx, cmd).await,
        _ => Err(format!("Command not found `{}`", cmd.data.name)),
    };

    respond(ctx, cmd, &result).await;
}

async fn respond(ctx: &Context, cmd: &ApplicationCommandInteraction, result: &CommandResult) {
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
                        let embed = create_error_embed(why);
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