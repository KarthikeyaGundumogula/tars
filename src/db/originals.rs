use uuid::Uuid;

use crate::{
    errors::ApiError,
    types::db::{
        original::Original,
        profile::{Role, RoleType},
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
    INSERT INTO roles (
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
