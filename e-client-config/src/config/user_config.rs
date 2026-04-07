use crate::config::shortcuts::ShortcutConfig;
use crate::config::theme::Theme;
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
    #[serde(default = "default_true")]
    pub group_keys_by_colon: bool,
    #[serde(default)]
    pub auto_refresh_ttl: bool,
    #[serde(default)]
    pub auto_expand: bool,
    #[serde(default = "default_auto_expand_threshold")]
    pub auto_expand_threshold: usize,
    #[serde(default = "default_true")]
    pub open_connections_in_new_tab: bool,
    #[serde(default)]
    pub theme: Theme,
}

impl Default for ConfigOfUser {
    fn default() -> Self {
        Self {
            language: Language::default().to_file_string(),
            auto_connect: false,
            shortcuts: ShortcutConfig::default(),
            show_unclosed_connections: true,
            allow_duplicate_connections: true,
            group_keys_by_colon: true,
            auto_refresh_ttl: false,
            auto_expand: true,
            auto_expand_threshold: default_auto_expand_threshold(),
            open_connections_in_new_tab: true,
            theme: Theme::default(),
        }
    }
}

fn default_auto_expand_threshold() -> usize {
    2
}

fn default_true() -> bool {
    true
}
