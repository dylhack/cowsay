use anyhow::{anyhow, Result};
use sqlx::{Pool, Postgres};


#[allow(dead_code)]
#[derive(sqlx::FromRow)]
struct DbServer {
    pub id: String,
    pub discord_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub async fn get_server_id(pool: &Pool<Postgres>, discord_id: &str) -> Result<String> {
    if let Ok(server) = sqlx::query!("INSERT INTO cowsay.servers (id, discord_id) VALUES (DEFAULT, $1) ON CONFLICT DO NOTHING RETURNING *;", discord_id).fetch_one(pool).await {
      return Ok(server.id);
    }

    let row = sqlx::query_as!(
        DbServer,
        "SELECT * FROM cowsay.servers WHERE discord_id = $1;",
        discord_id
    )
    .fetch_one(pool)
    .await
    .map_err(|why| anyhow!("Failed to get server \"{}\".\n{}", discord_id, why))?;

    Ok(row.id)
}
