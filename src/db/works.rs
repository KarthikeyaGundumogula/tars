use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::ApiError,
    types::db::work::{
        Edit, EditFormat, Poster, PosterFormat, Script, SupportedPlatforms, Work, WorkType,
    },
};

pub async fn insert_new_work(pool: &PgPool, data: Work) -> Result<Uuid, ApiError> {
    let id = Uuid::new_v4();
    Ok(
   sqlx::query_scalar!(
    r#"
    INSERT INTO works(id,title,artist_id,created_at,credits,category) VALUES ($1,$2,$3,NOW(),0,$4) RETURNING id;
    "#,
    id,
    data.title,
    Uuid::new_v4(),
    WorkType::EDIT as WorkType
   ).fetch_one(pool).await?
  )
}
pub async fn insert_new_edit(pool: &PgPool, data: Edit) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        "
    INSERT INTO edits (
        work_id,
        src_id,
        platform,
        format,
        created_at
      )
    VALUES ($1, $2, $3, $4, $5)
    RETURNING work_id;
",
        data.work_id,
        data.src_id,
        data.platform as SupportedPlatforms,
        data.format as EditFormat,
        data.created_at
    )
    .fetch_one(pool)
    .await?)
}

pub async fn insert_new_poster(pool: &PgPool, data: Poster) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        r#"
INSERT INTO posters (work_id, src_id, format, created_at)
VALUES ($1, $2, $3, $4)
RETURNING work_id;
"#,
        data.work_id,
        data.src_id,
        data.format as PosterFormat,
        data.created_at
    )
    .fetch_one(pool)
    .await?)
}

pub async fn insert_new_script(pool: &PgPool, data: Script) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        r#"
            INSERT INTO scripts (
                work_id,
                img_src_ids,
                thoughts,
                created_at
              )
            VALUES (
                $1,
                $2,
                $3,
                $4
              ) RETURNING work_id;
            "#,
        data.work_id,
        &data.img_src_ids,
        &data.thoughts,
        data.created_at
    )
    .fetch_one(pool)
    .await?)
}
