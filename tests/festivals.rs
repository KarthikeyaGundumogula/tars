mod common;
use common::{fixtures, setups::setup_festival_creation, spawn_app};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Create Festival
// ---------------------------------------------------------------------------

#[tokio::test]
async fn create_festival_returns_200_for_set_owner() {
    let (artists, app, set_id) = common::setups::setup_set_creation().await;

    let response = app
        .post_festival(
            set_id,
            &fixtures::create_festival_body(set_id, &[artists[1], artists[2]]),
        )
        .await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );
}

#[tokio::test]
async fn create_festival_returns_401_for_non_owner() {
    let (artists, app, set_id) = common::setups::setup_set_creation().await;

    app.post_login(&fixtures::login_body("user_1", "kApten@1023"))
        .await;

    let response = app
        .post_festival(
            set_id,
            &fixtures::create_festival_body(set_id, &[artists[1], artists[2]]),
        )
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

// ---------------------------------------------------------------------------
// Update Festival Details
// ---------------------------------------------------------------------------

#[tokio::test]
async fn update_festival_returns_200_for_organizer() {
    let (_, app, _, festival_id) = setup_festival_creation().await;
    // user_0 is still logged in as the organizer

    let response = app
        .post_update_festival(festival_id, &fixtures::update_festival_body())
        .await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let name: String = sqlx::query_scalar(r#"SELECT name FROM festivals WHERE id=$1"#)
        .bind(festival_id)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    assert_eq!(name, "Updated Festival Name");
}

#[tokio::test]
async fn update_festival_returns_401_for_non_organizer() {
    let (_, app, _, festival_id) = setup_festival_creation().await;

    // user_2 is a panelist, NOT the organizer
    app.post_login(&fixtures::login_body("user_2", "kApten@1023"))
        .await;

    let response = app
        .post_update_festival(festival_id, &fixtures::update_festival_body())
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn update_festival_returns_401_when_not_logged_in() {
    let app = spawn_app::spawn().await;

    let response = app
        .post_update_festival(Uuid::new_v4(), &fixtures::update_festival_body())
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

// ---------------------------------------------------------------------------
// Update Panelists (add / remove)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn add_panelist_returns_200_for_organizer() {
    let (artists, app, _, festival_id) = setup_festival_creation().await;
    // user_0 (organizer) adds user_3 as a new panelist

    let response = app
        .post_update_panelists(festival_id, &fixtures::add_panelist_body(artists[3]))
        .await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM panelists WHERE festival_id=$1 AND profile_id=$2"#,
    )
    .bind(festival_id)
    .bind(artists[3])
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(count, 1);
}

#[tokio::test]
async fn remove_panelist_returns_200_for_organizer() {
    let (artists, app, _, festival_id) = setup_festival_creation().await;
    // artists[1] and artists[2] were seeded as panelists in setup_festival_creation

    // Remove artist[1]
    let response = app
        .post_update_panelists(festival_id, &fixtures::remove_panelist_body(artists[1]))
        .await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM panelists WHERE festival_id=$1 AND profile_id=$2"#,
    )
    .bind(festival_id)
    .bind(artists[1])
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(count, 0);
}

#[tokio::test]
async fn update_panelists_returns_401_for_non_organizer() {
    let (artists, app, _, festival_id) = setup_festival_creation().await;

    app.post_login(&fixtures::login_body("user_2", "kApten@1023"))
        .await;

    let response = app
        .post_update_panelists(festival_id, &fixtures::add_panelist_body(artists[3]))
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn update_panelists_returns_401_when_not_logged_in() {
    let app = spawn_app::spawn().await;

    let response = app
        .post_update_panelists(Uuid::new_v4(), &fixtures::add_panelist_body(Uuid::new_v4()))
        .await;

    assert_eq!(response.status().as_u16(), 401);
}
