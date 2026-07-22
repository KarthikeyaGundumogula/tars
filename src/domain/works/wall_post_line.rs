use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WallPostLine(String);

impl WallPostLine {
    pub fn parse(line: String) -> Result<Self, String> {
        if line.trim().is_empty() {
            return Err("Wall post line cannot be empty".to_string());
        }
        if line.chars().count() > 500 {
            return Err("Wall post line cannot be more than 500 characters".to_string());
        }
        if line.trim() != line {
            return Err("Wall post line cannot have leading or trailing whitespace".to_string());
        }
        if is_unsafe_content(&line) {
            return Err("Wall post line contains unsafe HTML or script tags".to_string());
        }
        Ok(Self(line))
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

impl AsRef<str> for WallPostLine {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WallPostLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for WallPostLine {
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
    use super::WallPostLine;

    #[test]
    fn valid_wall_post_line_is_accepted() {
        assert!(WallPostLine::parse("Just watched an amazing edit!".to_string()).is_ok());
    }

    #[test]
    fn empty_wall_post_line_is_rejected() {
        assert!(WallPostLine::parse("".to_string()).is_err());
        assert!(WallPostLine::parse("   ".to_string()).is_err());
    }

    #[test]
    fn long_wall_post_line_is_rejected() {
        let line = "a".repeat(501);
        assert!(WallPostLine::parse(line).is_err());
    }

    #[test]
    fn a_500_char_line_is_accepted() {
        let line = "a".repeat(500);
        assert!(WallPostLine::parse(line).is_ok());
    }

    #[test]
    fn unsafe_script_tags_are_rejected() {
        assert!(WallPostLine::parse("<script>alert(1)</script>".to_string()).is_err());
        assert!(WallPostLine::parse("javascript:alert(1)".to_string()).is_err());
        assert!(WallPostLine::parse("<iframe src='x'>".to_string()).is_err());
    }
}
