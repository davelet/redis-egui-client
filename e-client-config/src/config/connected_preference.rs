use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionPreference {
    #[serde(default = "default_side_panel_width")]
    pub side_panel_width: f32,
    #[serde(default)]
    pub db: u32,
}

fn default_side_panel_width() -> f32 {
    300.0
}

impl Default for ConnectionPreference {
    fn default() -> Self {
        ConnectionPreference {
            side_panel_width: default_side_panel_width(),
            db: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectedPreferences {
    #[serde(default)]
    pub preferences: HashMap<String, ConnectionPreference>,
}

impl Default for ConnectedPreferences {
    fn default() -> Self {
        ConnectedPreferences {
            preferences: HashMap::new(),
        }
    }
}

impl ConnectedPreferences {
    pub fn get_preference(&self, connection_name: &str) -> Option<&ConnectionPreference> {
        self.preferences.get(connection_name)
    }

    pub fn set_preference(&mut self, connection_name: String, preference: ConnectionPreference) {
        self.preferences.insert(connection_name, preference);
    }

    pub fn update_side_panel_width(&mut self, connection_name: &str, width: f32) {
        let preference = self
            .preferences
            .entry(connection_name.to_string())
            .or_insert_with(ConnectionPreference::default);
        preference.side_panel_width = width.max(250.0).min(800.0).round();
    }

    pub fn update_db(&mut self, connection_name: &str, db: u32) {
        let preference = self
            .preferences
            .entry(connection_name.to_string())
            .or_insert_with(ConnectionPreference::default);
        preference.db = db;
    }
}
