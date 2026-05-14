use chrono::{DateTime, Utc};
use serde::{Deserialize};
use uuid::Uuid;

use crate::domain::{Genre, OriginalDescription, OriginalTitle, Password, Role};

#[derive(Debug, Deserialize)]
pub struct Crew {
    pub artist: Uuid,
    pub role: Role,
}

#[derive(Debug, Deserialize)]
pub struct CreateOriginalReq {
    pub title: OriginalTitle,
    pub description: OriginalDescription,
    pub cover_img: String,
    pub password: Password,
    pub associated_with: Uuid,
    pub release_date: Option<DateTime<Utc>>,
    pub genres: Vec<Genre>,
    pub stars: Vec<Crew>,
    pub makers: Vec<Crew>,
}
