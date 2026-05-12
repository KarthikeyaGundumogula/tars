use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::post};

use crate::{
    AppState,
    errors::ApiError,
    types::{requests::sets::CreateSetReq, response::ApiResponse},
    utils::auth::extractor::Artist,
};

pub async fn create_new_set_handler(
    State(state): State<Arc<AppState>>,
    Artist(user): Artist,
    Json(data): Json<CreateSetReq>,
) -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::OK)
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/new", post(create_new_set_handler))
}
