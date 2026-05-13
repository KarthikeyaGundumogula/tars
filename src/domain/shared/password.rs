use serde::{Deserialize,Deserializer};

#[derive(Debug)]
pub struct Password(String);

impl Password {
    pub fn parse(password: String) -> Result<Self, String> {
        if password.is_empty() {
            return Err("Password cannot be empty".to_string());
        }
        if password.len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }
        if !password.chars().any(|c| c.is_numeric()) {
            return Err("Password must contain at least one number".to_string());
        }
        if !password.chars().any(|c| c.is_uppercase()) {
            return Err("Password must contain at least one uppercase letter".to_string());
        }
        if !password.chars().any(|c| c.is_lowercase()) {
            return Err("Password must contain at least one lowercase letter".to_string());
        }
        Ok(Self(password))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Password {
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
    use super::Password;

    #[test]
    fn valid_password_is_parsed_successfully() {
        assert!(Password::parse("ValidPass123!".to_string()).is_ok());
    }

    #[test]
    fn empty_password_is_rejected() {
        assert!(Password::parse("".to_string()).is_err());
    }

    #[test]
    fn password_shorter_than_8_chars_is_rejected() {
        assert!(Password::parse("Short1!".to_string()).is_err());
    }

    #[test]
    fn password_without_numbers_is_rejected() {
        assert!(Password::parse("NoNumbersHere".to_string()).is_err());
    }

    #[test]
    fn password_without_uppercase_is_rejected() {
        assert!(Password::parse("nouppercase123".to_string()).is_err());
    }

    #[test]
    fn password_without_lowercase_is_rejected() {
        assert!(Password::parse("NOLOWERCASE123".to_string()).is_err());
    }
}

