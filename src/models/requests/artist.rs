use serde::Deserialize;
use uuid::Uuid;

use crate::domain::{Emoji, HexColor, SocialProfile, StageName, TagLine};

#[derive(Deserialize, Debug)]
pub struct UpdateProfileReq {
    pub tag_line: Option<TagLine>,
    pub profile_picture: Option<String>,
    pub youtube_profile: Option<SocialProfile>,
    pub twitter_profile: Option<SocialProfile>,
    pub instagram_profile: Option<SocialProfile>,
    pub stage_name: Option<StageName>,
    pub color_theme: Option<HexColor>,
}

#[derive(Deserialize)]
pub struct FavoriteActionReq {
    pub artist_id: Uuid,
}

#[derive(Deserialize)]
pub struct ReactionReq {
    pub wall_post_id: Uuid,
    pub reaction: Emoji,
}
