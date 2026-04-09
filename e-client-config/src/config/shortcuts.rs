use e_client_bilingual::translations::TranslationKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    EnumIter,
    EnumString,
    Display,
    AsRefStr,
)]
pub enum ShortcutAction {
    #[serde(rename = "new_connection")]
    #[strum(serialize = "NewConnection")]
    NewConnection,
    #[serde(rename = "confirm_new_connection")]
    #[strum(serialize = "ConfirmNewConnection")]
    ConfirmNewConnection,
    #[serde(rename = "cancel_new_connection")]
    #[strum(serialize = "CancelNewConnection")]
    CancelNewConnection,
    #[serde(rename = "connect_all_unclosed")]
    #[strum(serialize = "ConnectAllUnclosed")]
    ConnectAllUnclosed,
    #[serde(rename = "connect_connection_1")]
    #[strum(serialize = "ConnectConnection1")]
    ConnectConnection1,
    #[serde(rename = "connect_connection_2")]
    #[strum(serialize = "ConnectConnection2")]
    ConnectConnection2,
    #[serde(rename = "connect_connection_3")]
    #[strum(serialize = "ConnectConnection3")]
    ConnectConnection3,
    #[serde(rename = "connect_connection_4")]
    #[strum(serialize = "ConnectConnection4")]
    ConnectConnection4,
    #[serde(rename = "connect_connection_5")]
    #[strum(serialize = "ConnectConnection5")]
    ConnectConnection5,
    #[serde(rename = "connect_connection_6")]
    #[strum(serialize = "ConnectConnection6")]
    ConnectConnection6,
    #[serde(rename = "connect_connection_7")]
    #[strum(serialize = "ConnectConnection7")]
    ConnectConnection7,
    #[serde(rename = "connect_connection_8")]
    #[strum(serialize = "ConnectConnection8")]
    ConnectConnection8,
    #[serde(rename = "connect_connection_9")]
    #[strum(serialize = "ConnectConnection9")]
    ConnectConnection9,
    #[serde(rename = "new_tab")]
    #[strum(serialize = "NewTab")]
    NewTab,
    #[serde(rename = "close_tab")]
    #[strum(serialize = "CloseTab")]
    CloseTab,
    #[serde(rename = "refresh_key")]
    #[strum(serialize = "RefreshKey")]
    RefreshKey,
    #[serde(rename = "focus_filter")]
    #[strum(serialize = "FocusFilter")]
    FocusFilter,
    #[serde(rename = "close_settings")]
    #[strum(serialize = "CloseSettings")]
    CloseSettings,
    #[serde(rename = "open_settings")]
    #[strum(serialize = "OpenSettings")]
    OpenSettings,
    #[serde(rename = "toggle_command_line")]
    #[strum(serialize = "ToggleCommandLine")]
    ToggleCommandLine,
    #[serde(rename = "close_command_line")]
    #[strum(serialize = "CloseCommandLine")]
    CloseCommandLine,
    #[serde(rename = "switch_to_tab_1")]
    #[strum(serialize = "SwitchToTab1")]
    SwitchToTab1,
    #[serde(rename = "switch_to_tab_2")]
    #[strum(serialize = "SwitchToTab2")]
    SwitchToTab2,
    #[serde(rename = "switch_to_tab_3")]
    #[strum(serialize = "SwitchToTab3")]
    SwitchToTab3,
    #[serde(rename = "switch_to_tab_4")]
    #[strum(serialize = "SwitchToTab4")]
    SwitchToTab4,
    #[serde(rename = "switch_to_tab_5")]
    #[strum(serialize = "SwitchToTab5")]
    SwitchToTab5,
    #[serde(rename = "switch_to_tab_6")]
    #[strum(serialize = "SwitchToTab6")]
    SwitchToTab6,
    #[serde(rename = "switch_to_tab_7")]
    #[strum(serialize = "SwitchToTab7")]
    SwitchToTab7,
    #[serde(rename = "switch_to_tab_8")]
    #[strum(serialize = "SwitchToTab8")]
    SwitchToTab8,
    #[serde(rename = "switch_to_tab_9")]
    #[strum(serialize = "SwitchToTab9")]
    SwitchToTab9,
    #[serde(rename = "switch_to_last_tab")]
    #[strum(serialize = "SwitchToLastTab")]
    SwitchToLastTab,
    #[serde(rename = "remove_duplicate_and_invalid_tabs")]
    #[strum(serialize = "RemoveDuplicateAndInvalidTabs")]
    RemoveDuplicateAndInvalidTabs,
    #[serde(rename = "refresh_keys")]
    #[strum(serialize = "RefreshKeys")]
    RefreshKeys,
    #[serde(rename = "execute_ai_command")]
    #[strum(serialize = "ExecuteAiCommand")]
    ExecuteAiCommand,
    #[serde(rename = "cancel_ai_command")]
    #[strum(serialize = "CancelAiCommand")]
    CancelAiCommand,
    #[serde(rename = "close_ai_model_editor")]
    #[strum(serialize = "CloseAiModelEditor")]
    CloseAiModelEditor,
    #[serde(rename = "toggle_help")]
    #[strum(serialize = "ToggleHelp")]
    ToggleHelp,
}

