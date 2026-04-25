use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UploadEditData {
    pub title: Option<String>,
    pub src_id: String,
    pub platform: SupportedPlatforms,
    pub format: SupportedEditFormats,
    pub originals: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct UploadPosterData {
    pub title: Option<String>,
    pub src_id: String,
    pub format: SupportedPosterFormats,
    pub originals: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct UploadScriptData {
    pub title: Option<String>,
    pub src_ids: Vec<String>,
    pub originals: Vec<Uuid>,
    pub thoughts: Vec<Option<String>>,
}

#[derive(sqlx::Type,Deserialize)]
#[sqlx(type_name = "work_type", rename_all = "PascalCase")]
pub enum WorkType {
    Edit,
    Poster,
    Script,
}

#[derive(Deserialize)]
pub enum SupportedPlatforms {
    Youtube,
    Twitter,
    Native,
}

#[derive(Deserialize)]
pub enum SupportedEditFormats {
    Imax,
    Academy,
    Vertical,
    Square,
}

#[derive(Deserialize)]
pub enum SupportedPosterFormats {
    Canvas,
    Standard,
    Square,
    Vertical,
}
