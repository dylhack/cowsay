use super::{cowfiles::DbCowfileDescriptor, Client};
use anyhow::Result;

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
struct DbPreview {
    pub id: String,
    pub source_type: String,
    pub source: String,
    pub cowfile_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

const FILE_PATH: i16 = 0;
#[allow(dead_code)]
const URL: i16 = 1;

pub async fn get_unresolved(pool: &Client) -> Result<Vec<DbCowfileDescriptor>> {
    let data = sqlx::query_as!(
        DbCowfileDescriptor,
        "
SELECT
  *
FROM
  cowsay.cowfiles
WHERE
  cowsay.cowfiles.id NOT IN (
    SELECT
      cowsay.previews.cowfile_id
    FROM
      cowsay.previews
  );"
    )
    .fetch_all(pool)
    .await?;

    Ok(data)
}

pub async fn save_preview(pool: &Client, cow_id: &String, source: &String) -> Result<()> {
    sqlx::query!(
        "
INSERT INTO
  cowsay.previews (cowfile_id, source_type, source)
VALUES
  ($1, $2, $3);
 ",
        cow_id,
        FILE_PATH,
        source
    )
    .execute(pool)
    .await?;

    Ok(())
}
