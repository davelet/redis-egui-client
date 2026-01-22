mod app_window;
mod connections;
mod user_config;

use crate::connection::RedisConnectionConfig;
use crate::error::ConfigError;

use crate::config::app_window::ConfigOnWindowFace;
use crate::config::connections::ConfigOnConnections;
use crate::config::user_config::ConfigOfUser;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub window: ConfigOnWindowFace,
    pub settings: ConfigOfUser,
    pub connections: ConfigOnConnections,
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

        let config: Config = Config {
            window,
            settings,
            connections,
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
    pub fn update_window_size(&mut self, width: f32, height: f32) -> Result<(), ConfigError> {
        self.window.width = width.max(100.0);
        self.window.height = height.max(100.0);
        self.save_window_config()
    }

    pub fn update_window_position(&mut self, x: f32, y: f32) -> Result<(), ConfigError> {
        self.window.x = x;
        self.window.y = y;
        self.save_window_config()
    }

    pub fn update_maximized(&mut self, maximized: bool) -> Result<(), ConfigError> {
        self.window.maximized = maximized;
        self.save_window_config()
    }

    pub fn reset_window_params() -> Result<(), ConfigError> {
        let c = Config::default();
        c.save_window_config()
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

    pub fn update_language(&mut self, language: &str) -> Result<(), ConfigError> {
        self.settings.language = language.to_string();
        self.save_user_settings()
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