impl ShortcutAction {
    /// Returns all actions with their translation keys
    pub fn all_actions() -> Vec<(ShortcutAction, TranslationKey)> {
        use strum::IntoEnumIterator;
        ShortcutAction::iter()
            .map(|action| (action.clone(), action.translation_key()))
            .collect()
    }

    pub fn default_key(&self) -> String {
        // Use Cmd on macOS, Ctrl on other platforms
        let mod_key = if cfg!(target_os = "macos") {
            "Cmd"
        } else {
            "Ctrl"
        };
        match self {
            ShortcutAction::NewConnection => format!("{}+N", mod_key),
            ShortcutAction::ConfirmNewConnection => "Cmd+Enter".to_string(),
            ShortcutAction::CancelNewConnection => "Esc".to_string(),
            ShortcutAction::ConnectAllUnclosed => "0".to_string(),
            ShortcutAction::ConnectConnection1 => "1".to_string(),
            ShortcutAction::ConnectConnection2 => "2".to_string(),
            ShortcutAction::ConnectConnection3 => "3".to_string(),
            ShortcutAction::ConnectConnection4 => "4".to_string(),
            ShortcutAction::ConnectConnection5 => "5".to_string(),
            ShortcutAction::ConnectConnection6 => "6".to_string(),
            ShortcutAction::ConnectConnection7 => "7".to_string(),
            ShortcutAction::ConnectConnection8 => "8".to_string(),
            ShortcutAction::ConnectConnection9 => "9".to_string(),
            ShortcutAction::NewTab => format!("{}+T", mod_key),
            ShortcutAction::CloseTab => format!("{}+W", mod_key),
            ShortcutAction::RefreshKey => format!("{}+R", mod_key),
            ShortcutAction::FocusFilter => format!("{}+F", mod_key),
            ShortcutAction::CloseSettings => "Esc".to_string(),
            ShortcutAction::OpenSettings => format!("{}+Comma", mod_key),
            ShortcutAction::ToggleCommandLine => format!("{}+E", mod_key),
            ShortcutAction::CloseCommandLine => "Esc".to_string(),
            ShortcutAction::SwitchToTab1 => format!("{}+1", mod_key),
            ShortcutAction::SwitchToTab2 => format!("{}+2", mod_key),
            ShortcutAction::SwitchToTab3 => format!("{}+3", mod_key),
            ShortcutAction::SwitchToTab4 => format!("{}+4", mod_key),
            ShortcutAction::SwitchToTab5 => format!("{}+5", mod_key),
            ShortcutAction::SwitchToTab6 => format!("{}+6", mod_key),
            ShortcutAction::SwitchToTab7 => format!("{}+7", mod_key),
            ShortcutAction::SwitchToTab8 => format!("{}+8", mod_key),
            ShortcutAction::SwitchToTab9 => format!("{}+9", mod_key),
            ShortcutAction::SwitchToLastTab => format!("{}+0", mod_key),
            ShortcutAction::RemoveDuplicateAndInvalidTabs => format!("{}+Shift+D", mod_key),
            ShortcutAction::RefreshKeys => "F5".to_string(),
            ShortcutAction::ExecuteAiCommand => "Right".to_string(),
            ShortcutAction::CancelAiCommand => "Left".to_string(),
            ShortcutAction::CloseAiModelEditor => "Esc".to_string(),
            ShortcutAction::ToggleHelp => "F1".to_string(),
        }
    }

