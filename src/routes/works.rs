use std::sync::Arc;

use axum::{
    Router,
    body::Bytes,
    extract::{Path, State},
    routing::{delete, post},
};

use tracing::instrument;

use crate::{
    AppState,
    errors::ApiError,
    shared::{auth::extractor::Artist, works::upload_work},
    types::{db::work::WorkType, response::ApiResponse},
};

#[instrument(name = "create_new_work", skip(app, body), err)]
pub async fn create_new_work_handler(
    Path(work_type): Path<WorkType>,
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    body: Bytes,
) -> Result<ApiResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let res = upload_work(body, &mut txn, user.profile_id, work_type).await?;
    txn.commit().await?;
    Ok(ApiResponse::WorkCreated(res))
}

async fn update_work_handler() -> Result<ApiResponse, ApiError> {
    todo!()
}

async fn like_work_handler() -> Result<ApiResponse, ApiError> {
    todo!()
}

async fn delete_work_handler() -> Result<ApiResponse, ApiError> {
    todo!()
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/new/{work_type}", post(create_new_work_handler))
        .route("/{resource_id}/update", post(update_work_handler))
        .route("/like", post(like_work_handler))
        .route("/{resource_id}/delete", delete(delete_work_handler))
}
