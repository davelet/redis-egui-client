mod app_window;
mod connected_preference;
mod connections;
mod open_connections;
mod user_config;

use crate::config::connected_preference::{ConnectedPreferences, ConnectionPreference};
use crate::connection::RedisConnectionConfig;
use crate::error::ConfigError;

use crate::config::app_window::ConfigOnWindowFace;
use crate::config::connections::ConfigOnConnections;
use crate::config::user_config::ConfigOfUser;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub window: ConfigOnWindowFace,
    pub settings: ConfigOfUser,
    pub connections: ConfigOnConnections,
    pub connected_preferences: ConnectedPreferences,
    // Dirty flags for delayed saving
    dirty_window: bool,
    dirty_settings: bool,
    dirty_preferences: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: ConfigOnWindowFace::default(),
            settings: ConfigOfUser::default(),
            connections: ConfigOnConnections::default(),
            connected_preferences: ConnectedPreferences::default(),
            dirty_window: false,
            dirty_settings: false,
            dirty_preferences: false,
        }
    }
}

impl Config {
    fn config_path() -> Result<PathBuf, ConfigError> {
        let home = dirs::home_dir().ok_or(ConfigError::HomeDirMissing)?;
        Ok(home.join(".config/rudist"))
    }

    pub fn window_config_file_path() -> Result<PathBuf, ConfigError> {
        Ok(Self::config_path()?.join("window.toml"))
    }

    pub fn connections_file_path() -> Result<PathBuf, ConfigError> {
        Ok(Self::config_path()?.join("connections.toml"))
    }

    pub fn user_settings_file_path() -> Result<PathBuf, ConfigError> {
        Ok(Self::config_path()?.join("config.toml"))
    }

    pub fn connected_preferences_file_path() -> Result<PathBuf, ConfigError> {
        Ok(Self::config_path()?.join("connected_preference.toml"))
    }

    pub fn open_connections_file_path() -> Result<PathBuf, ConfigError> {
        Ok(Self::config_path()?.join("open_connections.toml"))
    }

    pub fn load() -> Result<Self, ConfigError> {
        let path = Self::config_path()?;
        if !path.exists() {
            // If the config file doesn't exist, create a default configuration
            let default_config = Config::default();
            default_config.init()?;
            return Ok(default_config);
        }

        let settings = Self::load_user_settings()?;
        let connections = Self::load_connections()?;
        let window = Self::load_window_params()?;
        let connected_preferences = Self::load_connected_preferences()?;

        let config: Config = Config {
            window,
            settings,
            connections,
            connected_preferences,
            dirty_window: false,
            dirty_settings: false,
            dirty_preferences: false,
        };
        Ok(config)
    }

    pub fn init(&self) -> Result<(), ConfigError> {
        let config_path = Self::config_path()?;
        fs::create_dir_all(&config_path)
            .map_err(|e| ConfigError::CreateDirFailed(e.to_string()))?;

        self.save_user_settings()?;
        self.save_connections()?;
        self.save_window_config()?;
        self.save_connected_preferences()?;

        Ok(())
    }
}

// fn for connection
impl Config {
    pub fn load_connections() -> Result<ConfigOnConnections, ConfigError> {
        let path = Self::connections_file_path()?;

        if !path.exists() {
            return Ok(ConfigOnConnections::default());
        }

        let content =
            fs::read_to_string(&path).map_err(|e| ConfigError::ReadFailed(format!("{}", e)))?;
        toml::from_str(&content).map_err(|e| ConfigError::ParseFailed(format!("{}", e)))
    }

    pub fn save_connections(&self) -> Result<(), ConfigError> {
        if self.connections.is_empty() {
            return Ok(());
        }

        for connection in &self.connections {
            connection.check_param(&self.settings.language)?;
        }

        let toml = toml::to_string_pretty(&self.connections)
            .map_err(|e| ConfigError::WriteFailed(e.to_string()))?;
        fs::write(Self::connections_file_path()?, toml)
            .map_err(|e| ConfigError::WriteFailed(e.to_string()))?;

        Ok(())
    }

