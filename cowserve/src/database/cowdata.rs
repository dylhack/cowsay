use anyhow::Result;

use super::Client;

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub struct DbCowdata {
    pub id: String,
    pub data: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub async fn save_cowdata(pool: &Client, data: &str) -> Result<String> {
    let res = sqlx::query_as!(
        DbCowdata,
        "INSERT INTO cowsay.cowdata (id, data) VALUES (DEFAULT, $1) RETURNING *;",
        data
    )
    .fetch_one(pool)
    .await?;

    Ok(res.id)
}

pub async fn get_cowdata(pool: &Client, cid: &str) -> Result<String> {
    let res = sqlx::query_as!(
        DbCowdata,
        "
SELECT
  cowsay.cowdata.*
FROM
  cowsay.cowdata
  INNER JOIN cowsay.cowfiles ON cowsay.cowdata.id = cowsay.cowfiles.data_id
WHERE
  cowsay.cowfiles.id = $1;
    ",
        cid
    )
    .fetch_one(pool)
    .await?;

    Ok(res.data)
}
