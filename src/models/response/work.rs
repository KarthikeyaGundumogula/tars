use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct EditWorkModal {
    pub id: Uuid,
    pub title: Option<String>,
    pub src: String,
    pub artist: String,
    pub originals: Vec<WorkCredit>,
}

#[derive(Serialize)]
pub struct PosterWorkModal {
    src: String,
}

#[derive(Serialize)]
pub struct WorkCredit {
    pub id: Uuid,
    pub name: String,
    pub cover_poster: String,
}

/// Work-related API responses
#[derive(Debug)]
pub enum WorkResponse {
    WorkCreated(Uuid),
    WorkUpdated(Uuid),
    WorkDeleted(Uuid),
    NewWallPostCreated(Uuid),
    AddedWorkStar(bool),
    RemovedWorkStar(bool),
    AddedWorkSave(bool),
    RemovedWorkSave(bool),
}

impl IntoResponse for WorkResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::WorkCreated(id) => {
                (StatusCode::ACCEPTED, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::WorkUpdated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::WorkDeleted(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::NewWallPostCreated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::AddedWorkStar(status) => (
                StatusCode::OK,
                Json(serde_json::json!({"is_addded": status})),
            )
                .into_response(),
            Self::RemovedWorkStar(status) => (
                StatusCode::OK,
                Json(serde_json::json!({"Is_removed": status})),
            )
                .into_response(),
            Self::AddedWorkSave(status) => (
                StatusCode::OK,
                Json(serde_json::json!({"Is_saved": status})),
            )
                .into_response(),
            Self::RemovedWorkSave(status) => (
                StatusCode::OK,
                Json(serde_json::json!({"Is_unsaved": status})),
            )
                .into_response(),
        }
    }
}
