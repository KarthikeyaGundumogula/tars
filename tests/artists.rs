mod common;
use common::{fixtures, setups::setup_original_registration, spawn_app};
use uuid::Uuid;

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

// ---------------------------------------------------------------------------
// Save / Unsave Work
// ---------------------------------------------------------------------------

#[tokio::test]
async fn save_work_returns_200_on_valid_request() {
    let (_, app, _, work_id) = common::setups::setup_work_uploaded().await;
    app.post_login(&fixtures::login_body("user_1", "kApten@1023")).await;

    let response = app.post_save_work(&fixtures::entity_action_body(work_id)).await;
    assert!(response.status().is_success());

    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM saved_works WHERE work_id=$1"#
    )
    .bind(work_id)
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");
    assert_eq!(count, 1);
}

#[tokio::test]
async fn unsave_work_returns_200_after_saving() {
    let (_, app, _, work_id) = common::setups::setup_work_uploaded().await;
    app.post_login(&fixtures::login_body("user_1", "kApten@1023")).await;

    app.post_save_work(&fixtures::entity_action_body(work_id)).await;
    let response = app.delete_unsave_work(&fixtures::entity_action_body(work_id)).await;
    assert!(response.status().is_success());

    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM saved_works WHERE work_id=$1"#
    )
    .bind(work_id)
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");
    assert_eq!(count, 0);
}

// ---------------------------------------------------------------------------
// Boost / Remove Boost Recommendation
// ---------------------------------------------------------------------------

#[tokio::test]
async fn boost_recommendation_returns_200_on_valid_request() {
    let (_, app, original_id, _) = common::setups::setup_work_uploaded().await;
    app.post_login(&fixtures::login_body("user_0", "kApten@1023")).await;

    // Create a recommendation
    app.post_recommendation(&fixtures::create_recommendation_body(original_id)).await;
    let rec_id: Uuid = sqlx::query_scalar!(r#"SELECT id FROM recommendations WHERE original_id=$1"#, original_id)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    app.post_login(&fixtures::login_body("user_1", "kApten@1023")).await;
    let response = app.post_boost_recommendation(&fixtures::entity_action_body(rec_id)).await;
    assert!(response.status().is_success());

    let count: i64 = sqlx::query_scalar(
        r#"SELECT boost_number FROM recommendations WHERE id=$1"#
    )
    .bind(rec_id)
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");
    assert_eq!(count, 1);
}

// ---------------------------------------------------------------------------
// Save / Unsave Recommendation
// ---------------------------------------------------------------------------

#[tokio::test]
async fn save_recommendation_returns_200_on_valid_request() {
    let (_, app, original_id, _) = common::setups::setup_work_uploaded().await;
    app.post_login(&fixtures::login_body("user_0", "kApten@1023")).await;

    app.post_recommendation(&fixtures::create_recommendation_body(original_id)).await;
    let rec_id: Uuid = sqlx::query_scalar!(r#"SELECT id FROM recommendations WHERE original_id=$1"#, original_id)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    app.post_login(&fixtures::login_body("user_1", "kApten@1023")).await;
    let response = app.post_save_recommendation(&fixtures::entity_action_body(rec_id)).await;
    assert!(response.status().is_success());

    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM saved_recommendations WHERE recommendation_id=$1"#
    )
    .bind(rec_id)
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");
    assert_eq!(count, 1);
}

#[tokio::test]
async fn unsave_recommendation_returns_200_after_saving() {
    let (_, app, original_id, _) = common::setups::setup_work_uploaded().await;
    app.post_login(&fixtures::login_body("user_0", "kApten@1023")).await;

    app.post_recommendation(&fixtures::create_recommendation_body(original_id)).await;
    let rec_id: Uuid = sqlx::query_scalar!(r#"SELECT id FROM recommendations WHERE original_id=$1"#, original_id)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    app.post_login(&fixtures::login_body("user_1", "kApten@1023")).await;
    app.post_save_recommendation(&fixtures::entity_action_body(rec_id)).await;
    let response = app.delete_unsave_recommendation(&fixtures::entity_action_body(rec_id)).await;
    assert!(response.status().is_success());

    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM saved_recommendations WHERE recommendation_id=$1"#
    )
    .bind(rec_id)
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");
    assert_eq!(count, 0);
}

#[tokio::test]
async fn remove_recommendation_boost_returns_200_after_boosting() {
    let (_, app, original_id, _) = common::setups::setup_work_uploaded().await;
    app.post_login(&fixtures::login_body("user_0", "kApten@1023")).await;

    // Create a recommendation
    app.post_recommendation(&fixtures::create_recommendation_body(original_id)).await;
    let rec_id: Uuid = sqlx::query_scalar!(r#"SELECT id FROM recommendations WHERE original_id=$1"#, original_id)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    app.post_login(&fixtures::login_body("user_1", "kApten@1023")).await;
    app.post_boost_recommendation(&fixtures::entity_action_body(rec_id)).await;
    let response = app.delete_remove_recommendation_boost(&fixtures::entity_action_body(rec_id)).await;
    assert!(response.status().is_success());

    let count: i64 = sqlx::query_scalar(
        r#"SELECT boost_number FROM recommendations WHERE id=$1"#
    )
    .bind(rec_id)
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");
    assert_eq!(count, 0);
}
