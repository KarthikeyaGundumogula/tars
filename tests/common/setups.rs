#![allow(dead_code)]
use uuid::Uuid;

use crate::common::{fixtures, spawn_app::{self, TestApp}};

/// Creates 4 registered artists and returns their IDs + the running TestApp.
/// All subsequent setup functions build on this as the base layer.
pub async fn setup_original_registration() -> (Vec<Uuid>, TestApp) {
    let mut artists = Vec::new();
    let app = spawn_app::spawn().await;

    for i in 0..4 {
        let handle = format!("user_{}", i);
        let body = fixtures::register_body(&handle, "kApten@1023");

        let response = app.post_register(&body).await;
        assert!(
            response.status().is_success(),
            "Failed to register artist {}: status {}",
            handle,
            response.status()
        );

        let artist =
            sqlx::query_scalar!(r#"SELECT id FROM profiles WHERE user_name=$1"#, handle)
                .fetch_one(&app.state.db_pool)
                .await
                .expect("db query failed");
        artists.push(artist);
    }

    (artists, app)
}

/// Registers 4 artists, creates an original, and returns all IDs + the TestApp.
pub async fn setup_edit_upload() -> (Vec<Uuid>, TestApp, Uuid) {
    let (artists, app) = setup_original_registration().await;

    let body = fixtures::create_original_body(&artists);
    let response = app.post_original(&body).await;
    assert!(
        response.status().is_success(),
        "Failed to create original: status {}",
        response.status()
    );

    let original_id = sqlx::query_scalar!(
        r#"SELECT id FROM originals WHERE title=$1"#,
        "They Call him Og"
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    (artists, app, original_id)
}

/// Registers 4 artists, logs in as user_0, creates a Set, and returns all IDs + the TestApp.
pub async fn setup_set_creation() -> (Vec<Uuid>, TestApp, Uuid) {
    let (artists, app) = setup_original_registration().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023")).await;

    let body = fixtures::create_set_body();
    let response = app.post_set(&body).await;
    assert!(
        response.status().is_success(),
        "Failed to create set: status {}",
        response.status()
    );

    let set_id = sqlx::query_scalar!(
        r#"SELECT id FROM sets WHERE name=$1"#,
        "My Awesome Set"
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    (artists, app, set_id)
}
