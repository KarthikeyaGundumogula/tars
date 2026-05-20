use std::sync::Arc;

use axum::{Router, extract::State, routing::post};
use tracing::instrument;

use crate::{
    AppState,
    db::mutations::artists::{
        delete_favorite, delete_follower, insert_new_favorite, insert_new_follwing,
        update_profile_details,
    },
    errors::ApiError,
    services::{auth_service::extractor::Artist, json_extractor::AppJson},
    types::{
        requests::artist::{ArtistActionReq, UpdateProfileReq},
        response::ProfileResponse,
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

#[instrument(name = "follow artist", skip(app, user, data),fields(user_id = %user.profile_id.to_string(), artist_id = %data.artist_id))]
async fn follow_artist_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<ArtistActionReq>,
) -> Result<ProfileResponse, ApiError> {
    let res = insert_new_follwing(&app.db_pool, user.profile_id, data.artist_id).await?;
    Ok(ProfileResponse::FollowedArtist(res))
}

#[instrument(name = "unfollow artist", skip(app, user, data),fields(user_id = %user.profile_id.to_string(), artist_id = %data.artist_id))]
async fn unfollow_artist_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<ArtistActionReq>,
) -> Result<ProfileResponse, ApiError> {
    let res = delete_follower(&app.db_pool, user.profile_id, data.artist_id).await?;
    Ok(ProfileResponse::UnfollowedArtist(res))
}

#[instrument(name = "add to favorite profiles", skip(app, user, data),fields(user_id = %user.profile_id.to_string(), artist_id = %data.artist_id))]
async fn add_to_favorite_profiles_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<ArtistActionReq>,
) -> Result<ProfileResponse, ApiError> {
    let res = insert_new_favorite(&app.db_pool, user.profile_id, data.artist_id).await?;
    Ok(ProfileResponse::FavoritedArtist(res))
}

#[instrument(name = "remove from favorite profiles", skip(app, user, data),fields(user_id = %user.profile_id.to_string(), artist_id = %data.artist_id))]
async fn remove_from_favorite_profiles_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<ArtistActionReq>,
) -> Result<ProfileResponse, ApiError> {
    let res = delete_favorite(&app.db_pool, user.profile_id, data.artist_id).await?;
    Ok(ProfileResponse::FavoriteArtistRemoved(res))
}

// async fn update_profile_presence_handler() -> Result<ApiResponse, ApiError> {
//     todo!()
// }

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/update", post(update_stage_details_handler))
        .route("/follow", post(follow_artist_handler))
        .route("/unfollow", post(unfollow_artist_handler))
        .route("/favorite", post(add_to_favorite_profiles_handler))
        .route("/unfavorite", post(remove_from_favorite_profiles_handler))
    // .route(
    //     "/update_profile_presence",
    //     post(update_profile_presence_handler),
    // )
}
