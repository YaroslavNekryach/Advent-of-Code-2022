use super::super::super::utils::Result;
use regex::Regex;
use std::ops::Index;

#[derive(Clone, Debug)]
pub struct File {
    pub name: String,
    pub size: u64,
}

impl File {
    pub fn new(name: &str, size: u64) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }

    pub fn from_string(s: &str) -> Result<Self> {
        let re = Regex::new(r"^(\d+) (.+)$")?;
        let captures = re.captures(s).ok_or("Invalid input")?;
        Ok(Self::new(captures.index(2), captures.index(1).parse()?))
    }
}
