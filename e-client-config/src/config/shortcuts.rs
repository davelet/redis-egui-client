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
        self.key == key.to_uppercase()
    }

    pub fn is_mod_pressed(&self, is_macos: bool, ctrl: bool, command: bool) -> bool {
        if is_macos {
            (self.command && command) || (self.ctrl && ctrl)
        } else {
            self.ctrl && (ctrl || command)
        }
    }
}
