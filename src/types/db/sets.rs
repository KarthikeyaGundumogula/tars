use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::ApiError,
    shared::auth::extractor::{Entity, Resource},
};

#[derive(Serialize, Debug, Deserialize, sqlx::Type)]
#[sqlx(type_name = "set_role")]
pub enum SetRole {
    CURATOR,
    MEMBER,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
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

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
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
    ) -> Result<Option<(Uuid, Self)>, ApiError>
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

impl Entity for SetMember {
    async fn fetch_membership_and_entity(
        db: &sqlx::PgPool,
        entity_id: Uuid,
        member_id: Uuid,
    ) -> Result<Option<(bool, Self)>, ApiError>
    where
        Self: Send,
    {
        let set_id = sqlx::query_scalar!(
            "SELECT set_id from festivals WHERE id = $1",
            entity_id
        )
        .fetch_one(db)
        .await?;
        let set_member = sqlx::query_as!(
            SetMember,
            r#"SELECT set_id, profile_id, set_role as "set_role: SetRole", created_at FROM set_members WHERE set_id = $1 AND profile_id = $2"#,
            set_id,
            member_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(ApiError::NotFound)?;
        Ok(Some((true, set_member)))
    }
}
