use std::sync::Arc;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    serve::Serve,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::{AppState, routes};

pub async fn run(
    listner: TcpListener,
    app: AppState,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    let cors_layer = CorsLayer::new()
        .allow_origin(
            HeaderValue::from_str("http://localhost:3000")
                .map_err(|_| std::io::Error::other("Invalid header value"))?,
        )
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_credentials(true);
    let app_state = Arc::new(app);
    let app = Router::new()
        .merge(routes::build_router())
        .layer(cors_layer)
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(5 * 1024 * 1024))
        .with_state(app_state.clone());
    println!(
        "Server started successfully at {}",
        listner.local_addr().unwrap()
    );
    Ok(axum::serve(listner, app))
}
