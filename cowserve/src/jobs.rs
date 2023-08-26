pub mod previews;
use crate::config;
use anyhow::Result;
use celery::Celery;
use std::sync::Arc;

pub async fn get_client() -> Result<Arc<Celery>> {
    let url = config::get_queue_url();
    let queue = celery::app!(
        broker = AMQPBroker { url },
        tasks = [previews::gen_previews],
        task_routes = [
            "*" => "celery",
        ],
    )
    .await?;

    Ok(queue)
}

pub async fn init_jobs(client: &Celery) {
    if let Err(why) = client.send_task(previews::gen_previews::new()).await {
        println!("Failed to send task: {:?}", why);
    }
}
