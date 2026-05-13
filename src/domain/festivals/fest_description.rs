use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct FestivalDescription(String);

impl FestivalDescription {
    pub fn parse(description: String) -> Result<Self, String> {
        if description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        if description.len() > 500 {
            return Err("Description cannot be longer than 500 characters".to_string());
        }
        if description.chars().all(|c| {
            c.is_alphabetic() || c.is_whitespace() || c == '.' || c == ',' || c == '!' || c == '?'
        }) {
            return Err("Description cannot contain special characters".to_string());
        }
        Ok(Self(description))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for FestivalDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for FestivalDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for FestivalDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(s).map_err(serde::de::Error::custom)
    }
}
