mod bot;
mod client;
mod config;
mod fortune;
mod types;

#[tokio::main]
async fn main() {
    #[cfg(not(feature = "standalone"))]
    println!("Starting cowbot...");
    #[cfg(feature = "standalone")]
    println!("Starting cowbot... (standalone)");
    bot::start().await;
}
