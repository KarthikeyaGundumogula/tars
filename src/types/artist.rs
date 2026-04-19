use uuid::Uuid;

pub struct Artist {
    pub id: Uuid,
    pub user_name: String,
    pub bio: String,
    pub presence: i64,
    pub socials: Vec<Socials>,
}

pub enum Socials {
    Youtube { channel_url: String },
    Twitter { profile_url: String },
    Instagram { page_url: String },
}
