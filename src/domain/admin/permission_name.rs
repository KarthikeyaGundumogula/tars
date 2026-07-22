use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionName(String);

impl PermissionName {
    pub fn parse(name: String) -> Result<Self, String> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err("Permission name cannot be empty".to_string());
        }
        if trimmed.chars().count() > 50 {
            return Err("Permission name cannot be longer than 50 characters".to_string());
        }
        if !trimmed.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
            return Err("Permission name can only contain alphanumeric characters, underscores, and hyphens".to_string());
        }
        Ok(Self(trimmed.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PermissionName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PermissionName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for PermissionName {
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
    use super::PermissionName;

    #[test]
    fn valid_permission_names_are_accepted() {
        assert!(PermissionName::parse("can_manage_festivals".to_string()).is_ok());
        assert!(PermissionName::parse("create_original".to_string()).is_ok());
    }

    #[test]
    fn empty_permission_name_is_rejected() {
        assert!(PermissionName::parse("".to_string()).is_err());
        assert!(PermissionName::parse("   ".to_string()).is_err());
    }

    #[test]
    fn invalid_characters_in_permission_are_rejected() {
        assert!(PermissionName::parse("manage festivals!".to_string()).is_err());
    }
}
