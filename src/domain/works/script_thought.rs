use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct ScriptThought(String);

impl ScriptThought {
    pub fn parse(script_thought: String) -> Result<Self, String> {
        if script_thought.len() > 500 {
            return Err("Script thought cannot be more than 500 characters".to_string());
        }
        Ok(Self(script_thought))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ScriptThought {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ScriptThought {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
        // Based on parse logic, empty is not rejected unless we add a check.
        // Current logic only checks for length > 500.
        assert!(ScriptThought::parse("".to_string()).is_ok());
    }

    #[test]
    fn long_thought_is_rejected() {
        let thought = "a".repeat(501);
        assert!(ScriptThought::parse(thought).is_err());
    }
}
