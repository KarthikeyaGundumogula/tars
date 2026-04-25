use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
    serve::Serve,
};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use crate::routes::{
    artists::register_artist_handler, health_check::health_check_handler,
    works::create_new_work_handler,
};

pub async fn run(
    listner: TcpListener,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("database url must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("failed to connect to the database");
    let app_state = Arc::new(pool);
    let app = Router::new()
        .route("/artist/register", post(register_artist_handler))
        .route("/health_check", get(health_check_handler))
        // .route("/artists", get(get_artist_handler))
        .route("/works/new/{work_type}", post(create_new_work_handler))
        .with_state(app_state.clone());
    println!(
        "Server started successfully at {}",
        listner.local_addr().unwrap()
    );
    Ok(axum::serve(listner, app))
}
