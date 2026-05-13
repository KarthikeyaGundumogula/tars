use chrono::Utc;
use uuid::Uuid;

use crate::common::spawn_app::{self, TestApp};

pub async fn setup_original_registration() -> (Vec<Uuid>, TestApp) {
    let mut artists = Vec::new();
    let app = spawn_app::spawn().await;
    for i in 0..4 {
        let user_name = format!("user_{}", i);
        println!("Creating artist {}", user_name);
        let body = serde_json::json!({
            "handle": user_name,
            "tag_line": "I dont give a dmn about your opinion",
            "password": "kApten@1023",
            "profile_picture": "aofdjosfjosf",
            "youtube_profile": "aojojfosjf"
        });
        
        let response = app.post_register(&body).await;
        assert!(response.status().is_success());
        
        let artist =
            sqlx::query_scalar!(r#"SELECT id FROM profiles WHERE user_name=$1"#, user_name)
                .fetch_one(&app.state.db_pool)
                .await
                .expect("db query failed");
        artists.push(artist);
    }
    (artists, app)
}

pub async fn setup_edit_upload() -> (Vec<Uuid>, TestApp, Uuid) {
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
    
    let response = app.post_original(&body).await;
    assert!(response.status().is_success());
    
    let original_id = sqlx::query_scalar!(
        r#"SELECT id FROM originals WHERE title=$1"#,
        "They Call him Og"
    )
    .fetch_one(&app.state.db_pool)
    .await
    .expect("db query failed");
    
    (artists, app, original_id)
}
