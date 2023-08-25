use anyhow::{anyhow, Result};
use sqlx::{Pool, Postgres};

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
struct DbUser {
    pub id: String,
    pub discord_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub async fn get_user_id(pool: &Pool<Postgres>, discord_id: &str) -> Result<String> {
    if let Ok(user) = sqlx::query_as!(DbUser, "INSERT INTO cowsay.users (id, discord_id) VALUES (DEFAULT, $1) ON CONFLICT DO NOTHING RETURNING *;", discord_id).fetch_one(pool).await {
        return Ok(user.id);
    }

    let row = sqlx::query_as!(
        DbUser,
        "SELECT * FROM cowsay.users WHERE discord_id = $1;",
        discord_id
    )
    .fetch_one(pool)
    .await
    .map_err(|why| anyhow!("Failed to get user \"{}\".\n{}", discord_id, why))?;
    Ok(row.id)
}
