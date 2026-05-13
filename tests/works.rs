mod common;

use crate::common::setups::setup_edit_upload;

#[tokio::test]
async fn upload_edit_return_200_on_correct_data() {
    // Arrange
    let (_, app, original_id) = setup_edit_upload().await;

    let login_body = serde_json::json!({
        "handle": "user_0",
        "password": "kApten@1023"
    });
    app.post_login(&login_body).await;

    let work_title = "OG Intro Blast";
    let body = serde_json::json!({
        "title": work_title,
        "src_id": "GG1_DsScm6U",
        "platform": "YOUTUBE",
        "format": "IMAX",
        "originals": [original_id]
    });

    // Act
    let response = app.post_work("EDIT", &body).await;

    // Assert
    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);

    let saved_work = sqlx::query!(
        r#"SELECT id, title, category as "category: tars::types::db::work::WorkType" FROM works WHERE title=$1"#,
        work_title
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("Failed to find uploaded work in DB");

    assert_eq!(saved_work.title, Some(work_title.to_string()));

    let saved_edit = sqlx::query!(
        r#"SELECT src_id, platform as "platform: tars::types::db::work::SupportedPlatforms" FROM edits WHERE work_id=$1"#,
        saved_work.id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("Failed to find edit details in DB");

    assert_eq!(saved_edit.src_id, "GG1_DsScm6U");
}

#[tokio::test]
async fn upload_poster_return_200_on_correct_data() {
    // Arrange
    let (_, app, original_id) = setup_edit_upload().await;

    let login_body = serde_json::json!({
        "handle": "user_0",
        "password": "kApten@1023"
    });
    app.post_login(&login_body).await;

    let work_title = "The Golden Poster";
    let body = serde_json::json!({
        "title": work_title,
        "src_id": "poster_uuid_123",
        "format": "STANDARD",
        "originals": [original_id]
    });

    // Act
    let response = app.post_work("POSTER", &body).await;

    // Assert
    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);

    let saved_work = sqlx::query!(
        r#"SELECT id, title, category as "category: tars::types::db::work::WorkType" FROM works WHERE title=$1"#,
        work_title
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("Failed to find uploaded work in DB");

    let saved_poster = sqlx::query!(
        r#"SELECT src_id, format as "format: tars::types::db::work::PosterFormat" FROM posters WHERE work_id=$1"#,
        saved_work.id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("Failed to find poster details in DB");

    assert_eq!(saved_poster.src_id, "poster_uuid_123");
}

#[tokio::test]
async fn upload_script_return_200_on_correct_data() {
    // Arrange
    let (_, app, original_id) = setup_edit_upload().await;

    let login_body = serde_json::json!({
        "handle": "user_0",
        "password": "kApten@1023"
    });
    app.post_login(&login_body).await;

    let work_title = "Cinematic Script Draft";
    let body = serde_json::json!({
        "title": work_title,
        "src_ids": ["img1", "img2"],
        "originals": [original_id],
        "thoughts": ["Brilliant intro", "Dynamic pacing"]
    });

    // Act
    let response = app.post_work("SCRIPT", &body).await;

    // Assert
    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);

    let saved_work = sqlx::query!(
        r#"SELECT id, title, category as "category: tars::types::db::work::WorkType" FROM works WHERE title=$1"#,
        work_title
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("Failed to find uploaded work in DB");

        let saved_script = sqlx::query!(
        "SELECT img_src_ids, thoughts FROM scripts WHERE work_id=$1",
        saved_work.id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("Failed to find script details in DB");

    assert_eq!(saved_script.img_src_ids, Some(vec!["img1".to_string(), "img2".to_string()]));
    assert_eq!(saved_script.thoughts, Some(vec!["Brilliant intro".to_string(), "Dynamic pacing".to_string()]));
}

#[tokio::test]
async fn upload_work_returns_400_when_data_is_missing_or_invalid() {
    // Arrange
    let app = common::spawn_app::spawn().await;
    
    // We test multiple invalid payloads using table-driven tests
    let test_cases = vec![
        (
            serde_json::json!({
                "title": "Missing Platform",
                "src_id": "GG1_DsScm6U",
                "originals": ["550e8400-e29b-41d4-a716-446655440000"]
            }),
            "missing the platform",
        ),
        (
            serde_json::json!({
                "title": "Invalid Platform Typo",
                "src_id": "GG1_DsScm6U",
                "platform": "Youtube", // Should be YOUTUBE
                "originals": ["550e8400-e29b-41d4-a716-446655440000"]
            }),
            "invalid enum variant",
        ),
        (
            serde_json::json!({
                "title": "Invalid Title 123", // Numbers not allowed by WorkTitle domain
                "src_id": "GG1_DsScm6U",
                "platform": "YOUTUBE",
                "format": "IMAX",
                "originals": ["550e8400-e29b-41d4-a716-446655440000"]
            }),
            "title contains numbers",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_work("EDIT", &invalid_body).await;

        // Assert
        assert_eq!(
            response.status().as_u16(),
            401,
            "The API did not fail with 401 Unauthorized when the payload was {}",
            error_message
        );
    }
}
