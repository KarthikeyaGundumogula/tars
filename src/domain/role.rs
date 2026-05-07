use std::fmt;

use serde::{Deserialize, Deserializer};

#[derive( Debug)]
pub struct Role(String);

impl Role {
    pub fn parse(role: String) -> Result<Self, String> {
        if role.is_empty() {
            return Err("Role cannot be empty".to_string());
        }
        if role.len() > 50 {
            return Err("Role cannot be longer than 50 characters".to_string());
        }
        if !role.chars().all(|c| c.is_alphabetic() || c == ' ') {
            return Err("Role can only contain alphabetic characters and spaces".to_string());
        }
        if role.starts_with(' ') || role.ends_with(' ') {
            return Err("Role cannot start or end with a space".to_string());
        }
        if role != role.trim() {
            return Err("Role cannot have leading or trailing whitespace".to_string());
        }
        Ok(Self(role))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Role {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(s).map_err(serde::de::Error::custom)
    }
}
