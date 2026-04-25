use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProfileSignup {
    user_name: String,
    password: String,
    profile_pic_id: String,
    youtube_url: Option<String>,
    twitter_url: Option<String>,
    instagram_url: Option<String>,
}

#[derive(Serialize,Deserialize)]
pub struct ProfileLogin {
    user_name: String,
    password: String
}