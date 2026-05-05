use sqlx::postgres::PgPoolOptions;
use tars::{AppState, configuration::get_configuration, startup::run};
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=warn,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();
    init_tracing();
    let config = get_configuration().expect("failed to read configuration");
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address).await.unwrap();
    let db_url = std::env::var("DATABASE_URL").expect("database url must be set");
    let secret: String =
        std::env::var("JWT_SIGNER_SECRET").expect("SECRET NOT SET FOR THE JWT SIGNING");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&db_url)
        .expect("failed to connect to the database");
    let app = AppState { pool, secret };
    run(listener, app).await?.await
}
