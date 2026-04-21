use axum::{
    Json, Router,
    routing::{get, post},
    serve::Serve,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::{self, net::TcpListener};

use crate::{
    errors::ApiError, response::ApiResponse, types::requests::upload_works::UploadEditData,
};

mod errors;
mod response;
mod types;

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
        .route("/", get(home_handler))
        .route("/health_check", get(health_check_handler))
        .route("/artists", get(get_artist_handler))
        .route("/works/edits/new", post(create_new_work_handler))
        .with_state(app_state);
    println!(
        "Server started successfully at {}",
        listner.local_addr().unwrap()
    );
    Ok(axum::serve(listner, app))
}

async fn health_check_handler() -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::OK)
}

async fn create_new_work_handler(
    Json(data): Json<UploadEditData>,
) -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::OK)
}

async fn home_handler() {}

async fn get_artist_handler() {}
