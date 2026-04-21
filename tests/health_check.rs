use reqwest::Client;
use tokio::net::TcpListener;

#[tokio::test]
async fn health_check_test() {
    let address = spawn_app().await;
    println!("out test is running at address {}", address);
    let client = Client::new();
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = tars::run(listener).await.expect("Failed to bind address");
    let _ = tokio::spawn(async move {
        let _ = server.await;
    });
    format!("http://127.0.0.1:{}", port)
}
