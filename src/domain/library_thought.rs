use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryThought(String);

impl LibraryThought {
    pub fn parse(s: String) -> Result<Self, String> {
        if s.is_empty() {
            return Err("Library thought cannot be empty".to_string());
        }
        if s.len() > 10000 {
            return Err("Library thought cannot be longer than 10000 characters".to_string());
        }
        if is_unsafe_content(&s) {
            return Err("Library thought cannot contain script or unsafe HTML tags".to_string());
        }
        Ok(LibraryThought(s))
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

impl<'de> serde::Deserialize<'de> for LibraryThought {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(s).map_err(serde::de::Error::custom)
    }
}

impl Display for LibraryThought {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for LibraryThought {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::LibraryThought;

    #[test]
    fn valid_thought_is_accepted() {
        assert!(LibraryThought::parse("Thoughtful impression of the original".to_string()).is_ok());
    }

    #[test]
    fn empty_thought_is_rejected() {
        assert!(LibraryThought::parse("".to_string()).is_err());
    }

    #[test]
    fn long_thought_is_rejected() {
        let thought = "a".repeat(10001);
        assert!(LibraryThought::parse(thought).is_err());
    }

    #[test]
    fn unsafe_script_tags_are_rejected() {
        assert!(LibraryThought::parse("<script>alert(1)</script>".to_string()).is_err());
        assert!(LibraryThought::parse("javascript:alert(1)".to_string()).is_err());
        assert!(LibraryThought::parse("<iframe src='x'>".to_string()).is_err());
    }
}
