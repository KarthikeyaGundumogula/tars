use std::sync::Arc;

use axum::{Router, routing::post};

use crate::AppState;

async fn create_new_role_handler() {}

async fn create_new_permission_handler() {}

// this completly revokes the permission to a specific role
async fn revoke_permission_handler() {}

// this updates the role for a specific profile
async fn update_role_handler() {}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/new_role", post(create_new_role_handler))
        .route("/new_permission", post(create_new_permission_handler))
        .route("/revoke_permission", post(revoke_permission_handler))
        .route("/update_role", post(update_role_handler))
}
