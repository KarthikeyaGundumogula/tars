use tars::startup::run;
use tokio::net::TcpListener;

pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).await.expect("Failed to bind address");
    let _ = tokio::spawn(async move {
        let _ = server.await;
    });
    format!("http://127.0.0.1:{}", port)
}
