use uuid::Uuid;

use crate::{errors::ApiError, types::db::festivals::{Festival, Panelist}};

pub async fn insert_new_festival(txn: &mut sqlx::Transaction<'_, sqlx::Postgres>, festival: Festival) -> Result<Uuid, ApiError> {
    Ok(
    sqlx::query_scalar!(
      "INSERT INTO festivals (id, name, description, set_id, organizer, start_date, end_date, rules, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id",
      festival.id,
      festival.name,
      festival.description,
      festival.set_id,
      festival.organizer,
      festival.start_date,
      festival.end_date,
      festival.rules,
      festival.created_at
    )
    .fetch_one(&mut **txn)
    .await?
  )
}

pub async fn insert_new_panelist(txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,panelist: Panelist) -> Result<Uuid, ApiError> {
    Ok(
    sqlx::query_scalar!(
      "INSERT INTO panelists (festival_id, profile_id, work_id, created_at) VALUES ($1, $2, $3, $4) RETURNING festival_id",
      panelist.festival_id,
      panelist.profile_id,
      panelist.work_id,
      panelist.created_at
    )
    .fetch_one(&mut **txn)
    .await?
  )
}