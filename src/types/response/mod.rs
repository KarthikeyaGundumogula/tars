use axum::{Json, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

pub enum ApiResponse {
    OK,
    WorkCreated(Uuid),
    SetCreated(Uuid),
    ProfileAuthenticated(CookieJar),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::WorkCreated(id) => (StatusCode::ACCEPTED, Json(serde_json::json!({"id": id}))).into_response(),
            Self::SetCreated(id) => (StatusCode::ACCEPTED, Json(serde_json::json!({"id": id}))).into_response(),
            Self::ProfileAuthenticated(jar) => (
                StatusCode::OK,
                jar,
                Json(serde_json::json!({"message":"logged_in"})),
            )
                .into_response(),
        }
    }
}
