use std::sync::Arc;

use axum::{extract::State};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;
use time;

use crate::{
    AppState,
    db::artists::{get_profile_auth_details, insert_new_profile},
    errors::ApiError,
    types::{
        db::profile::{Profile, ProfileType},
        requests::auth::{ProfileLogin, ProfileSignupReq},
        response::ApiResponse,
    },
    utils::{auth::{create_jwt, get_password_hash, verify_password}, json_extractor::AppJson},
};

#[instrument(name = "sign_up_artist", skip(app, data), err,fields(user_name = %data.user_name))]
pub async fn sign_up_artist_handler(
    State(app): State<Arc<AppState>>,
    AppJson(data): AppJson<ProfileSignupReq>,
) -> Result<ApiResponse, ApiError> {
    let password = data.password;
    let password_hash = get_password_hash(password.as_ref())?;
    let artist = Profile {
        user_name: data.user_name.as_ref().to_string(),
        is_claimed: false,
        presence: 100,
        id: Uuid::new_v4(),
        tag_line: data.tag_line.as_ref().to_string(),
        profile_picture: data.profile_picture,
        profile_type: ProfileType::ARTIST,
        youtube_profile: data.youtube_profile,
        twitter_profile: data.twitter_profile,
        instagram_profile: data.instagram_profile,
        password_hash,
        created_at: Utc::now(),
    };
    let profile = insert_new_profile(&app.pool, artist).await?;
    match profile {
        Some(_) => tracing::info!("Artist created"),
        None => tracing::error!("Failed to create artist"),
    };
    Ok(ApiResponse::OK)
}

#[instrument(name = "log_in_artist", skip(app, jar, data), err,fields(user_name = %data.user_name))]
pub async fn log_in_artist_handler(
    State(app): State<Arc<AppState>>,
    jar: CookieJar,
    AppJson(data): AppJson<ProfileLogin>,
) -> Result<ApiResponse, ApiError> {
    let password = data.password;
    let (password_hash, user_id) = get_profile_auth_details(&app.pool, data.user_name).await?;
    verify_password(password.as_ref(), &password_hash)?;
    let token = create_jwt(&user_id, "USER", app.secret.as_bytes())?;
    let cookie = Cookie::build(("auth_token", token))
        .http_only(true)
        .secure(true)
        .path("/")
        .max_age(time::Duration::days(7))
        .build();
    tracing::info!("Artist logged in successfully");
    Ok(ApiResponse::ProfileLoggedIn(jar.add(cookie)))
}
