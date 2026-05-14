use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::utils::auth::extractor::Resource;

#[derive(Serialize, Debug)]
pub enum SetRole {
    CURATOR,
    MEMBER,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Set {
    pub id: Uuid,
    pub name: String,
    pub statement: String,
    pub description: String,
    pub profile_picture: Option<String>, // remove this
    pub presence: i64,
    pub curator: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct SetMember {
    pub set_id: Uuid,
    pub profile_id: Uuid,
    pub set_role: SetRole,
    pub created_at: DateTime<Utc>,
}

impl Resource for Set {
    async fn fetch_by_id(
        db: &sqlx::PgPool,
        resource_id: Uuid,
    ) -> Result<Option<(Uuid, Self)>, crate::errors::ApiError>
    where
        Self: Send,
    {
        let set = sqlx::query_as!(
            Set,
            r#"SELECT id, name, statement, description, profile_picture, presence, curator, created_at FROM sets WHERE id = $1"#,
            resource_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(crate::errors::ApiError::NotFound)?;
        Ok(Some((set.curator, set)))
    }
}