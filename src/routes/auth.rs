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
    db::mutations::{
        admins::{get_admin_auth_details, insert_new_admin},
        artists::{get_profile_auth_details, insert_new_profile, update_profile_password},
    },
    errors::ApiError,
    models::{
        db::{
            admin::Admin,
            profile::{Profile, ProfileType},
        },
        requests::{
            admin::AdminAuthRequest,
            auth::{ProfileLogin, ProfileSignupReq, ResetPasswordReq},
        },
        response::{AdminResponse, ProfileResponse},
    },
    services::{
        auth_service::{
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
) -> Result<ProfileResponse, ApiError> {
    let password = data.password;
    let password_hash = get_password_hash(password.as_ref())?;
    let artist = Profile {
        user_name: data.handle.as_ref().to_string(),
        is_claimed: false,
        stage_name: data.stage_name.to_string(),
        spirit: 100,
        id: Uuid::new_v4(),
        tag_line: data.tag_line.as_ref().to_string(),
        profile_picture: data.profile_picture,
        profile_type: ProfileType::ARTIST,
        youtube_profile: data.youtube_profile,
        twitter_profile: data.twitter_profile,
        instagram_profile: data.instagram_profile,
        password_hash,
        color_theme: data.color_theme.to_string(),
        role_name: None,
        current_peak_recommendations: 0,
        current_peak_library: 0,
        created_at: Utc::now(),
    };
    let profile = insert_new_profile(&app.db_pool, artist).await?;
    match profile {
        Some(profile) => {
            tracing::info!("Artist created: {:?}", profile.id);
            Ok(ProfileResponse::ProfileCreated(profile.id))
        }
        None => {
            tracing::error!("Failed to create artist");
            Err(ApiError::DbError(sqlx::Error::RowNotFound))
        }
    }
}

#[instrument(name = "log_in_artist", skip(app, jar, data), err,fields(user_name = %data.handle))]
pub async fn login_profile(
    State(app): State<Arc<AppState>>,
    jar: CookieJar,
    AppJson(data): AppJson<ProfileLogin>,
) -> Result<ProfileResponse, ApiError> {
    let password = data.password;
    let profile = get_profile_auth_details(&app.db_pool, &data.handle.to_string()).await?;
    let fallback_hash = get_password_hash("invalid-login-placeholder")?;
    let password_hash = match &profile {
        Some(profile) => profile.password_hash.as_str(),
        None => fallback_hash.as_str(),
    };
    let valid_password = verify_password(password.as_ref(), password_hash)?;
    let user = match (profile, valid_password) {
        (Some(profile), true) => profile,
        _ => return Err(ApiError::Unauthorized("Invalid credentials".to_string())),
    };
    let token = create_jwt(
        &user.user_name,
        &user.role_name.unwrap_or("artist".to_string()),
        &app.jwt_secret,
        user.id,
    )?;
    let cookie = Cookie::build(("auth_token", token))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .build();
    Ok(ProfileResponse::ProfileAuthenticated(jar.add(cookie)))
}

#[instrument(name = "logout_artist", skip(jar), err)]
pub async fn logout_profile_handler(jar: CookieJar) -> Result<ProfileResponse, ApiError> {
    let cookie = Cookie::build(("auth_token", ""))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::seconds(0))
        .build();
    Ok(ProfileResponse::ProfileAuthenticated(jar.remove(cookie)))
}

#[instrument(name = "insert_new_admin",skip(app,data),fields(admin_name = %data.admin_name))]
pub async fn insert_new_admin_handler(
    State(app): State<Arc<AppState>>,
    AppJson(data): AppJson<AdminAuthRequest>,
) -> Result<AdminResponse, ApiError> {
    let password = data.admin_password;
    let password_hash = get_password_hash(password.as_ref())?;
    let admin = Admin {
        admin_name: data.admin_name.to_string(),
        admin_password_hash: password_hash,
        admin_id: Uuid::new_v4(),
        created_at: Utc::now(),
    };
    let res = insert_new_admin(&app.db_pool, admin).await?;
    Ok(AdminResponse::AdminCreated(res))
}

#[instrument(name = "admin_login",skip(app,data,jar),fields(admin_name = %data.admin_name))]
pub async fn admin_login_handler(
    State(app): State<Arc<AppState>>,
    jar: CookieJar,
    AppJson(data): AppJson<AdminAuthRequest>,
) -> Result<AdminResponse, ApiError> {
    let password = data.admin_password;
    // we have to get the role of the user here, this role should go to the jwt claim
    let admin = get_admin_auth_details(&app.db_pool, data.admin_name.as_ref()).await?;
    let fallback_hash = get_password_hash("invalid-login-placeholder")?;
    let password_hash = match &admin {
        Some(admin) => admin.admin_password_hash.as_str(),
        None => fallback_hash.as_str(),
    };
    let valid_password = verify_password(password.as_ref(), password_hash)?;
    let user = match (admin, valid_password) {
        (Some(admin), true) => {
            tracing::info!("Admin login successful for admin: {}", admin.admin_name);
            admin
        }
        _ => return Err(ApiError::Unauthorized("Invalid credentials".to_string())),
    };
    let token = create_jwt(&user.admin_name, "admin", &app.jwt_secret, user.admin_id)?;
    let cookie = Cookie::build(("auth_token", token))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .build();
    Ok(AdminResponse::AdminAuthenticated(jar.add(cookie)))
}

#[instrument(name = "logout_admin", skip(jar))]
pub async fn logout_admin_handler(jar: CookieJar) -> Result<AdminResponse, ApiError> {
    let cookie = Cookie::build(("auth_token", ""))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::seconds(0))
        .build();
    Ok(AdminResponse::AdminAuthenticated(jar.remove(cookie)))
}

#[instrument(name = "reset_password", skip(app, user, data), err, fields(user_id = %user.profile_id))]
pub async fn reset_password(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<ResetPasswordReq>,
) -> Result<ProfileResponse, ApiError> {
    let old_profile = get_profile_auth_details(&app.db_pool, &user.handle).await?;
    let fallback_hash = get_password_hash("invalid-login-placeholder")?;
    let password_hash = match &old_profile {
        Some(profile) => profile.password_hash.as_str(),
        None => fallback_hash.as_str(),
    };
    let valid_password = verify_password(data.old_password.as_ref(), password_hash)?;
    let user = match (old_profile, valid_password) {
        (Some(profile), true) => profile,
        _ => return Err(ApiError::Unauthorized("Invalid Operation".to_string())),
    };
    let password_hash = get_password_hash(data.new_password.as_ref())?;
    let user_id = update_profile_password(&app.db_pool, user.id, password_hash)
        .await?
        .ok_or(ApiError::DbError(sqlx::Error::RowNotFound))?;
    Ok(ProfileResponse::PasswordUpdated(user_id))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(sign_up_artist_handler))
        .route("/login", post(login_profile))
        .route("/logout", post(logout_profile_handler))
        .route("/reset-password", post(reset_password))
        .route("/admin/login", post(admin_login_handler))
        .route("/admin/logout", post(logout_admin_handler))
        .route("/admin/register", post(insert_new_admin_handler))
}
