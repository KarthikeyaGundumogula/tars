use axum::{http::StatusCode, response::IntoResponse};
use uuid::Uuid;

pub enum ApiResponse {
    OK,
    WorkCreated(Uuid),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::WorkCreated(_) => (StatusCode::ACCEPTED).into_response(),
        }
    }
}
