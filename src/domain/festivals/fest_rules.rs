#[derive(Debug)]
pub struct FestivalRules(String);

impl FestivalRules {
    pub fn parse(rules: String) -> Result<Self, String> {
        if rules.is_empty() {
            return Err("Rules cannot be empty".to_string());
        }
        if rules.len() > 500 {
            return Err("Rules cannot be longer than 500 characters".to_string());
        }
        if !rules.chars().all(|c| {
            c.is_alphanumeric() || c.is_whitespace() || c == '.' || c == ',' || c == '!' || c == '?'
        }) {
            return Err("Rules cannot contain special characters".to_string());
        }
        Ok(Self(rules))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for FestivalRules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for FestivalRules {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'de> serde::Deserialize<'de> for FestivalRules {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::FestivalRules;

    #[test]
    fn valid_rules_are_accepted() {
        assert!(FestivalRules::parse("Rule 1. Rule 2!".to_string()).is_ok());
    }

    #[test]
    fn empty_rules_are_rejected() {
        assert!(FestivalRules::parse("".to_string()).is_err());
    }

    #[test]
    fn long_rules_are_rejected() {
        let rules = "a".repeat(501);
        assert!(FestivalRules::parse(rules).is_err());
    }

    #[test]
    fn special_characters_are_rejected() {
        assert!(FestivalRules::parse("Rule #1".to_string()).is_err());
    }
}

