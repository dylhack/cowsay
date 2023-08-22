use super::{config::get_database_url, proto::cowfiles::Cowfile};
use anyhow::Result;
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
) -> Result<Vec<Cowfile>> {
    let server = if let Some(id) = &server_id {
        get_server_id(&pool, id).await?
    } else {
        "0".to_string()
    };
    let result = sqlx::query_as!(
        DbCowfile,
        "
SELECT DISTINCT
  cowsay.cowfiles.*
FROM
  cowsay.cowfiles
WHERE
  server_id = (
    SELECT
      cowsay.servers.id
    FROM
      cowsay.servers
    WHERE
      cowsay.servers.discord_id = $1
  )
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
        cowfiles.push(map_db_cowfile(&row));
    }

    Ok(cowfiles)
}

/// Get a cowfile by name.
/// - server_id is Discord server ID
pub async fn get_cowfile(pool: &Pool<Postgres>, name: &str, server_id: &str) -> Option<Cowfile> {
    let result = sqlx::query_as!(
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
    .await;

    if let Ok(row) = &result {
        Some(map_db_cowfile(row))
    } else {
        None
    }
}

pub async fn get_server_id(pool: &Pool<Postgres>, discord_id: &str) -> Result<String> {
    sqlx::query!("INSERT INTO cowsay.servers (id, discord_id) VALUES (DEFAULT, $1) ON CONFLICT DO NOTHING RETURNING id;", discord_id).fetch_one(pool).await?;
    let row = sqlx::query_as!(
        DbServer,
        "SELECT * FROM cowsay.servers WHERE discord_id = $1;",
        discord_id
    )
    .fetch_one(pool)
    .await?;
    Ok(row.id)
}

pub async fn get_user_id(pool: &Pool<Postgres>, discord_id: &str) -> Result<String> {
    sqlx::query!("INSERT INTO cowsay.users (id, discord_id) VALUES (DEFAULT, $1) ON CONFLICT DO NOTHING RETURNING id;", discord_id).fetch_one(pool).await?;
    let row = sqlx::query_as!(
        DbUser,
        "SELECT * FROM cowsay.users WHERE discord_id = $1;",
        discord_id
    )
    .fetch_one(pool)
    .await?;
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

    if let Some(_) = get_cowfile(pool, name, server_id).await {
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
