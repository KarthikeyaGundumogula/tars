use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UploadEditData {
    pub title: Option<String>,
    pub src_id: String,
    pub platform: String,
    pub format: String,
    pub originals: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct UploadPosterData {
    pub title: Option<String>,
    pub src_id: String,
    pub format: String,
    pub originals: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct UploadScriptData {
    pub title: Option<String>,
    pub src_ids: Vec<String>,
    pub originals: Vec<Uuid>,
    pub thoughts: Vec<Option<String>>,
}
