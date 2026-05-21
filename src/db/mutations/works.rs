use sqlx::{self, PgPool};
use uuid::Uuid;

use crate::{
    errors::ApiError,
    types::db::work::{
        Edit, EditFormat, Poster, PosterFormat, Script, SupportedPlatforms, Work, WorkCategory,
    },
};

pub async fn insert_new_work(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: Work,
) -> Result<Uuid, ApiError> {
    Ok(
   sqlx::query_scalar!(
    r#"
    INSERT INTO works(id,title,artist_id,created_at,credits,category) VALUES ($1,$2,$3,NOW(),0,$4) RETURNING id;
    "#,
    data.id,
    data.title,
    data.artist_id,
    data.category as WorkCategory
   ).fetch_one(&mut **txn).await?
  )
}

pub async fn update_work_title(
    pool: &PgPool,
    id: Uuid,
    new_title: String,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        r#"
            UPDATE works SET title = $1 WHERE id = $2;
            "#,
        new_title,
        id
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn insert_work_like(
    pool: &PgPool,
    work_id: Uuid,
    profile_id: Uuid,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        r#"
            INSERT INTO work_likes (work_id,profile_id,created_at) VALUES ($1,$2,NOW()) ON CONFLICT(work_id,profile_id) DO NOTHING;
            "#,
        work_id,
        profile_id
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn delete_work_like(
    pool: &PgPool,
    work_id: Uuid,
    profile_id: Uuid,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        r#"
            DELETE FROM work_likes WHERE work_id = $1 AND profile_id = $2;
            "#,
        work_id,
        profile_id
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn delete_work(pool: &PgPool, work_id: Uuid) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        r#"
            DELETE FROM works WHERE id = $1;
            "#,
        work_id
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn insert_new_edit(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: Edit,
) -> Result<Uuid, ApiError> {
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

pub async fn insert_new_poster(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: Poster,
) -> Result<Uuid, ApiError> {
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

pub async fn insert_new_script(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: Script,
) -> Result<Uuid, ApiError> {
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

pub async fn insert_new_work_credit(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    original_id: Uuid,
    work_id: Uuid,
) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        "
            INSERT INTO originals_credits (
            original_id,
            work_id)
            VALUES ($1, $2)
            RETURNING original_id;
            ",
        original_id,
        work_id
    )
    .fetch_one(&mut **txn)
    .await?)
}
