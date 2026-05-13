use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::post};
use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    AppState,
    db::sets::insert_new_set,
    errors::ApiError,
    types::{db::sets::Set, requests::sets::CreateSetReq, response::ApiResponse},
    utils::auth::extractor::Artist,
};

#[instrument(name = "create_new_set", skip(state, user, data), fields(curator= %user.handle, set_name = %data.name))]
pub async fn create_new_set_handler(
    State(state): State<Arc<AppState>>,
    Artist(user): Artist,
    Json(data): Json<CreateSetReq>,
) -> Result<ApiResponse, ApiError> {
    let set = Set {
        id: Uuid::new_v4(),
        name: data.name,
        statement: data.statement,
        description: data.description,
        curator: user.profile_id,
        presence: 0,
        created_at: Utc::now(),
    };
    let set_id = insert_new_set(&state.db_pool, set).await?;
    Ok(ApiResponse::SetCreated(set_id))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/new", post(create_new_set_handler))
}
