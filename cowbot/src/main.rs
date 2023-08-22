mod bot;
mod client;
mod config;
mod cowsay;
mod fortune;
mod tmp;
mod types;

#[tokio::main]
async fn main() {
    client::connect()
        .await
        .expect("Failed to connect to cowserve");
    bot::start().await;
}
