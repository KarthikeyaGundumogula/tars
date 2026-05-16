use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::post};
use tracing::instrument;

use crate::{
    AppState,
    db::artists::{delete_follower, delete_favorite, insert_new_favorite, insert_new_follwing, update_profile_details},
    errors::ApiError,
    types::{
        requests::artist::{ArtistActionReq, UpdateProfileReq},
        response::ApiResponse,
    },
    shared::auth::extractor::Artist,
};

#[instrument(name = "update profile details", skip(app, user, data),fields(profile_id = %user.profile_id.to_string()))]
pub async fn update_stage_details_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    Json(data): Json<UpdateProfileReq>,
) -> Result<ApiResponse, ApiError> {
    let res = update_profile_details(&app.db_pool, data, user.profile_id)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(ApiResponse::ProfileUpdated(res))
}

async fn follow_artist_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    data: Json<ArtistActionReq>,
) -> Result<ApiResponse, ApiError> {
    let res = insert_new_follwing(&app.db_pool, user.profile_id, data.artist_id).await?;
    Ok(ApiResponse::FollowedArtist(res))
}

async fn unfollow_artist_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    Json(data): Json<ArtistActionReq>,
) -> Result<ApiResponse, ApiError> {
    let res = delete_follower(&app.db_pool, user.profile_id, data.artist_id).await?;
    Ok(ApiResponse::FollowedArtist(res))
}

async fn add_to_favorite_profiles_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    Json(data): Json<ArtistActionReq>,
) -> Result<ApiResponse, ApiError> {
    let res = insert_new_favorite(&app.db_pool, user.profile_id, data.artist_id).await?;
    Ok(ApiResponse::FavoritedArtist(res))
}

async fn remove_from_favorite_profiles_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    Json(data): Json<ArtistActionReq>,
) -> Result<ApiResponse, ApiError> {
    let res = delete_favorite(&app.db_pool, user.profile_id, data.artist_id).await?;
    Ok(ApiResponse::FavoriteArtistRemoved(res))
}

async fn update_profile_presence_handler() -> Result<ApiResponse, ApiError> {
    todo!()
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/update", post(update_stage_details_handler))
        .route("/follow", post(follow_artist_handler))
        .route("/unfollow", post(unfollow_artist_handler))
        .route("/favorite", post(add_to_favorite_profiles_handler))
        .route("/unfavorite", post(remove_from_favorite_profiles_handler))
        .route(
            "/update_profile_presence",
            post(update_profile_presence_handler),
        )
}
