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
