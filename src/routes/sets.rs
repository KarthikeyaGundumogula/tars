use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::post};
use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    AppState,
    db::sets::{insert_new_set, insert_new_set_member, update_set},
    errors::ApiError,
    shared::{
        auth::extractor::{Artist, OwnedResourceOrAdmin},
        json_extractor::AppJson,
    },
    types::{
        db::sets::{Set, SetRole},
        requests::sets::{CreateSetReq, JoinSetRequest, UpdateSetReq},
        response::ApiResponse,
    },
};

#[instrument(name = "create_new_set", skip(app, user, data), fields(curator= %user.handle, set_name = %data.name))]
pub async fn create_new_set_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<CreateSetReq>,
) -> Result<ApiResponse, ApiError> {
    let set = Set {
        id: Uuid::new_v4(),
        name: data.name.to_string(),
        statement: data.statement.to_string(),
        description: data.description.to_string(),
        profile_picture: Some(data.profile_picture),
        curator: user.profile_id,
        presence: 0,
        created_at: Utc::now(),
    };
    let mut txn = app.db_pool.begin().await?;
    let set_id = insert_new_set(&mut txn, set).await?;
    let role = insert_new_set_member(
        &mut txn,
        user.profile_id,
        set_id,
        SetRole::CURATOR,
        Utc::now(),
    )
    .await?;
    txn.commit().await?;
    Ok(ApiResponse::SetCreated(set_id))
}

async fn update_set_details_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<Set>,
    AppJson(data): AppJson<UpdateSetReq>,
) -> Result<ApiResponse, ApiError> {
    let res = update_set(&app.db_pool, data, resource_id).await?;
    Ok(ApiResponse::UpdatedSet(res))
}
async fn join_set_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<JoinSetRequest>,
) -> Result<ApiResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let role = insert_new_set_member(
        &mut txn,
        user.profile_id,
        data.set_id,
        SetRole::MEMBER,
        Utc::now(),
    )
    .await?;
    txn.commit().await?;
    Ok(ApiResponse::JoinedSet(role))
}
async fn leave_set_handler() -> Result<ApiResponse, ApiError> {
    todo!()
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/new", post(create_new_set_handler))
        .route("/{resource_id}/update", post(update_set_details_handler))
        .route("/join", post(join_set_handler))
        .route("/leave", post(leave_set_handler))
}
