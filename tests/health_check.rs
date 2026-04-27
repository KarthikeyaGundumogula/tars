use reqwest::Client;
mod utils;

use utils::spawn_app;

#[tokio::test]
async fn health_check_test() {
    let app = spawn_app::spawn().await;
    println!("out test is running at address {}",app.address);
    let client = Client::new();
    let response = client
        .get(&format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
