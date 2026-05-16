mod common;
use chrono::Utc;
use common::{fixtures, setups::setup_original_registration};

#[tokio::test]
async fn create_original_return_success_on_correct_data() {
    let (artists, app) = setup_original_registration().await;

    let body = fixtures::create_original_body(&artists);
    let response = app.post_original(&body).await;

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let description = sqlx::query_scalar!(
        r#"SELECT description FROM originals WHERE title=$1"#,
        "They Call him Og"
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    let actors: Vec<String> = sqlx::query_scalar!(
        r#"SELECT role_name FROM roles ORDER BY role_name DESC"#
    )
    .fetch_all(&app.state.db_pool)
    .await
    .expect("db query failed");

    assert_eq!(description, "A cinematic masterpiece from the streets");
    assert_eq!(actors[0], "Ojas Ghambheera".to_string());
    assert_eq!(actors[2], "Kanmani".to_string());
}

#[tokio::test]
async fn create_original_returns_422_when_data_is_missing() {
    let (artists, app) = setup_original_registration().await;

    let test_cases = vec![
        (
            serde_json::json!({
                // missing `password` and `title`
                "description": "A cinematic masterpiece",
                "cover_img": "https://cdn.example.com/og_cover.jpg",
                "associated_with": artists[0],
                "release_date": Utc::now(),
                "genres": ["action", "drama"],
                "stars": [],
                "makers": []
            }),
            "missing password and title",
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
                // missing cover_img
            }),
            "missing cover image",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_original(&invalid_body).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Expected 422 when payload was: {}",
            error_message
        );
    }
}
