pub mod cowfiles;
pub mod servers;
pub mod previews;
pub mod users;
use super::config::get_database_url;
use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub use cowfiles::{get_cowfile, get_cowfiles, get_cowfile_by_name, save_cowfile};
pub use servers::*;
pub use users::*;
pub use previews::*;
pub type Client = Pool<Postgres>;

pub async fn get_client() -> Result<Client> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(get_database_url().as_str())
        .await?;

    Ok(pool)
}

pub async fn init() -> Result<Client> {
    let pool = get_client().await?;
    sqlx::migrate!()
        .run(&pool)
        .await?;

    Ok(pool)
}
