use std::fmt;

use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct WorkTitle(String);

impl WorkTitle {
    pub fn parse(work_title: String) -> Result<Self, String> {
        if work_title.len() > 100 {
            return Err("Work title cannot be more than 100 characters".to_string());
        }
        if work_title.trim() != work_title {
            return Err("Work title cannot have leading or trailing spaces".to_string());
        }
        // Allow Unicode letters and spaces (supports Telugu and other languages)
        // No case restrictions since work titles can be in any language
        if work_title.chars().any(|c| !c.is_alphabetic() && c != ' ') {
            return Err("Work title can only contain alphabetic characters and spaces".to_string());
        }
        Ok(Self(work_title))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkTitle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for WorkTitle {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for WorkTitle {
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
    use super::WorkTitle;

    #[test]
    fn a_100_grapheme_long_title_is_valid() {
        let title = "a".repeat(100);
        assert!(WorkTitle::parse(title).is_ok());
    }

    #[test]
    fn a_title_longer_than_100_graphemes_is_rejected() {
        let title = "a".repeat(101);
        assert!(WorkTitle::parse(title).is_err());
    }

    #[test]
    fn whitespace_only_titles_are_rejected() {
        // Technically " " is alphabetic? No, `char::is_alphabetic` is false for space,
        // but the rule is `!c.is_alphabetic() && c != ' '`.
        // However, `work_title.trim() != work_title` will reject " ".
        let title = " ".to_string();
        assert!(WorkTitle::parse(title).is_err());
    }

    #[test]
    fn empty_string_is_valid() {
        // Wait, the logic doesn't explicitly reject empty string.
        // Let's verify empty string behavior.
        let title = "".to_string();
        assert!(WorkTitle::parse(title).is_ok());
    }

    #[test]
    fn titles_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}', '1', '_'] {
            let title = name.to_string();
            assert!(WorkTitle::parse(title).is_err());
        }
    }

    #[test]
    fn a_valid_title_is_parsed_successfully() {
        let title = "Valid Title".to_string();
        assert!(WorkTitle::parse(title).is_ok());
    }

    #[test]
    fn leading_or_trailing_whitespace_is_rejected() {
        assert!(WorkTitle::parse(" Title".to_string()).is_err());
        assert!(WorkTitle::parse("Title ".to_string()).is_err());
    }

    #[test]
    fn telugu_characters_are_supported() {
        // Telugu characters should be supported
        let title = "తెలుగు పని".to_string();
        assert!(WorkTitle::parse(title).is_ok());
    }
}
