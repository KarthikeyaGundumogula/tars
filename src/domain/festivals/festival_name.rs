#[derive(Debug)]
pub struct FestivalName(String);

impl FestivalName {
    pub fn parse(name: String) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if name.len() > 100 {
            return Err("Name cannot be longer than 100 characters".to_string());
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for FestivalName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for FestivalName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'de> serde::Deserialize<'de> for FestivalName {
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
    use super::FestivalName;

    #[test]
    fn valid_name_is_accepted() {
        assert!(FestivalName::parse("Cannes Film Festival".to_string()).is_ok());
    }

    #[test]
    fn empty_name_is_rejected() {
        assert!(FestivalName::parse("".to_string()).is_err());
    }

    #[test]
    fn long_name_is_rejected() {
        let name = "a".repeat(101);
        assert!(FestivalName::parse(name).is_err());
    }
}
