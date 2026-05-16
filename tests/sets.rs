mod common;
use common::{fixtures, setups::setup_original_registration, spawn_app};

#[tokio::test]
async fn create_set_return_success_on_correct_data() {
    let (_, app) = setup_original_registration().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023")).await;

    let body = fixtures::create_set_body();
    let response = app.post_set(&body).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let saved_set = sqlx::query!(
        r#"SELECT id, name, statement, description, curator FROM sets WHERE name=$1"#,
        "My Awesome Set"
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("Failed to find uploaded set in DB");

    assert_eq!(saved_set.name, "My Awesome Set");
    assert_eq!(saved_set.statement, "This is a statement about the set");
    assert_eq!(saved_set.description, "This is a longer description of the set");
}

#[tokio::test]
async fn create_set_returns_401_without_login() {
    let app = spawn_app::spawn().await;

    let response = app.post_set(&fixtures::create_set_body()).await;

    assert_eq!(response.status().as_u16(), 401);
}
