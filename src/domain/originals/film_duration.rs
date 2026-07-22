use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilmDuration(String);

impl FilmDuration {
    pub fn parse(duration: String) -> Result<Self, String> {
        let trimmed = duration.trim();
        if trimmed.is_empty() {
            return Err("Film duration cannot be empty".to_string());
        }
        if trimmed.chars().count() > 20 {
            return Err("Film duration cannot be longer than 20 characters".to_string());
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for FilmDuration {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FilmDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for FilmDuration {
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
    use super::FilmDuration;

    #[test]
    fn valid_durations_are_accepted() {
        assert!(FilmDuration::parse("2h 15m".to_string()).is_ok());
        assert!(FilmDuration::parse("135 mins".to_string()).is_ok());
    }

    #[test]
    fn empty_duration_is_rejected() {
        assert!(FilmDuration::parse("".to_string()).is_err());
        assert!(FilmDuration::parse("   ".to_string()).is_err());
    }

    #[test]
    fn long_duration_is_rejected() {
        let duration = "a".repeat(21);
        assert!(FilmDuration::parse(duration).is_err());
    }
}
