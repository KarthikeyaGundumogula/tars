use std::convert::Infallible;

use axum::{
    Json,
    extract::rejection::{JsonRejection, PathRejection},
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
    #[error("Unable to process the incoming request")]
    Serialization(#[from] JsonRejection),
    #[error("Invalid Url")]
    UrlError(#[from] PathRejection),
    #[error("There is an error at the database")]
    DbError(#[from] sqlx::Error),
    #[error("password hashing failed")]
    Argon2Error(#[from] argon2::password_hash::Error),
    #[error("jwt failure")]
    JWTError(#[from] jsonwebtoken::errors::Error),
    #[error("json parsing failed")]
    JsonError(#[from] serde_json::Error),
    #[error("unauthorized")]
    Unauthorized(String),
    #[error("Cookie Jar rejection")]
    CookieJarRejection(#[from] Infallible),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let message = self.to_string();
        match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new(
                    StatusCode::NOT_FOUND.to_string(),
                    message,
                )),
            )
                .into_response(),
            Self::Serialization(_) => (
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
            Self::JWTError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    message,
                )),
            )
                .into_response(),
            Self::JsonError(_) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    StatusCode::BAD_REQUEST.to_string(),
                    message,
                )),
            )
                .into_response(),
            Self::Unauthorized(e) => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::new(StatusCode::UNAUTHORIZED.to_string(), e)),
            )
                .into_response(),
            Self::UrlError(_) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    StatusCode::BAD_REQUEST.to_string(),
                    message,
                )),
            )
                .into_response(),
            Self::CookieJarRejection(_) => (
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
