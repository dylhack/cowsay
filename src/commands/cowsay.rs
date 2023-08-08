mod fortune;
mod internal;
mod say;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::application_command::ApplicationCommandInteraction, prelude::Context,
};

use crate::types::CommandResult;

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
