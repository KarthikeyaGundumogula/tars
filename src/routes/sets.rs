use std::sync::Arc;

use axum::{Router, extract::State, routing::post};
use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    AppState,
    db::sets::insert_new_set,
    errors::ApiError,
    types::{db::sets::Set, requests::sets::CreateSetReq, response::ApiResponse},
    utils::{auth::extractor::Artist, json_extractor::AppJson},
};

#[instrument(name = "create_new_set", skip(state, user, data), fields(curator= %user.handle, set_name = %data.name))]
pub async fn create_new_set_handler(
    State(state): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<CreateSetReq>,
) -> Result<ApiResponse, ApiError> {
    let set = Set {
        id: Uuid::new_v4(),
        name: data.name.to_string(),
        statement: data.statement.to_string(),
        description: data.description.to_string(),
        profile_picture: data.profile_picture,
        curator: user.profile_id,
        presence: 0,
        created_at: Utc::now(),
    };
    let mut txn = state.db_pool.begin().await?;
    let set_id = insert_new_set(&mut txn, set).await?;
    txn.commit().await?;
    Ok(ApiResponse::SetCreated(set_id))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/new", post(create_new_set_handler))
}
