use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use axum_extra::extract::CookieJar;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{AppState, errors::ApiError, shared::auth::password::validate_jwt};

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
        let claims = validate_jwt(&token, &state.jwt_secret)?;
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

pub struct OwnedResourceOrAdmin<T: Resource> {
    pub user_id: Uuid,
    pub resource_id: Uuid,
    pub resource: T,
}

pub trait Resource: Send + Sync + Sized {
    fn fetch_by_id(
        db: &PgPool,
        resource_id: Uuid,
    ) -> impl Future<Output = Result<Option<(Uuid, Self)>, ApiError>> + Send
    where
        Self: Send;
}

impl<T: Resource + Send> FromRequestParts<Arc<AppState>> for OwnedResourceOrAdmin<T> {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_user = AuthUser::from_request_parts(parts, state).await?;
        let Path(resource_id) = Path::<Uuid>::from_request_parts(parts, state).await?;
        let res = T::fetch_by_id(&state.db_pool, resource_id).await?;
        let (owner, resource) = res.ok_or(ApiError::NotFound)?;
        if owner != auth_user.profile_id && auth_user.role != "admin" {
            return Err(ApiError::Unauthorized("Not permitted action".into()));
        }
        Ok(OwnedResourceOrAdmin {
            resource_id,
            user_id: owner,
            resource,
        })
    }
}

pub struct EntityMemberOrAdmin<T: Entity> {
    pub user_id: Uuid,
    pub entity_id: Uuid,
    pub entity: T,
}

#[derive(serde::Deserialize)]
pub struct EntityParam {
    pub entity_id: Uuid,
}

pub trait Entity: Send + Sync + Sized {
    fn fetch_membership_and_entity(
        db: &PgPool,
        entity_id: Uuid,
        member_id: Uuid,
    ) -> impl Future<Output = Result<Option<(bool, Self)>, ApiError>> + Send
    where
        Self: Send;
}

impl<T: Entity + Send> FromRequestParts<Arc<AppState>> for EntityMemberOrAdmin<T> {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_user = AuthUser::from_request_parts(parts, state).await?;
        let Path(EntityParam { entity_id }) =
            Path::<EntityParam>::from_request_parts(parts, state).await?;
        let (is_member, entity) =
            T::fetch_membership_and_entity(&state.db_pool, entity_id, auth_user.profile_id)
                .await?
                .ok_or(ApiError::NotFound)?;
        if !is_member && auth_user.role != "admin" {
            return Err(ApiError::Unauthorized("Not permitted action".into()));
        }
        Ok(EntityMemberOrAdmin {
            entity_id,
            user_id: auth_user.profile_id,
            entity,
        })
    }
}
