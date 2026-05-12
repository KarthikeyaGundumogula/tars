use std::fmt;

use serde::{Deserialize,Deserializer};

#[derive(Debug)]
pub struct OriginalTitle(String);

impl OriginalTitle {
    pub fn parse(title: String) -> Result<Self, String> {
        if title.is_empty() {
            return Err("Original title cannot be empty".to_string());
        }
        if title.len() > 100 {
            return Err("Original title cannot be longer than 100 characters".to_string());
        }
        Ok(Self(title))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for OriginalTitle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for OriginalTitle {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for OriginalTitle {
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
    use super::OriginalTitle;

    #[test]
    fn valid_title_is_accepted() {
        assert!(OriginalTitle::parse("A valid title".to_string()).is_ok());
    }

    #[test]
    fn empty_title_is_rejected() {
        assert!(OriginalTitle::parse("".to_string()).is_err());
    }

    #[test]
    fn long_title_is_rejected() {
        let title = "a".repeat(101);
        assert!(OriginalTitle::parse(title).is_err());
    }
}

