mod common;
use common::setups::setup_original_registration;

#[tokio::test]
async fn create_set_return_success_on_correct_data() {
    // Arrange
    let (_, app) = setup_original_registration().await;

    // Login user
    let login_body = serde_json::json!({
        "handle": "user_0",
        "password": "kApten@1023"
    });
    app.post_login(&login_body).await;

    let set_name = "My Awesome Set";
    let body = serde_json::json!({
        "name": set_name,
        "statement": "This is a statement about the set",
        "description": "This is a longer description of the set",
        "profile_picture": "no_profile picture"
    });

    let response = app.post_set(&body).await;

    println!(
        "Response status: {:?}",
        response.json::<serde_json::Value>().await
    );
    

    let saved_set = sqlx::query!(
        r#"SELECT id, name, statement, description, curator FROM sets WHERE name=$1"#,
        set_name
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("Failed to find uploaded set in DB");

    assert_eq!(saved_set.name, set_name);
    assert_eq!(saved_set.statement, "This is a statement about the set");
    assert_eq!(
        saved_set.description,
        "This is a longer description of the set"
    );
}

#[tokio::test]
async fn create_set_returns_401_without_login() {
    let app = common::spawn_app::spawn().await;

    let body = serde_json::json!({
        "name": "My Awesome Set",
        "statement": "This is a statement about the set",
        "description": "This is a longer description of the set",
        "profile_picture": "no_profile picture" 
    });

    let response = app.post_set(&body).await;
    assert_eq!(response.status().as_u16(), 401);
}
