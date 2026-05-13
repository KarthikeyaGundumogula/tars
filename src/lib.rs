use sqlx::{Pool, Postgres};

pub mod configuration;
pub mod db;
pub mod domain;
pub mod errors;
pub mod routes;
pub mod startup;
pub mod types;
pub mod utils;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
    pub jwt_secret: String,
}
