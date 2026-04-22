mod utils;
use axum::Json;
use reqwest::{Client, Response};
use utils::spawn_app;

#[tokio::test]
async fn upload_edit_return_200_on_correct_data() {
    let address = spawn_app().await;
    let client = Client::new();
    let body = serde_json::json!({
        "title": "OG Intro Blast",
        "src_id": "GG1_DsScm6U",
        "platform": "Youtube",
        "format": "Imax",
        "originals": ["550e8400-e29b-41d4-a716-446655440000"]
    });
    let response: Response = client
        .post(&format!("{}/works/new/edit", address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn upload_edit_return_error_on_incorrect_data() {
    let address = spawn_app().await;
    let client = Client::new();
    let body = serde_json::json!({
        "title": "OG Intro Blast",
        "src_id": "GG1_DsScm6U",
        "platform": "Youtube",
        "originals": ["550e8400-e29b-41d4-a716-446655440000"]
    });
    let response: Response = client
        .post(&format!("{}/works/new/edit", address))
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
