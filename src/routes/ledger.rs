use std::sync::Arc;

use axum::{Router, extract::State, routing::post};
use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    AppState,
    db::ledger::insert_new_ledger_entry,
    errors::ApiError,
    types::{db::ledger::LedgerEntry, requests::ledger::LedgerEntryReq, response::ApiResponse},
    utils::{auth::extractor::Artist, json_extractor::AppJson},
};

#[instrument(name = "new_ledger_entry", skip(state, user, data), err, fields(user_id = %user.profile_id))]
pub async fn new_ledger_entry_handler(
    State(state): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<LedgerEntryReq>,
) -> Result<ApiResponse, ApiError> {
    let entry = LedgerEntry {
        id: Uuid::new_v4(),
        original_id: data.original_id,
        episode_id: data.episode_id,
        profile_id: user.profile_id,
        pub_visibility: data.visibility,
        tagged_works: data.tagged_works,
        pre_thought: data.pre_thought.map(|t| t.to_string()),
        post_impression: data.post_impression.map(|t| t.to_string()),
        status: Some(data.status),
        entry_type: data.entry_type,
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };
    let entry = insert_new_ledger_entry(&state.db_pool, entry).await?;
    Ok(ApiResponse::LedgerEntryLogged(entry))
}

async fn update_ledger_entry_handler() -> Result<ApiResponse, ApiError> {
    todo!()
}

async fn tag_work_handler() -> Result<ApiResponse, ApiError> {
    todo!()
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/new", post(new_ledger_entry_handler))
        .route("/update_ledger_entry", post(update_ledger_entry_handler))
        .route("/tag_work", post(tag_work_handler))
}
