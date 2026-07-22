use axum::{Json, http::StatusCode, response::IntoResponse};
use uuid::Uuid;
use crate::models::db::sets::SetRole;

/// Set-related API responses
#[derive(Debug)]
pub enum SetResponse {
    SetCreated(Uuid),
    UpdatedSet(Uuid),
    JoinedSet(SetRole),
    SetMemberDeleted(Uuid, Uuid),
    CreatedDiscussion(Uuid),
    CreatedComment(Uuid),
    UpdatedDiscussionPost(Uuid),
    UpdatedComment(Uuid),
    DeletedDiscussionPost(Uuid),
    DeletedComment(Uuid),
}

impl IntoResponse for SetResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::SetCreated(id) => {
                (StatusCode::ACCEPTED, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::UpdatedSet(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::JoinedSet(role) => {
                (StatusCode::OK, Json(serde_json::json!({"role": role}))).into_response()
            }
            Self::SetMemberDeleted(set_id, profile_id) => (
                StatusCode::OK,
                Json(serde_json::json!({"set_id": set_id, "profile_id": profile_id})),
            )
                .into_response(),
            Self::CreatedDiscussion(discussion_id) => (
                StatusCode::OK,
                Json(serde_json::json!({"discussion_id": discussion_id})),
            )
                .into_response(),
            Self::CreatedComment(comment_id) => (
                StatusCode::OK,
                Json(serde_json::json!({"comment_id": comment_id})),
            )
                .into_response(),
            Self::UpdatedDiscussionPost(id) => (
                StatusCode::OK,
                Json(serde_json::json!({"discussion_id": id})),
            )
                .into_response(),
            Self::UpdatedComment(id) => (
                StatusCode::OK,
                Json(serde_json::json!({"comment_id": id})),
            )
                .into_response(),
            Self::DeletedDiscussionPost(id) => (
                StatusCode::OK,
                Json(serde_json::json!({"discussion_id": id})),
            )
                .into_response(),
            Self::DeletedComment(id) => (
                StatusCode::OK,
                Json(serde_json::json!({"comment_id": id})),
            )
                .into_response(),
        }
    }
}
