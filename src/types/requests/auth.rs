use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProfileSignup {
    pub user_name: String,
    pub tag_line: String,
    pub password: String,
    pub profile_picture: String,
    pub youtube_profile: Option<String>,
    pub twitter_profile: Option<String>,
    pub instagram_profile: Option<String>,
}

#[derive(Serialize,Deserialize)]
pub struct ProfileLogin {
    user_name: String,
    password: String
}