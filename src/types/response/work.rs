use axum::{Json, http::StatusCode, response::IntoResponse};
use uuid::Uuid;

/// Work-related API responses
#[derive(Debug)]
pub enum WorkResponse {
    WorkCreated(Uuid),
    WorkUpdated(Uuid),
    WorkDeleted(Uuid),
    AddedWorkLike(bool),
    RemovedWorkLike(bool),
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
            Self::AddedWorkLike(status) => (
                StatusCode::OK,
                Json(serde_json::json!({"is_addded": status})),
            )
                .into_response(),
            Self::RemovedWorkLike(status) => (
                StatusCode::OK,
                Json(serde_json::json!({"Is_removed": status})),
            )
                .into_response(),
        }
    }
}
