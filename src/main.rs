mod bot;
mod commands;
mod config;
mod cowsay;
mod embeds;
mod events;
mod tmp;
mod types;

#[tokio::main]
async fn main() {
    bot::start().await;
}
