use uuid::Uuid;

use crate::{errors::ApiError, types::db::sets::Set};

pub async fn insert_new_set(txn: &mut sqlx::Transaction<'_, sqlx::Postgres>, set: Set) -> Result<Uuid, ApiError> {
    let set_id = sqlx::query_scalar!(
        "
        INSERT INTO sets (id, name, statement, description, profile_picture, presence, curator, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id;
        ",
        set.id,
        set.name,
        set.statement,
        set.description,
        set.profile_picture,
        set.presence,
        set.curator,
        set.created_at
    )
    .fetch_one(&mut **txn)
    .await?;

    sqlx::query!(
        "
        INSERT INTO set_members (profile_id, set_id, set_role, created_at)
        VALUES ($1, $2, 'CURATOR', $3);
        ",
        set.curator,
        set.id,
        set.created_at
    )
    .execute(&mut **txn)
    .await?;

    Ok(set_id)
}