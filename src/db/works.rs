use uuid::Uuid;
use sqlx;

use crate::{
    errors::ApiError,
    types::db::work::{
        Edit, EditFormat, Poster, PosterFormat, Script, SupportedPlatforms, Work, WorkType,
    },
};

pub async fn insert_new_work(txn: &mut sqlx::Transaction<'_, sqlx::Postgres>, data: Work) -> Result<Uuid, ApiError> {
    Ok(
   sqlx::query_scalar!(
    r#"
    INSERT INTO works(id,title,artist_id,created_at,credits,category) VALUES ($1,$2,$3,NOW(),0,$4) RETURNING id;
    "#,
    data.id,
    data.title,
    data.artist_id,
    data.category as WorkType
   ).fetch_one(&mut **txn).await?
  )
}
pub async fn insert_new_edit(txn: &mut sqlx::Transaction<'_, sqlx::Postgres>, data: Edit) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        "
    INSERT INTO edits (
        work_id,
        src_id,
        platform,
        format
      )
    VALUES ($1, $2, $3, $4)
    RETURNING work_id;
",
        data.work_id,
        data.src_id,
        data.platform as SupportedPlatforms,
        data.format as EditFormat
    )
    .fetch_one(&mut **txn)
    .await?)
}

pub async fn insert_new_poster(txn: &mut sqlx::Transaction<'_, sqlx::Postgres>, data: Poster) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        r#"
INSERT INTO posters (work_id, src_id, format)
VALUES ($1, $2, $3)
RETURNING work_id;
"#,
        data.work_id,
        data.src_id,
        data.format as PosterFormat
    )
    .fetch_one(&mut **txn)
    .await?)
}

pub async fn insert_new_script(txn: &mut sqlx::Transaction<'_, sqlx::Postgres>, data: Script) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        r#"
            INSERT INTO scripts (
                work_id,
                img_src_ids,
                thoughts
              )
            VALUES (
                $1,
                $2,
                $3
              ) RETURNING work_id;
            "#,
        data.work_id,
        &data.img_src_ids,
        &data.thoughts
    )
    .fetch_one(&mut **txn)
    .await?)
}
