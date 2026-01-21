use e_client_bilingual::language::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigOfUser {
    #[serde(default)]
    pub language: String,
}

impl Default for ConfigOfUser {
    fn default() -> Self {
        Self {
            language: Language::default().to_file_string(),
        }
    }
}
