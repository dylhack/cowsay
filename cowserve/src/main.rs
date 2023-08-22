use std::sync::Arc;

mod config;
mod database;
mod proto;
mod server;

#[tokio::main]
async fn main() {
    let pool = Arc::new(database::init().await);
    server::start_server(Arc::clone(&pool)).await;
}
