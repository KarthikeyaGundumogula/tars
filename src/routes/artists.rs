use std::sync::Arc;

use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db::artists::insert_new_artist,
    errors::ApiError,
    types::{
        db::profile::{Profile, ProfileType},
        requests::auth::ProfileSignupReq,
        response::ApiResponse,
    },
    utils::password::get_password_hash,
};

pub async fn register_artist_handler(
    State(pool): State<Arc<PgPool>>,
    Json(data): Json<ProfileSignupReq>,
) -> Result<ApiResponse, ApiError> {
    let password = data.password;
    let password_hash = get_password_hash(&password)?;
    let artist = Profile {
        user_name: data.user_name,
        is_claimed: false,
        presence: 100,
        id: Uuid::new_v4(),
        tag_line: data.tag_line,
        profile_picture: data.profile_picture,
        profile_type: ProfileType::ARTIST,
        youtube_profile: data.youtube_profile,
        twitter_profile: data.twitter_profile,
        instagram_profile: data.instagram_profile,
        password_hash,
        created_at: Utc::now(),
    };
    insert_new_artist(&pool, artist).await?;
    // verify_password(&password, &password_hash)?;
    Ok(ApiResponse::OK)
}
