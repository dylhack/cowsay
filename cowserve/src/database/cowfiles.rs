use crate::{
    database::{servers::get_server_id, users::get_user_id},
    proto::cowfiles::CowfileDescriptor,
};
use anyhow::{anyhow, Result};
use sqlx::{Pool, Postgres};

use super::cowdata::save_cowdata;

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub struct DbCowfileDescriptor {
    pub id: String,
    pub name: String,
    pub uploader_id: String,
    pub data_id: String,
    pub author: Option<String>,
    pub server_id: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub fn map_db_cowfile(row: &DbCowfileDescriptor) -> CowfileDescriptor {
    let author = if let Some(author) = &row.author {
        author.clone()
    } else {
        "Unknown".to_string()
    };

    CowfileDescriptor {
        id: row.id.clone(),
        name: row.name.clone(),
        server_id: row.server_id.clone(),
        uploader_id: row.uploader_id.clone(),
        author,
    }
}

/// Get all cowfiles for a server & globally.
/// - server_id is Discord server ID
pub async fn get_cowfiles(
    pool: &Pool<Postgres>,
    server_id: Option<String>,
) -> Result<Vec<CowfileDescriptor>> {
    let server = if let Some(id) = &server_id {
        get_server_id(&pool, id).await?
    } else {
        "0".to_string()
    };
    let result = sqlx::query_as!(
        DbCowfileDescriptor,
        "
SELECT * FROM cowsay.cowfiles
WHERE
  server_id = $1
  OR cowsay.cowfiles.server_id IS NULL
  AND cowsay.cowfiles.deleted_at IS NULL
ORDER BY
  cowsay.cowfiles.server_id;
",
        server
    )
    .fetch_all(pool)
    .await?;

    let mut cowfiles = Vec::new();

    for row in result {
        cowfiles.push(map_db_cowfile(&row));
    }

    Ok(cowfiles)
}

/// Get a cowfile by name.
/// - server_id is Discord server ID
pub async fn get_cowfile(pool: &Pool<Postgres>, id: &str) -> Result<CowfileDescriptor> {
    let row = sqlx::query_as!(
        DbCowfileDescriptor,
        "
SELECT
  cowsay.cowfiles.*
FROM
  cowsay.cowfiles
WHERE
  id = $1;
",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|why| anyhow!("Failed to get cowfile \"{}\".\n{}", id, why))?;

    Ok(map_db_cowfile(&row))
}

/// Get a cowfile by name.
/// - server_id is Discord server ID
pub async fn get_cowfile_by_name(
    pool: &Pool<Postgres>,
    name: &str,
    server_id: &str,
) -> Result<CowfileDescriptor> {
    let row = sqlx::query_as!(
        DbCowfileDescriptor,
        "
SELECT
  cowsay.cowfiles.*
FROM
  cowsay.cowfiles
WHERE
  name = $1
  AND server_id = (
    SELECT
      cowsay.servers.id
    FROM
      cowsay.servers
    WHERE
      cowsay.servers.discord_id = $2
  );
",
        name,
        server_id
    )
    .fetch_one(pool)
    .await
    .map_err(|why| anyhow!("Failed to get cowfiles.\n{}", why))?;

    Ok(map_db_cowfile(&row))
}

/// Save a cowfile for a user.
///  - uploader_id is Discord user ID
///  - server_id is Discord server ID
pub async fn save_cowfile(
    pool: &Pool<Postgres>,
    name: &str,
    server_id: &str,
    uploader_id: &str,
    author: &Option<String>,
    data: &str,
) -> Result<CowfileDescriptor> {
    let server = get_server_id(pool, server_id).await?;
    let user = get_user_id(pool, uploader_id).await?;
    let default_author = "Unknown".to_string();

    if let Ok(_) = get_cowfile_by_name(pool, name, server_id).await {
        return Err(anyhow::anyhow!("Cowfile already exists."));
    }

    let data_id = save_cowdata(pool, data).await?;

    let row = sqlx::query_as!(
        DbCowfileDescriptor,
        "
  INSERT INTO
    cowsay.cowfiles (name, server_id, uploader_id, author, data_id)
  VALUES
    (
      $1,
      $2,
      $3,
      $4,
      $5
    )
  RETURNING *; 
  ",
        name,
        server,
        user,
        // TODO(dylhack): investigate why Option<str> isn't working here
        author.clone().unwrap_or(default_author),
        data_id
    )
    .fetch_one(pool)
    .await?;

    Ok(map_db_cowfile(&row))
}