    pub fn add_connection(&mut self, conn: RedisConnectionConfig) -> Result<(), ConfigError> {
        // Check if a connection with the same name already exists
        if self
            .connections
            .connections
            .iter()
            .any(|c| c.name == conn.name)
        {
            return Err(ConfigError::ConnectionNameExists);
        }

        self.connections.push(conn);
        self.save_connections()
    }

    pub fn clear_connections() -> Result<(), ConfigError> {
        let c = Config::default();
        c.save_connections()
    }

    pub fn remove_connection(&mut self, name: &str) -> Result<(), ConfigError> {
        self.connections.retain(|c| c.name != name);
        self.save_connections()
    }

    pub fn update_single_connection(
        &mut self,
        old_name: &str,
        new_conn: RedisConnectionConfig,
    ) -> Result<(), ConfigError> {
        // First check if the connection exists
        if !self
            .connections
            .connections
            .iter()
            .any(|c| c.name == old_name)
        {
            return Err(ConfigError::ConnectionNotFound);
        }

        // Then check for name conflicts if the name is being changed
        if old_name != new_conn.name
            && self
                .connections
                .connections
                .iter()
                .any(|c| c.name == new_conn.name)
        {
            return Err(ConfigError::ConnectionNameExists);
        }

        // Now we can safely update the connection
        if let Some(conn) = self
            .connections
            .connections
            .iter_mut()
            .find(|c| c.name == old_name)
        {
            *conn = new_conn;
            self.save_connections()?;
        }

        Ok(())
    }
}

// fn for window
impl Config {
    pub fn load_window_params() -> Result<ConfigOnWindowFace, ConfigError> {
        let path = Self::window_config_file_path()?;

        if !path.exists() {
            return Ok(ConfigOnWindowFace::default());
        }

        let content =
            fs::read_to_string(&path).map_err(|e| ConfigError::ReadFailed(format!("{}", e)))?;
        toml::from_str(&content).map_err(|e| ConfigError::ParseFailed(format!("{}", e)))
    }

    pub fn save_window_config(&self) -> Result<(), ConfigError> {
        let toml = toml::to_string_pretty(&self.window)
            .map_err(|e| ConfigError::WriteFailed(e.to_string()))?;
        fs::write(Self::window_config_file_path()?, toml)
            .map_err(|e| ConfigError::WriteFailed(e.to_string()))?;

        Ok(())
    }
    pub fn update_window_size(&mut self, width: f32, height: f32) {
        self.window.width = width.max(100.0);
        self.window.height = height.max(100.0);
        self.dirty_window = true;
    }

    pub fn update_window_position(&mut self, x: f32, y: f32) {
        self.window.x = x;
        self.window.y = y;
        self.dirty_window = true;
    }

    pub fn update_maximized(&mut self, maximized: bool) {
        self.window.maximized = maximized;
        self.dirty_window = true;
    }

    pub fn reset_window_params() -> Result<(), ConfigError> {
        let c = Config::default();
        c.save_window_config()
    }
}

// fn for connected preferences
impl Config {
    pub fn load_connected_preferences() -> Result<ConnectedPreferences, ConfigError> {
        let path = Self::connected_preferences_file_path()?;

        if !path.exists() {
            return Ok(ConnectedPreferences::default());
        }

        let content =
            fs::read_to_string(&path).map_err(|e| ConfigError::ReadFailed(format!("{}", e)))?;
        toml::from_str(&content).map_err(|e| ConfigError::ParseFailed(format!("{}", e)))
    }

    pub fn save_connected_preferences(&self) -> Result<(), ConfigError> {
        let toml = toml::to_string_pretty(&self.connected_preferences)
            .map_err(|e| ConfigError::WriteFailed(e.to_string()))?;
        fs::write(Self::connected_preferences_file_path()?, toml)
            .map_err(|e| ConfigError::WriteFailed(e.to_string()))?;

        Ok(())
    }

