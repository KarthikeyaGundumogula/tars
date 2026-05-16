mod common;
use common::{fixtures, spawn_app};

#[tokio::test]
async fn register_profile_return_200_on_correct_data() {
    let app = spawn_app::spawn().await;

    let body = fixtures::register_body("kapten", "kApten@1023");
    let response = app.post_register(&body).await;

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let saved = sqlx::query_scalar!(
        r#"SELECT youtube_profile FROM profiles WHERE user_name=$1"#,
        "kapten"
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(saved, Some("aojojfosjf".to_string()));
}

#[tokio::test]
async fn login_artist_return_200_on_correct_data() {
    let app = spawn_app::spawn().await;

    // Arrange: Register first
    let register_res = app.post_register(&fixtures::register_body("kapten", "kApten@1023")).await;
    assert_eq!(register_res.status(), reqwest::StatusCode::OK);

    // Act: Login
    let response = app.post_login(&fixtures::login_body("kapten", "kApten@1023")).await;

    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn reset_password_returns_200_for_valid_credentials() {
    let app = spawn_app::spawn().await;

    // Register + Login
    app.post_register(&fixtures::register_body("kapten", "kApten@1023")).await;
    app.post_login(&fixtures::login_body("kapten", "kApten@1023")).await;

    // Act: Reset password
    let response = app
        .post_reset_password(&fixtures::reset_password_body("kApten@1023", "NewPass@2024"))
        .await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );
}

#[tokio::test]
async fn reset_password_returns_401_without_login() {
    let app = spawn_app::spawn().await;

    // Register but DON'T login
    app.post_register(&fixtures::register_body("kapten", "kApten@1023")).await;

    let response = app
        .post_reset_password(&fixtures::reset_password_body("kApten@1023", "NewPass@2024"))
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn reset_password_returns_401_for_wrong_old_password() {
    let app = spawn_app::spawn().await;

    app.post_register(&fixtures::register_body("kapten", "kApten@1023")).await;
    app.post_login(&fixtures::login_body("kapten", "kApten@1023")).await;

    // Old password is wrong
    let response = app
        .post_reset_password(&fixtures::reset_password_body("wrongpassword", "NewPass@2024"))
        .await;

    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn register_profile_returns_422_when_data_is_missing() {
    let app = spawn_app::spawn().await;

    let test_cases = vec![
        (
            serde_json::json!({
                "handle": "kapten",
                "password": "kapten@1023" // missing tag_line, profile_picture
            }),
            "missing required fields",
        ),
        (
            serde_json::json!({
                "handle": "KAPTEN_INVALID_UPPERCASE",
                "tag_line": "I dont give a dmn about your opinion",
                "password": "kApten@1023",
                "profile_picture": "aofdjosfjosf"
            }),
            "handle domain constraint failed (uppercase)",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_register(&invalid_body).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Expected 422 when payload was: {}",
            error_message
        );
    }
}
