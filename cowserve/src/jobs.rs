pub mod previews;
use std::sync::Arc;
use anyhow::Result;
use celery::Celery;
use crate::config;

pub async fn get_client() -> Result<Arc<Celery>> {
    let url = config::get_queue_url();
    let queue = celery::app!(
        broker = AMQPBroker { url },
        tasks = [previews::task],
        task_routes = [
            "*" => "celery",
        ],
    )
    .await?;

    Ok(queue)
}


pub async fn init_jobs(client: &Celery) {

    if let Err(why) = client.send_task(previews::task::new()).await {
        println!("Failed to send task: {:?}", why);
    }
}
