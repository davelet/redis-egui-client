use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Language {
    Chinese,
    English,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::Chinese => write!(f, "zh"),
            Language::English => write!(f, "en"),
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}
