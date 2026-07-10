mod common;
use common::{fixtures, setups::setup_edit_upload, spawn_app};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Create Recommendation
// ---------------------------------------------------------------------------

#[tokio::test]
async fn create_recommendation_returns_200_for_authenticated_artist() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_recommendation_body(original_id);
    let response = app.post_recommendation(&body).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );
}

#[tokio::test]
async fn create_recommendation_returns_401_for_unauthenticated_user() {
    let app = spawn_app::spawn().await;

    let body = fixtures::create_recommendation_body(Uuid::new_v4());
    let response = app.post_recommendation(&body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn create_recommendation_correctly_saves_notes_and_score() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_recommendation_body(original_id);
    app.post_recommendation(&body).await;

    let saved = sqlx::query!(
        r#"SELECT notes, surge_score FROM recommendations WHERE original_id=$1"#,
        original_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(
        saved.notes,
        Some("This is a great recommendation".to_string())
    );
    assert_eq!(saved.surge_score, 100);
}

#[tokio::test]
async fn create_recommendation_updates_profile_peak_recommendations() {
    let (artists, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let mut body = fixtures::create_recommendation_body(original_id);
    body["score"] = serde_json::json!(1100);
    app.post_recommendation(&body).await;

    let peak: i64 = sqlx::query_scalar(
        r#"SELECT COALESCE(current_peak_recommendations, 0) FROM profiles WHERE id=$1"#,
    )
    .bind(artists[0])
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(peak, 1100);
}

// ---------------------------------------------------------------------------
// Update Recommendation
// ---------------------------------------------------------------------------

#[tokio::test]
async fn update_recommendation_returns_200_for_owner() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    // Create recommendation first
    let body = fixtures::create_recommendation_body(original_id);
    app.post_recommendation(&body).await;

    let recommendation_id: Uuid = sqlx::query_scalar!(
        r#"SELECT id FROM recommendations WHERE original_id=$1"#,
        original_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    // Update it
    let update_body = fixtures::update_recommendation_body();
    let response = app
        .post_update_recommendation(recommendation_id, &update_body)
        .await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );
}

#[tokio::test]
async fn update_recommendation_returns_401_for_non_owner() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    // Create recommendation as user_0
    let body = fixtures::create_recommendation_body(original_id);
    app.post_recommendation(&body).await;

    let recommendation_id: Uuid = sqlx::query_scalar!(
        r#"SELECT id FROM recommendations WHERE original_id=$1"#,
        original_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    // Try to update as user_1
    app.post_login(&fixtures::login_body("user_1", "kApten@1023"))
        .await;

    let update_body = fixtures::update_recommendation_body();
    let response = app
        .post_update_recommendation(recommendation_id, &update_body)
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn update_recommendation_returns_404_for_nonexistent_recommendation() {
    let (_, app, _) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let update_body = fixtures::update_recommendation_body();
    let response = app
        .post_update_recommendation(Uuid::new_v4(), &update_body)
        .await;

    assert_eq!(response.status().as_u16(), 404);
}

#[tokio::test]
async fn update_recommendation_with_new_score_updates_profile_peak() {
    let (artists, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    // Create recommendation with score 100
    let body = fixtures::create_recommendation_body(original_id);
    app.post_recommendation(&body).await;

    let recommendation_id: Uuid = sqlx::query_scalar!(
        r#"SELECT id FROM recommendations WHERE original_id=$1"#,
        original_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    // Update with higher score
    let update_body = serde_json::json!({
        "score": 1200
    });
    app.post_update_recommendation(recommendation_id, &update_body)
        .await;

    let peak: i64 = sqlx::query_scalar(
        r#"SELECT COALESCE(current_peak_recommendations, 0) FROM profiles WHERE id=$1"#,
    )
    .bind(artists[0])
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(peak, 1200);
}

#[tokio::test]
async fn update_recommendation_lines_only() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_recommendation_body(original_id);
    app.post_recommendation(&body).await;

    let recommendation_id: Uuid = sqlx::query_scalar!(
        r#"SELECT id FROM recommendations WHERE original_id=$1"#,
        original_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    let update_body = fixtures::update_recommendation_lines_only();
    app.post_update_recommendation(recommendation_id, &update_body)
        .await;

    let saved = sqlx::query!(
        r#"SELECT notes, surge_score FROM recommendations WHERE id=$1"#,
        recommendation_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(saved.notes, Some("Updated lines only".to_string()));
    assert_eq!(saved.surge_score, 100); // Score should remain unchanged
}