    pub fn update_connection_preference(
        &mut self,
        connection_name: &str,
        preference: ConnectionPreference,
    ) -> Result<(), ConfigError> {
        self.connected_preferences
            .set_preference(connection_name.to_string(), preference);
        self.save_connected_preferences()
    }

    /// Update side panel width for a connection (marks as dirty, doesn't save immediately)
    pub fn update_side_panel_width_for_connection(&mut self, connection_name: &str, width: f32) {
        self.connected_preferences
            .update_side_panel_width(connection_name, width);
        self.dirty_preferences = true;
    }

    /// Get the side panel width for a connection
    pub fn get_side_panel_width_for_connection(&self, connection_name: &str) -> f32 {
        self.connected_preferences
            .get_preference(connection_name)
            .map(|pref| pref.side_panel_width)
            .unwrap_or(300.0)
    }

    /// Get the hash table column widths
    pub fn get_hash_column_widths(&self) -> (u32, u32) {
        (self.window.hash_field_width, self.window.hash_value_width)
    }

    pub fn update_hash_column_widths(&mut self, field_width: u32, value_width: u32) {
        self.window.hash_field_width = field_width.max(80).min(600);
        self.window.hash_value_width = value_width.max(100).min(1200);
        self.dirty_window = true;
    }

    pub fn update_db_for_connection(
        &mut self,
        connection_name: &str,
        db: u32,
    ) -> Result<(), ConfigError> {
        self.connected_preferences.update_db(connection_name, db);
        self.save_connected_preferences()
    }

    pub fn get_connection_preference(&self, connection_name: &str) -> Option<ConnectionPreference> {
        self.connected_preferences
            .get_preference(connection_name)
            .cloned()
    }
}

// fn for open connections
impl Config {
    pub fn add_open_connection(&mut self, name: &str) {
        self.window.open_connections.add_connection(name);
        self.dirty_window = true;
    }

    pub fn remove_open_connection(&mut self, name: &str) {
        self.window.open_connections.remove_connection(name);
        self.dirty_window = true;
    }

    pub fn clear_open_connections(&mut self) {
        self.window.open_connections.clear();
        self.dirty_window = true;
    }
}

// fn for user
impl Config {
    pub fn load_user_settings() -> Result<ConfigOfUser, ConfigError> {
        let path = Self::user_settings_file_path()?;

        if !path.exists() {
            return Ok(ConfigOfUser::default());
        }

        let content =
            fs::read_to_string(&path).map_err(|e| ConfigError::ReadFailed(format!("{}", e)))?;
        toml::from_str(&content).map_err(|e| ConfigError::ParseFailed(format!("{}", e)))
    }

    pub fn update_language(&mut self, language: &str) {
        self.settings.language = language.to_string();
        self.dirty_settings = true;
    }

    pub fn update_auto_connect(&mut self, auto_connect: bool) {
        self.settings.auto_connect = auto_connect;
        self.dirty_settings = true;
    }

    /// Mark window config as dirty (for when window.open_connections is modified directly)
    pub fn mark_window_dirty(&mut self) {
        self.dirty_window = true;
    }

    /// Save all dirty configurations (call on app exit)
    pub fn save_all_if_dirty(&mut self) -> Result<(), ConfigError> {
        if self.dirty_window {
            self.save_window_config()?;
            self.dirty_window = false;
        }
        if self.dirty_settings {
            self.save_user_settings()?;
            self.dirty_settings = false;
        }
        if self.dirty_preferences {
            self.save_connected_preferences()?;
            self.dirty_preferences = false;
        }
        Ok(())
    }

    pub fn save_user_settings(&self) -> Result<(), ConfigError> {
        let toml = toml::to_string_pretty(&self.settings)
            .map_err(|e| ConfigError::WriteFailed(e.to_string()))?;
        fs::write(Self::user_settings_file_path()?, toml)
            .map_err(|e| ConfigError::WriteFailed(e.to_string()))?;

        Ok(())
    }

    pub fn reset_user_settings() -> Result<(), ConfigError> {
        let c = Config::default();
        c.save_user_settings()
    }
}
