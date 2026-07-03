use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    domain::{Genre, OriginalDescription, OriginalTitle, Password, Role},
    models::db::profile::RoleType,
};

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

#[derive(Debug, Deserialize)]
pub struct UpdateOrignalReq {
    pub title: Option<OriginalTitle>,
    pub description: Option<OriginalDescription>,
    pub cover_image: Option<String>,
    pub release_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct AddNewRoleReq {
    pub profile_id: Uuid,
    pub role_name: Role,
    pub category: RoleType,
}

#[derive(Debug, Deserialize)]
pub struct RemoveRoleReq {
    pub profile_id: Uuid,
    pub role_name: Role,
}
