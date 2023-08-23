use crate::proto::cowfiles::CowfileDescriptor;

use super::{config::get_database_url, proto::cowfiles::Cowfile};
use anyhow::{anyhow, Result};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
struct DbCowfile {
    pub id: String,
    pub name: String,
    pub data: String,
    pub uploader_id: String,
    pub author: Option<String>,
    pub server_id: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
struct DbCowfileDescriptor {
    pub id: String,
    pub name: String,
    pub uploader_id: String,
    pub author: Option<String>,
    pub server_id: Option<String>,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
struct DbServer {
    pub id: String,
    pub discord_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
struct DbUser {
    pub id: String,
    pub discord_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

fn map_db_cowfile(row: &DbCowfile) -> Cowfile {
    let author = if let Some(author) = &row.author {
        author.clone()
    } else {
        "Unknown".to_string()
    };

    Cowfile {
        id: row.id.clone(),
        name: row.name.clone(),
        data: row.data.clone(),
        server_id: row.server_id.clone(),
        uploader_id: row.uploader_id.clone(),
        author,
    }
}

fn map_db_cowfile_descriptor(row: &DbCowfileDescriptor) -> CowfileDescriptor {
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

pub async fn init() -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(get_database_url().as_str())
        .await
        .expect("Failed to connect to database.");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations.");

    pool
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
SELECT DISTINCT
  id,
  name,
  uploader_id,
  author,
  server_id
FROM
  cowsay.cowfiles
WHERE
  server_id = $1
  OR cowsay.cowfiles.server_id IS NULL
ORDER BY
  cowsay.cowfiles.server_id;
",
        server
    )
    .fetch_all(pool)
    .await?;

    let mut cowfiles = Vec::new();

    for row in result {
        cowfiles.push(map_db_cowfile_descriptor(&row));
    }

    Ok(cowfiles)
}

/// Get a cowfile by name.
/// - server_id is Discord server ID
pub async fn get_cowfile(pool: &Pool<Postgres>, id: &str) -> Result<Cowfile> {
    let row = sqlx::query_as!(
        DbCowfile,
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
) -> Result<Cowfile> {
    let row = sqlx::query_as!(
        DbCowfile,
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

/// Save a cowfile for a user.
///  - uploader_id is Discord user ID
///  - server_id is Discord server ID
pub async fn save_cowfile(
    pool: &Pool<Postgres>,
    name: &str,
    server_id: &str,
    uploader_id: &str,
    author: Option<String>,
    data: &str,
) -> Result<Cowfile> {
    let server = get_server_id(pool, server_id).await?;
    let user = get_user_id(pool, uploader_id).await?;
    let default_author = "Unknown".to_string();

    if let Ok(_) = get_cowfile_by_name(pool, name, server_id).await {
        return Err(anyhow::anyhow!("Cowfile already exists."));
    }

    let row = sqlx::query_as!(
        DbCowfile,
        "
  INSERT INTO
    cowsay.cowfiles (name, server_id, uploader_id, author, data)
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
        author.unwrap_or(default_author),
        data
    )
    .fetch_one(pool)
    .await?;

    Ok(map_db_cowfile(&row))
}
