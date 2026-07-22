use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscussionPostContent(String);

impl DiscussionPostContent {
    pub fn parse(content: String) -> Result<Self, String> {
        if content.trim().is_empty() {
            return Err("Discussion post content cannot be empty".to_string());
        }
        if content.chars().count() > 40_000 {
            return Err("Discussion post content cannot be longer than 40,000 characters".to_string());
        }
        if is_unsafe_content(&content) {
            return Err("Discussion post content contains unsafe HTML or script tags".to_string());
        }
        Ok(Self(content))
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

impl AsRef<str> for DiscussionPostContent {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DiscussionPostContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for DiscussionPostContent {
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
    use super::DiscussionPostContent;

    #[test]
    fn valid_content_is_accepted() {
        assert!(DiscussionPostContent::parse("Here is my full review...".to_string()).is_ok());
    }

    #[test]
    fn empty_content_is_rejected() {
        assert!(DiscussionPostContent::parse("".to_string()).is_err());
        assert!(DiscussionPostContent::parse("   ".to_string()).is_err());
    }

    #[test]
    fn content_longer_than_40000_chars_is_rejected() {
        let content = "a".repeat(40_001);
        assert!(DiscussionPostContent::parse(content).is_err());
    }

    #[test]
    fn a_40000_char_content_is_accepted() {
        let content = "a".repeat(40_000);
        assert!(DiscussionPostContent::parse(content).is_ok());
    }

    #[test]
    fn unsafe_script_tags_are_rejected() {
        assert!(DiscussionPostContent::parse("<script>alert(1)</script>".to_string()).is_err());
        assert!(DiscussionPostContent::parse("javascript:alert(1)".to_string()).is_err());
        assert!(DiscussionPostContent::parse("<iframe src='x'>".to_string()).is_err());
    }
}
