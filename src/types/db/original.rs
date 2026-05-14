use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub use uuid::Uuid;

use crate::{errors::ApiError, utils::auth::extractor::Resource};

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "original_category")]
pub enum OriginalCategory {
    MOVIE,
    SERIES,
}

#[derive(sqlx::FromRow)]
pub struct Original {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub cover_img: String,
    pub presence: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub password_hash: String,
    pub associated_with: Option<Uuid>,
    pub genres: Option<Vec<String>>,
    pub release_date: Option<DateTime<Utc>>,
    pub duration: Option<String>,
    pub parent: Option<Uuid>,
    pub category: OriginalCategory,
}

impl Resource for Original {
    async fn fetch_by_id(
        db: &sqlx::PgPool,
        resource_id: Uuid,
    ) -> Result<Option<(Uuid, Self)>, ApiError>
    where
        Self: Send,
    {
        let original = sqlx::query_as!(Original,r#"SELECT id, title, description, cover_img, presence, password_hash, associated_with, release_date, genres, created_at,parent,category as "category:OriginalCategory",duration FROM originals WHERE id = $1"#, resource_id).fetch_optional(db).await?.ok_or(ApiError::NotFound)?;
        let id = original.associated_with.unwrap_or(original.id);
        Ok(Some((id, original)))
    }
}
