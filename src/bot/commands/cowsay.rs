mod fortune;
mod say;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::application_command::ApplicationCommandInteraction, prelude::Context,
};
use crate::{
    cowsay::{cowsay, images::cowsay_to_image},
    tmp::get_path,
    types::{CommandResult, Response},
};


async fn respond(
    ctx: &Context,
    cmd: &ApplicationCommandInteraction,
    character: &str,
    text: &str,
) -> CommandResult {
    let sample = cowsay(character, text)?;
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

pub fn register() -> CreateApplicationCommand {
    let mut cmd = CreateApplicationCommand::default();
    cmd.name("cowsay")
        .description("The cowsay command!")
        .create_option(|opt| {
            fortune::register(opt);
            opt
        })
        .create_option(|opt| {
            say::register(opt);
            opt
        });
    cmd
}

pub async fn handle(ctx: &Context, cmd: &ApplicationCommandInteraction) -> CommandResult {
    let subcmd = cmd.data.options.get(0).ok_or("Subcommand not found")?;

    match subcmd.name.as_str() {
        "fortune" => fortune::handle(ctx, cmd).await,
        "say" => say::handle(ctx, cmd, subcmd).await,
        other => Err(format!("Command not found `{}`", other)),
    }
}
