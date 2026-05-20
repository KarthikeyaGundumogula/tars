use axum::{Json, http::StatusCode, response::IntoResponse};
use uuid::Uuid;

/// Festival-related API responses
#[derive(Debug)]
pub enum FestivalResponse {
    FestivalDetailsUpdated(Uuid),
    PanelistAdded(Uuid),
    PanelistDeleted(Uuid),
}

impl IntoResponse for FestivalResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::FestivalDetailsUpdated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::PanelistAdded(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::PanelistDeleted(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
        }
    }
}
