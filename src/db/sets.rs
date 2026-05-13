use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::ApiError, types::db::sets::Set};

pub async fn insert_new_set(pool:&PgPool,set:Set) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        "
        INSERT INTO sets (name, statement, description, presence, curator, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id;
        ",
        set.name,
        set.statement,
        set.description,
        set.presence,
        set.curator,
        set.created_at
    )
    .fetch_one(pool)
    .await?)
}