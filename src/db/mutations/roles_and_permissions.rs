use sqlx::PgPool;

use crate::{errors::ApiError, models::db::roles_and_permissions::{Permission, UserRole}};

pub async fn insert_new_user_role(pool: &PgPool, role: UserRole) -> Result<UserRole, ApiError> {
    Ok(sqlx::query_as!(
        UserRole,
        "INSERT INTO user_roles (name,description,created_at) VALUES ($1,$2,$3) RETURNING *",
        role.name,
        role.description,
        role.created_at
    )
    .fetch_one(pool)
    .await?)
}

pub async fn insert_new_permission(pool: &PgPool, permission: Permission) -> Result<Permission, ApiError> {
    Ok(sqlx::query_as!(
        Permission,
        "INSERT INTO permissions (name,description,created_at) VALUES ($1,$2,$3) RETURNING *",
        permission.name,
        permission.description,
        permission.created_at
    )
    .fetch_one(pool)
    .await?)
}

pub async fn insert_permission_for_role(pool: &PgPool, role_name: String, permission_name: String) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        "INSERT INTO role_permissions (role_name,permission_name,created_at) VALUES ($1,$2,NOW())",
        role_name,
        permission_name
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn delete_permission_for_role(pool: &PgPool, role_name: String, permission_name: String) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        "DELETE FROM role_permissions WHERE role_name = $1 AND permission_name = $2",
        role_name,
        permission_name
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}
