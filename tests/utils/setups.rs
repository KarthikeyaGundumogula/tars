use reqwest::{Client, Response};
use uuid::Uuid;

use crate::utils::spawn_app::{self, TestApp};

pub async fn setup_original_registration() -> (Vec<Uuid>,TestApp) {
    let mut artists = Vec::new();
    let app = spawn_app::spawn().await;
    let client = Client::new();
    for i in 0..4 {
        let user_name = Uuid::new_v4().to_string();
        let body = serde_json::json!({
          "user_name":user_name,
          "tag_line":"I will never care for you",
          "password":"kapten@1023",
          "profile_picture":"aofdjosfjosf",
          "youtube_profile":"aojojfosjf"
        });
        let response: Response = client
            .post(&format!("{}/artist/register", app.address))
            .json(&body)
            .send()
            .await
            .expect("failed to execute request");
        assert!(response.status().is_success());
        let artist =
            sqlx::query_scalar!(r#"SELECT id FROM profiles WHERE user_name=$1"#, user_name)
                .fetch_one(&app.db_pool)
                .await
                .expect("db query failed");
        println!("artist created {}",i);
        artists.push(artist);
    }
    (artists,app)
}
