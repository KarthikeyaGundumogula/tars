use crate::{errors::ApiError, types::response::ApiResponse};

pub async fn health_check_handler() -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::OK)
}
