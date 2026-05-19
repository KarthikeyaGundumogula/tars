use serde::Deserialize;
use uuid::Uuid;

use crate::{domain::{ScriptThought, WorkTitle}, types::db::work::{EditFormat, PosterFormat, SupportedPlatforms}};

#[derive(Deserialize)]
pub struct UploadEditReq {
    pub title: Option<WorkTitle>,
    pub src_id: String,
    pub platform: SupportedPlatforms,
    pub format: EditFormat,
    pub originals: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct UploadPosterReq {
    pub title: Option<WorkTitle>,
    pub src_id: String,
    pub format: PosterFormat,
    pub originals: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct UploadScriptReq {
    pub title: Option<WorkTitle>,
    pub src_ids: Vec<String>,
    pub originals: Vec<Uuid>,
    pub thoughts: Vec<ScriptThought>,
}

#[derive(Deserialize)]
pub struct UpdateWorkReq {
    pub title: WorkTitle,
}

#[derive(Deserialize)]
pub struct LikeWork{
    pub work_id: Uuid
}