    /// Returns the translation key for this action
    pub fn translation_key(&self) -> TranslationKey {
        match self {
            ShortcutAction::NewConnection => TranslationKey::ShortcutNewConnection,
            ShortcutAction::ConfirmNewConnection => TranslationKey::ShortcutConfirmNewConnection,
            ShortcutAction::CancelNewConnection => TranslationKey::ShortcutCancelNewConnection,
            ShortcutAction::ConnectAllUnclosed => TranslationKey::ShortcutConnectAllUnclosed,
            ShortcutAction::ConnectConnection1 => TranslationKey::ShortcutConnectConnection1,
            ShortcutAction::ConnectConnection2 => TranslationKey::ShortcutConnectConnection2,
            ShortcutAction::ConnectConnection3 => TranslationKey::ShortcutConnectConnection3,
            ShortcutAction::ConnectConnection4 => TranslationKey::ShortcutConnectConnection4,
            ShortcutAction::ConnectConnection5 => TranslationKey::ShortcutConnectConnection5,
            ShortcutAction::ConnectConnection6 => TranslationKey::ShortcutConnectConnection6,
            ShortcutAction::ConnectConnection7 => TranslationKey::ShortcutConnectConnection7,
            ShortcutAction::ConnectConnection8 => TranslationKey::ShortcutConnectConnection8,
            ShortcutAction::ConnectConnection9 => TranslationKey::ShortcutConnectConnection9,
            ShortcutAction::NewTab => TranslationKey::ShortcutNewTab,
            ShortcutAction::CloseTab => TranslationKey::ShortcutCloseTab,
            ShortcutAction::RefreshKey => TranslationKey::ShortcutRefreshKey,
            ShortcutAction::FocusFilter => TranslationKey::ShortcutFocusFilter,
            ShortcutAction::CloseSettings => TranslationKey::ShortcutCloseSettings,
            ShortcutAction::OpenSettings => TranslationKey::ShortcutOpenSettings,
            ShortcutAction::ToggleCommandLine => TranslationKey::ShortcutToggleCommandLine,
            ShortcutAction::CloseCommandLine => TranslationKey::ShortcutCloseCommandLine,
            ShortcutAction::SwitchToTab1 => TranslationKey::ShortcutSwitchToTab1,
            ShortcutAction::SwitchToTab2 => TranslationKey::ShortcutSwitchToTab2,
            ShortcutAction::SwitchToTab3 => TranslationKey::ShortcutSwitchToTab3,
            ShortcutAction::SwitchToTab4 => TranslationKey::ShortcutSwitchToTab4,
            ShortcutAction::SwitchToTab5 => TranslationKey::ShortcutSwitchToTab5,
            ShortcutAction::SwitchToTab6 => TranslationKey::ShortcutSwitchToTab6,
            ShortcutAction::SwitchToTab7 => TranslationKey::ShortcutSwitchToTab7,
            ShortcutAction::SwitchToTab8 => TranslationKey::ShortcutSwitchToTab8,
            ShortcutAction::SwitchToTab9 => TranslationKey::ShortcutSwitchToTab9,
            ShortcutAction::SwitchToLastTab => TranslationKey::ShortcutSwitchToLastTab,
            ShortcutAction::RemoveDuplicateAndInvalidTabs => {
                TranslationKey::ShortcutRemoveDuplicateAndInvalidTabs
            }
            ShortcutAction::RefreshKeys => TranslationKey::ShortcutRefreshKeys,
            ShortcutAction::ExecuteAiCommand => TranslationKey::ShortcutExecuteAiCommand,
            ShortcutAction::CancelAiCommand => TranslationKey::ShortcutCancelAiCommand,
            ShortcutAction::CloseAiModelEditor => TranslationKey::ShortcutCloseAiModelEditor,
            ShortcutAction::ToggleHelp => TranslationKey::Help,
        }
    }

