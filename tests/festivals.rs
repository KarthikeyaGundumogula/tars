mod common;
use common::{fixtures, setups::setup_set_creation, spawn_app};

#[tokio::test]
async fn create_festival_return_success_on_correct_data() {
    let (artists, app, set_id) = setup_set_creation().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023")).await;

    let body = fixtures::create_festival_body(set_id, &[artists[1], artists[2]]);
    let response = app.post_festival(&body).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let saved_festival = sqlx::query!(
        r#"SELECT id, name, description, set_id, organizer FROM festivals WHERE name=$1"#,
        "Grand Cinematic Festival"
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("Failed to find uploaded festival in DB");

    assert_eq!(saved_festival.name, "Grand Cinematic Festival");
    assert_eq!(saved_festival.description, "Annual film festival celebrating the arts");
    assert_eq!(saved_festival.set_id, set_id);

    let saved_panelists = sqlx::query!(
        r#"SELECT profile_id FROM panelists WHERE festival_id=$1"#,
        saved_festival.id
    )
    .fetch_all(&app.state.db_pool)
    .await
    .expect("Failed to find panelists in DB");

    assert_eq!(saved_panelists.len(), 2);
}

#[tokio::test]
async fn create_festival_returns_401_without_login() {
    let app = spawn_app::spawn().await;

    // Body doesn't matter — auth guard fires first
    let body = fixtures::create_festival_body(uuid::Uuid::new_v4(), &[]);
    let response = app.post_festival(&body).await;

    assert_eq!(response.status().as_u16(), 401);
}
