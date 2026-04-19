pub use uuid::Uuid;

pub struct Original{
  pub title: String,
  pub original_id: Uuid,
  pub description: String,
  pub cover_image: String,
  pub stats: OriginalStats,
  pub stars: Vec<Artist>,
  pub makers: Vec<Artist>
}

pub struct OriginalStats{
  pub presence: u64,
  pub releases: u64,
}

pub struct Artist{
  artist_id:Uuid,
  role:String,
}