    /// Returns true if this shortcut is not customizable
    pub fn is_non_editable(&self) -> bool {
        matches!(
            self,
            ShortcutAction::OpenSettings
                | ShortcutAction::CloseSettings
                | ShortcutAction::CloseCommandLine
                | ShortcutAction::ExecuteAiCommand
                | ShortcutAction::CancelAiCommand
                | ShortcutAction::CancelNewConnection
                | ShortcutAction::SwitchToTab1
                | ShortcutAction::SwitchToTab2
                | ShortcutAction::SwitchToTab3
                | ShortcutAction::SwitchToTab4
                | ShortcutAction::SwitchToTab5
                | ShortcutAction::SwitchToTab6
                | ShortcutAction::SwitchToTab7
                | ShortcutAction::SwitchToTab8
                | ShortcutAction::SwitchToTab9
                | ShortcutAction::SwitchToLastTab
                | ShortcutAction::ConnectAllUnclosed
                | ShortcutAction::ConnectConnection1
                | ShortcutAction::ConnectConnection2
                | ShortcutAction::ConnectConnection3
                | ShortcutAction::ConnectConnection4
                | ShortcutAction::ConnectConnection5
                | ShortcutAction::ConnectConnection6
                | ShortcutAction::ConnectConnection7
                | ShortcutAction::ConnectConnection8
                | ShortcutAction::ConnectConnection9
                | ShortcutAction::RefreshKeys
                | ShortcutAction::CloseAiModelEditor
                | ShortcutAction::ToggleHelp
        )
    }

    /// Returns the tab index for switch actions (1-9), or None if not a switch action
    pub fn tab_index(&self) -> Option<usize> {
        match self {
            ShortcutAction::SwitchToTab1 => Some(0),
            ShortcutAction::SwitchToTab2 => Some(1),
            ShortcutAction::SwitchToTab3 => Some(2),
            ShortcutAction::SwitchToTab4 => Some(3),
            ShortcutAction::SwitchToTab5 => Some(4),
            ShortcutAction::SwitchToTab6 => Some(5),
            ShortcutAction::SwitchToTab7 => Some(6),
            ShortcutAction::SwitchToTab8 => Some(7),
            ShortcutAction::SwitchToTab9 => Some(8),
            _ => None,
        }
    }

