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
    AppState, db::mutations::{
        artists::{
            decrement_spirit_tokens, delete_boost_recommendation, delete_favorite,
            delete_save_recommendation, increment_spirit_relation, insert_boost_recommendation,
            insert_new_favorite, insert_save_recommendation, update_profile_details,
        }, works::{
            delete_wall_post_by_id, delete_wall_post_reaction, delete_work_save, delete_work_star, insert_new_wall_post, insert_wall_post_reaction, insert_work_save, insert_work_star,
        },
    }, errors::ApiError, models::{
        db::work::WallPost,
        requests::{
            artist::{FavoriteActionReq, ReactionReq, UpdateProfileReq},
            works::{EntityAction, NewWallPostReq},
        },
        response::{LibraryResponse, ProfileResponse, WorkResponse},
    }, services::{
        auth_service::extractor::{Artist, OwnedResourceOrAdmin},
        json_extractor::AppJson,
    },
};

#[instrument(name = "update profile details", skip(app, user, data),fields(profile_id = %user.profile_id.to_string()))]
pub async fn update_stage_details_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<UpdateProfileReq>,
) -> Result<ProfileResponse, ApiError> {
    let res = update_profile_details(&app.db_pool, data, user.profile_id)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(ProfileResponse::ProfileUpdated(res))
}
#[instrument(name = "add to favorite profiles", skip(app, user, data),fields(user_id = %user.profile_id.to_string(), artist_id = %data.artist_id))]
async fn add_to_favorite_profiles_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<FavoriteActionReq>,
) -> Result<ProfileResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let res = insert_new_favorite(&mut txn, user.profile_id, data.artist_id).await?;
    increment_spirit_relation(&mut txn, data.artist_id, user.profile_id).await?;
    txn.commit().await?;
    Ok(ProfileResponse::FavoritedArtist(res))
}

#[instrument(name = "remove from favorite profiles", skip(app, user, data),fields(user_id = %user.profile_id.to_string(), artist_id = %data.artist_id))]
async fn remove_from_favorite_profiles_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<FavoriteActionReq>,
) -> Result<ProfileResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let res = delete_favorite(&mut txn, user.profile_id, data.artist_id).await?;
    decrement_spirit_tokens(&mut txn, data.artist_id, user.profile_id).await?;
    txn.commit().await?;
    Ok(ProfileResponse::FavoriteArtistRemoved(res))
}

#[instrument(name = "star_work", skip(app,data), err,fields(work_id = data.entity_id.to_string(), profile_id = user.profile_id.to_string()))]
async fn star_work_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<EntityAction>,
) -> Result<WorkResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let artist = insert_work_star(&mut txn, data.entity_id, user.profile_id).await?;
    increment_spirit_relation(&mut txn, artist, user.profile_id).await?;
    txn.commit().await?;
    Ok(WorkResponse::AddedWorkStar(true))
}

#[instrument(name = "remove work star",skip(app,data),fields(artist_id = %user.profile_id, work_id = %data.entity_id),err)]
async fn dislike_work_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<EntityAction>,
) -> Result<WorkResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let artist = delete_work_star(&mut txn, data.entity_id, user.profile_id).await?;
    decrement_spirit_tokens(&mut txn, artist, user.profile_id).await?;
    txn.commit().await?;
    Ok(WorkResponse::RemovedWorkStar(true))
}

#[instrument(name = "save work",skip(app,data),fields(artist_id=%user.profile_id,work_id = %data.entity_id))]
async fn save_work_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    Json(data): Json<EntityAction>,
) -> Result<WorkResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let artist = insert_work_save(&mut txn, data.entity_id, user.profile_id).await?;
    increment_spirit_relation(&mut txn, artist, user.profile_id).await?;
    txn.commit().await?;
    Ok(WorkResponse::AddedWorkSave(true))
}

#[instrument(name = "unsave work",skip(app,data),fields(artist_id=%user.profile_id,work_id = %data.entity_id))]
async fn unsave_work_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    Json(data): Json<EntityAction>,
) -> Result<WorkResponse, ApiError> {
    delete_work_save(&app.db_pool, data.entity_id, user.profile_id).await?;
    Ok(WorkResponse::RemovedWorkSave(true))
}

