mod common;
use common::{fixtures, setups::setup_work_uploaded, spawn_app};

// ---------------------------------------------------------------------------
// GET /profiles/get_profile_details/{user_name}
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_profile_details_returns_200_for_artist_with_works() {
    let (_artists, app, _original_id, _work_id) = setup_work_uploaded().await;

    // user_0 has a registered profile + an uploaded work, so the INNER JOIN succeeds
    let response = app.get_profile_details("user_0").await;

    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    // Verify the JSON body contains the expected profile data
    let body: serde_json::Value = response.json().await.expect("Failed to parse JSON body");
    let stage = &body["artist_stage"];

    assert_eq!(stage["user_name"], "user_0");
    assert_eq!(stage["stage_name"], "kapten");
    assert_eq!(stage["tag_line"], "I dont give a dmn about your opinion");
    assert_eq!(stage["color_theme"], "#FF0000");

    // Works array should contain the uploaded work
    let works = stage["works"].as_array().expect("works should be an array");
    assert!(!works.is_empty(), "Expected at least one work preview");
    assert_eq!(works[0]["title"], "OG Intro Blast");
}

#[tokio::test]
async fn get_profile_details_returns_404_for_nonexistent_user() {
    let app = spawn_app::spawn().await;

    let response = app.get_profile_details("nonexistent_user_xyz").await;

    assert_eq!(
        response.status().as_u16(),
        404,
        "Expected 404 for a user that does not exist"
    );
}

#[tokio::test]
async fn get_profile_details_returns_404_for_artist_without_works() {
    let app = spawn_app::spawn().await;

    // Register an artist but do NOT upload any works.
    // The INNER JOIN in get_profile_details_by_username will yield 0 rows → NotFound
    app.post_register(&fixtures::register_body("lonely_artist", "kApten@1023"))
        .await;

    let response = app.get_profile_details("lonely_artist").await;

    assert_eq!(
        response.status().as_u16(),
        404,
        "Expected 404 for artist with no works (INNER JOIN returns empty)"
    );
}

#[tokio::test]
async fn get_profile_details_returns_correct_social_profiles() {
    let (_artists, app, _original_id, _work_id) = setup_work_uploaded().await;

    let response = app.get_profile_details("user_0").await;
    assert!(response.status().is_success());

    let body: serde_json::Value = response.json().await.expect("Failed to parse JSON body");
    let stage = &body["artist_stage"];

    // The register fixture sets youtube_profile = "aojojfosjf"
    assert_eq!(stage["youtube_profile"], "aojojfosjf");
    // profile_picture should be present
    assert_eq!(stage["profile_picture"], "aofdjosfjosf");
}

#[tokio::test]
async fn get_profile_details_does_not_require_authentication() {
    let (_artists, app, _original_id, _work_id) = setup_work_uploaded().await;

    // Build a brand new client with no cookies (no auth)
    let anon_client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let response = anon_client
        .get(&format!(
            "{}/profiles/get_profile_details/user_0",
            &app.address
        ))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(
        response.status().is_success(),
        "Profile endpoint should be publicly accessible, got {}",
        response.status()
    );
}
