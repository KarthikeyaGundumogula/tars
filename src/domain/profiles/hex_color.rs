use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug)]
pub struct HexColor(String);

impl HexColor {
    pub fn parse(color: String) -> Result<Self, String> {
        if color.is_empty() {
            return Err("Color cannot be empty".to_string());
        }
        if !color.starts_with('#') {
            return Err("Color must start with '#'".to_string());
        }
        let hex_digits = &color[1..];
        if hex_digits.len() != 6 {
            return Err("Color must be exactly 7 characters long (e.g. #FF0000)".to_string());
        }
        if !hex_digits.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(
                "Color must only contain valid hex digits (0-9, A-F, a-f) after '#'".to_string(),
            );
        }
        Ok(Self(color.to_uppercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HexColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for HexColor {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for HexColor {
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
    use super::HexColor;

    // -------------------------------------------------------------------
    // Valid colors
    // -------------------------------------------------------------------

    #[test]
    fn valid_uppercase_hex_color_is_accepted() {
        assert!(HexColor::parse("#FF0000".to_string()).is_ok());
    }

    #[test]
    fn valid_lowercase_hex_color_is_accepted() {
        assert!(HexColor::parse("#abcdef".to_string()).is_ok());
    }

    #[test]
    fn valid_mixed_case_hex_color_is_accepted() {
        assert!(HexColor::parse("#aAbBcC".to_string()).is_ok());
    }

    #[test]
    fn black_color_is_accepted() {
        assert!(HexColor::parse("#000000".to_string()).is_ok());
    }

    #[test]
    fn white_color_is_accepted() {
        assert!(HexColor::parse("#FFFFFF".to_string()).is_ok());
    }

    #[test]
    fn parsed_color_is_normalized_to_uppercase() {
        let color = HexColor::parse("#abcdef".to_string()).unwrap();
        assert_eq!(color.as_str(), "#ABCDEF");
    }

    // -------------------------------------------------------------------
    // Invalid colors
    // -------------------------------------------------------------------

    #[test]
    fn empty_string_is_rejected() {
        assert!(HexColor::parse("".to_string()).is_err());
    }

    #[test]
    fn missing_hash_prefix_is_rejected() {
        assert!(HexColor::parse("FF0000".to_string()).is_err());
    }

    #[test]
    fn too_short_hex_is_rejected() {
        assert!(HexColor::parse("#FFF".to_string()).is_err());
    }

    #[test]
    fn too_long_hex_is_rejected() {
        assert!(HexColor::parse("#FF00000".to_string()).is_err());
    }

    #[test]
    fn non_hex_characters_are_rejected() {
        assert!(HexColor::parse("#GGGGGG".to_string()).is_err());
        assert!(HexColor::parse("#ZZZZZZ".to_string()).is_err());
    }

    #[test]
    fn special_characters_are_rejected() {
        assert!(HexColor::parse("#FF00!0".to_string()).is_err());
        assert!(HexColor::parse("#FF 000".to_string()).is_err());
    }

    #[test]
    fn hash_only_is_rejected() {
        assert!(HexColor::parse("#".to_string()).is_err());
    }
}
