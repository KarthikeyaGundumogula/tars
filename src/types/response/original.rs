use axum::{Json, http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::types::response::artist::WorkPreview;

#[derive(Serialize)]
pub struct OriginalStage {
    pub title: String,
    pub description:String,
    pub release_date:DateTime<Utc>,
    pub number_of_artists: u64,
    pub number_of_works: u64,
    pub releases:Vec<Releases>,
    pub stars:Vec<Crew>,
    pub makers:Vec<Crew>,
    pub spotlight_artists:Vec<SpotlightArtist>,
    pub works:Vec<WorkPreview>,
}

#[derive(Serialize)]
pub struct Releases {
    pub src: String,
    pub title:String
}

#[derive(Serialize)]
pub struct Crew{
    pub name:String,
    pub user_handle:String,
    pub profile_picture:String,
    pub role:String
}

#[derive(Serialize)]
pub struct SpotlightArtist{
    pub profile_picture:String,
    pub user_name:String,
    pub number_of_works:String
}

/// Original-related API responses
#[derive(Debug)]
pub enum OriginalResponse {
    OriginalCreated(Uuid),
    OriginalUpdated(Uuid),
    OriginalDeleted(Uuid),
    RoleDeleted(Uuid),
    RoleCreated(Uuid),
    RoleExists(Uuid),
    OrignalReleaseCreated(Uuid),
}

impl IntoResponse for OriginalResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::OriginalCreated(id) => {
                (StatusCode::ACCEPTED, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::OriginalUpdated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::OriginalDeleted(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::RoleDeleted(id) => {
                (StatusCode::OK, Json(serde_json::json!({"profile_id": id}))).into_response()
            }
            Self::RoleCreated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"profile_id": id}))).into_response()
            }
            Self::RoleExists(id) => {
                (StatusCode::OK, Json(serde_json::json!({"profile_id": id}))).into_response()
            }
            Self::OrignalReleaseCreated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"release_id": id}))).into_response()
            }
        }
    }
}
