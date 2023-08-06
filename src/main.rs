mod bot;
mod commands;
mod config;
mod cowsay;
mod events;
mod embeds;
mod tmp;
mod types;


#[tokio::main]
async fn main() {
    bot::start().await;
}
