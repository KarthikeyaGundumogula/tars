use tars::{AppState, configuration::get_configuration, startup::run};
use tokio::net::TcpListener;
use uuid::Uuid;

use crate::utils::postgres_config::configure_postgres;

pub struct TestApp {
    pub address: String,
    pub state: AppState,
}

pub async fn spawn() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let mut config = get_configuration().expect("unable to get the config");
    config.database.database_name = Uuid::new_v4().to_string();
    let pool = configure_postgres(config.database).await;
    let app = AppState {
        pool,
        secret: config.jwt_secret,
    };
    let server = run(listener, app.clone())
        .await
        .expect("Failed to bind address");
    let _ = tokio::spawn(async move {
        let _ = server.await;
    });
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        state: app,
    }
}
