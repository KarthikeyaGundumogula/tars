use serde::Deserialize;
use uuid::Uuid;

use crate::{domain::LedgerThought, types::db::ledger::{LedgerEntryType, WatchlistStatus}};

#[derive(Deserialize)]
pub struct LedgerEntryReq {
    pub original_id: Option<Uuid>,
    pub visibility: bool,
    pub tagged_works: Option<Vec<Uuid>>,
    pub pre_thought: Option<LedgerThought>,
    pub post_impression: Option<LedgerThought>,
    pub status: WatchlistStatus,
    pub entry_type: LedgerEntryType,
    pub episode_id: Option<Uuid>,
}
