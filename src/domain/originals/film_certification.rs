use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilmCertification(String);

impl FilmCertification {
    pub fn parse(cert: String) -> Result<Self, String> {
        let trimmed = cert.trim();
        if trimmed.is_empty() {
            return Err("Film certification cannot be empty".to_string());
        }
        if trimmed.chars().count() > 10 {
            return Err("Film certification cannot be longer than 10 characters".to_string());
        }
        Ok(Self(trimmed.to_uppercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for FilmCertification {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FilmCertification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for FilmCertification {
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
    use super::FilmCertification;

    #[test]
    fn valid_certifications_are_accepted() {
        assert!(FilmCertification::parse("U/A".to_string()).is_ok());
        assert!(FilmCertification::parse("PG-13".to_string()).is_ok());
        assert!(FilmCertification::parse("R".to_string()).is_ok());
    }

    #[test]
    fn empty_certification_is_rejected() {
        assert!(FilmCertification::parse("".to_string()).is_err());
        assert!(FilmCertification::parse("   ".to_string()).is_err());
    }

    #[test]
    fn long_certification_is_rejected() {
        let cert = "a".repeat(11);
        assert!(FilmCertification::parse(cert).is_err());
    }
}
