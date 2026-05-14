use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{errors::ApiError, utils::auth::extractor::Resource};

#[derive(sqlx::FromRow, Deserialize, Debug)]
pub struct Festival {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub set_id: Uuid,
    pub organizer: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub rules: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Deserialize, Debug)]
pub struct Panelist {
    pub festival_id: Uuid,
    pub profile_id: Uuid,
    pub work_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl Resource for Festival {
    async fn fetch_by_id(
        db: &sqlx::PgPool,
        resource_id: Uuid,
    ) -> Result<Option<(Uuid, Self)>, ApiError>
    where
        Self: Send,
    {
        let festival = sqlx::query_as!(
            Festival,
            "SELECT * FROM festivals WHERE id = $1",
            resource_id
        )
        .fetch_optional(db)
        .await?.ok_or(ApiError::NotFound)?;
        Ok(Some((festival.organizer, festival)))
    }
}
