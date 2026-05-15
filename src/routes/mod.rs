pub mod auth;
pub mod health_check;
pub mod originals;
pub mod works;
pub mod sets;
pub mod festivals;
pub mod ledger;
pub mod artists;

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
        .nest("/ledger", ledger::router())
        .nest("/artists", artists::router())
}
