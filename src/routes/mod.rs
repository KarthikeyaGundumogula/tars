pub mod artists;
pub mod health_check;
pub mod originals;
pub mod works;

use axum::Router;
use std::sync::Arc;
use crate::AppState;

pub fn build_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health_check", axum::routing::get(health_check::health_check_handler))
        .nest("/artist", artists::router())
        .nest("/originals", originals::router())
        .nest("/works", works::router())
}
