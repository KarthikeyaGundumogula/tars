use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize,sqlx::Type,Debug)]
#[sqlx(type_name = "watchlist_status")]
pub enum WatchlistStatus {
    WATCHED,
    WATCHING,
    WANT_TO_WATCH,
}
#[derive(Deserialize,sqlx::Type,Debug)]
#[sqlx(type_name = "ledger_entry_type")]
pub enum LedgerEntryType {
    MOVIE,
    SERIES,
    EPISODE,
    SEASON,
}

#[derive(sqlx::FromRow)]
pub struct LedgerEntry {
    pub id: Uuid,
    pub original_id: Option<Uuid>,
    pub episode_id: Option<Uuid>,
    pub profile_id: Uuid,
    pub visibility: bool,
    pub tagged_works: Option<Vec<Uuid>>,
    pub pre_thought: Option<String>,
    pub post_impression: Option<String>,
    pub status: WatchlistStatus, // it should not be null
    pub entry_type: LedgerEntryType,
}