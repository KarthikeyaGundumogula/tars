use std::sync::Arc;

use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::{
    errors::ApiError,
    types::{requests::originals::CreateOriginalReq, response::ApiResponse},
};

pub async fn create_new_original_handler(
    State(pool): State<Arc<PgPool>>,
    Json(data): Json<CreateOriginalReq>,
) -> Result<ApiResponse, ApiError> {
    
    Ok(ApiResponse::OK)
}
