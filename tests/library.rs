mod common;
use common::{fixtures, setups::setup_edit_upload, spawn_app};
use tars::models::db::library::{LibraryEntryType, WatchlistStatus};
use uuid::Uuid;

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

// ---------------------------------------------------------------------------
// Library Entry with Surge Score
// ---------------------------------------------------------------------------

#[tokio::test]
async fn create_library_entry_with_surge_updates_original_statistics() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_library_body_with_surge(original_id, 1500);
    app.post_library(&body).await;

    let number_of_surges: i32 =
        sqlx::query_scalar(r#"SELECT COALESCE(number_of_surges, 0) FROM originals WHERE id=$1"#)
            .bind(original_id)
            .fetch_one(&app.state.db_pool)
            .await
            .expect("db query failed");

    let mean_surge: i64 =
        sqlx::query_scalar(r#"SELECT COALESCE(mean_surge, 0) FROM originals WHERE id=$1"#)
            .bind(original_id)
            .fetch_one(&app.state.db_pool)
            .await
            .expect("db query failed");

    assert!(number_of_surges >= 1);
    assert!(mean_surge > 0);
}

#[tokio::test]
async fn create_library_entry_updates_profile_peak_library() {
    let (artists, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_library_body_with_surge(original_id, 1500);
    app.post_library(&body).await;

    let peak: i64 =
        sqlx::query_scalar(r#"SELECT COALESCE(current_peak_library, 0) FROM profiles WHERE id=$1"#)
            .bind(artists[0])
            .fetch_one(&app.state.db_pool)
            .await
            .expect("db query failed");

    assert_eq!(peak, 1500);
}

#[tokio::test]
async fn create_multiple_library_entries_calculates_surge_spread() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    // Create multiple entries with different surge scores
    app.post_library(&fixtures::create_library_body_with_surge(original_id, 100))
        .await;
    app.post_library(&fixtures::create_library_body_with_surge(original_id, 200))
        .await;
    app.post_library(&fixtures::create_library_body_with_surge(original_id, 150))
        .await;

    let number_of_surges: i32 =
        sqlx::query_scalar(r#"SELECT COALESCE(number_of_surges, 0) FROM originals WHERE id=$1"#)
            .bind(original_id)
            .fetch_one(&app.state.db_pool)
            .await
            .expect("db query failed");

    assert_eq!(number_of_surges, 3);
}

// ---------------------------------------------------------------------------
// Update Library Entry
// ---------------------------------------------------------------------------

#[tokio::test]
async fn update_library_entry_returns_200_for_owner() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    // Create library entry first
    let body = fixtures::create_library_body(original_id);
    app.post_library(&body).await;

    let entry_id: Uuid = sqlx::query_scalar!(
        r#"SELECT id FROM library WHERE original_id=$1"#,
        original_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    // Update it
    let update_body = fixtures::update_library_entry_with_surge();
    let response = app.post_update_library_entry(entry_id, &update_body).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );
}

#[tokio::test]
async fn update_library_entry_returns_401_for_non_owner() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_library_body(original_id);
    app.post_library(&body).await;

    let entry_id: Uuid = sqlx::query_scalar!(
        r#"SELECT id FROM library WHERE original_id=$1"#,
        original_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    // Try to update as different user
    app.post_login(&fixtures::login_body("user_1", "kApten@1023"))
        .await;

    let update_body = fixtures::update_library_entry_with_surge();
    let response = app.post_update_library_entry(entry_id, &update_body).await;

    assert_eq!(response.status().as_u16(), 401);
}

// ---------------------------------------------------------------------------
// Tag Work to Library Entry
// ---------------------------------------------------------------------------

#[tokio::test]
async fn tag_work_to_library_entry_returns_200_for_owner() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_library_body(original_id);
    app.post_library(&body).await;

    let entry_id: Uuid = sqlx::query_scalar!(
        r#"SELECT id FROM library WHERE original_id=$1"#,
        original_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    let work_id = Uuid::new_v4();
    let tag_body = fixtures::tag_work_to_library_body(work_id);
    let response = app.post_tag_work_to_library(entry_id, &tag_body).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );
}

#[tokio::test]
async fn tag_work_to_library_entry_correctly_adds_work() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_library_body(original_id);
    app.post_library(&body).await;

    let entry_id: Uuid = sqlx::query_scalar!(
        r#"SELECT id FROM library WHERE original_id=$1"#,
        original_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    let work_id = Uuid::new_v4();
    let tag_body = fixtures::tag_work_to_library_body(work_id);
    app.post_tag_work_to_library(entry_id, &tag_body).await;

    let tagged_works: Option<Vec<Uuid>> =
        sqlx::query_scalar!(r#"SELECT tagged_works FROM library WHERE id=$1"#, entry_id)
            .fetch_one(&app.state.db_pool)
            .await
            .expect("db query failed");

    assert!(tagged_works.unwrap().contains(&work_id));
}

// ---------------------------------------------------------------------------
// Delete Library Entry
// ---------------------------------------------------------------------------

#[tokio::test]
async fn delete_library_entry_returns_200_for_owner() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_library_body(original_id);
    app.post_library(&body).await;

    let entry_id: Uuid = sqlx::query_scalar!(
        r#"SELECT id FROM library WHERE original_id=$1"#,
        original_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    let response = app.delete_library_entry(entry_id).await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );
}

#[tokio::test]
async fn delete_library_entry_removes_from_database() {
    let (_, app, original_id) = setup_edit_upload().await;

    app.post_login(&fixtures::login_body("user_0", "kApten@1023"))
        .await;

    let body = fixtures::create_library_body(original_id);
    app.post_library(&body).await;

    let entry_id: Uuid = sqlx::query_scalar!(
        r#"SELECT id FROM library WHERE original_id=$1"#,
        original_id
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");

    app.delete_library_entry(entry_id).await;

    let count: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM library WHERE id=$1"#)
        .bind(entry_id)
        .fetch_one(&app.state.db_pool)
        .await
        .expect("db query failed");

    assert_eq!(count, 0);
}
