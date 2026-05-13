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
        if rules.chars().all(|c| {
            c.is_alphabetic() || c.is_whitespace() || c == '.' || c == ',' || c == '!' || c == '?'
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
