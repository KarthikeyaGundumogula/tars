use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Crew {
    pub artist: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOriginalReq {
    pub title: String,
    pub description: String,
    pub password: String,
    pub associated_with:String,
    pub release_date: DateTime<Utc>,
    pub genere: Vec<String>,
    pub stars: Vec<Crew>,
    pub makers: Vec<Crew>,
}
