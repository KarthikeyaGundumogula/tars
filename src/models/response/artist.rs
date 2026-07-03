use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ArtistStage {
    pub profile_picture: String,
    pub stage_name: String,
    pub user_name: String,
    pub tag_line: String,
    pub youtube_profile: Option<String>,
    pub twitter_profile: Option<String>,
    pub instagram_profile: Option<String>,
    pub spirit: i64,
    pub color_theme: String,
    pub works: Vec<WorkPreview>,
}

#[derive(Debug, Serialize)]
pub struct WorkPreview {
    pub title: Option<String>,
    pub work_type: String,
    // pub thumbnail: String,
}

#[derive(Debug, Serialize)]
pub struct ArtistModal {
    pub profile_picture: String,
    pub stage_name: String,
    pub user_name: String,
    pub tag_line: String,
    pub youtube_profile: Option<String>,
    pub twitter_profile: Option<String>,
    pub instagram_profile: Option<String>,
    pub spirit: i64,
    pub originals: Vec<String>,
}

#[derive(Debug)]
pub enum ArtistResponse {
    ArtistStage(ArtistStage),
    ArtistModal(ArtistModal),
}

impl IntoResponse for ArtistResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            ArtistResponse::ArtistStage(artist_stage) => (
                StatusCode::OK,
                axum::Json(serde_json::json!({"artist_stage":artist_stage})),
            )
                .into_response(),
            ArtistResponse::ArtistModal(artist_modal) => (
                StatusCode::OK,
                axum::Json(serde_json::json!({"artist_modal":artist_modal})),
            )
                .into_response(),
        }
    }
}
