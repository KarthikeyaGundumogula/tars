use axum::{http::StatusCode, response::IntoResponse};

pub enum ApiResponse {
    OK,
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
        }
    }
}
