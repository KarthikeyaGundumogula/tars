use std::sync::Arc;

use axum::{Json, extract::State};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use chrono::Utc;
use uuid::Uuid;

use crate::{
    AppState, db::artists::{get_profile_auth_details, insert_new_profile}, errors::ApiError, types::{
        db::profile::{Profile, ProfileType},
        requests::auth::{ProfileLogin, ProfileSignupReq},
        response::ApiResponse,
    }, utils::auth::{create_jwt, get_password_hash, verify_password}
};

pub async fn sign_up_artist_handler(
    State(app): State<Arc<AppState>>,
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
    insert_new_profile(&app.pool, artist).await?;
    Ok(ApiResponse::OK)
}

pub async fn log_in_artist_handler(
    State(app): State<Arc<AppState>>,
    jar: CookieJar,
    Json(data): Json<ProfileLogin>,
) -> Result<ApiResponse, ApiError> {
    let password = data.password;
    let (password_hash, user_id) = get_profile_auth_details(&app.pool, data.user_name).await?;
    verify_password(&password, &password_hash)?;
    let token = create_jwt(&user_id, "USER", app.secret.as_bytes())?;
     let cookie = Cookie::build(("auth_token", token))
        .http_only(true)
        .secure(true)
        .path("/")
        .max_age(time::Duration::days(7))
        .build();
    Ok(ApiResponse::ProfileLoggedIn(jar.add(cookie)))
}
