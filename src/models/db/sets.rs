use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::ApiError,
    services::auth_service::extractor::{Entity, Resource},
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
    pub color_theme: String,
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

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct DiscussionPost {
    pub id: Uuid,
    pub set_id: Option<Uuid>, // TODO add the not null constraint
    pub author_id: Option<Uuid>,
    pub title: String,
    pub content: String,
    pub total_reactions: i64, // TODO will be removed
    pub work_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct DiscussionComment {
    pub id: Uuid,
    pub discussion_post_id: Uuid,
    pub author_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
            r#"SELECT id, name, statement, description, color_theme, presence, curator, created_at FROM sets WHERE id = $1"#,
            resource_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(crate::errors::ApiError::NotFound)?;
        Ok(Some((set.curator, set)))
    }
}

impl Resource for DiscussionComment {
    async fn fetch_by_id(
        db: &sqlx::PgPool,
        resource_id: Uuid,
    ) -> Result<Option<(Uuid, Self)>, ApiError>
    where
        Self: Send,
    {
        let discussion_comment = sqlx::query_as!(
            DiscussionComment,
            r#"SELECT id, discussion_post_id, author_id, parent_id, content, created_at, updated_at FROM discussion_comments WHERE id = $1"#,
            resource_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(crate::errors::ApiError::NotFound)?;
        match discussion_comment.author_id {
            Some(id) => Ok(Some((id, discussion_comment))),
            None => Err(ApiError::BadRequest("post not found".to_string())),
        }
    }
}

impl Resource for DiscussionPost {
    async fn fetch_by_id(
        db: &sqlx::PgPool,
        resource_id: Uuid,
    ) -> Result<Option<(Uuid, Self)>, ApiError>
    where
        Self: Send,
    {
        let discussion_post = sqlx::query_as!(
            DiscussionPost,
            r#"SELECT id, set_id, author_id, title, content, total_reactions, work_id, created_at, updated_at, last_active FROM discussion_post WHERE id = $1"#,
            resource_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(crate::errors::ApiError::NotFound)?;
        match discussion_post.author_id {
            Some(id) => Ok(Some((id, discussion_post))),
            None => Err(ApiError::BadRequest("post not found".to_string())),
        }
    }
}

#[derive(Debug)]
pub struct FestivalMember(pub SetMember);

impl Entity for FestivalMember {
    async fn fetch_membership_and_entity(
        db: &sqlx::PgPool,
        entity_id: Uuid,
        member_id: Uuid,
    ) -> Result<Option<(bool, Self)>, ApiError>
    where
        Self: Send,
    {
        let set_member = sqlx::query_as!(
            SetMember,
            r#"SELECT sm.set_id, sm.profile_id, sm.set_role as "set_role: SetRole", sm.created_at FROM set_members AS sm INNER JOIN festivals ON sm.set_id = festivals.set_id WHERE festivals.id = $1 AND sm.profile_id = $2"#,
            entity_id,
            member_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(ApiError::NotFound)?;
        Ok(Some((true, FestivalMember(set_member))))
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
        let set_member = sqlx::query_as!(
            SetMember,
            r#"SELECT set_id, profile_id, set_role as "set_role: SetRole", created_at FROM set_members WHERE set_id = $1 AND profile_id = $2"#,
            entity_id,
            member_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(ApiError::NotFound)?;
        Ok(Some((true, set_member)))
    }
}
