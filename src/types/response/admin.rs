use axum::{Json, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

/// Admin-related API responses
#[derive(Debug)]
pub enum AdminResponse {
    AdminCreated(Uuid),
    AdminAuthenticated(CookieJar),
}

impl IntoResponse for AdminResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::AdminCreated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::AdminAuthenticated(jar) => (StatusCode::OK, jar).into_response(),
        }
    }
}
