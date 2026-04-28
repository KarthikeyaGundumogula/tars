use chrono::{DateTime, Utc};
pub use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct Original{
  pub id: Uuid,
  pub title: String,
  pub description: String,
  pub cover_image: String,
  pub presence: String,
  pub password_hash: String,
  pub created_at: DateTime<Utc>
}
