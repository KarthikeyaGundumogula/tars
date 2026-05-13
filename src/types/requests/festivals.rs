use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateFestivalReq {
    pub name: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub set_id: Uuid,
    pub panelists: Vec<Uuid>,
}
