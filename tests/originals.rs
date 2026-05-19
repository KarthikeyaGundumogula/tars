mod common;
use common::{
    fixtures,
    setups::{login_as_admin, setup_edit_upload},
    spawn_app,
};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Update Original (admin-only)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn update_original_returns_200_for_admin() {
    // Step 1: Create artists and the original — everything in ONE app/database
    let (_, app, original_id) = setup_edit_upload().await;

    // Step 2: Register + login admin in the SAME app (same database as above)
    login_as_admin(&app).await;

    // Step 3: Admin acts on the original that exists in this database
    let response = app
        .post_update_original(original_id, &fixtures::update_original_body())
        .await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let title: String = sqlx::query_scalar(r#"SELECT title FROM originals WHERE id=$1"#)
        .bind(original_id)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    assert_eq!(title, "They Call Him OG Redux");
}

#[tokio::test]
async fn update_original_returns_401_for_artist() {
    let (_, app, original_id) = setup_edit_upload().await;

    // user_0 is an artist — NOT an admin
    app.post_login(&fixtures::login_body("user_0", "kApten@1023")).await;

    let response = app
        .post_update_original(original_id, &fixtures::update_original_body())
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn update_original_returns_401_when_not_logged_in() {
    let app = spawn_app::spawn().await;

    let response = app
        .post_update_original(Uuid::new_v4(), &fixtures::update_original_body())
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

// ---------------------------------------------------------------------------
// Add Role to Original
// ---------------------------------------------------------------------------

#[tokio::test]
async fn add_role_returns_200_on_valid_request() {
    let (artists, app, original_id) = setup_edit_upload().await;

    let response = app
        .post_add_role(original_id, &fixtures::add_role_body(artists[0]))
        .await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM cast_and_crew_roles WHERE original_id=$1 AND profile_id=$2 AND role_name='Cinematographer'"#,
    )
    .bind(original_id)
    .bind(artists[0])
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(count, 1);
}

#[tokio::test]
async fn add_role_returns_200_with_idempotent_behavior_when_role_exists() {
    // Inserting the same role twice must not crash — handler returns RoleExists variant
    let (artists, app, original_id) = setup_edit_upload().await;

    app.post_add_role(original_id, &fixtures::add_role_body(artists[0])).await;

    let response = app
        .post_add_role(original_id, &fixtures::add_role_body(artists[0]))
        .await;

    // RoleExists still returns 200 OK
    assert!(
        response.status().is_success(),
        "Second insert should still return 200, got {}",
        response.status()
    );
}

// ---------------------------------------------------------------------------
// Delete Role from Original
// ---------------------------------------------------------------------------

#[tokio::test]
async fn delete_role_returns_200_after_adding() {
    let (artists, app, original_id) = setup_edit_upload().await;

    // Add role first
    app.post_add_role(original_id, &fixtures::add_role_body(artists[0])).await;

    // Then delete it
    let response = app
        .delete_role(original_id, &fixtures::remove_role_body(artists[0]))
        .await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM cast_and_crew_roles WHERE original_id=$1 AND profile_id=$2 AND role_name='Cinematographer'"#,
    )
    .bind(original_id)
    .bind(artists[0])
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(count, 0);
}

// ---------------------------------------------------------------------------
// Delete Original (admin-only)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn delete_original_returns_200_for_admin() {
    // Step 1: Create original in app's DB
    let (_, app, original_id) = setup_edit_upload().await;

    // Step 2: Login as admin in the SAME app
    login_as_admin(&app).await;

    // Step 3: Admin deletes it
    let response = app.delete_original(original_id).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let count: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM originals WHERE id=$1"#)
        .bind(original_id)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    assert_eq!(count, 0);
}

#[tokio::test]
async fn delete_original_cascades_to_roles() {
    let (_, app, original_id) = setup_edit_upload().await;

    // Verify roles exist before deletion (the original setup inserts stars + makers)
    let count_before: i64 =
        sqlx::query_scalar(r#"SELECT COUNT(*) FROM cast_and_crew_roles WHERE original_id=$1"#)
            .bind(original_id)
            .fetch_one(&app.state.db_pool)
            .await
            .expect("db query failed");

    assert!(count_before > 0, "Test assumes roles exist before deletion");

    // Login as admin in the SAME app and delete
    login_as_admin(&app).await;
    app.delete_original(original_id).await;

    let count_after: i64 =
        sqlx::query_scalar(r#"SELECT COUNT(*) FROM cast_and_crew_roles  WHERE original_id=$1"#)
            .bind(original_id)
            .fetch_one(&app.state.db_pool)
            .await
            .expect("db query failed");

    assert_eq!(count_after, 0);
}

#[tokio::test]
async fn delete_original_returns_401_for_artist() {
    let (_, app, original_id) = setup_edit_upload().await;

    // user_0 is an artist — not admin
    app.post_login(&fixtures::login_body("user_0", "kApten@1023")).await;

    let response = app.delete_original(original_id).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn delete_original_returns_401_when_not_logged_in() {
    let app = spawn_app::spawn().await;

    let response = app.delete_original(Uuid::new_v4()).await;

    assert_eq!(response.status().as_u16(), 401);
}
