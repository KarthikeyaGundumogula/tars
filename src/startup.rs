use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
    serve::Serve,
};
use tokio::net::TcpListener;

use crate::{AppState, routes::{
    artists::sign_up_artist_handler, health_check::health_check_handler,
    originals::create_new_original_handler, works::create_new_work_handler,
}};

pub async fn run(
    listner: TcpListener,
    app:AppState
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    let app_state = Arc::new(app);
    let app = Router::new()
        .route("/health_check", get(health_check_handler))
        .route("/artist/register", post(sign_up_artist_handler))
        .route("/originals/new", post(create_new_original_handler))
        // .route("/artists", get(get_artist_handler))
        .route("/works/new/{work_type}", post(create_new_work_handler))
        .with_state(app_state.clone());
    println!(
        "Server started successfully at {}",
        listner.local_addr().unwrap()
    );
    Ok(axum::serve(listner, app))
}
