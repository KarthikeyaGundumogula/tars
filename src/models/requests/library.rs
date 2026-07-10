use serde::Deserialize;
use uuid::Uuid;

use crate::{
    domain::LibraryThought,
    models::db::library::{LibraryEntryType, WatchlistStatus},
};

#[derive(Deserialize)]
pub struct LibraryEntryReq {
    pub original_id: Option<Uuid>,
    pub visibility: bool,
    pub tagged_works: Option<Vec<Uuid>>,
    pub pre_thought: Option<LibraryThought>,
    pub post_impression: Option<LibraryThought>,
    pub status: WatchlistStatus,
    pub entry_type: LibraryEntryType,
    pub episode_id: Option<Uuid>,
    pub surge_score:Option<i64>,
}

#[derive(Deserialize)]
pub struct UpdateLibraryEntryReq {
    pub pre_thought: Option<LibraryThought>,
    pub post_impression: Option<LibraryThought>,
    pub status: Option<WatchlistStatus>,
    pub surge_score: Option<i64>,
}

#[derive(Deserialize)]
pub struct NewRecommendationReq {
    pub lines: String,
    pub original_id: Uuid,
    pub score: i64,
}

#[derive(Deserialize)]
pub struct UpdateRecommendationReq {
    pub lines: Option<String>,
    pub score: Option<i64>
}

#[derive(Deserialize)]
pub struct TagWorkToLibraryEntryReq {
    pub work_id: Uuid,
}
