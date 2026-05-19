use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::{FestivalDescription, FestivalName, FestivalRules};

#[derive(Deserialize)]
pub struct CreateFestivalReq {
    pub name: FestivalName,
    pub description: FestivalDescription,
    pub rules: Option<FestivalRules>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub set_id: Uuid,
    pub panelists: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct UpdateFestivalReq {
    pub name: Option<FestivalName>,
    pub description: Option<FestivalDescription>,
    pub rules: Option<FestivalRules>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct UpdateFestivalPanlist {
    pub insert: bool,
    pub artist_id: Uuid,
}
