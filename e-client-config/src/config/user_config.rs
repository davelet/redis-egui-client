use crate::config::shortcuts::ShortcutConfig;
use crate::config::theme::Theme;
use e_client_bilingual::language::Language;
use serde::{Deserialize, Serialize};

/// Configuration for the update checker
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateConfig {
    /// Whether to check for updates on startup
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Check interval in hours (default: 24)
    #[serde(default = "default_check_interval")]
    pub check_interval_hours: u32,
    /// Version to skip (user chose "skip this version")
    #[serde(default)]
    pub skip_version: Option<String>,
    /// Last check time as ISO 8601 string
    #[serde(default)]
    pub last_check: Option<String>,
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval_hours: 24,
            skip_version: None,
            last_check: None,
        }
    }
}

fn default_check_interval() -> u32 {
    24
}

fn default_true() -> bool {
    true
}

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
    /// Update checker configuration
    #[serde(default)]
    pub update_config: UpdateConfig,
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
            update_config: UpdateConfig::default(),
        }
    }
}

fn default_auto_expand_threshold() -> usize {
    2
}
