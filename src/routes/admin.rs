use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::post};
use tracing::instrument;

use crate::{
    AppState,
    db::mutations::{
        artists::update_profile_role,
        roles_and_permissions::{
            delete_permission_for_role, insert_new_permission, insert_new_user_role,
            insert_permission_for_role,
        },
    },
    errors::ApiError,
    models::{
        db::roles_and_permissions::{Permission, UserRole},
        requests::admin::{
            AssignPermissionRequest, CreatePermissionRequest, CreateRoleRequest,
            RevokePermissionRequest, UpdateProfileRoleReq,
        },
        response::AdminResponse,
    },
    services::auth_service::extractor::AdminUser,
};

#[instrument(name = "create new role", skip(app))]
async fn create_new_role_handler(
    State(app): State<Arc<AppState>>,
    AdminUser(_): AdminUser,
    Json(data): Json<CreateRoleRequest>,
) -> Result<AdminResponse, ApiError> {
    let res = insert_new_user_role(
        &app.db_pool,
        UserRole {
            name: data.name.to_string(),
            description: data.description,
            created_at: chrono::Utc::now(),
        },
    )
    .await?;
    Ok(AdminResponse::NewRoleCreated(res.name))
}

#[instrument(name = "create new permission", skip(app))]
async fn create_new_permission_handler(
    State(app): State<Arc<AppState>>,
    AdminUser(_): AdminUser,
    Json(data): Json<CreatePermissionRequest>,
) -> Result<AdminResponse, ApiError> {
    let res = insert_new_permission(
        &app.db_pool,
        Permission {
            name: data.name.to_string(),
            description: data.description,
            created_at: chrono::Utc::now(),
        },
    )
    .await?;
    Ok(AdminResponse::NewPermissionCreated(res.name))
}

#[instrument(name = "assign new permission to the role", skip(app))]
async fn assign_new_permission_handlr(
    State(app): State<Arc<AppState>>,
    AdminUser(_): AdminUser,
    Json(data): Json<AssignPermissionRequest>,
) -> Result<AdminResponse, ApiError> {
    let _ = insert_permission_for_role(&app.db_pool, data.role.to_string(), data.permission.to_string()).await?;
    Ok(AdminResponse::PermissionAssigned)
}

// this completly revokes the permission to a specific role
async fn revoke_permission_handler(
    State(app): State<Arc<AppState>>,
    AdminUser(_): AdminUser,
    Json(data): Json<RevokePermissionRequest>,
) -> Result<AdminResponse, ApiError> {
    let res = delete_permission_for_role(&app.db_pool, data.role.to_string(), data.permission.to_string()).await?;
    Ok(AdminResponse::PermissionRevoked(res))
}

// this updates the role for a specific profile
async fn update_profile_role_handler(
    State(app): State<Arc<AppState>>,
    AdminUser(_): AdminUser,
    Json(data): Json<UpdateProfileRoleReq>,
) -> Result<AdminResponse, ApiError> {
    let _ = update_profile_role(&app.db_pool, data.new_role.to_string(), data.profile_id).await?;
    Ok(AdminResponse::ProfileRoleUpdated)
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/new_role", post(create_new_role_handler))
        .route("/new_permission", post(create_new_permission_handler))
        .route("/revoke_permission", post(revoke_permission_handler))
        .route("/assign_permission", post(assign_new_permission_handlr))
        .route("/update_user_role", post(update_profile_role_handler))
}
