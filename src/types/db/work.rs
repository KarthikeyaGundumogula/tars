use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

/// for thumbnails
/// youtube - free thumbnail api
/// twitter display a poster from official releases / a movie poster
#[derive(Deserialize,sqlx::Type)]
#[sqlx(type_name = "supported_platforms")]
pub enum SupportedPlatforms {
    YOUTUBE,
    TWITTER,
    NATIVE
}

#[derive(Deserialize,sqlx::Type)]
#[sqlx(type_name = "edit_format")]
pub enum EditFormat {
    IMAX,     // 2.35:1
    ACADEMY,  // 1.85:1
    SQUARE,   // 1:1
    VERTICAL, // 9:16
}

pub enum PosterFormat {
    CANVAS,   // 2.35:1
    STANDARD, // 2:3
    SQUARE,   // 1:1
    VERTICAL, // 9:16
}

#[derive(sqlx::Type, Deserialize)]
#[sqlx(type_name = "work_type")]
pub enum WorkType {
    EDIT,
    POSTER,
    SCRIPT,
}

pub struct Work {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub credits: i64,
    pub created_at:DateTime<Utc>,
    pub category:WorkType
}

pub struct Edit {
    pub work_id: Uuid,
    pub src_id: String,
    pub platform: SupportedPlatforms,
    pub format:EditFormat
}
