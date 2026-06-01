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
        // Only enforce lowercase for Latin script (allow Unicode scripts like Telugu to have any case)
        let has_latin_chars = handle.chars().any(|c| c.is_ascii());
        if has_latin_chars && handle != handle.to_lowercase() {
            return Err("Artist handle must be lowercase".to_string());
        }
        // Allow Unicode letters, numbers, and underscores (supports Telugu and other languages)
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

#[cfg(test)]
mod tests {
    use super::Handle;

    #[test]
    fn a_25_grapheme_long_handle_is_valid() {
        let handle = "a".repeat(25);
        assert!(Handle::parse(handle).is_ok());
    }

    #[test]
    fn a_handle_longer_than_25_graphemes_is_rejected() {
        let handle = "a".repeat(26);
        assert!(Handle::parse(handle).is_err());
    }

    #[test]
    fn whitespace_only_handles_are_rejected() {
        let handle = " ".to_string();
        assert!(Handle::parse(handle).is_err());
    }

    #[test]
    fn empty_string_is_rejected() {
        let handle = "".to_string();
        assert!(Handle::parse(handle).is_err());
    }

    #[test]
    fn handles_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let handle = name.to_string();
            assert!(Handle::parse(handle).is_err());
        }
    }

    #[test]
    fn a_valid_handle_is_parsed_successfully() {
        let handle = "valid_handle_123".to_string();
        assert!(Handle::parse(handle).is_ok());
    }

    #[test]
    fn uppercase_characters_are_rejected() {
        let handle = "InvalidHandle".to_string();
        assert!(Handle::parse(handle).is_err());
    }

    #[test]
    fn starting_or_ending_with_underscore_is_rejected() {
        assert!(Handle::parse("_invalid".to_string()).is_err());
        assert!(Handle::parse("invalid_".to_string()).is_err());
    }

    #[test]
    fn telugu_characters_are_supported() {
        // Telugu characters should be supported
        let handle = "తెలుగు_artist".to_string();
        assert!(Handle::parse(handle).is_ok());
    }
}
