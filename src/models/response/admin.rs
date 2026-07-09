use axum::{Json, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

/// Admin-related API responses
#[derive(Debug)]
pub enum AdminResponse {
    AdminCreated(Uuid),
    AdminAuthenticated(CookieJar),
    NewRoleCreated(String),
    NewPermissionCreated(String),
    PermissionAssigned,
    PermissionRevoked(bool),
    ProfileRoleUpdated,
}

impl IntoResponse for AdminResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::AdminCreated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::AdminAuthenticated(jar) => (StatusCode::OK, jar).into_response(),
            Self::NewRoleCreated(role) => {
                (StatusCode::OK, Json(serde_json::json!({"role": role}))).into_response()
            }
            Self::NewPermissionCreated(permission) => (
                StatusCode::OK,
                Json(serde_json::json!({"permission": permission})),
            )
                .into_response(),
            Self::PermissionAssigned => {
                (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
            }
            Self::PermissionRevoked(success) => (
                StatusCode::OK,
                Json(serde_json::json!({"success": success})),
            )
                .into_response(),
            Self::ProfileRoleUpdated => {
                (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
            }
        }
    }
}
