#![allow(dead_code)]
/// Centralized test fixtures for all integration tests.
/// Build a body ONCE here. Every test file imports from this module.
/// When a field changes (e.g. an API contract update), you change it in ONE place.
use chrono::Utc;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Auth fixtures
// ---------------------------------------------------------------------------

/// Default register body. `handle` and `password` are parameterized so
/// callers can create multiple distinct users.
pub fn register_body(handle: &str, password: &str) -> serde_json::Value {
    serde_json::json!({
        "handle": handle,
        "tag_line": "I dont give a dmn about your opinion",
        "password": password,
        "profile_picture": "aofdjosfjosf",
        "youtube_profile": "aojojfosjf",
        "stage_name": "kapten",
        "color_theme": "#FF0000"
    })
}

pub fn login_body(handle: &str, password: &str) -> serde_json::Value {
    serde_json::json!({
        "handle": handle,
        "password": password
    })
}

pub fn reset_password_body(old: &str, new: &str) -> serde_json::Value {
    serde_json::json!({
        "old_password": old,
        "new_password": new
    })
}

// ---------------------------------------------------------------------------
// Original fixtures
// ---------------------------------------------------------------------------

pub fn create_original_body(artists: &[Uuid]) -> serde_json::Value {
    serde_json::json!({
        "title": "They Call him Og",
        "description": "A cinematic masterpiece from the streets",
        "cover_img": "https://cdn.example.com/og_cover.jpg",
        "password": "Kap@123456",
        "associated_with": artists[0],
        "release_date": Utc::now(),
        "genres": ["action", "drama"],
        "stars": [
            { "role": "Ojas Ghambheera", "artist": artists[1] },
            { "role": "Kanmani",          "artist": artists[2] }
        ],
        "makers": [
            { "role": "Music Director", "artist": artists[3] },
            { "role": "Director",       "artist": artists[1] }
        ],
        "category": "MOVIE"
    })
}

// ---------------------------------------------------------------------------
// Work fixtures
// ---------------------------------------------------------------------------

pub fn create_edit_body(original_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "title": "OG Intro Blast",
        "src_id": "GG1_DsScm6U",
        "platform": "YOUTUBE",
        "format": "IMAX",
        "originals": [original_id],
        "independent":false
    })
}

pub fn create_poster_body(original_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "title": "The Golden Poster",
        "src_id": "poster_uuid_123",
        "format": "STANDARD",
        "originals": [original_id],
        "independent":false
    })
}

pub fn create_script_body(original_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "title": "Cinematic Script Draft",
        "src_ids": ["img1", "img2"],
        "originals": [original_id],
        "thoughts": ["Brilliant intro", "Dynamic pacing"],
        "independent":false
    })
}

// ---------------------------------------------------------------------------
// Set fixtures
// ---------------------------------------------------------------------------

pub fn create_set_body() -> serde_json::Value {
    serde_json::json!({
        "name": "My Awesome Set",
        "statement": "This is a statement about the set",
        "description": "This is a longer description of the set",
        "color_theme": "#FFFFFF"
    })
}

// ---------------------------------------------------------------------------
// Festival fixtures
// ---------------------------------------------------------------------------

pub fn create_festival_body(set_id: Uuid, panelists: &[Uuid]) -> serde_json::Value {
    serde_json::json!({
        "name": "Grand Cinematic Festival",
        "description": "Annual film festival celebrating the arts",
        "rules": "1. Be respectful. 2. Submit original work.",
        "set_id": set_id,
        "start_date": Utc::now(),
        "end_date": Utc::now(),
        "panelists": panelists
    })
}

// ---------------------------------------------------------------------------
// Library fixtures
// ---------------------------------------------------------------------------

pub fn create_library_body(original_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "original_id": original_id,
        "episode_id": null,
        "visibility": true,
        "tagged_works": [],
        "pre_thought": "I'm excited to watch this!",
        "post_impression": "It was amazing!",
        "status": "WATCHING",
        "entry_type": "MOVIE"
    })
}

// ---------------------------------------------------------------------------
// Artist action fixtures
// ---------------------------------------------------------------------------

pub fn artist_action_body(artist_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "artist_id": artist_id
    })
}

pub fn update_profile_body() -> serde_json::Value {
    serde_json::json!({
        "tag_line": "updated tagline",
        "stage_name": "kapten og"
    })
}

// ---------------------------------------------------------------------------
// Admin fixtures — same credential shape as artist for simplicity
// ---------------------------------------------------------------------------

pub fn admin_register_body() -> serde_json::Value {
    serde_json::json!({
        "admin_name": "superadmin",
        "admin_password": "Admin@12345"
    })
}

pub fn admin_login_body() -> serde_json::Value {
    serde_json::json!({
        "admin_name": "superadmin",
        "admin_password": "Admin@12345"
    })
}

// ---------------------------------------------------------------------------
// Set fixtures — update/join
// ---------------------------------------------------------------------------

