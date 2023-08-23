mod bot;
mod client;
mod config;
mod cowsay;
mod fortune;
mod tmp;
mod types;

#[tokio::main]
async fn main() {
    bot::start().await;
}
