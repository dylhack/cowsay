use std::sync::Arc;

mod bot;
mod client;
mod config;
mod cowsay;
mod fortune;
mod tmp;
mod types;

#[tokio::main]
async fn main() {
    let client = Arc::new(
        client::connect()
            .await
            .expect("Failed to connect to cowserve"),
    );
    bot::start(Arc::clone(&client)).await;
}
