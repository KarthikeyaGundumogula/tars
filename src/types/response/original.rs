use axum::{Json, http::StatusCode, response::IntoResponse};
use uuid::Uuid;

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
