mod utils;
use reqwest::{Client, Response};
use sqlx::{Connection, PgConnection};
use tars::configuration::get_configuration;
use utils::spawn_app;

use crate::utils::setups::setup_edit_upload;

#[tokio::test]
async fn upload_edit_return_200_on_correct_data() {
    let (_, app, original_id) = setup_edit_upload().await;
    let config = get_configuration().expect("unable to get the config");
    let connection_string = config.database.connection_string();
    let _connection = PgConnection::connect(&connection_string)
        .await
        .expect("unable to connect to postgres");
    let client = Client::new();
    let body = serde_json::json!({
        "title": "OG Intro Blast",
        "src_id": "GG1_DsScm6U",
        "platform": "YOUTUBE",
        "format": "IMAX",
        "originals": [original_id]
    });
    let auth_token = "test_auth_token";
    let response: Response = client
        .post(&format!("{}/works/new/EDIT", app.address))
        .header("Cookie", format!("auth_token={}", auth_token))
        .send()
        .await
        .expect("Failed to execute request.");
    println!(
        "{:?}",
        response.json::<serde_json::Value>().await.unwrap()
    );
    assert!(0 == 1);
}

#[tokio::test]
async fn upload_edit_return_error_on_incorrect_data() {
    let app = spawn_app::spawn().await;
    let config = get_configuration().expect("unable to get the config");
    let connection_string = config.database.connection_string();
    let _connection = PgConnection::connect(&connection_string)
        .await
        .expect("unable to connect to postgres");
    let client = Client::new();
    let body = serde_json::json!({
        "title": "OG Intro Blast",
        "src_id": "GG1_DsScm6U",
        "platform": "Youtube",
        "originals": ["550e8400-e29b-41d4-a716-446655440000"]
    });
    let response: Response = client
        .post(&format!("{}/works/new/Poster", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");
    let status = response.status();
    let body = response.text().await.unwrap();
    println!("Status: {}", status);
    println!("Body: {}", body);
    assert!(status.is_client_error());
}
