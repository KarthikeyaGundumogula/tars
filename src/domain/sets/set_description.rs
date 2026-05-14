
#[derive(Debug)]
pub struct SetDescription(String);

impl SetDescription {
    pub fn parse(description: String) -> Result<Self, String> {
        if description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        if description.len() > 500 {
            return Err("Description cannot be longer than 500 characters".to_string());
        }
        if !description.chars().all(|c| {
            c.is_alphabetic() || c.is_whitespace() || c == '.' || c == ',' || c == '!' || c == '?'
        }) {
            return Err("Description cannot contain special characters".to_string());
        }
        Ok(Self(description))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SetDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for SetDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'de> serde::Deserialize<'de> for SetDescription {
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
    use super::SetDescription;

    #[test]
    fn valid_description_is_accepted() {
        assert!(SetDescription::parse("A valid description with punctuation!".to_string()).is_ok());
    }

    #[test]
    fn empty_description_is_rejected() {
        assert!(SetDescription::parse("".to_string()).is_err());
    }

    #[test]
    fn long_description_is_rejected() {
        let description = "a".repeat(501);
        assert!(SetDescription::parse(description).is_err());
    }

    #[test]
    fn special_characters_are_rejected() {
        assert!(SetDescription::parse("Description #1".to_string()).is_err());
    }
}

