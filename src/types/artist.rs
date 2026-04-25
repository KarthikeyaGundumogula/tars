use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

#[derive(sqlx::FromRow,Serialize,Clone)]
pub struct Artist {
    pub id: Uuid,
    pub user_name: String,
    pub tag_line: String,
    pub profile_picture:String,
    pub youtube_profile: Option<String>,
    pub twitter_profile: Option<String>,
    pub instagram_profile: Option<String>,
    pub password_hash: String,
    pub is_claimed: Option<bool>, // TODO - remove the optional after migrating
    pub created_at: chrono::DateTime<Utc>,
}
