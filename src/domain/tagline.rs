use std::fmt;

use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub struct TagLine(String);

impl TagLine {
    pub fn parse(tag_line: String) -> Result<Self, String> {
        if tag_line.is_empty() {
            return Err("Tag line cannot be empty".to_string());
        }
        if tag_line.len() > 100 {
            return Err("Tag line cannot be more than 100 characters".to_string());
        }
        Ok(Self(tag_line))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for TagLine {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TagLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for TagLine {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(s).map_err(serde::de::Error::custom)
    }
}