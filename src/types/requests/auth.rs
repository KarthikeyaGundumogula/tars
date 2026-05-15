use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{Handle, Password, StageName, TagLine};

#[derive(Deserialize)]
pub struct ProfileSignupReq {
    pub handle: Handle,
    pub tag_line: TagLine,
    pub password: Password,
    pub profile_picture: String,
    pub youtube_profile: Option<String>,
    pub twitter_profile: Option<String>,
    pub instagram_profile: Option<String>,
    pub stage_name: StageName,
    pub text_color: String,
    pub background_color: String,
}

#[derive(Deserialize)]
pub struct ProfileLogin {
    pub handle: Handle,
    pub password: Password,
}

#[derive(Deserialize)]
pub struct ResetPasswordReq {
    pub old_password: Password,
    pub new_password: Password,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub profile_id: Uuid,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}
