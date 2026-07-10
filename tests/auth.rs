mod common;
use common::{fixtures, spawn_app};

#[tokio::test]
async fn register_profile_return_201_on_correct_data() {
    let app = spawn_app::spawn().await;

    let body = fixtures::register_body("kapten", "kApten@1023");
    let response = app.post_register(&body).await;

    let status = response.status();
    if !status.is_success() {
        let text = response.text().await.unwrap();
        panic!("API Error 500 on register: {}", text);
    }
    assert_eq!(status, reqwest::StatusCode::CREATED);

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
async fn login_artist_return_201_on_correct_data() {
    let app = spawn_app::spawn().await;

    // Arrange: Register first
    let register_res = app
        .post_register(&fixtures::register_body("kapten", "kApten@1023"))
        .await;
    println!("{:?}",register_res);
    assert_eq!(register_res.status(), reqwest::StatusCode::CREATED);

    // Act: Login
    let response = app
        .post_login(&fixtures::login_body("kapten", "kApten@1023"))
        .await;

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);
}

#[tokio::test]
async fn reset_password_returns_200_for_valid_credentials() {
    let app = spawn_app::spawn().await;

    // Register + Login
    app.post_register(&fixtures::register_body("kapten", "kApten@1023"))
        .await;
    app.post_login(&fixtures::login_body("kapten", "kApten@1023"))
        .await;

    // Act: Reset password
    let response = app
        .post_reset_password(&fixtures::reset_password_body(
            "kApten@1023",
            "NewPass@2024",
        ))
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
    app.post_register(&fixtures::register_body("kapten", "kApten@1023"))
        .await;

    let response = app
        .post_reset_password(&fixtures::reset_password_body(
            "kApten@1023",
            "NewPass@2024",
        ))
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn reset_password_returns_401_for_wrong_old_password() {
    let app = spawn_app::spawn().await;

    app.post_register(&fixtures::register_body("kapten", "kApten@1023"))
        .await;
    app.post_login(&fixtures::login_body("kapten", "kApten@1023"))
        .await;

    // Old password is wrong
    let response = app
        .post_reset_password(&fixtures::reset_password_body(
            "WrongPass123",
            "NewPass@2024",
        ))
        .await;

    assert_eq!(response.status().as_u16(), 401);
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

// ---------------------------------------------------------------------------
// Admin Auth
// ---------------------------------------------------------------------------

#[tokio::test]
async fn create_admin_returns_200_on_correct_data() {
    let app = spawn_app::spawn().await;

    let response = app
        .post_admin_register(&fixtures::admin_register_body())
        .await;
    // let json = response.json::<serde_json::Value>().await.unwrap();
    // println!("{:?}", json);
    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let count: i64 =
        sqlx::query_scalar(r#"SELECT COUNT(*) FROM admins WHERE admin_name='superadmin'"#)
            .fetch_one(&app.state.db_pool)
            .await
            .expect("db query failed");

    assert_eq!(count, 1);
}

#[tokio::test]
async fn admin_login_returns_200_with_cookie_on_correct_credentials() {
    let app = spawn_app::spawn().await;

    // Register first
    app.post_admin_register(&fixtures::admin_register_body())
        .await;

    // Login
    let response = app.post_admin_login(&fixtures::admin_login_body()).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    // The response must set an auth_token cookie
    let has_cookie = response
        .headers()
        .get_all("set-cookie")
        .iter()
        .any(|v| v.to_str().unwrap_or("").contains("auth_token"));

    assert!(
        has_cookie,
        "Expected auth_token cookie to be set on admin login"
    );
}

#[tokio::test]
async fn admin_login_returns_401_for_wrong_password() {
    let app = spawn_app::spawn().await;

    app.post_admin_register(&fixtures::admin_register_body())
        .await;

    let wrong_login = serde_json::json!({
        "admin_name": "superadmin",
        "admin_password": "WrongPass@99"
    });
    let response = app.post_admin_login(&wrong_login).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn admin_login_returns_401_for_nonexistent_admin() {
    let app = spawn_app::spawn().await;

    // No admin registered — timing-safe path should still return 401
    let response = app.post_admin_login(&fixtures::admin_login_body()).await;

    assert_eq!(response.status().as_u16(), 401);
}
