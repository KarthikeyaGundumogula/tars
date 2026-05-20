#![allow(non_camel_case_types)]
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{errors::ApiError, services::auth_service::extractor::Resource};

#[derive(Deserialize, sqlx::Type, Debug)]
#[sqlx(type_name = "watchlist_status")]
pub enum WatchlistStatus {
    WATCHED,
    WATCHING,
    WANT_TO_WATCH,
}
#[derive(Deserialize, sqlx::Type, Debug)]
#[sqlx(type_name = "ledger_entry_type")]
pub enum LedgerEntryType {
    MOVIE,
    SERIES,
    EPISODE,
    SEASON,
}

#[derive(sqlx::FromRow)]
pub struct LedgerEntry {
    pub id: Uuid,
    pub original_id: Option<Uuid>,
    pub episode_id: Option<Uuid>,
    pub profile_id: Uuid,
    pub pub_visibility: bool,
    pub tagged_works: Option<Vec<Uuid>>,
    pub pre_thought: Option<String>,
    pub post_impression: Option<String>,
    pub status: Option<WatchlistStatus>, // it should not be null
    pub entry_type: LedgerEntryType,
    pub created_at: Option<DateTime<Utc>>, // it should not be null
    pub updated_at: Option<DateTime<Utc>>, // it should not be null
}

impl Resource for LedgerEntry {
    async fn fetch_by_id(
        db: &sqlx::PgPool,
        resource_id: Uuid,
    ) -> Result<Option<(Uuid, Self)>, ApiError>
    where
        Self: Send,
    {
        let ledger_entry = sqlx::query_as!(
            LedgerEntry,
            r#"SELECT id, original_id, episode_id, profile_id, pub_visibility, tagged_works, pre_thought, post_impression, status as "status:WatchlistStatus", entry_type as "entry_type:LedgerEntryType", created_at, updated_at FROM ledger WHERE id = $1"#,
            resource_id
        )
        .fetch_optional(db)
        .await?.ok_or(crate::errors::ApiError::NotFound)?;
        Ok(Some((ledger_entry.profile_id, ledger_entry)))
    }
}
