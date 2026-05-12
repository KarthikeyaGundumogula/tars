use std::sync::Arc;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    serve::Serve,
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::{
    AppState,
    routes,
};

pub async fn run(
    listner: TcpListener,
    app: AppState,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    let app_state = Arc::new(app);
    let app = Router::new()
        .merge(routes::build_router())
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(5 * 1024 * 1024))
        .with_state(app_state.clone());
    println!(
        "Server started successfully at {}",
        listner.local_addr().unwrap()
    );
    Ok(axum::serve(listner, app))
}
