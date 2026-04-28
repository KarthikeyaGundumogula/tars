use serde::Deserialize;
use uuid::Uuid;

/// for thumbnails
/// youtube - free thumbnail api
/// twitter display a poster from afficial releases / a movie poster
pub enum EditSrcPlatforms {
    Youtube,
    Twitter,
}

pub enum EditFormat {
    IMAX,     // 2.35:1
    Academy,  // 1.85:1
    Square,   // 1:1
    Vertical, // 9:16
}

pub enum PosterFormat {
    Standard, // 2:3
    Vertical, // 9:16
    Square,   // 1:1
    Canvas,   // 2.35:1
}

pub enum WorkCategory {
    Edit {
        platform: EditSrcPlatforms,
        src_id: String,
        format: EditFormat,
    },
    Poster {
        img_id: String,
        format: PosterFormat,
    },
    Script,
}

#[derive(sqlx::Type,Deserialize)]
#[sqlx(type_name = "work_type", rename_all = "PascalCase")]
pub enum WorkType {
    Edit,
    Poster,
    Script,
}

pub struct Work {
    pub id: Uuid,
    pub title: String,
    pub category: WorkCategory,
    pub originals: Vec<Uuid>,
    pub credits: i64,
    pub artist_id: Uuid,
}