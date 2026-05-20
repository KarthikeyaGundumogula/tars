use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::ApiError,
    types::{
        db::{
            original::Original,
            profile::{Role, RoleType},
        },
        requests::originals::UpdateOrignalReq,
    },
};

pub async fn insert_new_original(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: Original,
) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        r#"
        INSERT INTO originals (
            id,
            title,
            description,
            cover_img,
            presence,
            created_at,
            password_hash,
            associated_with,
            release_date,
            genres
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id;
        "#,
        data.id,
        data.title,
        data.description,
        data.cover_img,
        data.presence,
        data.created_at,
        data.password_hash,
        data.associated_with,
        data.release_date,
        data.genres.as_deref()
    )
    .fetch_one(&mut **txn)
    .await?)
}

pub async fn insert_new_role(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: Role,
) -> Result<String, ApiError> {
    Ok(sqlx::query_scalar!(
        r#"
    INSERT INTO cast_and_crew_roles (
        profile_id,
        original_id,
        category,
        role_name,
        created_at
    )
    VALUES ($1, $2, $3, $4, $5)
    RETURNING role_name;
    "#,
        data.profile_id,
        data.original_id,
        data.category as RoleType,
        data.role_name,
        data.created_at
    )
    .fetch_one(&mut **txn)
    .await?)
}

pub async fn update_original(
    pool: &sqlx::PgPool,
    data: UpdateOrignalReq,
    id: Uuid,
) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        "
            UPDATE originals
            SET title = COALESCE($1, title),
                description = COALESCE($2, description),
                cover_img = COALESCE($3, cover_img),
                release_date = COALESCE($4, release_date)
            WHERE id = $5
            RETURNING id;
            ",
        data.title.as_ref().map(|t| t.as_str()),
        data.description.as_ref().map(|d| d.as_str()),
        data.cover_image,
        data.release_date,
        id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn delete_original(pool: &sqlx::PgPool, id: Uuid) -> Result<bool, ApiError> {
    Ok(sqlx::query!("DELETE FROM originals WHERE id = $1", id)
        .execute(pool)
        .await?
        .rows_affected()
        == 1)
}

pub async fn add_new_role_if_not_exists(pool: &PgPool, data: Role) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        "
            INSERT INTO cast_and_crew_roles (profile_id, original_id, category, role_name, created_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (profile_id, original_id, role_name) DO NOTHING;
            ",
        data.profile_id,
        data.original_id,
        data.category as RoleType,
        data.role_name,
        data.created_at
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn delete_role(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    role_name: String,
    original_id: Uuid,
    profile_id: Uuid,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        "
            DELETE FROM cast_and_crew_roles
            WHERE role_name = $1 AND original_id = $2 AND profile_id = $3;
            ",
        role_name,
        original_id,
        profile_id
    )
    .execute(&mut **txn)
    .await?
    .rows_affected()
        == 1)
}
