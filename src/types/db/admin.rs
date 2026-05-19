use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Admin {
    pub admin_name: String,
    pub admin_password_hash: String,
    pub admin_id: Uuid,
    pub created_at: DateTime<Utc>,
}
