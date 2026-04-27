mod utils;

use reqwest::{Client, Response};
use sqlx::{Connection, PgConnection};
use tars::configuration::get_configuration;
use tars::types::db::artist::Artist;
use utils::spawn_app;

#[tokio::test]
async fn register_profile_return_200_on_correct_data() {
    let app = spawn_app::spawn().await;

    let client = Client::new();
    let body = serde_json::json!({
      "user_name":"kapten",
      "tag_line":"I will never care for you",
      "password":"kapten@1023",
      "profile_picture":"aofdjosfjosf",
      "youtube_profile":"aojojfosjf"
    });
    let response: Response = client
        .post(&format!("{}/artist/register", app.address))
        .json(&body)
        .send()
        .await
        .expect("failed to execute request");
    assert!(response.status().is_success());
    let saved = sqlx::query_as!(
        Artist,
        r#"SELECT * FROM profiles WHERE user_name=$1"#,
        "kapten"
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("db query failed");
    assert_eq!(saved.youtube_profile, Some("aojojfosjf".to_string()));
}

#[tokio::test]
async fn register_profile_return_error_on_incorrect_data() {
    let app = spawn_app::spawn().await;
    let config = get_configuration().expect("unable to get the config");
    let connection_string = config.database.connection_string();
    let _ = PgConnection::connect(&connection_string)
        .await
        .expect("unable to connect to postgres");
    let client = Client::new();
    let body = serde_json::json!({
      "user_name":"kapten",
      "password":"kapten@1023",
    });
    let response: Response = client
        .post(&format!("{}/artist/register", app.address))
        .json(&body)
        .send()
        .await
        .expect("failed to execute request");
    assert!(response.status().is_client_error());
}