#[instrument(name = "boost recommendation",skip(app,data),fields(artist_id=%user.profile_id,recommendation_id= %data.entity_id))]
async fn boost_recommendation_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    Json(data): Json<EntityAction>,
) -> Result<LibraryResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let artist_id = insert_boost_recommendation(&mut txn, user.profile_id, data.entity_id).await?;
    increment_spirit_relation(&mut txn, artist_id, user.profile_id).await?;
    txn.commit().await?;
    Ok(LibraryResponse::BoostedRecommendation(true))
}

#[instrument(name = "remove recommendation boost",skip(app,data),fields(artist_id=%user.profile_id,recommendation_id = %data.entity_id))]
async fn remove_recommendation_boost_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    Json(data): Json<EntityAction>,
) -> Result<LibraryResponse, ApiError> {
    delete_boost_recommendation(&app.db_pool, user.profile_id, data.entity_id).await?;
    Ok(LibraryResponse::UnBoostedRecommendation(true))
}

#[instrument(name = "save recommendation",skip(app,data),fields(artist_id=%user.profile_id,recommendation_id= %data.entity_id))]
async fn save_recommendation_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    Json(data): Json<EntityAction>,
) -> Result<LibraryResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let artist_id = insert_save_recommendation(&mut txn, user.profile_id, data.entity_id).await?;
    increment_spirit_relation(&mut txn, artist_id, user.profile_id).await?;
    txn.commit().await?;
    Ok(LibraryResponse::SavedRecommendation(true))
}

#[instrument(name = "unsave recommendation",skip(app,data),fields(artist_id=%user.profile_id,recommendation_id= %data.entity_id))]
async fn unsave_recommendation_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    Json(data): Json<EntityAction>,
) -> Result<LibraryResponse, ApiError> {
    delete_save_recommendation(&app.db_pool, user.profile_id, data.entity_id).await?;
    Ok(LibraryResponse::UnSavedRecommendation(true))
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
        text_line: data.text_line.map(|t| t.to_string()),
        original_id: data.original_id,
        recommendation_id: data.recommendation_id,
        total_views: 0,
        total_saves: 0,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let res = insert_new_wall_post(&app.db_pool, wall_post).await?;
    Ok(WorkResponse::NewWallPostCreated(res.id))
}

async fn delete_wall_post_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<WallPost>,
) -> Result<WorkResponse, ApiError> {
    let res = delete_wall_post_by_id(&app.db_pool, resource_id).await?;
    Ok(WorkResponse::WallPostDeleted(res))
}

async fn add_reaction_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<ReactionReq>,
) -> Result<WorkResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    insert_wall_post_reaction(&mut txn, data.wall_post_id, user.profile_id, data.reaction.to_string()).await?;
    //TODO! In the later phases when we think if the wall experiment succeeds we will enable spirit triggering here also okay for now let's dont add this cheap reactions to the spirit system
    txn.commit().await?;
    Ok(WorkResponse::ReactionAdded)
}

async fn remove_reaction_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<ReactionReq>,
) -> Result<WorkResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    delete_wall_post_reaction(&mut txn, data.wall_post_id, user.profile_id).await?;
    txn.commit().await?;
    Ok(WorkResponse::ReactionRemoved)
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/update_stage", post(update_stage_details_handler))
        .route("/favorite_artist", post(add_to_favorite_profiles_handler))
        .route(
            "/unfavorite_artist",
            post(remove_from_favorite_profiles_handler),
        )
        .route("/save_work", post(save_work_handler))
        .route("/unsave_work", delete(unsave_work_handler))
        .route("/boost_recommendation", post(boost_recommendation_handler))
        .route(
            "/remove_recommendation_boost",
            delete(remove_recommendation_boost_handler),
        )
        .route("/save_recommendation", post(save_recommendation_handler))
        .route(
            "/unsave_recommendation",
            delete(unsave_recommendation_handler),
        )
        .route("/star_work", post(star_work_handler))
        .route("/unstar_work", delete(dislike_work_handler))
        .route("/new/wall_post", post(create_new_wall_post_handler))
        .route(
            "/delete/wall_post/{resource_id}",
            delete(delete_wall_post_handler),
        )
        .route("/add_reaction", post(add_reaction_handler))
        .route("/remove_reaction", post(remove_reaction_handler))
}
