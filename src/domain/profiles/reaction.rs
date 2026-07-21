use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Emoji(String);

impl Emoji {
    pub fn parse(s: String) -> Result<Self, String> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err("Emoji reaction cannot be empty".to_string());
        }
        if trimmed.len() > 32 {
            return Err("Emoji reaction is too long".to_string());
        }

        let has_emoji = trimmed.chars().any(is_emoji_char);
        if !has_emoji {
            return Err("String does not contain a valid emoji".to_string());
        }

        for c in trimmed.chars() {
            if !is_emoji_char(c) && !c.is_whitespace() {
                return Err(format!("Character '{}' is not an acceptable emoji", c));
            }
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_emoji_char(c: char) -> bool {
    let u = c as u32;
    matches!(
        u,
        0x1F300..=0x1F5FF   | // Misc Symbols and Pictographs (includes skin tone modifiers 0x1F3FB..=0x1F3FF)
        0x1F600..=0x1F64F   | // Emoticons
        0x1F680..=0x1F6FF   | // Transport and Map Symbols
        0x1F900..=0x1F9FF   | // Supplemental Symbols and Pictographs
        0x1FA70..=0x1FAFF   | // Symbols and Pictographs Extended-A
        0x2600..=0x26FF     | // Misc Symbols
        0x2700..=0x27BF     | // Dingbats
        0x1F1E6..=0x1F1FF   | // Regional Indicator Symbols
        0xFE00..=0xFE0F     | // Variation Selectors
        0x200D              | // Zero-width joiner
        0x20E3                // Keycap
    )
}

impl AsRef<str> for Emoji {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Emoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for Emoji {
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
    use super::Emoji;

    #[test]
    fn valid_emojis_are_accepted() {
        assert!(Emoji::parse("🔥".to_string()).is_ok());
        assert!(Emoji::parse("❤️".to_string()).is_ok());
        assert!(Emoji::parse("👍🏽".to_string()).is_ok());
        assert!(Emoji::parse("😀".to_string()).is_ok());
    }

    #[test]
    fn empty_emoji_is_rejected() {
        assert!(Emoji::parse("".to_string()).is_err());
        assert!(Emoji::parse("   ".to_string()).is_err());
    }

    #[test]
    fn plain_text_is_rejected() {
        assert!(Emoji::parse("hello".to_string()).is_err());
        assert!(Emoji::parse("123".to_string()).is_err());
    }

    #[test]
    fn text_with_emoji_is_rejected() {
        assert!(Emoji::parse("fire 🔥".to_string()).is_err());
    }
}
