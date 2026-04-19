use std::sync::Arc;
use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use tokio;

mod types;

#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL").expect("database url must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("failed to connect to the database");
    let app_state = Arc::new(pool);
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/artists", get(get_artist_handler))
        .with_state(app_state);

    let listner = tokio::net::TcpListener::bind("127.0.0.1:8484")
        .await
        .unwrap();
    println!(
        "Server started successfully at {}",
        listner.local_addr().unwrap()
    );
    axum::serve(listner, app).await.unwrap();
}

async fn home_handler() {}

async fn get_artist_handler() {}
