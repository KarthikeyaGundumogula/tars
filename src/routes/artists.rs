use crate::{errors::ApiError, types::response::ApiResponse};

pub async fn register_artist() -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::OK)
}
