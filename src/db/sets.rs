use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::ApiError,
    types::{
        db::sets::{Set, SetRole},
        requests::sets::UpdateSetReq,
    },
};

pub async fn insert_new_set(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    set: Set,
) -> Result<Uuid, ApiError> {
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
    Ok(set_id)
}

pub async fn insert_new_set_member(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    profile_id: Uuid,
    set_id: Uuid,
    set_role: SetRole,
    created_at: chrono::DateTime<chrono::Utc>,
) -> Result<SetRole, ApiError> {
    Ok(sqlx::query_scalar!(
        r#"
        INSERT INTO set_members (profile_id, set_id, set_role, created_at)
        VALUES ($1, $2, $3, $4) RETURNING set_role as "set_role:SetRole";
        "#,
        profile_id,
        set_id,
        set_role as SetRole,
        created_at
    )
    .fetch_one(&mut **txn)
    .await?)
}

pub async fn update_set(pool: &PgPool, set: UpdateSetReq, id: Uuid) -> Result<Uuid, ApiError> {
    Ok(
 sqlx::query_scalar!(
        "
        UPDATE sets
        SET name = COALESCE($1, name), statement = COALESCE($2, statement), description = COALESCE($3, description), profile_picture = COALESCE($4, profile_picture)
        WHERE id = $5 RETURNING id;
        ",
        set.name.as_ref().map(|n| n.as_str()),
        set.statement.as_ref().map(|s| s.as_str()),
        set.description.as_ref().map(|d| d.as_str()),
        set.profile_picture.as_ref().map(|p| p.as_str()),
        id
    )
    .fetch_one(pool)
    .await?)
}
