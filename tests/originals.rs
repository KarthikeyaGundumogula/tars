mod common;
use chrono::Utc;

use crate::common::setups::setup_original_registration;

#[tokio::test]
async fn create_original_return_success_on_correct_data() {
    // Arrange
    let (artists, app) = setup_original_registration().await;

    let body = serde_json::json!({
        "title": "They Call him Og",
        "description": "fuck you staya dada",
        "cover_img": "canada is fucked",
        "password": "Kap@123456",
        "associated_with": artists[0],
        "release_date": Utc::now(),
        "genres": ["action", "drama"],
        "stars": [{
            "role": "Ojas Ghambheera",
            "artist": artists[1]
        },{
            "role": "Kanmani",
            "artist": artists[2]
        }],
        "makers": [{
            "role": "Music Director",
            "artist": artists[3]
        },{
            "role": "Director",
            "artist": artists[1]
        }]
    });

    // Act
    let response = app.post_original(&body).await;

    // Assert
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let description = sqlx::query_scalar!(
        r#"SELECT description FROM originals WHERE title=$1"#,
        "They Call him Og"
    )
    .fetch_one(&app.state.pool)
    .await
    .expect("db query failed");

    let actors: Vec<String> = sqlx::query_scalar!(
        r#"SELECT role_name FROM roles ORDER BY role_name DESC"#
    )
    .fetch_all(&app.state.pool)
    .await
    .expect("db query failed");
    println!("actors: {:?}", actors);
    assert_eq!(description, "fuck you staya dada".to_string());
    assert_eq!(actors[0], "Ojas Ghambheera".to_string());
    assert_eq!(actors[2], "Kanmani".to_string());
}

#[tokio::test]
async fn create_original_returns_400_when_data_is_missing_or_invalid() {
    // Arrange
    let (artists, app) = setup_original_registration().await;
    
    // Table-driven tests for validation
    let test_cases = vec![
        (
            serde_json::json!({
                // missing password - should faail
                "description": "fuck you staya dada",
                "cover_img": "canada is fucked",
                "associated_with": artists[0],
                "release_date": Utc::now(),
                "genres": ["action", "drama"],
                "stars": [],
                "makers": []
            }),
            "missing password",
        ),
        (
            serde_json::json!({
                "title": "They Call him Og",
                "description": "<>", 
                "password": "Kap@123456",
                "associated_with": artists[0],
                "release_date": Utc::now(),
                "genres": ["action", "drama"],
                "stars": [],
                "makers": []
            }),
            "missing cover image",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_original(&invalid_body).await;

        // Assert
        assert_eq!(
            response.status().as_u16(),
            422,
            "The API did not fail with 422 Unprocessable Entity when the payload was {}",
            error_message
        );
    }
}
