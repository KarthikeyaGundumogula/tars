#[derive(Debug)]
pub struct Statement(String);

impl Statement {
    pub fn parse(statement: String) -> Result<Self, String> {
        if statement.is_empty() {
            return Err("Statement cannot be empty".to_string());
        }
        if statement.len() > 100 {
            return Err("Statement cannot be longer than 100 characters".to_string());
        }
        Ok(Self(statement))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Statement {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl serde::Serialize for Statement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for Statement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::Statement;

    #[test]
    fn valid_statement_is_accepted() {
        assert!(Statement::parse("A powerful artistic statement".to_string()).is_ok());
    }

    #[test]
    fn empty_statement_is_rejected() {
        assert!(Statement::parse("".to_string()).is_err());
    }

    #[test]
    fn long_statement_is_rejected() {
        let statement = "a".repeat(101);
        assert!(Statement::parse(statement).is_err());
    }
}
