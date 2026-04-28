use chrono::{DateTime, Utc};
pub use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct Original{
  pub id: Uuid,
  pub title: String,
  pub description: String,
  pub cover_img: String,
  pub presence: i64,
  pub password_hash: String,
  pub associated_with: Uuid,
  pub release_date: DateTime<Utc>,
  pub generes: Vec<String>,
  pub created_at: DateTime<Utc>
}
