use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug)]
pub struct Genre(String);

impl Genre {
    pub fn parse(genre: String) -> Result<Self, String> {
        if genre.is_empty() {
            return Err("Genre cannot be empty".to_string());
        }
        if genre.len() > 50 {
            return Err("Genre cannot be longer than 50 characters".to_string());
        }
        if !genre.chars().all(|c| c.is_alphanumeric() || c == ' ') {
            return Err("Genre can only contain alphanumeric characters and spaces".to_string());
        }
        if genre.starts_with(' ') || genre.ends_with(' ') {
            return Err("Genre cannot start or end with a space".to_string());
        }
        if genre != genre.trim() {
            return Err("Genre cannot have leading or trailing whitespace".to_string());
        }
        Ok(Self(genre))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Genre {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for Genre {
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
    use super::Genre;

    #[test]
    fn valid_genre_is_accepted() {
        assert!(Genre::parse("Action".to_string()).is_ok());
        assert!(Genre::parse("Sci Fi".to_string()).is_ok());
        assert!(Genre::parse("Drama 123".to_string()).is_ok());
    }

    #[test]
    fn empty_genre_is_rejected() {
        assert!(Genre::parse("".to_string()).is_err());
    }

    #[test]
    fn long_genre_is_rejected() {
        let genre = "a".repeat(51);
        assert!(Genre::parse(genre).is_err());
    }

    #[test]
    fn special_characters_are_rejected() {
        assert!(Genre::parse("Action!".to_string()).is_err());
        assert!(Genre::parse("Action@".to_string()).is_err());
    }

    #[test]
    fn leading_trailing_space_is_rejected() {
        assert!(Genre::parse(" Action".to_string()).is_err());
        assert!(Genre::parse("Action ".to_string()).is_err());
    }
}
