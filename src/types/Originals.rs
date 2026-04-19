pub use uuid::Uuid;

pub struct Original{
  pub title: String,
  pub original_id: Uuid,
  pub description: String,
  pub cover_image: String,
  pub stats: OriginalStats
}

pub struct OriginalStats{
  pub presence: u64,
  pub releases: u64,
}