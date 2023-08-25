use anyhow::{Result, anyhow};
use base64::Engine;
use celery::prelude::*;
use charasay::{format_character, Chara};
use tokio::task::JoinSet;
use crate::{
    database::{self, get_unresolved, cowfiles::DbCowfile, save_preview as save_preview_db}, 
    services::previews::{SavablePreview, save_preview}
};

const MESSAGE: &str = "Hello, world!";

fn decode_base64(data: &String) -> Result<String> {
    let decoded = base64::engine::general_purpose::STANDARD.decode(data)?;
    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}

fn render(cowfile: &DbCowfile) -> Result<SavablePreview> {
    let chara = Chara::Raw(decode_base64(&cowfile.data)?);
    let result = format_character(MESSAGE, &chara, 80, charasay::bubbles::BubbleType::Round).map_err(|err| anyhow!("{}", err.to_string()))?;
    let font = include_bytes!("../../../assets/font/JetBrainsMonoNerdFont-Regular.ttf").to_vec();
    let bold_font = include_bytes!("../../../assets/font/JetBrainsMonoNerdFont-Bold.ttf").to_vec();

    let image = cowparse::ImageBuilder::from(&result)
        .set_font(font)
        .set_bubble_font(bold_font)
        .build()
        .map_err(|err| anyhow!("{}", err.to_string()))?;

    Ok(image)
}

#[celery::task]
pub async fn task() -> TaskResult<()> {
    let pool = database::get_client().await.map_err(|err| TaskError::UnexpectedError(err.to_string()))?;
    let cowfiles = get_unresolved(&pool).await.map_err(|err| TaskError::UnexpectedError(err.to_string()))?;
    let mut job_set = JoinSet::<Result<()>>::new();

    for cowfile in cowfiles {
        let pool = pool.clone();
        job_set.spawn(async move {
            let cid = cowfile.id.clone();
            let image = render(&cowfile).unwrap();
            let path = save_preview(&cid, &image).await?;
            save_preview_db(&pool, &cid, &path).await?;
            Ok(())
        });
    }

     while let Some(res) = job_set.join_next().await {
         let result = res.unwrap();
         if let Err(why) = result {
             println!("Error: {}", why);
         }
     }

    Ok(())
}
