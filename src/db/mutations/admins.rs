use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::admin::Admin;

pub async fn insert_new_admin(pool: &PgPool, admin: Admin) -> Result<Uuid, sqlx::Error> {
    Ok(
        sqlx::query!("INSERT INTO admins (admin_id,admin_name, admin_password_hash,created_at) VALUES ($1, $2,$3,$4) RETURNING admin_id", admin.admin_id,admin.admin_name,admin.admin_password_hash,admin.created_at)
            .fetch_one(pool)
            .await?
            .admin_id
    )
}

pub async fn get_admin_auth_details(
    pool: &PgPool,
    admin_name: &str,
) -> Result<Option<Admin>, sqlx::Error> {
    sqlx::query_as!(
        Admin,
        "SELECT admin_id, admin_name, admin_password_hash, created_at FROM admins WHERE admin_name = $1",
        admin_name
    )
    .fetch_optional(pool)
    .await
}
