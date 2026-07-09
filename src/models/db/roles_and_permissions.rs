use chrono::{DateTime, Utc};

pub struct UserRole {
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub struct Permission {
  pub name: String,
  pub description: Option<String>,
  pub created_at: DateTime<Utc>
}