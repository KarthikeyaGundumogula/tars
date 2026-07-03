mod common;
use common::{fixtures, setups::setup_edit_upload, spawn_app};
use tars::models::db::library::{LibraryEntryType, WatchlistStatus};

#[tokio::test]
async fn create_library_entry_return_success_on_correct_data() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_library_body(original_id);
    let response = app.post_library(&body).await;

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let saved_entry = sqlx::query!(
        r#"SELECT id, original_id, pre_thought, post_impression, status as "status: WatchlistStatus", entry_type as "entry_type: LibraryEntryType" FROM library WHERE original_id=$1"#,
        original_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("Failed to find uploaded library entry in DB");

    assert_eq!(saved_entry.original_id, Some(original_id));
    assert_eq!(
        saved_entry.pre_thought,
        Some("I'm excited to watch this!".to_string())
    );
    assert_eq!(
        saved_entry.post_impression,
        Some("It was amazing!".to_string())
    );
}

#[tokio::test]
async fn create_library_entry_returns_401_without_login() {
    let app = spawn_app::spawn().await;

    let body = fixtures::create_library_body(uuid::Uuid::new_v4());
    let response = app.post_library(&body).await;

    assert_eq!(response.status().as_u16(), 401);
}
