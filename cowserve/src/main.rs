mod config;
mod cowsay;
mod database;
mod jobs;
mod proto;
mod server;
mod services;
use anyhow::Result;
use std::{env, sync::Arc};

async fn consumer() -> Result<()> {
    let queue = jobs::get_client().await.unwrap();
    queue.display_pretty().await;
    queue.consume_from(&["celery"]).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args();

    if args.last().unwrap_or("".to_string()) == "consume" {
        println!("Running as consumer");
        consumer().await?;
        return Ok(());
    }

    println!("Starting cowserve");
    println!("Queue...");
    let queue = jobs::get_client().await?;
    println!("Database...");
    let pool = Arc::new(database::init().await?);
    println!("Initial jobs...");
    jobs::init_jobs(queue.as_ref()).await;
    server::start_server(Arc::clone(&pool), queue).await;

    Ok(())
}
