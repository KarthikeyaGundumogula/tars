use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscussionPostTitle(String);

impl DiscussionPostTitle {
    pub fn parse(title: String) -> Result<Self, String> {
        if title.trim().is_empty() {
            return Err("Discussion post title cannot be empty".to_string());
        }
        if title.chars().count() > 300 {
            return Err("Discussion post title cannot be longer than 300 characters".to_string());
        }
        if title.trim() != title {
            return Err("Discussion post title cannot have leading or trailing whitespace".to_string());
        }
        if is_unsafe_content(&title) {
            return Err("Discussion post title contains unsafe HTML or script tags".to_string());
        }
        Ok(Self(title))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_unsafe_content(s: &str) -> bool {
    let lower = s.to_lowercase();
    lower.contains("<script")
        || lower.contains("</script")
        || lower.contains("javascript:")
        || lower.contains("<iframe")
        || lower.contains("onerror=")
        || lower.contains("onload=")
}

impl AsRef<str> for DiscussionPostTitle {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DiscussionPostTitle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for DiscussionPostTitle {
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
    use super::DiscussionPostTitle;

    #[test]
    fn valid_title_is_accepted() {
        assert!(DiscussionPostTitle::parse("What do you think of this film?".to_string()).is_ok());
    }

    #[test]
    fn empty_title_is_rejected() {
        assert!(DiscussionPostTitle::parse("".to_string()).is_err());
        assert!(DiscussionPostTitle::parse("   ".to_string()).is_err());
    }

    #[test]
    fn title_longer_than_300_chars_is_rejected() {
        let title = "a".repeat(301);
        assert!(DiscussionPostTitle::parse(title).is_err());
    }

    #[test]
    fn a_300_char_title_is_accepted() {
        let title = "a".repeat(300);
        assert!(DiscussionPostTitle::parse(title).is_ok());
    }

    #[test]
    fn leading_or_trailing_whitespace_is_rejected() {
        assert!(DiscussionPostTitle::parse(" Title".to_string()).is_err());
        assert!(DiscussionPostTitle::parse("Title ".to_string()).is_err());
    }

    #[test]
    fn unsafe_script_tags_are_rejected() {
        assert!(DiscussionPostTitle::parse("<script>alert(1)</script>".to_string()).is_err());
        assert!(DiscussionPostTitle::parse("javascript:alert(1)".to_string()).is_err());
        assert!(DiscussionPostTitle::parse("<iframe src='x'>".to_string()).is_err());
    }
}
