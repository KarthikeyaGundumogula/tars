use std::fmt;

use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub struct Handle(String);

impl Handle {
    pub fn parse(handle: String) -> Result<Self, String> {
        if handle.is_empty() {
            return Err("Artist handle cannot be empty".to_string());
        }
        if handle.len() > 25 {
            return Err("Artist handle cannot be longer than 25 characters".to_string());
        }
        if handle.contains(' ') {
            return Err("Artist handle cannot contain spaces".to_string());
        }
        if handle != handle.to_lowercase() {
            return Err("Artist handle must be lowercase".to_string());
        }
        if !handle.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(
                "Artist handle can only contain alphanumeric characters and underscores"
                    .to_string(),
            );
        }
        if handle.starts_with('_') || handle.ends_with('_') {
            return Err("Artist handle cannot start or end with an underscore".to_string());
        }
        if handle != handle.trim() {
            return Err("Artist handle cannot have leading or trailing whitespace".to_string());
        }
        Ok(Self(handle))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Handle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Handle {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Handle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(s).map_err(serde::de::Error::custom)
    }
}
