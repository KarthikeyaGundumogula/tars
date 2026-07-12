mod common;
use common::{fixtures, setups::setup_work_uploaded, spawn_app};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Update Work
// ---------------------------------------------------------------------------

#[tokio::test]
async fn update_work_returns_200_for_owner() {
    let (_, app, _, work_id) = setup_work_uploaded().await;
    // user_0 is still logged in as the work owner

    let response = app
        .post_update_work(work_id, &fixtures::update_work_body())
        .await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let title: Option<String> = sqlx::query_scalar(r#"SELECT title FROM works WHERE id=$1"#)
        .bind(work_id)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    assert_eq!(title.unwrap(), "Updated Title");
}

#[tokio::test]
async fn update_work_returns_401_for_non_owner() {
    let (_, app, _, work_id) = setup_work_uploaded().await;

    // Login as user_1 who does NOT own this work
    app.post_login(&fixtures::login_body("user_1", "kApten@1023"))
        .await;

    let response = app
        .post_update_work(work_id, &fixtures::update_work_body())
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn update_work_returns_401_when_not_logged_in() {
    let app = spawn_app::spawn().await;

    let response = app
        .post_update_work(Uuid::new_v4(), &fixtures::update_work_body())
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn update_work_returns_404_for_nonexistent_work() {
    let (_, app, _, _) = setup_work_uploaded().await;
    // user_0 is logged in, but this work_id doesn't exist

    let response = app
        .post_update_work(Uuid::new_v4(), &fixtures::update_work_body())
        .await;

    assert_eq!(response.status().as_u16(), 404);
}

// ---------------------------------------------------------------------------
// Like / Dislike Work
// ---------------------------------------------------------------------------

#[tokio::test]
async fn like_work_returns_200_for_logged_in_artist() {
    let (_, app, _, work_id) = setup_work_uploaded().await;

    // user_1 likes user_0's work
    app.post_login(&fixtures::login_body("user_1", "kApten@1023"))
        .await;

    let response = app.post_star_work(&fixtures::entity_action_body(work_id)).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let count: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM work_stars WHERE work_id=$1"#)
        .bind(work_id)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    assert_eq!(count, 1);
}

#[tokio::test]
async fn like_work_returns_401_when_not_logged_in() {
    let (_, _, _, work_id) = setup_work_uploaded().await;

    // Logout first to ensure fresh unauthenticated state
    let app = spawn_app::spawn().await;

    let response = app.post_star_work(&fixtures::entity_action_body(work_id)).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn dislike_work_returns_200_after_liking() {
    let (_, app, _, work_id) = setup_work_uploaded().await;

    app.post_login(&fixtures::login_body("user_1", "kApten@1023"))
        .await;

    // Like first
    app.post_star_work(&fixtures::entity_action_body(work_id)).await;

    // Then dislike
    let response = app
        .delete_unstar_work(&fixtures::entity_action_body(work_id))
        .await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let count: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM work_stars WHERE work_id=$1"#)
        .bind(work_id)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    assert_eq!(count, 0);
}

// ---------------------------------------------------------------------------
// Delete Work
// ---------------------------------------------------------------------------

#[tokio::test]
async fn delete_work_returns_200_for_owner() {
    let (_, app, _, work_id) = setup_work_uploaded().await;
    // user_0 owns the work and is still logged in

    let response = app.delete_work(work_id).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let count: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM works WHERE id=$1"#)
        .bind(work_id)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    assert_eq!(count, 0);
}

#[tokio::test]
async fn delete_work_also_deletes_edit_via_cascade() {
    let (_, app, _, work_id) = setup_work_uploaded().await;

    app.delete_work(work_id).await;

    // edits table should also be cleaned up via ON DELETE CASCADE
    let count: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM edits WHERE work_id=$1"#)
        .bind(work_id)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    assert_eq!(count, 0);
}

#[tokio::test]
async fn delete_work_returns_401_for_non_owner() {
    let (_, app, _, work_id) = setup_work_uploaded().await;

    app.post_login(&fixtures::login_body("user_1", "kApten@1023"))
        .await;

    let response = app.delete_work(work_id).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn delete_work_returns_401_when_not_logged_in() {
    let app = spawn_app::spawn().await;

    let response = app.delete_work(Uuid::new_v4()).await;

    assert_eq!(response.status().as_u16(), 401);
}

// ---------------------------------------------------------------------------
// Wall Post Integration Tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn create_wall_post_for_work_returns_200() {
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
async fn create_wall_post_without_text_creates_pin() {
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
