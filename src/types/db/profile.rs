use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::ApiError, services::auth_service::extractor::Resource};

#[derive(Clone, Serialize, sqlx::Type)]
#[sqlx(type_name = "profile_type")]
pub enum ProfileType {
    ARTIST,
    STAR,
    MAKER,
}

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "role_type")]
pub enum RoleType {
    STAR,
    MAKER,
}

#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct Profile {
    pub id: Uuid,
    pub user_name: String,
    pub stage_name: String,
    pub tag_line: String,
    pub is_claimed: bool,
    pub presence: i64,
    pub profile_picture: String,
    pub youtube_profile: Option<String>,
    pub twitter_profile: Option<String>,
    pub instagram_profile: Option<String>,
    pub password_hash: String,
    pub profile_type: ProfileType,
    pub background_color: String,
    pub text_color: String,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct Role {
    pub profile_id: Uuid,
    pub original_id: Uuid,
    pub role_name: String,
    pub category: RoleType,
    pub created_at: DateTime<Utc>,
}

impl Resource for Profile {
    async fn fetch_by_id(
        db: &sqlx::PgPool,
        resource_id: Uuid,
    ) -> Result<Option<(Uuid, Self)>, ApiError>
    where
        Self: Send,
    {
        let profile = sqlx::query_as!(
            Profile,
            r#"SELECT id, user_name, stage_name, tag_line, presence, profile_picture, youtube_profile, twitter_profile, instagram_profile, password_hash, is_claimed, profile_type as "profile_type:ProfileType", background_color, text_color, created_at FROM profiles WHERE id = $1"#,
            resource_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(ApiError::NotFound)?;
        Ok(Some((profile.id, profile)))
    }
}
