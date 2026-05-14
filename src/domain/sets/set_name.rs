#[derive(Debug)]
pub struct SetName(String);

impl SetName {
    pub fn parse(name: String) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if name.len() > 100 {
            return Err("Name cannot be longer than 100 characters".to_string());
        }
        if !name.chars().all(|c| {
            c.is_alphabetic() || c.is_whitespace() || c == '.' || c == ',' || c == '!' || c == '?'
        }) {
            return Err("Name cannot contain special characters".to_string());
        }
        if name.trim() != name {
            return Err("Name cannot start or end with whitespace".to_string());
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SetName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SetName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> serde::Deserialize<'de> for SetName {
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
    use super::SetName;

    #[test]
    fn valid_name_is_accepted() {
        assert!(SetName::parse("My Awesome Set".to_string()).is_ok());
    }

    #[test]
    fn empty_name_is_rejected() {
        assert!(SetName::parse("".to_string()).is_err());
    }

    #[test]
    fn long_name_is_rejected() {
        let name = "a".repeat(101);
        assert!(SetName::parse(name).is_err());
    }

    #[test]
    fn special_characters_are_rejected() {
        assert!(SetName::parse("Set #1".to_string()).is_err());
        assert!(SetName::parse("Set @ Home".to_string()).is_err());
    }

    #[test]
    fn leading_trailing_whitespace_is_rejected() {
        assert!(SetName::parse(" Set".to_string()).is_err());
        assert!(SetName::parse("Set ".to_string()).is_err());
    }
}
