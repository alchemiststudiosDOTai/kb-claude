use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Represents the type of knowledge base entry
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryType {
    Metadata,
    Debug,
    #[serde(rename = "qa")]
    QA,
    CodeIndex,
    Pattern,
    Cheatsheet,
    Delta,
}

impl EntryType {
    /// Returns the subdirectory name for this entry type
    pub fn to_subdir(&self) -> &'static str {
        match self {
            EntryType::Metadata => "metadata",
            EntryType::Debug => "debug_history",
            EntryType::QA => "qa",
            EntryType::CodeIndex => "code_index",
            EntryType::Pattern => "patterns",
            EntryType::Cheatsheet => "cheatsheets",
            EntryType::Delta => "delta",
        }
    }

    /// Returns all valid entry types
    pub fn all() -> &'static [EntryType] {
        &[
            EntryType::Metadata,
            EntryType::Debug,
            EntryType::QA,
            EntryType::CodeIndex,
            EntryType::Pattern,
            EntryType::Cheatsheet,
            EntryType::Delta,
        ]
    }
}

impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EntryType::Metadata => "metadata",
            EntryType::Debug => "debug",
            EntryType::QA => "qa",
            EntryType::CodeIndex => "code_index",
            EntryType::Pattern => "pattern",
            EntryType::Cheatsheet => "cheatsheet",
            EntryType::Delta => "delta",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for EntryType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "metadata" => Ok(EntryType::Metadata),
            "debug" => Ok(EntryType::Debug),
            "qa" => Ok(EntryType::QA),
            "code_index" => Ok(EntryType::CodeIndex),
            "pattern" => Ok(EntryType::Pattern),
            "cheatsheet" => Ok(EntryType::Cheatsheet),
            "delta" => Ok(EntryType::Delta),
            _ => Err(anyhow!(
                "Invalid entry type: '{}'. Valid types: metadata, debug, qa, code_index, pattern, cheatsheet, delta",
                s
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("metadata".parse::<EntryType>().unwrap(), EntryType::Metadata);
        assert_eq!("debug".parse::<EntryType>().unwrap(), EntryType::Debug);
        assert_eq!("qa".parse::<EntryType>().unwrap(), EntryType::QA);
        assert_eq!("code_index".parse::<EntryType>().unwrap(), EntryType::CodeIndex);
        assert!("invalid".parse::<EntryType>().is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(EntryType::Metadata.to_string(), "metadata");
        assert_eq!(EntryType::Debug.to_string(), "debug");
        assert_eq!(EntryType::QA.to_string(), "qa");
        assert_eq!(EntryType::CodeIndex.to_string(), "code_index");
    }

    #[test]
    fn test_to_subdir() {
        assert_eq!(EntryType::Metadata.to_subdir(), "metadata");
        assert_eq!(EntryType::Debug.to_subdir(), "debug_history");
        assert_eq!(EntryType::QA.to_subdir(), "qa");
        assert_eq!(EntryType::Pattern.to_subdir(), "patterns");
    }
}

