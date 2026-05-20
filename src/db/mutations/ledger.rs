use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::ApiError,
    types::{
        db::ledger::{LedgerEntry, LedgerEntryType, WatchlistStatus},
        requests::ledger::{TagWorkToLedgerEntryReq, UpdateLedgerEntryReq},
    },
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
        data.pub_visibility,
        data.tagged_works.as_deref(),
        data.pre_thought,
        data.post_impression,
        data.status as Option<WatchlistStatus>,
        data.entry_type as LedgerEntryType,
        data.episode_id,
        data.id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn update_ledger_entry(
    pool: &PgPool,
    data: UpdateLedgerEntryReq,
    id: Uuid,
) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        r#"
      UPDATE ledger
      SET
          pre_thought = COALESCE($1, pre_thought),
          post_impression = COALESCE($2, post_impression),
          status = COALESCE($3,status),
          updated_at = NOW()
      WHERE id = $4
      RETURNING id;
      "#,
        data.pre_thought.as_ref().map(|t| t.to_string()),
        data.post_impression.as_ref().map(|t| t.to_string()),
        data.status as Option<WatchlistStatus>,
        id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn add_new_tagged_work(
    pool: &PgPool,
    data: TagWorkToLedgerEntryReq,
    entry_id: Uuid,
) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        "
      Update ledger
      SET tagged_works = array_append(tagged_works, $1)
      WHERE id = $2
      RETURNING id;
      ",
        data.work_id,
        entry_id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn delete_ledger_entry(pool: &PgPool, entry_id: Uuid) -> Result<(), ApiError> {
    sqlx::query!(
        "
      DELETE FROM ledger
      WHERE id = $1
      ",
        entry_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
