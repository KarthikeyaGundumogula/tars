#![allow(dead_code)]
use uuid::Uuid;

use crate::common::{
    fixtures,
    spawn_app::{self, TestApp},
};

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

        let artist = sqlx::query_scalar!(r#"SELECT id FROM profiles WHERE user_name=$1"#, handle)
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

    login_as_admin(&app).await;
    let body = fixtures::create_original_body(&artists);
    let response = app.post_original(&body).await;
    let status = response.status();
    if !status.is_success() {
        let text = response.text().await.unwrap();
        panic!("Failed to create original: status {} body: {}", status, text);
    }

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

    // Make user_0 an organizer so they can create sets
    login_as_admin(&app).await;
    app.post_create_role(&serde_json::json!({
        "name": "organizer",
        "description": "Can organize sets"
    })).await;
    app.post_update_profile_role(&serde_json::json!({
        "profile_id": artists[0],
        "new_role": "organizer"
    })).await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_set_body();
    let response = app.post_set(&body).await;
    assert!(
        response.status().is_success(),
        "Failed to create set: status {}",
        response.status()
    );

    let set_id = sqlx::query_scalar!(r#"SELECT id FROM sets WHERE name=$1"#, "My Awesome Set")
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    (artists, app, set_id)
}

/// Registers 4 artists, logs in as user_0, creates a Set and a Festival.
/// Returns (artists, app, set_id, festival_id).
pub async fn setup_festival_creation() -> (Vec<Uuid>, TestApp, Uuid, Uuid) {
    let (artists, app, set_id) = setup_set_creation().await;

    let body = fixtures::create_festival_body(set_id, &[artists[1], artists[2]]);
    let response = app.post_festival(set_id, &body).await;
    assert!(
        response.status().is_success(),
        "Failed to create festival: status {}",
        response.status()
    );

    let festival_id = sqlx::query_scalar!(
        r#"SELECT id FROM festivals WHERE name=$1"#,
        "Grand Cinematic Festival"
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    (artists, app, set_id, festival_id)
}

/// Full stack setup: 4 artists, an original, a work upload.
/// Returns (artists, app, original_id, work_id).
pub async fn setup_work_uploaded() -> (Vec<Uuid>, TestApp, Uuid, Uuid) {
    let (artists, app, original_id) = setup_edit_upload().await;

    // Login as user_0 and upload a work
    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;
    let body = fixtures::create_edit_body(original_id);
    let res = app.post_work("EDIT", &body).await;
    assert!(
        res.status().is_success(),
        "Failed to upload work: status {}",
        res.status()
    );

    let work_id: Uuid =
        sqlx::query_scalar!(r#"SELECT id FROM works WHERE title=$1"#, "OG Intro Blast")
            .fetch_one(&app.state.db_pool)
            .await
            .expect("db query failed");

    (artists, app, original_id, work_id)
}

/// Registers and logs in an admin user in an EXISTING TestApp.
/// This ensures the admin operates in the SAME database as all artist data.
/// Call this AFTER setup_edit_upload / setup_set_creation so the original/set
/// data already exists in the app's database before the admin acts on it.
pub async fn login_as_admin(app: &TestApp) {
    let res = app
        .post_admin_register(&fixtures::admin_register_body())
        .await;
    assert!(
        res.status().is_success(),
        "Failed to register admin: status {}",
        res.status()
    );
    let res = app.post_admin_login(&fixtures::admin_login_body()).await;
    assert!(
        res.status().is_success(),
        "Failed to login admin: status {}",
        res.status()
    );
}

/// Registers a user with organizer role and returns their ID + the running TestApp.
/// This is used for testing organizer-specific functionality like set/festival creation.
pub async fn setup_organizer_user() -> (Uuid, TestApp) {
    let app = spawn_app::spawn().await;

    // Register as organizer
    let handle = "organizer_user";
    let body = fixtures::register_body(handle, "kApten@1023");
    let response = app.post_register(&body).await;
    assert!(
        response.status().is_success(),
        "Failed to register organizer: status {}",
        response.status()
    );

    let profile_id = sqlx::query_scalar!(r#"SELECT id FROM profiles WHERE user_name=$1"#, handle)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    // Update the user's role to organizer via admin
    login_as_admin(&app).await;
    app.post_create_role(&serde_json::json!({
        "name": "organizer",
        "description": "Can organize sets and festivals"
    })).await;
    let role_body = serde_json::json!({
        "profile_id": profile_id,
        "new_role": "organizer"
    });
    let response = app.post_update_profile_role(&role_body).await;
    assert!(
        response.status().is_success(),
        "Failed to update user role to organizer: status {}",
        response.status()
    );

    // Login as the organizer user
    app.post_login(&fixtures::login_body(handle, "kApten@1023"))
        .await;

    (profile_id, app)
}
