mod bot;
mod config;
mod cowsay;
mod tmp;
mod types;

#[tokio::main]
async fn main() {
    bot::start().await;
}
