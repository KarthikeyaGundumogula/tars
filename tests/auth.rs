mod common;
use common::spawn_app;

#[tokio::test]
async fn register_profile_return_200_on_correct_data() {
    let app = spawn_app::spawn().await;
    let body = serde_json::json!({
        "handle": "kapten",
        "tag_line": "I dont give a dmn about your opinion",
        "password": "kApten@1023",
        "profile_picture": "aofdjosfjosf",
        "youtube_profile": "aojojfosjf"
    });
    
    let response = app.post_register(&body).await;
    
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    
    let saved = sqlx::query_scalar!(
        r#"SELECT youtube_profile FROM profiles WHERE user_name=$1"#,
        "kapten"
    )
    .fetch_one(&app.state.pool)
    .await
    .expect("db query failed");
    assert_eq!(saved, Some("aojojfosjf".to_string()));
}

#[tokio::test]
async fn login_artist_return_200_on_correct_data() {
    let app = spawn_app::spawn().await;
    
    // Arrange: Register first
    let register_body = serde_json::json!({
        "handle": "kapten",
        "tag_line": "I dont give a dmn about your opinion",
        "password": "kApten@1023",
        "profile_picture": "aofdjosfjosf",
        "youtube_profile": "aojojfosjf"
    });
    let register_response = app.post_register(&register_body).await;
    assert_eq!(register_response.status(), reqwest::StatusCode::OK);
    
    // Act: Login
    let login_body = serde_json::json!({
        "handle": "kapten",
        "password": "kApten@1023"
    });
    let login_response = app.post_login(&login_body).await;
    
    // Assert
    assert_eq!(login_response.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn register_profile_returns_400_when_data_is_missing_or_invalid() {
    let app = spawn_app::spawn().await;
    
    // Table-driven tests for validation
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
            "The API did not fail with 422 Unprocessable Entity when the payload was {}",
            error_message
        );
    }
}
