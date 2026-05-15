mod common;
use common::spawn_app;

#[tokio::test]
async fn update_profile_returns_200_for_valid_data() {
    let app = spawn_app::spawn().await;
    
    // Arrange: Register and login
    let handle = "kapten";
    let password = "kApten@1023";
    let register_body = serde_json::json!({
        "handle": handle,
        "tag_line": "initial tagline",
        "password": password,
        "profile_picture": "initial_pic",
        "youtube_profile": "initial_yt",
        "stage_name":"kapten",
        "text_color":"#000000",
        "background_color":"#FFFFFF"
    });
    app.post_register(&register_body).await;
    
    let login_body = serde_json::json!({
        "handle": handle,
        "password": password
    });
    app.post_login(&login_body).await;
    
    // Act: Update profile
    let update_body = serde_json::json!({
        "tag_line": "updated tagline",
        "stage_name": "kapten og"
    });
    let response = app.post_update_profile(&update_body).await;
    
    // Assert
    assert!(response.status().is_success());
    
    // Verify in database
    let saved = sqlx::query!(
        r#"SELECT tag_line, stage_name FROM profiles WHERE user_name = $1"#,
        handle
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("Failed to fetch profile");
    
    assert_eq!(saved.tag_line, "updated tagline");
    assert_eq!(saved.stage_name, Some("kapten og".to_string()));
}

#[tokio::test]
async fn update_profile_returns_401_for_unauthorized_user() {
    let app = spawn_app::spawn().await;
    
    // Act: Try to update without logging in
    let update_body = serde_json::json!({
        "tag_line": "updated tagline"
    });
    let response = app.post_update_profile(&update_body).await;
    
    // Assert
    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
}
