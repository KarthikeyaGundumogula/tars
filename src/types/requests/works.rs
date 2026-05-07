use serde::Deserialize;
use uuid::Uuid;

use crate::domain::{ScriptThought, WorkTitle};

#[derive(Deserialize)]
pub struct UploadEditReq {
    pub title: Option<WorkTitle>,
    pub src_id: String,
    pub platform: String,
    pub format: String,
    pub originals: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct UploadPosterReq {
    pub title: Option<WorkTitle>,
    pub src_id: String,
    pub format: String,
    pub originals: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct UploadScriptReq {
    pub title: Option<WorkTitle>,
    pub src_ids: Vec<String>,
    pub originals: Vec<Uuid>,
    pub thoughts: Vec<Option<ScriptThought>>,
}
