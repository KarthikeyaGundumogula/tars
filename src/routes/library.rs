use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    routing::{delete, post},
};
use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    AppState,
    db::mutations::library::{
        add_new_tagged_work, delete_library_entry, insert_new_library_entry, update_library_entry,
    },
    errors::ApiError,
    services::{
        auth_service::extractor::{Artist, OwnedResourceOrAdmin},
        json_extractor::AppJson,
    },
    models::{
        db::library::LibraryEntry,
        requests::library::{LibraryEntryReq, TagWorkToLibraryEntryReq, UpdateLibraryEntryReq},
        response::LibraryResponse,
    },
};

#[instrument(name = "new_library_entry", skip(state, user, data), err, fields(user_id = %user.profile_id))]
pub async fn new_library_entry_handler(
    State(state): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<LibraryEntryReq>,
) -> Result<LibraryResponse, ApiError> {
    let entry = LibraryEntry {
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
    let entry = insert_new_library_entry(&state.db_pool, entry).await?;
    Ok(LibraryResponse::LibraryEntryLogged(entry))
}

#[instrument(name = "update_library_entry", skip(app, data), err, fields(entry_id = %resource_id))]
async fn update_library_entry_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<LibraryEntry>,
    AppJson(data): AppJson<UpdateLibraryEntryReq>,
) -> Result<LibraryResponse, ApiError> {
    let res = update_library_entry(&app.db_pool, data, resource_id).await?;
    Ok(LibraryResponse::LibraryEntryUpdated(res))
}

#[instrument(name = "tag_work", skip(app, data), err, fields(entry_id = %resource_id,work_id=%data.work_id))]
async fn tag_work_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<LibraryEntry>,
    AppJson(data): AppJson<TagWorkToLibraryEntryReq>,
) -> Result<LibraryResponse, ApiError> {
    let res = add_new_tagged_work(&app.db_pool, data, resource_id).await?;
    Ok(LibraryResponse::LibraryEntryUpdated(res))
}

#[instrument(name = "delete_library_entry", skip(app), err, fields(entry_id = %resource_id))]
async fn delete_library_entry_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<LibraryEntry>,
) -> Result<LibraryResponse, ApiError> {
    delete_library_entry(&app.db_pool, resource_id).await?;
    Ok(LibraryResponse::LibraryEntryDeleted(resource_id))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/new", post(new_library_entry_handler))
        .route("/{resource_id}/update", post(update_library_entry_handler))
        .route("/{resource_id}/tag_work", post(tag_work_handler))
        .route(
            "/{resource_id}/delete",
            delete(delete_library_entry_handler),
        )
}
