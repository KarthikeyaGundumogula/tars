use axum::{Json, extract::Path};

use crate::{
    errors::ApiError,
    types::{requests::upload_works::UploadEditData, response::ApiResponse},
};

pub async fn create_new_work_handler(
    Path(work_type): Path<String>,
    Json(data): Json<UploadEditData>
) -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::OK)
}
