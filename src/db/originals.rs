use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::ApiError, types::db::original::Original};

pub async fn create_new_original(pool: &PgPool, data: Original) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        r#"

INSERT INTO originals (
    id,
    title,
    description,
    cover_img,
    presence,
    created_at,
    password_hash,
    associated_with
  )
VALUES (
    $1,
$2,
$3,
$4,
$5,
$6,
$7,
$8
  ) RETURNING id;
      "#,
        data.id,
        data.title,
        data.description,
        data.cover_img,
        data.presence,
        data.created_at,
        data.password_hash,
        data.associated_with
    )
    .fetch_one(pool)
    .await?)
}

// pub async fn create_new_role(pool:&PgPool,data:Role)
