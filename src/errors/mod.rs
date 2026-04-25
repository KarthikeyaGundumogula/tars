use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Server responded with nothing")]
    NotFound,
    #[error("Uploads are missing some feild")]
    Serailization(#[from] JsonRejection),
    #[error("There is an error at the database")]
    DbError(#[from] sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::NotFound => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            Self::Serailization(_) => (StatusCode::UNPROCESSABLE_ENTITY).into_response(),
            Self::DbError(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}