    /// Returns the connection index for connect connection actions (1-9), or None if not a connect action
    pub fn connection_index(&self) -> Option<usize> {
        match self {
            ShortcutAction::ConnectConnection1 => Some(0),
            ShortcutAction::ConnectConnection2 => Some(1),
            ShortcutAction::ConnectConnection3 => Some(2),
            ShortcutAction::ConnectConnection4 => Some(3),
            ShortcutAction::ConnectConnection5 => Some(4),
            ShortcutAction::ConnectConnection6 => Some(5),
            ShortcutAction::ConnectConnection7 => Some(6),
            ShortcutAction::ConnectConnection8 => Some(7),
            ShortcutAction::ConnectConnection9 => Some(8),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortcutConfig {
    #[serde(flatten)]
    pub bindings: HashMap<String, String>, // action_name -> key_combination
}

impl Default for ShortcutConfig {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        for (action, _) in ShortcutAction::all_actions() {
            bindings.insert(format!("{:?}", action), action.default_key());
        }
        Self { bindings }
    }
}

impl ShortcutConfig {
    pub fn get_binding(&self, action: &ShortcutAction) -> String {
        let binding = self
            .bindings
            .get(&format!("{:?}", action))
            .cloned()
            .unwrap_or_else(|| action.default_key());

        // Convert to current platform's display format
        if cfg!(target_os = "macos") {
            binding.replace("Ctrl+", "Cmd+")
        } else {
            binding.replace("Cmd+", "Ctrl+")
        }
    }

    /// Check if the binding for this action is customized (different from default)
    pub fn is_customized(&self, action: &ShortcutAction) -> bool {
        let current_binding = self.get_binding(action);
        let default_binding = action.default_key();
        current_binding != default_binding
    }

    pub fn set_binding(&mut self, action: &ShortcutAction, key: String) {
        self.bindings.insert(format!("{:?}", action), key);
    }

    pub fn reset_to_default(&mut self) {
        self.bindings.clear();
        for (action, _) in ShortcutAction::all_actions() {
            self.bindings
                .insert(format!("{:?}", action), action.default_key());
        }
    }

    /// Check if the given key combination conflicts with any existing binding
    /// Returns the action that uses this key, or None if no conflict
    pub fn check_conflict(
        &self,
        key: &str,
        exclude_action: &ShortcutAction,
    ) -> Option<ShortcutAction> {
        let exclude_key = format!("{:?}", exclude_action);
        for (action_key, binding) in &self.bindings {
            if action_key != &exclude_key && binding == key {
                // Parse the action from the key string
                if let Some(action) = Self::parse_action_key(action_key) {
                    return Some(action);
                }
            }
        }
        None
    }

    /// Get all bindings that conflict with the given key
    pub fn get_conflicts(&self, key: &str) -> Vec<(ShortcutAction, String)> {
        let mut conflicts = Vec::new();
        for (action_key, binding) in &self.bindings {
            if binding == key {
                if let Some(action) = Self::parse_action_key(action_key) {
                    conflicts.push((action, binding.clone()));
                }
            }
        }
        conflicts
    }

    /// Parse action key string to ShortcutAction enum
    fn parse_action_key(key: &str) -> Option<ShortcutAction> {
        key.parse().ok()
    }
}

#[derive(Debug, Clone)]
pub struct ParsedShortcut {
    pub ctrl: bool,
    pub command: bool,
    pub alt: bool,
    pub shift: bool,
    pub key: String,
}

impl ParsedShortcut {
    pub fn parse(shortcut: &str) -> Option<Self> {
        let parts: Vec<&str> = shortcut.split('+').collect();
        let mut ctrl = false;
        let mut command = false;
        let mut alt = false;
        let mut shift = false;
        let mut key = String::new();

        for part in parts {
            let part = part.trim();
            match part.to_lowercase().as_str() {
                "ctrl" => ctrl = true,
                "cmd" | "command" => command = true,
                "alt" => alt = true,
                "shift" => shift = true,
                k if !k.is_empty() => key = k.to_uppercase(),
                _ => {}
            }
        }

        if key.is_empty() {
            None
        } else {
            Some(ParsedShortcut {
                ctrl,
                command,
                alt,
                shift,
                key,
            })
        }
    }

    /// Check if this shortcut matches the given egui modifiers and key
    /// This should be called from the app where egui is available
    pub fn key_matches(&self, key: &str) -> bool {
        let normalized_input = key.to_uppercase();
        // Handle numeric keys: egui reports "NUM1" but config stores "1"
        let normalized_config =
            if self.key.len() == 1 && self.key.chars().next().unwrap().is_ascii_digit() {
                format!("NUM{}", self.key)
            } else {
                self.key.clone()
            };
        normalized_config == normalized_input || self.key == normalized_input
    }

    pub fn is_mod_pressed(&self, is_macos: bool, ctrl: bool, command: bool) -> bool {
        if is_macos {
            (self.command && command) || (self.ctrl && ctrl)
        } else {
            self.ctrl && (ctrl || command)
        }
    }
}
