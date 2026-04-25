mod utils;
use reqwest::{Client, Response};
use sqlx::{Connection, PgConnection};
use tars::configuration::get_configuration;
use utils::spawn_app;

#[tokio::test]
async fn upload_edit_return_200_on_correct_data() {
    let address = spawn_app().await;
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
        "format": "Imax",
        "originals": ["550e8400-e29b-41d4-a716-446655440000"]
    });
    let response: Response = client
        .post(&format!("{}/works/new/Edit", address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    println!("{}",response.status())
}

#[tokio::test]
async fn upload_edit_return_error_on_incorrect_data() {
    let address = spawn_app().await;
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
        .post(&format!("{}/works/new/Poster", address))
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
