mod common;
use chrono::Utc;
use common::setups::setup_set_creation;

#[tokio::test]
async fn create_festival_return_success_on_correct_data() {
    // Arrange
    let (artists, app, set_id) = setup_set_creation().await;

    // Login user
    let login_body = serde_json::json!({
        "handle": "user_0",
        "password": "kApten@1023"
    });
    app.post_login(&login_body).await;

    let festival_name = "Grand Cinematic Festival";
    let body = serde_json::json!({
        "name": festival_name,
        "description": "Annual film festival",
        "rules":"1.ojaodjoaj, 2.jojafoha",
        "set_id": set_id,
        "start_date": Utc::now(),
        "end_date": Utc::now(),
        "panelists": [artists[1], artists[2]]
    });

    // Act
    let response = app.post_festival(&body).await;
    println!("Response status: {:?}", response.json::<serde_json::Value>().await);
    // Assert
    // assert_eq!(response.status(), reqwest::StatusCode::OK);

    let saved_festival = sqlx::query!(
        r#"SELECT id, name, description, set_id, organizer FROM festivals WHERE name=$1"#,
        festival_name
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("Failed to find uploaded festival in DB");

    assert_eq!(saved_festival.name, festival_name);
    assert_eq!(saved_festival.description, "Annual film festival");
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
    let app = common::spawn_app::spawn().await;

    let body = serde_json::json!({
        "name": "Festival Name",
        "description": "Description",
        "set_id": uuid::Uuid::new_v4(),
        "start_date": Utc::now(),
        "end_date": Utc::now(),
        "panelists": []
    });

    let response = app.post_festival(&body).await;
    assert_eq!(response.status().as_u16(), 401);
}
