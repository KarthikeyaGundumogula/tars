use sqlx::{self, PgPool};
use uuid::Uuid;

use crate::{
    db::mutations::artists::update_profile_spirit,
    errors::ApiError,
    models::db::work::{
        Edit, EditFormat, Poster, PosterFormat, Script, SupportedPlatforms, WallPost, Work,
        WorkCategory,
    },
};

pub async fn insert_new_work(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: Work,
) -> Result<Uuid, ApiError> {
    Ok(
   sqlx::query_scalar!(
    r#"
    INSERT INTO works(id,title,artist_id,created_at,stars,category) VALUES ($1,$2,$3,NOW(),0,$4) RETURNING id;
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

pub async fn insert_work_star(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    work_id: Uuid,
    profile_id: Uuid,
) -> Result<Uuid, ApiError> {
    let res = sqlx::query!(
        r#"
            INSERT INTO work_stars (work_id,profile_id,created_at) VALUES ($1,$2,NOW()) ON CONFLICT(work_id,profile_id) DO NOTHING;
            "#,
        work_id,
        profile_id
    )
    .execute(&mut **txn)
    .await?.rows_affected();
    if res > 0 {
        let artist = sqlx::query_scalar!(
            "
    UPDATE works
    SET stars = stars + 1
    WHERE id = $1
    RETURNING artist_id
    ",
            work_id
        )
        .fetch_one(&mut **txn)
        .await?;
        Ok(artist)
    } else {
        Err(ApiError::BadRequest("Already Starred".to_string()))
    }
}

pub async fn delete_work_star(
    pool: &PgPool,
    work_id: Uuid,
    profile_id: Uuid,
) -> Result<bool, ApiError> {
    let mut txn = pool.begin().await?;
    sqlx::query!(
        r#"
            DELETE FROM work_stars WHERE work_id = $1 AND profile_id = $2;
            "#,
        work_id,
        profile_id
    )
    .execute(&mut *txn)
    .await?;
    sqlx::query!(
        "
        UPDATE works
        SET stars = stars-1
        WHERE id = $1
        ",
        work_id
    )
    .execute(&mut *txn)
    .await?;
    txn.commit().await?;
    Ok(true)
}

pub async fn insert_wall_post(pool: &PgPool, data: WallPost) -> Result<WallPost, ApiError> {
    let mut tx = pool.begin().await?;
    let wall_post = sqlx::query_as!(
        WallPost,
        r#"
        INSERT INTO wall_posts (id,work_id, artist_id,text_line,original_id,recommendation_id, created_at,updated_at) VALUES ($1, $2, $3, $4,$5,$6,$7,$8) RETURNING *;
        "#,
        data.id,
        data.work_id,
        data.artist_id,
        data.text_line,
        data.original_id,
        data.recommendation_id,
        data.created_at,
        data.updated_at
    )
    .fetch_one(&mut *tx)
    .await?;
    if let Some(work_id) = data.work_id {
        let artist_id = sqlx::query_scalar!(
            "
            WITH work_data AS (
                SELECT artist_id from works
                WHERE id = $1
                )
                INSERT INTO work_pins (work_id, wall_post_id)
                VALUES ($1, $2) 
                ON CONFLICT DO NOTHING
                RETURNING (SELECT artist_id FROM work_data)",
            work_id,
            data.id
        )
        .fetch_one(&mut *tx)
        .await?;
        if let Some(artist_id) = artist_id {
            update_profile_spirit(&mut tx, artist_id, data.artist_id).await?;
        }
    }
    tx.commit().await?;
    Ok(wall_post)
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
            INSERT INTO work_credits (
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

pub async fn insert_work_save(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    work_id: Uuid,
    user_id: Uuid,
) -> Result<Uuid, ApiError> {
    let res = sqlx::query!(
        "
        INSERT INTO saved_works (work_id, artist_id,created_at)
        VALUES ($1, $2, NOW())
        ON CONFLICT DO NOTHING
        ",
        work_id,
        user_id
    )
    .execute(&mut **txn)
    .await?
    .rows_affected();
    if res > 0 {
        let artist_id = sqlx::query_scalar!(
            "
    UPDATE works 
    SET saves = saves+1
    WHERE id = $1
    RETURNING artist_id
    ",
            work_id
        )
        .fetch_one(&mut **txn)
        .await?;
        Ok(artist_id)
    } else {
        Err(ApiError::BadRequest("Work not found".to_string()))
    }
}

pub async fn delete_work_save(pool: &PgPool, work_id: Uuid, user_id: Uuid) -> Result<(), ApiError> {
    let mut txn = pool.begin().await?;
    sqlx::query!(
        "
        DELETE FROM saved_works WHERE work_id = $1 AND artist_id = $2;
        ",
        work_id,
        user_id
    )
    .execute(&mut *txn)
    .await?;
    sqlx::query!(
        "
    UPDATE works 
    SET saves = saves-1
    WHERE id = $1
    ",
        work_id
    )
    .execute(&mut *txn)
    .await?;
    txn.commit().await?;
    Ok(())
}
