use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::ApiError,
    types::db::ledger::{LedgerEntry, LedgerEntryType, WatchlistStatus},
};

pub async fn insert_new_ledger_entry(pool: &PgPool, data: LedgerEntry) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        r#"
      INSERT INTO ledger (
          original_id,
          profile_id,
          pub_visibility,
          tagged_works,
          pre_thought,
          post_impression,
          status,
          entry_type,
          episode_id,
          id
        )
      VALUES (
          $1,
          $2,
          $3,
          $4,
          $5,
          $6,
          $7,
          $8,
          $9,
          $10
        ) RETURNING id;
      "#,
        data.original_id,
        data.profile_id,
        data.visibility,
        data.tagged_works.as_deref(),
        data.pre_thought,
        data.post_impression,
        data.status as WatchlistStatus,
        data.entry_type as LedgerEntryType,
        data.episode_id,
        data.id
    )
    .fetch_one(pool)
    .await?)
}
