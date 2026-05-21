#![allow(dead_code)]
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

    pub async fn post_set(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/sets/new", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_festival(
        &self,
        set_id: uuid::Uuid,
        body: &serde_json::Value,
    ) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/sets/{}/new_festival", &self.address, set_id))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_ledger(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/ledger/new", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_update_profile(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/artists/update", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_follow(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/artists/follow", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_unfollow(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/artists/unfollow", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_favorite(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/artists/favorite", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_unfavorite(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/artists/unfavorite", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_reset_password(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/auth/reset-password", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    // -----------------------------------------------------------------------
    // Admin auth
    // -----------------------------------------------------------------------

    pub async fn post_admin_register(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/auth/admin/register", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_admin_login(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/auth/admin/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    // -----------------------------------------------------------------------
    // Sets
    // -----------------------------------------------------------------------

    pub async fn post_update_set(
        &self,
        set_id: uuid::Uuid,
        body: &serde_json::Value,
    ) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/sets/{}/update", &self.address, set_id))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_join_set(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/sets/join", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn delete_leave_set(&self, set_id: uuid::Uuid) -> reqwest::Response {
        self.api_client
            .delete(&format!("{}/sets/{}/leave", &self.address, set_id))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    // -----------------------------------------------------------------------
    // Works
    // -----------------------------------------------------------------------

    pub async fn post_update_work(
        &self,
        work_id: uuid::Uuid,
        body: &serde_json::Value,
    ) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/works/{}/update", &self.address, work_id))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_like_work(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/works/like", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn delete_dislike_work(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .delete(&format!("{}/works/dislike", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn delete_work(&self, work_id: uuid::Uuid) -> reqwest::Response {
        self.api_client
            .delete(&format!("{}/works/{}/delete", &self.address, work_id))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    // -----------------------------------------------------------------------
    // Originals
    // -----------------------------------------------------------------------

    pub async fn post_update_original(
        &self,
        original_id: uuid::Uuid,
        body: &serde_json::Value,
    ) -> reqwest::Response {
        self.api_client
            .post(&format!(
                "{}/originals/{}/update",
                &self.address, original_id
            ))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_add_role(
        &self,
        original_id: uuid::Uuid,
        body: &serde_json::Value,
    ) -> reqwest::Response {
        self.api_client
            .post(&format!(
                "{}/originals/{}/new_role",
                &self.address, original_id
            ))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn delete_role(
        &self,
        original_id: uuid::Uuid,
        body: &serde_json::Value,
    ) -> reqwest::Response {
        self.api_client
            .delete(&format!(
                "{}/originals/{}/delete_role",
                &self.address, original_id
            ))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn delete_original(&self, original_id: uuid::Uuid) -> reqwest::Response {
        self.api_client
            .delete(&format!(
                "{}/originals/{}/delete",
                &self.address, original_id
            ))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    // -----------------------------------------------------------------------
    // Stages
    // -----------------------------------------------------------------------

    pub async fn get_profile_stage(&self, user_name: &str) -> reqwest::Response {
        self.api_client
            .get(&format!(
                "{}/stages/get_profile_stage/{}",
                &self.address, user_name
            ))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    // -----------------------------------------------------------------------
    // Festivals
    // -----------------------------------------------------------------------

    pub async fn post_update_festival(
        &self,
        festival_id: uuid::Uuid,
        body: &serde_json::Value,
    ) -> reqwest::Response {
        self.api_client
            .post(&format!(
                "{}/festivals/{}/update",
                &self.address, festival_id
            ))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_update_panelists(
        &self,
        festival_id: uuid::Uuid,
        body: &serde_json::Value,
    ) -> reqwest::Response {
        self.api_client
            .post(&format!(
                "{}/festivals/{}/panelists/update",
                &self.address, festival_id
            ))
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
        db_pool: pool,
        jwt_secret: config.jwt_secret,
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
