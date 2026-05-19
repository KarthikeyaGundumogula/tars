mod common;
use common::{fixtures, setups::setup_set_creation, spawn_app};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Update Set
// ---------------------------------------------------------------------------

#[tokio::test]
async fn update_set_returns_200_for_curator() {
    let (_, app, set_id) = setup_set_creation().await;
    // user_0 is already logged in (curator from setup_set_creation)

    let response = app
        .post_update_set(set_id, &fixtures::update_set_body())
        .await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let saved = sqlx::query_scalar!(r#"SELECT name FROM sets WHERE id=$1"#, set_id)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    assert_eq!(saved, "Updated Set Name");
}

#[tokio::test]
async fn update_set_returns_401_for_non_owner() {
    let (artists, app, set_id) = setup_set_creation().await;

    // Login as user_1 who is NOT the curator
    app.post_login(&fixtures::login_body("user_1", "kApten@1023"))
        .await;

    let response = app
        .post_update_set(set_id, &fixtures::update_set_body())
        .await;

    assert_eq!(
        response.status().as_u16(),
        401,
        "Non-owner should not be allowed to update set, artists: {:?}",
        artists
    );
}

#[tokio::test]
async fn update_set_returns_401_when_not_logged_in() {
    let app = spawn_app::spawn().await;

    let response = app
        .post_update_set(Uuid::new_v4(), &fixtures::update_set_body())
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

// ---------------------------------------------------------------------------
// Join Set
// ---------------------------------------------------------------------------

#[tokio::test]
async fn join_set_returns_200_for_valid_artist() {
    let (_, app, set_id) = setup_set_creation().await;

    // user_1 joins the set created by user_0
    app.post_login(&fixtures::login_body("user_1", "kApten@1023"))
        .await;

    let response = app.post_join_set(&fixtures::join_set_body(set_id)).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM set_members WHERE set_id=$1 AND profile_id=(SELECT id FROM profiles WHERE user_name='user_1')"#,
    )
    .bind(set_id)
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(count, 1);
}

#[tokio::test]
async fn join_set_returns_401_when_not_logged_in() {
    let app = spawn_app::spawn().await;

    let response = app
        .post_join_set(&fixtures::join_set_body(Uuid::new_v4()))
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

// ---------------------------------------------------------------------------
// Leave Set
// ---------------------------------------------------------------------------

#[tokio::test]
async fn leave_set_returns_200_for_member() {
    let (_, app, set_id) = setup_set_creation().await;

    // user_1 joins first
    app.post_login(&fixtures::login_body("user_1", "kApten@1023"))
        .await;
    app.post_join_set(&fixtures::join_set_body(set_id)).await;

    // Then leaves
    let response = app.delete_leave_set(set_id).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM set_members WHERE set_id=$1 AND profile_id=(SELECT id FROM profiles WHERE user_name='user_1')"#,
    )
    .bind(set_id)
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(count, 0);
}

#[tokio::test]
async fn leave_set_returns_404_when_not_a_member() {
    let (_, app, set_id) = setup_set_creation().await;

    // user_1 never joined — trying to leave should 404
    app.post_login(&fixtures::login_body("user_1", "kApten@1023"))
        .await;

    let response = app.delete_leave_set(set_id).await;

    // The extractor will return NotFound since the set_member record doesn't exist
    assert_eq!(response.status().as_u16(), 404);
}

#[tokio::test]
async fn leave_set_returns_401_when_not_logged_in() {
    // Use a plain spawn — no login cookie is set on this client
    let app = spawn_app::spawn().await;

    let response = app.delete_leave_set(uuid::Uuid::new_v4()).await;

    assert_eq!(response.status().as_u16(), 401);
}
