use axum::Json;

use crate::{
    errors::ApiError,
    types::{requests::auth::ProfileSignup, response::ApiResponse},
    utils::password::{get_password_hash, verify_password},
};

pub async fn register_artist_handler(
    Json(body): Json<ProfileSignup>,
) -> Result<ApiResponse, ApiError> {
    let password = body.password;
    let password_hash = get_password_hash(&password)?;
    verify_password(&password, &password_hash)?;
    Ok(ApiResponse::OK)
}
