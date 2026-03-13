use crate::config::shortcuts::ShortcutConfig;
use e_client_bilingual::language::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigOfUser {
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub auto_connect: bool,
    #[serde(default)]
    pub shortcuts: ShortcutConfig,
    #[serde(default = "default_true")]
    pub show_unclosed_connections: bool,
    #[serde(default = "default_true")]
    pub allow_duplicate_connections: bool,
}

fn default_true() -> bool {
    true
}

impl Default for ConfigOfUser {
    fn default() -> Self {
        Self {
            language: Language::default().to_file_string(),
            auto_connect: false,
            shortcuts: ShortcutConfig::default(),
            show_unclosed_connections: true,
            allow_duplicate_connections: true,
        }
    }
}
