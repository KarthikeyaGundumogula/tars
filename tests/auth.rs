mod utils;

use chrono::Utc;
use reqwest::{Client, Response};
use utils::spawn_app;

use crate::utils::setups::setup_original_registration;

#[tokio::test]
async fn register_profile_return_200_on_correct_data() {
    let app = spawn_app::spawn().await;
    let client = Client::new();
    let body = serde_json::json!({
      "user_name":"kapten",
      "tag_line":"I dont give a dmn about your opinion",
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
async fn create_original_return_success_on_correct_data() {
    let (artists, app) = setup_original_registration().await;
    let client = Client::new();

    let body = serde_json::json!({
        "title":"They Call him Og",
        "description":"fuck you staya dada",
        "cover_img":"canada is fucked",
        "password": "1234",
        "associated_with":artists[0], // this is an uuid that can be set
        "release_date":Utc::now(),
        "genere":["action","drama"],
        "stars":[{
            "role":"Ojas Ghambheera",
            "artist":artists[1] // this is also a uuid
        },{
            "role":"Kanmani",
            "artist":artists[2]
        }],
        "makers":[{
            "role":"Music Director",
            "artist":artists[3]
        },{
            "role":"Director",
            "artist":artists[1]
        }]
    });
    let response: Response = client
        .post(&format!("{}/originals/new", app.address))
        .json(&body)
        .send()
        .await
        .expect("failed to execute request");
    let description = sqlx::query_scalar!(
        r#"SELECT description FROM originals WHERE title=$1"#,
        "They Call him Og"
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("db query failed");
    let role_name = sqlx::query_scalar!(
        r#"SELECT role_name FROM roles WHERE profile_id=$1"#,
        artists[1]
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("db query failed");
    assert!(response.status().is_success());
    assert_eq!(description, "fuck you staya dada".to_string());
    assert_eq!(role_name, "Ojas Ghambheera".to_string())
}
