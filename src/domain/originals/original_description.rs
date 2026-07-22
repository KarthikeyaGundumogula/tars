use std::fmt;

use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct OriginalDescription(String);

impl OriginalDescription {
    pub fn parse(description: String) -> Result<Self, String> {
        if description.is_empty() {
            return Err("Original description cannot be empty".to_string());
        }
        if description.len() > 500 {
            return Err("Original description cannot be longer than 500 characters".to_string());
        }
        Ok(Self(description))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for OriginalDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for OriginalDescription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for OriginalDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::OriginalDescription;

    #[test]
    fn valid_description_is_accepted() {
        assert!(OriginalDescription::parse("A valid description".to_string()).is_ok());
    }

    #[test]
    fn empty_description_is_rejected() {
        assert!(OriginalDescription::parse("".to_string()).is_err());
    }

    #[test]
    fn long_description_is_rejected() {
        let description = "a".repeat(501);
        assert!(OriginalDescription::parse(description).is_err());
    }
}
