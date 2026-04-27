use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status_code: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(status_code: String, message: String) -> Self {
        ErrorResponse {
            status_code,
            message,
        }
    }
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Server responded with nothing")]
    NotFound,
    #[error("Uploads are missing some feild")]
    Serailization(#[from] JsonRejection),
    #[error("There is an error at the database")]
    DbError(#[from] sqlx::Error),
    #[error("password hashing failed")]
    Argon2Error(#[from] argon2::password_hash::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let message = self.to_string();
        match self {
            Self::NotFound => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    message,
                )),
            )
                .into_response(),
            Self::Serailization(_) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ErrorResponse::new(
                    StatusCode::UNPROCESSABLE_ENTITY.to_string(),
                    message,
                )),
            )
                .into_response(),
            Self::DbError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    message,
                )),
            )
                .into_response(),
            Self::Argon2Error(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    message,
                )),
            )
                .into_response(),
        }
    }
}
