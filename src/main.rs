use sqlx::postgres::PgPoolOptions;
use tars::{configuration::get_configuration, startup::run};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();
    let config = get_configuration().expect("failed to read configuration");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).await.unwrap();
    let db_url = std::env::var("DATABASE_URL").expect("database url must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("failed to connect to the database");
    run(listener, pool).await?.await
}
