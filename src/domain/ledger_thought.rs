use std::fmt::Display;

#[derive(Debug)]
pub struct LedgerThought(String);

impl LedgerThought {
    pub fn parse(s: String) -> Result<Self, String> {
        if s.is_empty() {
            return Err("Ledger thought cannot be empty".to_string());
        }
        if s.len() > 10000 {
            return Err("Ledger thought cannot be longer than 1000 characters".to_string());
        }
        if s.contains("<script>") || s.contains("</script>") {
            return Err("Ledger thought cannot contain script tags".to_string());
        }
        Ok(LedgerThought(s))
    }
}

impl<'de> serde::Deserialize<'de> for LedgerThought {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(s).map_err(serde::de::Error::custom)
    }
}

impl Display for LedgerThought {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<String> for LedgerThought {
    fn as_ref(&self) -> &String {
        &self.0
    }
}
