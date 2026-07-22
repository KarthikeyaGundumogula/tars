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
    db::mutations::works::{delete_work, update_work_title},
    errors::ApiError,
    models::{
        db::work::{Work, WorkCategory},
        requests::works::UpdateWorkReq,
        response::WorkResponse,
    },
    services::{
        auth_service::extractor::{Artist, OwnedResourceOrAdmin},
        json_extractor::AppJson,
        upload_service::upload_work,
    },
};

#[instrument(name = "create_new_work", skip(app, body), err)]
pub async fn create_new_work_handler(
    Path(work_type): Path<WorkCategory>,
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    body: Bytes,
) -> Result<WorkResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let res = upload_work(body, &mut txn, user.profile_id, work_type).await?;
    txn.commit().await?;
    Ok(WorkResponse::WorkCreated(res))
}

#[instrument(name = "update_work", skip(app, data), err)]
async fn update_work_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<Work>,
    AppJson(data): AppJson<UpdateWorkReq>,
) -> Result<WorkResponse, ApiError> {
    let res = update_work_title(&app.db_pool, resource_id, data.title.to_string()).await?;
    match res {
        true => Ok(WorkResponse::WorkUpdated(resource_id)),
        false => Err(ApiError::NotFound),
    }
}

#[instrument(name = "delete work",skip(app),fields(work_id= resource_id.to_string()))]
async fn delete_work_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<Work>,
) -> Result<WorkResponse, ApiError> {
    delete_work(&app.db_pool, resource_id).await?;
    Ok(WorkResponse::WorkDeleted(resource_id))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/new/{work_type}", post(create_new_work_handler))
        .route("/{resource_id}/update", post(update_work_handler))
        .route("/{resource_id}/delete", delete(delete_work_handler))
}
