use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscussionCommentContent(String);

impl DiscussionCommentContent {
    pub fn parse(content: String) -> Result<Self, String> {
        if content.trim().is_empty() {
            return Err("Comment content cannot be empty".to_string());
        }
        if content.chars().count() > 10_000 {
            return Err("Comment content cannot be longer than 10,000 characters".to_string());
        }
        if is_unsafe_content(&content) {
            return Err("Comment content contains unsafe HTML or script tags".to_string());
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

impl AsRef<str> for DiscussionCommentContent {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DiscussionCommentContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for DiscussionCommentContent {
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
    use super::DiscussionCommentContent;

    #[test]
    fn valid_comment_is_accepted() {
        assert!(DiscussionCommentContent::parse("I agree with your analysis!".to_string()).is_ok());
    }

    #[test]
    fn empty_comment_is_rejected() {
        assert!(DiscussionCommentContent::parse("".to_string()).is_err());
        assert!(DiscussionCommentContent::parse("   ".to_string()).is_err());
    }

    #[test]
    fn comment_longer_than_10000_chars_is_rejected() {
        let content = "a".repeat(10_001);
        assert!(DiscussionCommentContent::parse(content).is_err());
    }

    #[test]
    fn a_10000_char_comment_is_accepted() {
        let content = "a".repeat(10_000);
        assert!(DiscussionCommentContent::parse(content).is_ok());
    }

    #[test]
    fn unsafe_script_tags_are_rejected() {
        assert!(DiscussionCommentContent::parse("<script>alert(1)</script>".to_string()).is_err());
        assert!(DiscussionCommentContent::parse("javascript:alert(1)".to_string()).is_err());
        assert!(DiscussionCommentContent::parse("<iframe src='x'>".to_string()).is_err());
    }
}
