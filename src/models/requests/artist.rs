use serde::Deserialize;
use uuid::Uuid;

use crate::domain::StageName;

#[derive(Deserialize, Debug)]
pub struct UpdateProfileReq {
    pub tag_line: Option<String>,
    pub profile_picture: Option<String>,
    pub youtube_profile: Option<String>,
    pub twitter_profile: Option<String>,
    pub instagram_profile: Option<String>,
    pub stage_name: Option<StageName>,
    pub color_theme: Option<String>,
}

#[derive(Deserialize)]
pub struct FavoriteActionReq {
    pub artist_id: Uuid,
}
