pub mod admin;
pub mod artists;
pub mod auth;
pub mod festivals;
pub mod health_check;
pub mod library;
pub mod originals;
pub mod profiles;
pub mod sets;
pub mod works;

use crate::AppState;
use axum::Router;
use std::sync::Arc;

pub fn build_router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/health_check",
            axum::routing::get(health_check::health_check_handler),
        )
        .nest("/auth", auth::router())
        .nest("/originals", originals::router())
        .nest("/works", works::router())
        .nest("/sets", sets::router())
        .nest("/festivals", festivals::router())
        .nest("/library", library::router())
        .nest("/artists", artists::router())
        .nest("/profiles", profiles::router())
        .nest("/admin", admin::router())
}
