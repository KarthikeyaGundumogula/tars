use tars::{AppState, configuration::get_configuration, startup::run};
use tokio::net::TcpListener;
use uuid::Uuid;

use crate::common::postgres_config::configure_postgres;

pub struct TestApp {
    pub address: String,
    pub state: AppState,
    pub api_client: reqwest::Client,
}

impl TestApp {
    pub async fn post_login(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/auth/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_register(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/auth/register", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_work(&self, work_type: &str, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/works/new/{}", &self.address, work_type))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_original(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/originals/new", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
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

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        state: app,
        api_client: client,
    }
}
