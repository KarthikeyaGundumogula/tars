use serde::{Deserialize, Serialize};

use crate::domain::{Handle, Password, TagLine};

#[derive(Deserialize)]
pub struct ProfileSignupReq {
    pub user_name: Handle,
    pub tag_line: TagLine,
    pub password: Password,
    pub profile_picture: String,
    pub youtube_profile: Option<String>,
    pub twitter_profile: Option<String>,
    pub instagram_profile: Option<String>,
}

#[derive(Deserialize)]
pub struct ProfileLogin {
    pub user_name: Handle,
    pub password: Password,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}
