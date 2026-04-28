use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize, sqlx::Type)]
#[sqlx(type_name = "profile_type")]
pub enum ProfileType {
    ARTIST,
    STAR,
    MAKER,
}

#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct Profile {
    pub id: Uuid,
    pub user_name: String,
    pub tag_line: String,
    pub presence: i64,
    pub profile_picture: String,
    pub youtube_profile: Option<String>,
    pub twitter_profile: Option<String>,
    pub instagram_profile: Option<String>,
    pub password_hash: String,
    pub is_claimed: bool,
    pub profile_type: ProfileType,
    pub created_at: chrono::DateTime<Utc>,
}
