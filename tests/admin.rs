mod common;
use common::{fixtures, setups::login_as_admin, spawn_app};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Create Role
// ---------------------------------------------------------------------------

#[tokio::test]
async fn create_role_returns_200_for_admin_user() {
    let app = spawn_app::spawn().await;

    login_as_admin(&app).await;

    let body = fixtures::create_role_body();
    let response = app.post_create_role(&body).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );
}

#[tokio::test]
async fn create_role_returns_401_for_non_admin_user() {
    let app = spawn_app::spawn().await;

    // Register and login as regular artist
    let body = fixtures::register_body("regular_user", "kApten@1023");
    app.post_register(&body).await;
    app.post_login(&fixtures::login_body("regular_user", "kApten@1023"))
        .await;

    let role_body = fixtures::create_role_body();
    let response = app.post_create_role(&role_body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn create_role_returns_401_for_unauthenticated_user() {
    let app = spawn_app::spawn().await;

    let body = fixtures::create_role_body();
    let response = app.post_create_role(&body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn create_role_correctly_saves_role_data() {
    let app = spawn_app::spawn().await;

    login_as_admin(&app).await;

    let body = fixtures::create_role_body();
    app.post_create_role(&body).await;

    let saved = sqlx::query!(
        r#"SELECT name, description FROM user_roles WHERE name=$1"#,
        "moderator"
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(saved.name, "moderator");
    assert_eq!(saved.description, Some("Can moderate content".to_string()));
}

// ---------------------------------------------------------------------------
// Create Permission
// ---------------------------------------------------------------------------

#[tokio::test]
async fn create_permission_returns_200_for_admin_user() {
    let app = spawn_app::spawn().await;

    login_as_admin(&app).await;

    let body = fixtures::create_permission_body();
    let response = app.post_create_permission(&body).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );
}

#[tokio::test]
async fn create_permission_returns_401_for_non_admin_user() {
    let app = spawn_app::spawn().await;

    // Register and login as regular artist
    let body = fixtures::register_body("regular_user", "kApten@1023");
    app.post_register(&body).await;
    app.post_login(&fixtures::login_body("regular_user", "kApten@1023"))
        .await;

    let permission_body = fixtures::create_permission_body();
    let response = app.post_create_permission(&permission_body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn create_permission_correctly_saves_permission_data() {
    let app = spawn_app::spawn().await;

    login_as_admin(&app).await;

    let body = fixtures::create_permission_body();
    app.post_create_permission(&body).await;

    let saved = sqlx::query!(
        r#"SELECT name, description FROM permissions WHERE name=$1"#,
        "delete_posts"
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(saved.name, "delete_posts");
    assert_eq!(
        saved.description,
        Some("Can delete any post".to_string())
    );
}

// ---------------------------------------------------------------------------
// Assign Permission to Role
// ---------------------------------------------------------------------------

#[tokio::test]
async fn assign_permission_returns_200_for_admin() {
    let app = spawn_app::spawn().await;

    login_as_admin(&app).await;

    // Create role and permission first
    app.post_create_role(&fixtures::create_role_body()).await;
    app.post_create_permission(&fixtures::create_permission_body())
        .await;

    let body = fixtures::assign_permission_body();
    let response = app.post_assign_permission(&body).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );
}

#[tokio::test]
async fn assign_permission_returns_401_for_non_admin() {
    let app = spawn_app::spawn().await;

    // Register and login as regular artist
    let body = fixtures::register_body("regular_user", "kApten@1023");
    app.post_register(&body).await;
    app.post_login(&fixtures::login_body("regular_user", "kApten@1023"))
        .await;

    let assign_body = fixtures::assign_permission_body();
    let response = app.post_assign_permission(&assign_body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn assign_permission_correctly_stores_relationship() {
    let app = spawn_app::spawn().await;

    login_as_admin(&app).await;

    app.post_create_role(&fixtures::create_role_body()).await;
    app.post_create_permission(&fixtures::create_permission_body())
        .await;

    let body = fixtures::assign_permission_body();
    app.post_assign_permission(&body).await;

    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM role_permissions WHERE role_name=$1 AND permission_name=$2"#,
    )
    .bind("moderator")
    .bind("delete_posts")
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(count, 1);
}

// ---------------------------------------------------------------------------
// Revoke Permission from Role
// ---------------------------------------------------------------------------

#[tokio::test]
async fn revoke_permission_returns_200_for_admin() {
    let app = spawn_app::spawn().await;

    login_as_admin(&app).await;

    // Setup: create role, permission, and assign
    app.post_create_role(&fixtures::create_role_body()).await;
    app.post_create_permission(&fixtures::create_permission_body())
        .await;
    app.post_assign_permission(&fixtures::assign_permission_body())
        .await;

    let body = fixtures::revoke_permission_body();
    let response = app.post_revoke_permission(&body).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );
}

#[tokio::test]
async fn revoke_permission_returns_401_for_non_admin() {
    let app = spawn_app::spawn().await;

    // Register and login as regular artist
    let body = fixtures::register_body("regular_user", "kApten@1023");
    app.post_register(&body).await;
    app.post_login(&fixtures::login_body("regular_user", "kApten@1023"))
        .await;

    let revoke_body = fixtures::revoke_permission_body();
    let response = app.post_revoke_permission(&revoke_body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn revoke_permission_correctly_removes_relationship() {
    let app = spawn_app::spawn().await;

    login_as_admin(&app).await;

    app.post_create_role(&fixtures::create_role_body()).await;
    app.post_create_permission(&fixtures::create_permission_body())
        .await;
    app.post_assign_permission(&fixtures::assign_permission_body())
        .await;

    let body = fixtures::revoke_permission_body();
    app.post_revoke_permission(&body).await;

    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM role_permissions WHERE role_name=$1 AND permission_name=$2"#,
    )
    .bind("moderator")
    .bind("delete_posts")
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(count, 0);
}

// ---------------------------------------------------------------------------
// Update Profile Role
// ---------------------------------------------------------------------------

#[tokio::test]
async fn update_profile_role_returns_200_for_admin() {
    let app = spawn_app::spawn().await;

    // Register a regular user
    let body = fixtures::register_body("regular_user", "kApten@1023");
    app.post_register(&body).await;

    let profile_id: Uuid = sqlx::query_scalar!(
        r#"SELECT id FROM profiles WHERE user_name=$1"#,
        "regular_user"
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    login_as_admin(&app).await;

    // Create the 'moderator' role first
    app.post_create_role(&fixtures::create_role_body()).await;

    let role_body = fixtures::update_profile_role_body(profile_id);
    let response = app.post_update_profile_role(&role_body).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );
}

#[tokio::test]
async fn update_profile_role_returns_401_for_non_admin() {
    let app = spawn_app::spawn().await;

    // Register a regular user
    let body = fixtures::register_body("regular_user", "kApten@1023");
    app.post_register(&body).await;

    let profile_id: Uuid = sqlx::query_scalar!(
        r#"SELECT id FROM profiles WHERE user_name=$1"#,
        "regular_user"
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    // Try to update role as another regular user
    let body2 = fixtures::register_body("another_user", "kApten@1023");
    app.post_register(&body2).await;
    app.post_login(&fixtures::login_body("another_user", "kApten@1023"))
        .await;

    let role_body = fixtures::update_profile_role_body(profile_id);
    let response = app.post_update_profile_role(&role_body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn update_profile_role_correctly_updates_user_role() {
    let app = spawn_app::spawn().await;

    // Register a regular user
    let body = fixtures::register_body("regular_user", "kApten@1023");
    app.post_register(&body).await;

    let profile_id: Uuid = sqlx::query_scalar!(
        r#"SELECT id FROM profiles"#
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    login_as_admin(&app).await;

    // Create the 'moderator' role first
    app.post_create_role(&fixtures::create_role_body()).await;

    let role_body = fixtures::update_profile_role_body(profile_id);
    app.post_update_profile_role(&role_body).await;

    let updated_role: String = sqlx::query_scalar!(
        r#"SELECT role_name FROM profiles WHERE id=$1"#,
        profile_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(updated_role, "moderator");
}
