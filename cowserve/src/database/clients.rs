use anyhow::Result;

use super::Client;

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub struct DbAuthClient {
    pub id: String,
    pub token: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub async fn get_client_creds(pool: &Client, token: &String) -> Result<DbAuthClient> {
    Ok(sqlx::query_as!(
        DbAuthClient,
        "SELECT * FROM cowsay.clients WHERE token = $1 AND deleted_at IS NULL",
        token
    )
    .fetch_one(pool)
    .await?)
}
