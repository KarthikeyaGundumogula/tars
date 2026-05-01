use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::ApiError,
    types::{db::work::WorkType, requests::works::UploadEditData},
};

pub async fn create_edit_work(pool: &PgPool, data: UploadEditData) -> Result<Uuid, ApiError> {
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
