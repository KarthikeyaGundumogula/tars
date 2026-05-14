mod common;
use common::setups::setup_edit_upload;
use tars::types::db::ledger::{LedgerEntryType, WatchlistStatus};

#[tokio::test]
async fn create_ledger_entry_return_success_on_correct_data() {
    // Arrange
    let (_, app, original_id) = setup_edit_upload().await;

    // Login user
    let login_body = serde_json::json!({
        "handle": "user_0",
        "password": "kApten@1023"
    });
    app.post_login(&login_body).await;

    let body = serde_json::json!({
        "original_id": original_id,
        "episode_id": null,
        "visibility": true,
        "tagged_works": [],
        "pre_thought": "I'm excited to watch this!",
        "post_impression": "It was amazing!",
        "status": "WATCHING",
        "entry_type": "MOVIE"
    });

    // Act
    let response = app.post_ledger(&body).await;

    // Assert
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let saved_entry = sqlx::query!(
        r#"SELECT id, original_id, pre_thought, post_impression, status as "status: WatchlistStatus", entry_type as "entry_type: LedgerEntryType" FROM ledger WHERE original_id=$1"#,
        original_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("Failed to find uploaded ledger entry in DB");

    assert_eq!(saved_entry.original_id, Some(original_id));
    assert_eq!(saved_entry.pre_thought, Some("I'm excited to watch this!".to_string()));
    assert_eq!(saved_entry.post_impression, Some("It was amazing!".to_string()));
}

#[tokio::test]
async fn create_ledger_entry_returns_401_without_login() {
    let app = common::spawn_app::spawn().await;

    let body = serde_json::json!({
        "original_id": uuid::Uuid::new_v4(),
        "episode_id": null,
        "visibility": "PUBLIC",
        "tagged_works": [],
        "pre_thought": "I'm excited to watch this!",
        "post_impression": "It was amazing!",
        "status": "COMPLETED",
        "entry_type": "REVIEW"
    });

    let response = app.post_ledger(&body).await;
    assert_eq!(response.status().as_u16(), 401);
}
