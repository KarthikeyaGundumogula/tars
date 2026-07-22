use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug)]
pub struct StageName(String);

impl StageName {
    pub fn parse(stage_name: String) -> Result<Self, String> {
        if stage_name.is_empty() {
            return Err("Artist stage name cannot be empty".to_string());
        }
        if stage_name.chars().count() > 15 {
            return Err("Artist stage name cannot be longer than 15 characters".to_string());
        }
        // Only enforce lowercase for Latin script (allow Unicode scripts like Telugu to have any case)
        let has_latin_chars = stage_name.chars().any(|c| c.is_ascii());
        if has_latin_chars && stage_name != stage_name.to_lowercase() {
            return Err("Artist stage name must be lowercase".to_string());
        }
        // Allow Unicode letters and spaces (supports Telugu and other languages)
        if !stage_name.chars().all(|c| c.is_alphabetic() || c == ' ') {
            return Err(
                "Artist stage name can only contain alphabetic characters and spaces".to_string(),
            );
        }
        if stage_name.starts_with(' ') || stage_name.ends_with(' ') {
            return Err("Artist stage name cannot start or end with a space".to_string());
        }
        if stage_name != stage_name.trim() {
            return Err("Artist stage name cannot have leading or trailing whitespace".to_string());
        }
        Ok(Self(stage_name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StageName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for StageName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for StageName {
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
    use super::StageName;

    #[test]
    fn valid_stage_name_is_accepted() {
        assert!(StageName::parse("kapten og".to_string()).is_ok());
    }

    #[test]
    fn empty_stage_name_is_rejected() {
        assert!(StageName::parse("".to_string()).is_err());
    }

    #[test]
    fn long_stage_name_is_rejected() {
        let name = "a".repeat(16);
        assert!(StageName::parse(name).is_err());
    }

    #[test]
    fn uppercase_is_rejected() {
        assert!(StageName::parse("Kapten OG".to_string()).is_err());
    }

    #[test]
    fn non_alphabetic_characters_are_rejected() {
        assert!(StageName::parse("kapten 1".to_string()).is_err());
        assert!(StageName::parse("kapten!".to_string()).is_err());
    }

    #[test]
    fn leading_trailing_space_is_rejected() {
        assert!(StageName::parse(" kapten".to_string()).is_err());
        assert!(StageName::parse("kapten ".to_string()).is_err());
    }

    #[test]
    fn telugu_characters_are_supported() {
        // Telugu characters should be supported
        let stage_name = "తెలుగు కళ".to_string();
        assert!(StageName::parse(stage_name).is_ok());
    }
}
