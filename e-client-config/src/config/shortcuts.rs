use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum ShortcutAction {
    #[serde(rename = "new_tab")]
    NewTab,
    #[serde(rename = "close_tab")]
    CloseTab,
    #[serde(rename = "refresh_key")]
    RefreshKey,
    #[serde(rename = "focus_filter")]
    FocusFilter,
    #[serde(rename = "close_settings")]
    CloseSettings,
    #[serde(rename = "open_settings")]
    OpenSettings,
    #[serde(rename = "toggle_command_line")]
    ToggleCommandLine,
    #[serde(rename = "close_command_line")]
    CloseCommandLine,
    #[serde(rename = "switch_to_tab_1")]
    SwitchToTab1,
    #[serde(rename = "switch_to_tab_2")]
    SwitchToTab2,
    #[serde(rename = "switch_to_tab_3")]
    SwitchToTab3,
    #[serde(rename = "switch_to_tab_4")]
    SwitchToTab4,
    #[serde(rename = "switch_to_tab_5")]
    SwitchToTab5,
    #[serde(rename = "switch_to_tab_6")]
    SwitchToTab6,
    #[serde(rename = "switch_to_tab_7")]
    SwitchToTab7,
    #[serde(rename = "switch_to_tab_8")]
    SwitchToTab8,
    #[serde(rename = "switch_to_tab_9")]
    SwitchToTab9,
    #[serde(rename = "switch_to_last_tab")]
    SwitchToLastTab,
    #[serde(rename = "open_all_tabs_dropdown")]
    OpenAllTabsDropdown,
    #[serde(rename = "remove_duplicate_and_invalid_tabs")]
    RemoveDuplicateAndInvalidTabs,
}

impl ShortcutAction {
    /// Returns all actions with their translation keys
    pub fn all_actions() -> Vec<(ShortcutAction, &'static str)> {
        vec![
            (ShortcutAction::OpenSettings, "shortcut_open_settings"),
            (ShortcutAction::NewTab, "shortcut_new_tab"),
            (ShortcutAction::CloseTab, "shortcut_close_tab"),
            (ShortcutAction::RefreshKey, "shortcut_refresh_key"),
            (ShortcutAction::FocusFilter, "shortcut_focus_filter"),
            (ShortcutAction::CloseSettings, "shortcut_close_settings"),
            (
                ShortcutAction::ToggleCommandLine,
                "shortcut_toggle_command_line",
            ),
            (
                ShortcutAction::CloseCommandLine,
                "shortcut_close_command_line",
            ),
            (ShortcutAction::SwitchToTab1, "shortcut_switch_to_tab_1"),
            (ShortcutAction::SwitchToTab2, "shortcut_switch_to_tab_2"),
            (ShortcutAction::SwitchToTab3, "shortcut_switch_to_tab_3"),
            (ShortcutAction::SwitchToTab4, "shortcut_switch_to_tab_4"),
            (ShortcutAction::SwitchToTab5, "shortcut_switch_to_tab_5"),
            (ShortcutAction::SwitchToTab6, "shortcut_switch_to_tab_6"),
            (ShortcutAction::SwitchToTab7, "shortcut_switch_to_tab_7"),
            (ShortcutAction::SwitchToTab8, "shortcut_switch_to_tab_8"),
            (ShortcutAction::SwitchToTab9, "shortcut_switch_to_tab_9"),
            (
                ShortcutAction::SwitchToLastTab,
                "shortcut_switch_to_last_tab",
            ),
            (
                ShortcutAction::OpenAllTabsDropdown,
                "shortcut_open_all_tabs_dropdown",
            ),
            (
                ShortcutAction::RemoveDuplicateAndInvalidTabs,
                "shortcut_remove_duplicate_and_invalid_tabs",
            ),
        ]
    }

    pub fn default_key(&self) -> String {
        // Use Cmd on macOS, Ctrl on other platforms
        let mod_key = if cfg!(target_os = "macos") {
            "Cmd"
        } else {
            "Ctrl"
        };
        match self {
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
            ShortcutAction::OpenAllTabsDropdown => format!("{}+Shift+A", mod_key),
            ShortcutAction::RemoveDuplicateAndInvalidTabs => format!("{}+Shift+D", mod_key),
        }
    }

