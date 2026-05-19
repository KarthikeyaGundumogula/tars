mod common;
use common::{fixtures, setups::setup_original_registration, spawn_app};

// ---------------------------------------------------------------------------
// Update Profile
// ---------------------------------------------------------------------------

#[tokio::test]
async fn update_profile_returns_200_for_valid_data() {
    let app = spawn_app::spawn().await;

    app.post_register(&fixtures::register_body("kapten", "kApten@1023"))
        .await;
    app.post_login(&fixtures::login_body("kapten", "kApten@1023"))
        .await;

    let response = app
        .post_update_profile(&fixtures::update_profile_body())
        .await;

    assert!(response.status().is_success());

    let saved = sqlx::query!(
        r#"SELECT tag_line, stage_name FROM profiles WHERE user_name = $1"#,
        "kapten"
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("Failed to fetch profile");

    assert_eq!(saved.tag_line, "updated tagline");
    assert_eq!(saved.stage_name, "kapten og".to_string());
}

#[tokio::test]
async fn update_profile_returns_401_for_unauthorized_user() {
    let app = spawn_app::spawn().await;

    let response = app
        .post_update_profile(&fixtures::update_profile_body())
        .await;

    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
}

// ---------------------------------------------------------------------------
// Follow / Unfollow
// ---------------------------------------------------------------------------

#[tokio::test]
async fn follow_artist_returns_200_on_valid_request() {
    // Need 2 artists: a follower and someone to follow
    let (artists, app) = setup_original_registration().await;

    // Login as user_0, follow user_1
    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::artist_action_body(artists[1]);
    let response = app.post_follow(&body).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    // Verify in DB
    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM followings WHERE follower_id=$1 AND following_id=$2"#,
    )
    .bind(artists[0])
    .bind(artists[1])
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(count, 1);
}

#[tokio::test]
async fn follow_artist_returns_401_when_not_logged_in() {
    let (artists, app) = setup_original_registration().await;

    let body = fixtures::artist_action_body(artists[1]);
    let response = app.post_follow(&body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn unfollow_artist_returns_200_after_following() {
    let (artists, app) = setup_original_registration().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    // Follow first
    app.post_follow(&fixtures::artist_action_body(artists[1]))
        .await;

    // Then unfollow
    let response = app
        .post_unfollow(&fixtures::artist_action_body(artists[1]))
        .await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    // Verify removed from DB
    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM followings WHERE follower_id=$1 AND following_id=$2"#,
    )
    .bind(artists[0])
    .bind(artists[1])
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(count, 0);
}

// ---------------------------------------------------------------------------
// Favorite / Unfavorite
// ---------------------------------------------------------------------------

#[tokio::test]
async fn favorite_artist_returns_200_on_valid_request() {
    let (artists, app) = setup_original_registration().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let response = app
        .post_favorite(&fixtures::artist_action_body(artists[1]))
        .await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM favorite_profiles WHERE profile_id=$1 AND favorited_id=$2"#,
    )
    .bind(artists[0])
    .bind(artists[1])
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(count, 1);
}

#[tokio::test]
async fn favorite_artist_returns_401_when_not_logged_in() {
    let (artists, app) = setup_original_registration().await;

    let response = app
        .post_favorite(&fixtures::artist_action_body(artists[1]))
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn unfavorite_artist_returns_200_after_favoriting() {
    let (artists, app) = setup_original_registration().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    // Favorite first
    app.post_favorite(&fixtures::artist_action_body(artists[1]))
        .await;

    // Then unfavorite
    let response = app
        .post_unfavorite(&fixtures::artist_action_body(artists[1]))
        .await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM favorite_profiles WHERE profile_id=$1 AND favorited_id=$2"#,
    )
    .bind(artists[0])
    .bind(artists[1])
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(count, 0);
}
