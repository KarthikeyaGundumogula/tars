use std::fmt;

use serde::{Deserialize,Deserializer};

#[derive(Debug)]
pub struct WorkTitle(String);

impl WorkTitle {
    pub fn parse(work_title: String) -> Result<Self, String> {
        if work_title.len() > 100 {
            return Err("Work title cannot be more than 100 characters".to_string());
        }
        if work_title.trim() != work_title {
            return Err("Work title cannot have leading or trailing spaces".to_string());
        }
        if work_title.chars().any(|c| !c.is_alphabetic() && c != ' ') {
            return Err("Work title can only contain alphabetic characters and spaces".to_string());
        }
        Ok(Self(work_title))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkTitle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for WorkTitle {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for WorkTitle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(s).map_err(serde::de::Error::custom)
    }
}