    /// Returns the translation key for this action
    pub fn translation_key(&self) -> &'static str {
        match self {
            ShortcutAction::NewTab => "shortcut_new_tab",
            ShortcutAction::CloseTab => "shortcut_close_tab",
            ShortcutAction::RefreshKey => "shortcut_refresh_key",
            ShortcutAction::FocusFilter => "shortcut_focus_filter",
            ShortcutAction::CloseSettings => "shortcut_close_settings",
            ShortcutAction::OpenSettings => "shortcut_open_settings",
            ShortcutAction::ToggleCommandLine => "shortcut_toggle_command_line",
            ShortcutAction::CloseCommandLine => "shortcut_close_command_line",
            ShortcutAction::SwitchToTab1 => "shortcut_switch_to_tab_1",
            ShortcutAction::SwitchToTab2 => "shortcut_switch_to_tab_2",
            ShortcutAction::SwitchToTab3 => "shortcut_switch_to_tab_3",
            ShortcutAction::SwitchToTab4 => "shortcut_switch_to_tab_4",
            ShortcutAction::SwitchToTab5 => "shortcut_switch_to_tab_5",
            ShortcutAction::SwitchToTab6 => "shortcut_switch_to_tab_6",
            ShortcutAction::SwitchToTab7 => "shortcut_switch_to_tab_7",
            ShortcutAction::SwitchToTab8 => "shortcut_switch_to_tab_8",
            ShortcutAction::SwitchToTab9 => "shortcut_switch_to_tab_9",
            ShortcutAction::SwitchToLastTab => "shortcut_switch_to_last_tab",
            ShortcutAction::OpenAllTabsDropdown => "shortcut_open_all_tabs_dropdown",
            ShortcutAction::RemoveDuplicateAndInvalidTabs => {
                "shortcut_remove_duplicate_and_invalid_tabs"
            }
        }
    }

    /// Returns true if this shortcut is not customizable
    pub fn is_non_editable(&self) -> bool {
        matches!(
            self,
            ShortcutAction::OpenSettings
                | ShortcutAction::CloseSettings
                | ShortcutAction::CloseCommandLine
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
        self.bindings
            .get(&format!("{:?}", action))
            .cloned()
            .unwrap_or_else(|| action.default_key())
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
        match key {
            "NewTab" => Some(ShortcutAction::NewTab),
            "CloseTab" => Some(ShortcutAction::CloseTab),
            "RefreshKey" => Some(ShortcutAction::RefreshKey),
            "FocusFilter" => Some(ShortcutAction::FocusFilter),
            "CloseSettings" => Some(ShortcutAction::CloseSettings),
            "OpenSettings" => Some(ShortcutAction::OpenSettings),
            "ToggleCommandLine" => Some(ShortcutAction::ToggleCommandLine),
            "CloseCommandLine" => Some(ShortcutAction::CloseCommandLine),
            "SwitchToTab1" => Some(ShortcutAction::SwitchToTab1),
            "SwitchToTab2" => Some(ShortcutAction::SwitchToTab2),
            "SwitchToTab3" => Some(ShortcutAction::SwitchToTab3),
            "SwitchToTab4" => Some(ShortcutAction::SwitchToTab4),
            "SwitchToTab5" => Some(ShortcutAction::SwitchToTab5),
            "SwitchToTab6" => Some(ShortcutAction::SwitchToTab6),
            "SwitchToTab7" => Some(ShortcutAction::SwitchToTab7),
            "SwitchToTab8" => Some(ShortcutAction::SwitchToTab8),
            "SwitchToTab9" => Some(ShortcutAction::SwitchToTab9),
            "SwitchToLastTab" => Some(ShortcutAction::SwitchToLastTab),
            "OpenAllTabsDropdown" => Some(ShortcutAction::OpenAllTabsDropdown),
            "RemoveDuplicateAndInvalidTabs" => Some(ShortcutAction::RemoveDuplicateAndInvalidTabs),
            _ => None,
        }
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
