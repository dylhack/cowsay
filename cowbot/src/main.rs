mod bot;
mod client;
mod config;
mod fortune;
mod types;

#[tokio::main]
async fn main() {
    #[cfg(feature = "cowserve")]
    println!("Starting cowbot...");
    #[cfg(not(feature = "cowserve"))]
    println!("Starting cowbot... (standalone)");
    bot::start().await;
}
