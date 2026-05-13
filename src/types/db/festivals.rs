use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(sqlx::FromRow,Deserialize,Debug)]
pub struct Festival {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub set_id: Uuid,
    pub organizer: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}

#[derive(sqlx::FromRow, Deserialize, Debug)]
pub struct Panelist {
    pub festival_id: Uuid,
    pub profile_id: Uuid,
    pub work_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}