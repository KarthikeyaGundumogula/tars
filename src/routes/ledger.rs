use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::post};
use uuid::Uuid;

use crate::{
    AppState, db::ledger::insert_new_ledger_entry, errors::ApiError, types::{db::ledger::LedgerEntry, requests::ledger::LedgerEntryReq, response::ApiResponse}, utils::auth::extractor::Artist
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/new", post(new_ledger_entry_handler))
}

pub async fn new_ledger_entry_handler(
    State(state): State<Arc<AppState>>,
    Artist(user): Artist,
    Json(data): Json<LedgerEntryReq>,
) -> Result<ApiResponse, ApiError> {
  let entry = LedgerEntry{
    id: Uuid::new_v4(),
    original_id: data.original_id,
    episode_id: data.episode_id,
    profile_id: user.profile_id,
    visibility: data.visibility,
    tagged_works: data.tagged_works,
    pre_thought: data.pre_thought.map(|t| t.to_string()),
    post_impression: data.post_impression.map(|t| t.to_string()),
    status: data.status,
    entry_type: data.entry_type,
  };
  let entry = insert_new_ledger_entry(&state.db_pool, entry).await?;
  Ok(ApiResponse::LedgerEntryLogged(entry))
}
