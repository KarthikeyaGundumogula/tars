mod common;
use common::{fixtures, setups::setup_work_uploaded, spawn_app};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Create Wall Post with Quote
// ---------------------------------------------------------------------------

#[tokio::test]
async fn create_wall_post_returns_200_for_authenticated_artist() {
    let (_, app, _, work_id) = setup_work_uploaded().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_wall_post_body(work_id);
    let response = app.post_wall_post(&body).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );
}

#[tokio::test]
async fn create_wall_post_returns_401_for_unauthenticated_user() {
    let app = spawn_app::spawn().await;

    let body = fixtures::create_wall_post_body(Uuid::new_v4());
    let response = app.post_wall_post(&body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn create_wall_post_without_text_line_creates_pin_entry() {
    let (_, app, _, work_id) = setup_work_uploaded().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_wall_post_pin_body(work_id);
    app.post_wall_post(&body).await;

    let wall_post_id: Uuid =
        sqlx::query_scalar!(r#"SELECT id FROM wall_posts WHERE work_id=$1"#, work_id)
            .fetch_one(&app.state.db_pool)
            .await
            .expect("db query failed");

    let count: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM work_pins WHERE wall_post_id=$1"#)
        .bind(wall_post_id)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    assert_eq!(count, 1);
}

#[tokio::test]
async fn create_wall_post_correctly_links_to_work() {
    let (_, app, _, work_id) = setup_work_uploaded().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_wall_post_body(work_id);
    app.post_wall_post(&body).await;

    let saved = sqlx::query!(
        r#"SELECT work_id, text_line FROM wall_posts WHERE work_id=$1"#,
        work_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(saved.work_id, Some(work_id));
    assert_eq!(
        saved.text_line,
        Some("This is a quote from the work".to_string())
    );
}

#[tokio::test]
async fn create_wall_post_standalone_without_work() {
    let app = spawn_app::spawn().await;

    // Register and login
    let body = fixtures::register_body("user_0", "kApten@1023");
    app.post_register(&body).await;
    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let wall_post_body = fixtures::create_wall_post_standalone_body();
    let response = app.post_wall_post(&wall_post_body).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let saved = sqlx::query!(
        r#"SELECT work_id, text_line FROM wall_posts WHERE text_line=$1"#,
        "Standalone wall post"
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(saved.work_id, None);
    assert_eq!(saved.text_line, Some("Standalone wall post".to_string()));
}

#[tokio::test]
async fn create_wall_post_sets_correct_artist_id() {
    let (artists, app, _, work_id) = setup_work_uploaded().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_wall_post_body(work_id);
    app.post_wall_post(&body).await;

    let saved_artist_id: Uuid = sqlx::query_scalar!(
        r#"SELECT artist_id FROM wall_posts WHERE work_id=$1"#,
        work_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(saved_artist_id, artists[0]);
}

#[tokio::test]
async fn create_wall_post_sets_timestamp() {
    let (_, app, _, work_id) = setup_work_uploaded().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_wall_post_body(work_id);
    app.post_wall_post(&body).await;

    let created_at = sqlx::query!(
        r#"SELECT created_at FROM wall_posts WHERE work_id=$1"#,
        work_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert!(created_at.created_at.timestamp() > 0);
}

// ---------------------------------------------------------------------------
// Wall Post Validation
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Wall Post with Optional Fields
// ---------------------------------------------------------------------------

#[tokio::test]
async fn create_wall_post_with_original_id_links_correctly() {
    let (_, app, original_id, _work_id) = common::setups::setup_work_uploaded().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_wall_post_with_original(original_id);
    app.post_wall_post(&body).await;

    let wall_post_id: Uuid =
        sqlx::query_scalar!(r#"SELECT id FROM wall_posts WHERE original_id=$1"#, original_id)
            .fetch_one(&app.state.db_pool)
            .await
            .expect("db query failed");

    let saved = sqlx::query!(
        r#"SELECT original_id FROM wall_posts WHERE id=$1"#,
        wall_post_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(saved.original_id, Some(original_id));
}

#[tokio::test]
async fn create_wall_post_with_recommendation_id_links_correctly() {
    let (_, app, original_id, _work_id) = common::setups::setup_work_uploaded().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    // Create a recommendation first
    let rec_body = fixtures::create_recommendation_body(original_id);
    app.post_recommendation(&rec_body).await;

    let recommendation_id: Uuid = sqlx::query_scalar!(
        r#"SELECT id FROM recommendations WHERE original_id=$1"#,
        original_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    let wall_post_body = fixtures::create_wall_post_with_recommendation(recommendation_id);
    let response = app.post_wall_post(&wall_post_body).await;
    assert!(response.status().is_success(), "Expected 2xx, got {} with body {:?}", response.status(), response.text().await.unwrap());

    let wall_post_id: Uuid =
        sqlx::query_scalar!(r#"SELECT id FROM wall_posts WHERE recommendation_id=$1"#, recommendation_id)
            .fetch_one(&app.state.db_pool)
            .await
            .expect("db query failed");

    let saved = sqlx::query!(
        r#"SELECT recommendation_id FROM wall_posts WHERE id=$1"#,
        wall_post_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(saved.recommendation_id, Some(recommendation_id));
}

#[tokio::test]
async fn create_wall_post_initializes_counters_to_zero() {
    let (_, app, _, work_id) = setup_work_uploaded().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_wall_post_body(work_id);
    app.post_wall_post(&body).await;

    let wall_post_id: Uuid =
        sqlx::query_scalar!(r#"SELECT id FROM wall_posts WHERE work_id=$1"#, work_id)
            .fetch_one(&app.state.db_pool)
            .await
            .expect("db query failed");

    let saved = sqlx::query!(
        r#"SELECT total_views, total_saves FROM wall_posts WHERE id=$1"#,
        wall_post_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(saved.total_views, 0);
    assert_eq!(saved.total_saves, 0);
}
