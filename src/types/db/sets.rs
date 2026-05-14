use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

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
    pub profile_picture: String,
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