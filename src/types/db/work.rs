use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::auth_service::extractor::Resource;

/// for thumbnails
/// youtube - free thumbnail api
/// twitter display a poster from official releases / a movie poster
#[derive(Deserialize, sqlx::Type, Serialize, Debug)]
#[sqlx(type_name = "supported_platforms")]
pub enum SupportedPlatforms {
    YOUTUBE,
    TWITTER,
    NATIVE,
}

#[derive(Deserialize, sqlx::Type, Serialize, Debug)]
#[sqlx(type_name = "edit_format")]
pub enum EditFormat {
    IMAX,     // 2.35:1
    ACADEMY,  // 1.85:1
    SQUARE,   // 1:1
    VERTICAL, // 9:16
}

#[derive(Deserialize, sqlx::Type, Serialize, Debug)]
#[sqlx(type_name = "poster_format")]
pub enum PosterFormat {
    CANVAS,   // 2.35:1
    STANDARD, // 2:3
    SQUARE,   // 1:1
    VERTICAL, // 9:16
}

#[derive(sqlx::Type, Deserialize, Serialize, Debug)]
#[sqlx(type_name = "work_category")]
pub enum WorkCategory {
    EDIT,
    POSTER,
    SCRIPT,
}

impl AsRef<str> for WorkCategory {
    fn as_ref(&self) -> &str {
        match self {
            WorkCategory::EDIT => "edit",
            WorkCategory::POSTER => "poster",
            WorkCategory::SCRIPT => "script",
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct WorkTypeParam {
    pub work_type: WorkCategory,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Work {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub title: Option<String>,
    pub stars: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub category: WorkCategory,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Edit {
    pub work_id: Uuid,
    pub src_id: String,
    pub platform: SupportedPlatforms,
    pub format: EditFormat,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Poster {
    pub work_id: Uuid,
    pub src_id: String,
    pub format: PosterFormat,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Script {
    pub work_id: Uuid,
    pub img_src_ids: Vec<String>,
    pub thoughts: Vec<String>,
}

impl Resource for Work {
    async fn fetch_by_id(
        db: &sqlx::PgPool,
        resource_id: Uuid,
    ) -> Result<Option<(Uuid, Self)>, crate::errors::ApiError>
    where
        Self: Send,
    {
        let work = sqlx::query_as!(
            Work,
            r#"SELECT id, artist_id, title, stars, created_at, category as "category:WorkCategory" FROM works WHERE id = $1"#,
            resource_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(crate::errors::ApiError::NotFound)?;
        Ok(Some((work.artist_id, work)))
    }
}
