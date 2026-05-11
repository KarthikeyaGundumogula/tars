use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
use std::sync::Arc;

use crate::{AppState, errors::ApiError, utils::auth::password::validate_jwt};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub profile_id: uuid::Uuid,
    pub handle: String,
    pub role: String,
}
impl FromRequestParts<Arc<AppState>> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await?;
        let token = jar
            .get("auth_token")
            .map(|cookie| cookie.value().to_owned())
            .ok_or_else(|| ApiError::Unauthorized("Token not found".into()))?;
        let claims = validate_jwt(&token, &state.secret)?;
        Ok(AuthUser {
            profile_id: claims.profile_id,
            handle: claims.sub,
            role: claims.role,
        })
    }
}

pub struct AdminUser(pub AuthUser);

impl FromRequestParts<Arc<AppState>> for AdminUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_user = AuthUser::from_request_parts(parts, state).await?;
        if auth_user.role != "admin" {
            return Err(ApiError::Unauthorized("Not an admin".into()));
        }
        Ok(AdminUser(auth_user))
    }
}

pub struct SetCaptain(pub AuthUser);

impl FromRequestParts<Arc<AppState>> for SetCaptain {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_user = AuthUser::from_request_parts(parts, state).await?;
        if auth_user.role != "captain" {
            return Err(ApiError::Unauthorized("Not a captain".into()));
        }
        Ok(SetCaptain(auth_user))
    }
}

pub struct FestivalPanelist(pub AuthUser);

impl FromRequestParts<Arc<AppState>> for FestivalPanelist {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_user = AuthUser::from_request_parts(parts, state).await?;
        if auth_user.role != "panelist" {
            return Err(ApiError::Unauthorized("Not a panelist".into()));
        }
        Ok(FestivalPanelist(auth_user))
    }
}

pub struct Artist(pub AuthUser);

impl FromRequestParts<Arc<AppState>> for Artist {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_user = AuthUser::from_request_parts(parts, state).await?;
        if auth_user.role != "artist" {
            return Err(ApiError::Unauthorized("Not an artist".into()));
        }
        Ok(Artist(auth_user))
    }
}

