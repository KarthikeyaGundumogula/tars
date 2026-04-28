mod utils;

use reqwest::{Client, Response};
use sqlx::{Connection, PgConnection};
use tars::configuration::get_configuration;
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
    let saved = sqlx::query_scalar!(
        r#"SELECT youtube_profile FROM profiles WHERE user_name=$1"#,
        "kapten"
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("db query failed");
    assert_eq!(saved, Some("aojojfosjf".to_string()));
}

#[tokio::test]
async fn register_profile_return_error_on_incorrect_data() {
    let app = spawn_app::spawn().await;
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

#[tokio::test]
async fn register_original_return_success_on_correct_data() {
    let app = spawn_app::spawn().await;
    let client = Client::new();
    let body = serde_json::json!({
        "title":"They Call him Og",
        "password": "1234",
        "associated_with":"DVV Entertainments",
        "actors":[{
            "role":"Ojas Ghambheera",
            "artist":"Pawan Kalyan"
        },{
            "role":"Kanmani",
            "artist":"Priyanka Arul Mohan"
        }],
        "makers":[{
            "role":"Music Director",
            "artist":"Thaman S.S"
        },{
            "role":"Director",
            "artist":"Sujeet"
        }]
        
    });
}