pub fn update_set_body() -> serde_json::Value {
    serde_json::json!({
        "name": "Updated Set Name",
        "statement": "New statement",
        "description": "New longer description",
        "profile_picture": null
    })
}

pub fn join_set_body(set_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "set_id": set_id
    })
}

// ---------------------------------------------------------------------------
// Work fixtures — update/like
// ---------------------------------------------------------------------------

pub fn update_work_body() -> serde_json::Value {
    serde_json::json!({
        "title": "Updated Title"
    })
}

pub fn entity_action_body(work_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "entity_id": work_id
    })
}

// ---------------------------------------------------------------------------
// Original fixtures — update/role management
// ---------------------------------------------------------------------------

pub fn update_original_body() -> serde_json::Value {
    serde_json::json!({
        "title": "They Call Him OG Redux",
        "description": "Updated description",
        "cover_image": null,
        "release_date": null
    })
}

pub fn add_role_body(profile_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "profile_id": profile_id,
        "role_name": "Cinematographer",
        "category": "MAKER"
    })
}

pub fn remove_role_body(profile_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "profile_id": profile_id,
        "role_name": "Cinematographer"
    })
}

// ---------------------------------------------------------------------------
// Festival fixtures — update/panelist update
// ---------------------------------------------------------------------------

pub fn update_festival_body() -> serde_json::Value {
    serde_json::json!({
        "name": "Updated Festival Name",
        "description": "Updated description",
        "rules": null,
        "start_date": null,
        "end_date": null
    })
}

pub fn add_panelist_body(artist_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "insert": true,
        "artist_id": artist_id
    })
}

pub fn remove_panelist_body(artist_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "insert": false,
        "artist_id": artist_id
    })
}

// ---------------------------------------------------------------------------
// Recommendation fixtures
// ---------------------------------------------------------------------------

pub fn create_recommendation_body(original_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "lines": "This is a great recommendation",
        "original_id": original_id,
        "score": 100
    })
}

pub fn update_recommendation_body() -> serde_json::Value {
    serde_json::json!({
        "lines": "Updated recommendation text",
        "score": 150
    })
}

pub fn update_recommendation_score_only() -> serde_json::Value {
    serde_json::json!({
        "score": 200
    })
}

pub fn update_recommendation_lines_only() -> serde_json::Value {
    serde_json::json!({
        "lines": "Updated lines only"
    })
}

// ---------------------------------------------------------------------------
// Wall post fixtures
// ---------------------------------------------------------------------------

pub fn create_wall_post_body(work_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "work_id": work_id,
        "text_line": "This is a quote from the work"
    })
}

pub fn create_wall_post_pin_body(work_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "work_id": work_id,
        "text_line": null
    })
}

pub fn create_wall_post_standalone_body() -> serde_json::Value {
    serde_json::json!({
        "work_id": null,
        "text_line": "Standalone wall post"
    })
}

pub fn create_wall_post_with_original(original_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "work_id": null,
        "text_line": "Quote with original",
        "original_id": original_id
    })
}

pub fn create_wall_post_with_recommendation(
    recommendation_id: Uuid,
) -> serde_json::Value {
    serde_json::json!({
        "work_id": null,
        "text_line": "Quote with recommendation",
        "recommendation_id": recommendation_id
    })
}

// ---------------------------------------------------------------------------
// Admin role management fixtures
// ---------------------------------------------------------------------------

pub fn create_role_body() -> serde_json::Value {
    serde_json::json!({
        "name": "moderator",
        "description": "Can moderate content"
    })
}

pub fn create_permission_body() -> serde_json::Value {
    serde_json::json!({
        "name": "delete_posts",
        "description": "Can delete any post"
    })
}

pub fn assign_permission_body() -> serde_json::Value {
    serde_json::json!({
        "role": "moderator",
        "permission": "delete_posts"
    })
}

pub fn revoke_permission_body() -> serde_json::Value {
    serde_json::json!({
        "role": "moderator",
        "permission": "delete_posts"
    })
}

pub fn update_profile_role_body(profile_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "profile_id": profile_id,
        "new_role": "moderator"
    })
}

// ---------------------------------------------------------------------------
// Library surge fixtures
// ---------------------------------------------------------------------------

pub fn create_library_body_with_surge(original_id: Uuid, surge_score: i64) -> serde_json::Value {
    serde_json::json!({
        "original_id": original_id,
        "episode_id": null,
        "visibility": true,
        "tagged_works": [],
        "pre_thought": "I'm excited to watch this!",
        "post_impression": "It was amazing!",
        "status": "WATCHING",
        "entry_type": "MOVIE",
        "surge_score": surge_score
    })
}

pub fn update_library_entry_with_surge() -> serde_json::Value {
    serde_json::json!({
        "pre_thought": "Updated pre-thought",
        "post_impression": "Updated post-impression",
        "status": "WATCHED",
        "surge_score": 200
    })
}

pub fn tag_work_to_library_body(work_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "work_id": work_id
    })
}
