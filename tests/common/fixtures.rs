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
        "background_color": "#FF0000",
        "text_color": "#000000"
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
        ]
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
        "originals": [original_id]
    })
}

pub fn create_poster_body(original_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "title": "The Golden Poster",
        "src_id": "poster_uuid_123",
        "format": "STANDARD",
        "originals": [original_id]
    })
}

pub fn create_script_body(original_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "title": "Cinematic Script Draft",
        "src_ids": ["img1", "img2"],
        "originals": [original_id],
        "thoughts": ["Brilliant intro", "Dynamic pacing"]
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
        "profile_picture": "no_profile_picture"
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
// Ledger fixtures
// ---------------------------------------------------------------------------

pub fn create_ledger_body(original_id: Uuid) -> serde_json::Value {
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
