use crate::{
    database::{self, get_unresolved},
    services::previews,
};
use anyhow::Result;
use celery::{prelude::TaskError, task::TaskResult};
use tokio::task::JoinSet;

#[celery::task]
pub async fn gen_previews() -> TaskResult<()> {
    let pool = database::get_client()
        .await
        .map_err(|err| TaskError::UnexpectedError(err.to_string()))?;
    let cowfiles = get_unresolved(&pool)
        .await
        .map_err(|err| TaskError::UnexpectedError(err.to_string()))?;
    let mut job_set = JoinSet::<Result<()>>::new();

    for cowfile in cowfiles {
        let pool = pool.clone();
        job_set.spawn(async move {
            previews::gen_preview(&pool, &cowfile.id).await?;
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
