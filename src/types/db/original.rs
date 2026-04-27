pub use uuid::Uuid;

pub struct Original{
  pub id: Uuid,
  pub title: String,
  pub description: String,
  pub cover_image: String,
  pub presence: i64,
  pub releases: i64,
}
