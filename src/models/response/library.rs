use axum::{Json, http::StatusCode, response::IntoResponse};
use uuid::Uuid;

/// Library-related API responses
#[derive(Debug)]
pub enum LibraryResponse {
    LibraryEntryLogged(Uuid),
    NewRecommendationCreated(Uuid),
    LibraryEntryUpdated(Uuid),
    RecommendationUpdated(Uuid),
    WorkTaggedToLibraryEntry(Uuid, Uuid),
    LibraryEntryDeleted(Uuid),
}

impl IntoResponse for LibraryResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::LibraryEntryLogged(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::LibraryEntryUpdated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::WorkTaggedToLibraryEntry(work_id, library_entry_id) => (
                StatusCode::OK,
                Json(serde_json::json!({"work_id": work_id, "library_entry_id": library_entry_id})),
            )
                .into_response(),
            Self::LibraryEntryDeleted(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::NewRecommendationCreated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::RecommendationUpdated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
        }
    }
}
