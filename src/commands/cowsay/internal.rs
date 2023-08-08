use crate::{
    cowsay::{cowsay, images::cowsay_to_image},
    tmp::get_path,
    types::{CommandResult, Response},
};
use serenity::{
    model::prelude::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

pub async fn respond(
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
