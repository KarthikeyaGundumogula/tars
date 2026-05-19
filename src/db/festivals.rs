use sqlx::{PgPool, Transaction};
use uuid::Uuid;

use crate::{
    errors::ApiError,
    types::{
        db::festivals::{Festival, Panelist},
        requests::festivals::UpdateFestivalReq,
    },
};

pub async fn insert_new_festival(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    festival: Festival,
) -> Result<Uuid, ApiError> {
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

pub async fn update_festival_details(
    pool: &PgPool,
    id: Uuid,
    festival: UpdateFestivalReq,
) -> Result<Option<Uuid>, ApiError> {
    Ok(
    sqlx::query_scalar!(
      "UPDATE festivals SET name = COALESCE($2,name), description = COALESCE($3,description), start_date = COALESCE($4,start_date), end_date = COALESCE($5,end_date), rules = COALESCE($6,rules) WHERE id = $1 RETURNING id",
      id,
      festival.name.as_ref().map(|n| n.as_str()),
      festival.description.as_ref().map(|d| d.as_str()),
      festival.start_date,
      festival.end_date,
      festival.rules.as_ref().map(|r| r.as_str())
    )
    .fetch_optional(pool)
    .await?
  )
}

pub async fn insert_new_panelist(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    panelist: Panelist,
) -> Result<Uuid, ApiError> {
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

pub async fn delete_panelist(
    pool: &PgPool,
    festival_id: Uuid,
    profile_id: Uuid,
) -> Result<Option<Uuid>, ApiError> {
    Ok(sqlx::query_scalar!(
        "DELETE FROM panelists WHERE festival_id = $1 AND profile_id = $2 RETURNING festival_id",
        festival_id,
        profile_id
    )
    .fetch_optional(pool)
    .await?)
}

pub async fn update_panelist_work(
    txn: &mut Transaction<'_, sqlx::Postgres>,
    festival_id: Uuid,
    profile_id: Uuid,
    work_id: Uuid,
) -> Result<Option<Uuid>, ApiError> {
    Ok(
    sqlx::query_scalar!(
      "UPDATE panelists SET work_id = $3 WHERE festival_id = $1 AND profile_id = $2 RETURNING festival_id",
      festival_id,
      profile_id,
      work_id
    )
    .fetch_optional(&mut **txn)
    .await?
  )
}

pub async fn insert_new_festival_work(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    festival_id: Uuid,
    work_id: Uuid,
) -> Result<Option<Uuid>, ApiError> {
    Ok(
    sqlx::query_scalar!(
      "INSERT INTO festival_works (festival_id, work_id, created_at) VALUES ($1, $2, $3) RETURNING festival_id",
      festival_id,
      work_id,
      chrono::Utc::now()
    )
    .fetch_optional(&mut **txn)
    .await?
  )
}
