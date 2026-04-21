use serde::Deserialize;

#[derive(Deserialize)]
pub enum SocialPlatforms {
    INSTAGRAM,
    YOUTUBE,
    TWITTER,
}

#[derive(Deserialize)]
pub struct Socials {
    pub platform: SocialPlatforms,
    pub handle: String,
}
