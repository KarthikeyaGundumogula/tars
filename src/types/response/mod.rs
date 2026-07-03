// Entity-specific response modules
pub mod admin;
pub mod artist;
pub mod festival;
pub mod library;
pub mod original;
pub mod profile;
pub mod set;
pub mod work;

// Re-export entity-specific response types for convenience
pub use admin::AdminResponse;
pub use artist::ArtistResponse;
pub use festival::FestivalResponse;
pub use library::LibraryResponse;
pub use original::OriginalResponse;
pub use profile::ProfileResponse;
pub use set::SetResponse;
pub use work::WorkResponse;
