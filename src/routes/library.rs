use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    routing::{delete, post},
};
use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    AppState, db::mutations::library::{
        add_new_tagged_work, delete_library_entry, delete_recommendation, insert_new_library_entry, insert_new_recommendation, update_library_entry, update_recommendation,
    }, errors::ApiError, models::{
        db::library::{LibraryEntry, Recommendation},
        requests::library::{
            LibraryEntryReq, NewRecommendationReq, TagWorkToLibraryEntryReq, UpdateLibraryEntryReq,
            UpdateRecommendationReq,
        },
        response::LibraryResponse,
    }, services::{
        auth_service::extractor::{Artist, OwnedResourceOrAdmin},
        json_extractor::AppJson,
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
        status: data.status,
        surge_score: data.surge_score.unwrap_or(0),
        entry_type: data.entry_type,
        created_at: Utc::now(),
        updated_at: Utc::now(),
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

#[instrument(name = "new recommendation",skip(app,data),fields(artist_id=%user.profile_id))]
async fn create_new_recommendation_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    Json(data): Json<NewRecommendationReq>,
) -> Result<LibraryResponse, ApiError> {
    let recommendation = Recommendation {
        id: Uuid::new_v4(),
        original_id: data.original_id,
        artist_id: user.profile_id,
        notes: Some(data.lines.to_string()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        surge_score: data.score,
        boost_number: 0,
        saves: 0,
    };
    let res = insert_new_recommendation(&app.db_pool, recommendation).await?;
    Ok(LibraryResponse::NewRecommendationCreated(res))
}

#[instrument(name = "update Recommendation", skip(app, data),fields(recommendation=%resource_id))]
async fn update_recommedation_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id,user_id, .. }: OwnedResourceOrAdmin<Recommendation>,
    Json(data): Json<UpdateRecommendationReq>,
) -> Result<LibraryResponse, ApiError> {
    let res = update_recommendation(&app.db_pool, data, resource_id, user_id).await?;
    Ok(LibraryResponse::RecommendationUpdated(res))
}

#[instrument(name = "delete recommendation", skip(app), err, fields(recommendation_id = %resource_id))]
async fn delete_recommendation_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, user_id,.. }: OwnedResourceOrAdmin<Recommendation>,
) -> Result<LibraryResponse, ApiError> {
    delete_recommendation(&app.db_pool, resource_id, user_id).await?;
    Ok(LibraryResponse::RecommendationDeleted(resource_id))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/new", post(new_library_entry_handler))
        .route("/{resource_id}/update", post(update_library_entry_handler))
        .route(
            "/recommendations/new",
            post(create_new_recommendation_handler),
        )
        .route(
            "/recommendations/{resource_id}/update",
            post(update_recommedation_handler),
        )
        .route("/{resource_id}/tag_work", post(tag_work_handler))
        .route(
            "/{resource_id}/delete",
            delete(delete_library_entry_handler),
        )
        .route(
            "/recommendations/{resource_id}/delete",
            delete(delete_recommendation_handler)
        )
}
