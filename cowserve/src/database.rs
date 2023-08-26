pub mod cowdata;
pub mod cowfiles;
pub mod previews;
pub mod servers;
pub mod users;
use super::config::get_database_url;
use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub use cowfiles::{get_cowfile, get_cowfile_by_name, get_cowfiles, save_cowfile};
pub use previews::*;
pub use servers::*;
pub use users::*;
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
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
