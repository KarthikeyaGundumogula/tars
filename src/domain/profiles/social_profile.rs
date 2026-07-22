use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SocialProfile(String);

impl SocialProfile {
    pub fn parse(s: String) -> Result<Self, String> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err("Social profile handle/link cannot be empty".to_string());
        }
        if trimmed.chars().count() > 100 {
            return Err("Social profile handle/link cannot be longer than 100 characters".to_string());
        }
        if trimmed.contains(char::is_whitespace) {
            return Err("Social profile handle/link cannot contain whitespace".to_string());
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SocialProfile {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SocialProfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for SocialProfile {
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
    use super::SocialProfile;

    #[test]
    fn valid_social_profiles_are_accepted() {
        assert!(SocialProfile::parse("https://youtube.com/@creator".to_string()).is_ok());
        assert!(SocialProfile::parse("creator_handle".to_string()).is_ok());
    }

    #[test]
    fn empty_social_profile_is_rejected() {
        assert!(SocialProfile::parse("".to_string()).is_err());
        assert!(SocialProfile::parse("   ".to_string()).is_err());
    }

    #[test]
    fn whitespace_in_profile_is_rejected() {
        assert!(SocialProfile::parse("creator handle".to_string()).is_err());
    }

    #[test]
    fn long_social_profile_is_rejected() {
        let long_url = format!("https://youtube.com/{}", "a".repeat(100));
        assert!(SocialProfile::parse(long_url).is_err());
    }
}
