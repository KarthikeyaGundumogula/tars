use std::sync::Arc;

use axum::{
    Router,
    body::Bytes,
    extract::{Path, State},
    routing::{delete, post},
};

use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    AppState,
    db::mutations::works::{
        delete_work, delete_work_like, insert_wall_post, insert_work_like, update_work_title,
    },
    errors::ApiError,
    models::{
        db::work::{WallPost, Work, WorkCategory},
        requests::works::{LikeWork, NewWallPostReq, UpdateWorkReq},
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

#[instrument(name = "like_work", skip(app,data), err,fields(work_id = data.work_id.to_string(), profile_id = user.profile_id.to_string()))]
async fn like_work_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<LikeWork>,
) -> Result<WorkResponse, ApiError> {
    let res = insert_work_like(&app.db_pool, data.work_id, user.profile_id).await?;

    Ok(WorkResponse::AddedWorkLike(res))
}

#[instrument(name = "new wall post",skip(app,data),err,fields(artist_id = user.profile_id.to_string()))]
async fn create_new_wall_post_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<NewWallPostReq>,
) -> Result<WorkResponse, ApiError> {
    let wall_post = WallPost {
        id: Uuid::new_v4(),
        artist_id: user.profile_id,
        work_id: data.work_id,
        text_line: data.text_line,
        created_at: Some(Utc::now()),
    };
    let res = insert_wall_post(&app.db_pool, wall_post).await?;
    Ok(WorkResponse::NewWallPostCreated(res.id))
}

#[instrument(name = "dislike work",skip(app,data),fields(artist_id = user.profile_id.to_string(), work_id = data.work_id.to_string()),err)]
async fn dislike_work_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<LikeWork>,
) -> Result<WorkResponse, ApiError> {
    let res = delete_work_like(&app.db_pool, data.work_id, user.profile_id).await?;
    Ok(WorkResponse::RemovedWorkLike(res))
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
        .route("/new/wall_post", post(create_new_wall_post_handler))
        .route("/{resource_id}/update", post(update_work_handler))
        .route("/like", post(like_work_handler))
        .route("/dislike", delete(dislike_work_handler))
        .route("/{resource_id}/delete", delete(delete_work_handler))
}
