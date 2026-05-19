mod common;
use common::{fixtures, setups::setup_edit_upload};

#[tokio::test]
async fn upload_edit_return_200_on_correct_data() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_edit_body(original_id);
    let response = app.post_work("EDIT", &body).await;

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);

    let saved_work = sqlx::query!(
        r#"SELECT id, title, category as "category: tars::types::db::work::WorkType" FROM works WHERE title=$1"#,
        "OG Intro Blast"
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("Failed to find uploaded work in DB");

    assert_eq!(saved_work.title, Some("OG Intro Blast".to_string()));

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
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_poster_body(original_id);
    let response = app.post_work("POSTER", &body).await;

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);

    let saved_work = sqlx::query!(
        r#"SELECT id, title, category as "category: tars::types::db::work::WorkType" FROM works WHERE title=$1"#,
        "The Golden Poster"
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
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_script_body(original_id);
    let response = app.post_work("SCRIPT", &body).await;

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);

    let saved_work = sqlx::query!(
        r#"SELECT id, title, category as "category: tars::types::db::work::WorkType" FROM works WHERE title=$1"#,
        "Cinematic Script Draft"
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

    assert_eq!(
        saved_script.img_src_ids,
        Some(vec!["img1".to_string(), "img2".to_string()])
    );
    assert_eq!(
        saved_script.thoughts,
        Some(vec![
            "Brilliant intro".to_string(),
            "Dynamic pacing".to_string()
        ])
    );
}

#[tokio::test]
async fn upload_work_returns_401_when_not_logged_in() {
    // Any work type is fine; auth is checked before the body
    let app = common::spawn_app::spawn().await;
    let body = serde_json::json!({ "title": "some work" });
    let response = app.post_work("EDIT", &body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn upload_work_returns_400_on_invalid_payload() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let test_cases = vec![
        (
            serde_json::json!({
                "title": "Missing Platform",
                "src_id": "GG1_DsScm6U",
                "originals": [original_id]
                // missing platform and format
            }),
            "missing platform field",
        ),
        (
            serde_json::json!({
                "title": "Bad Platform",
                "src_id": "GG1_DsScm6U",
                "platform": "Youtube", // must be YOUTUBE
                "format": "IMAX",
                "originals": [original_id]
            }),
            "invalid enum variant (case mismatch)",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_work("EDIT", &invalid_body).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Expected 400 when payload was: {}",
            error_message
        );
    }
}
