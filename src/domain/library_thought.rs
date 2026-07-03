use std::fmt::Display;

#[derive(Debug)]
pub struct LibraryThought(String);

impl LibraryThought {
    pub fn parse(s: String) -> Result<Self, String> {
        if s.is_empty() {
            return Err("Library thought cannot be empty".to_string());
        }
        if s.len() > 10000 {
            return Err("Library thought cannot be longer than 1000 characters".to_string());
        }
        if s.contains("<script>") || s.contains("</script>") {
            return Err("Library thought cannot contain script tags".to_string());
        }
        Ok(LibraryThought(s))
    }
}

impl<'de> serde::Deserialize<'de> for LibraryThought {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(s).map_err(serde::de::Error::custom)
    }
}

impl Display for LibraryThought {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<String> for LibraryThought {
    fn as_ref(&self) -> &String {
        &self.0
    }
}
