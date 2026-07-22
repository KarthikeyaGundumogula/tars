use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleName(String);

impl RoleName {
    pub fn parse(name: String) -> Result<Self, String> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err("Role name cannot be empty".to_string());
        }
        if trimmed.chars().count() > 50 {
            return Err("Role name cannot be longer than 50 characters".to_string());
        }
        if !trimmed.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
            return Err("Role name can only contain alphanumeric characters, underscores, and hyphens".to_string());
        }
        Ok(Self(trimmed.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for RoleName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RoleName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for RoleName {
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
    use super::RoleName;

    #[test]
    fn valid_role_names_are_accepted() {
        assert!(RoleName::parse("organizer".to_string()).is_ok());
        assert!(RoleName::parse("admin_user".to_string()).is_ok());
    }

    #[test]
    fn empty_role_name_is_rejected() {
        assert!(RoleName::parse("".to_string()).is_err());
        assert!(RoleName::parse("   ".to_string()).is_err());
    }

    #[test]
    fn invalid_characters_in_role_are_rejected() {
        assert!(RoleName::parse("admin role!".to_string()).is_err());
    }
}
