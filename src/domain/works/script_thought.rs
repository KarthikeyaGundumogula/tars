use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct ScriptThought(String);

impl ScriptThought {
    pub fn parse(script_thought: String) -> Result<Self, String> {
        if script_thought.len() > 5000 {
            return Err("Script thought cannot be more than 5000 characters".to_string());
        }
        if is_unsafe_content(&script_thought) {
            return Err("Script thought contains unsafe HTML or script tags".to_string());
        }
        Ok(Self(script_thought))
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

impl AsRef<str> for ScriptThought {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ScriptThought {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ScriptThought {
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
    use super::ScriptThought;

    #[test]
    fn valid_thought_is_accepted() {
        assert!(ScriptThought::parse("A brilliant idea".to_string()).is_ok());
    }

    #[test]
    fn empty_thought_is_accepted() {
        assert!(ScriptThought::parse("".to_string()).is_ok());
    }

    #[test]
    fn long_thought_is_rejected() {
        let thought = "a".repeat(5001);
        assert!(ScriptThought::parse(thought).is_err());
    }

    #[test]
    fn unsafe_script_tags_are_rejected() {
        assert!(ScriptThought::parse("<script>alert(1)</script>".to_string()).is_err());
        assert!(ScriptThought::parse("javascript:alert(1)".to_string()).is_err());
        assert!(ScriptThought::parse("<iframe src='x'>".to_string()).is_err());
    }
}
