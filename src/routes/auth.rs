use std::sync::Arc;

use axum::{Router, extract::State, routing::post};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use chrono::Utc;
use time;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    AppState,
    db::artists::{get_profile_auth_details, insert_new_profile, update_profile_password},
    errors::ApiError,
    types::{
        db::profile::{Profile, ProfileType},
        requests::auth::{ProfileLogin, ProfileSignupReq, ResetPasswordReq},
        response::ApiResponse,
    },
    utils::{
        auth::{
            extractor::Artist,
            password::{create_jwt, get_password_hash, verify_password},
        },
        json_extractor::AppJson,
    },
};

#[instrument(name = "sign_up_artist", skip(app, data), err,fields(user_name = %data.handle))]
pub async fn sign_up_artist_handler(
    State(app): State<Arc<AppState>>,
    AppJson(data): AppJson<ProfileSignupReq>,
) -> Result<ApiResponse, ApiError> {
    let password = data.password;
    let password_hash = get_password_hash(password.as_ref())?;
    let artist = Profile {
        user_name: data.handle.as_ref().to_string(),
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
    let profile = insert_new_profile(&app.db_pool, artist).await?;
    match profile {
        Some(_) => tracing::info!("Artist created"),
        None => tracing::error!("Failed to create artist"),
    };
    Ok(ApiResponse::OK)
}

#[instrument(name = "log_in_artist", skip(app, jar, data), err,fields(user_name = %data.handle))]
pub async fn login_profile(
    State(app): State<Arc<AppState>>,
    jar: CookieJar,
    AppJson(data): AppJson<ProfileLogin>,
) -> Result<ApiResponse, ApiError> {
    let password = data.password;
    let profile = get_profile_auth_details(&app.db_pool, &data.handle.to_string()).await?;
    let password_hash = match &profile {
        Some(profile) => &profile.password_hash,
        None => "$argon2id$v=19$m=19456,t=2,p=1$dummysaltdummysalt$dummyhash",
    };
    let valid_password = verify_password(password.as_ref(), password_hash)?;
    let user = match (profile, valid_password) {
        (Some(profile), true) => profile,
        _ => return Err(ApiError::Unauthorized("Invalid credentials".to_string())),
    };
    let token = create_jwt(&user.user_name, "artist", &app.jwt_secret, user.id)?;
    let cookie = Cookie::build(("auth_token", token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .build();
    Ok(ApiResponse::ProfileAuthenticated(jar.add(cookie)))
}

#[instrument(name = "logout_artist", skip(jar), err)]
pub async fn logout_profile(jar: CookieJar) -> Result<ApiResponse, ApiError> {
    let cookie = Cookie::build(("auth_token", ""))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::seconds(0))
        .build();
    Ok(ApiResponse::ProfileAuthenticated(jar.remove(cookie)))
}

#[instrument(name = "reset_password", skip(app, user, data), err, fields(user_id = %user.profile_id))]
pub async fn reset_password(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<ResetPasswordReq>,
) -> Result<ApiResponse, ApiError> {
    let old_profile = get_profile_auth_details(&app.db_pool, &user.handle).await?;
    let password_hash = match &old_profile {
        Some(profile) => &profile.password_hash,
        None => "$argon2id$v=19$m=19456,t=2,p=1$dummysaltdummysalt$dummyhash",
    };
    let valid_password = verify_password(data.old_password.as_ref(), password_hash)?;
    let user = match (old_profile, valid_password) {
        (Some(profile), true) => profile,
        _ => return Err(ApiError::Unauthorized("Invlaid Operation".to_string())),
    };
    let password_hash = get_password_hash(data.new_password.as_ref())?;
    let user_id = update_profile_password(&app.db_pool, user.id, password_hash)
        .await?
        .ok_or(ApiError::DbError(sqlx::Error::RowNotFound))?;
    Ok(ApiResponse::PasswordUpdated(user_id))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(sign_up_artist_handler))
        .route("/login", post(login_profile))
        .route("/logout", post(logout_profile))
        .route("/reset-password", post(reset_password))
}
