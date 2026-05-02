use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProfileSignupReq {
    pub user_name: String,
    pub tag_line: String,
    pub password: String,
    pub profile_picture: String,
    pub youtube_profile: Option<String>,
    pub twitter_profile: Option<String>,
    pub instagram_profile: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ProfileLogin {
    pub user_name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub:String,
    pub role: String,
    pub exp: usize,
}
