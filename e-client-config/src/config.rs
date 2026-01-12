use crate::connection::RedisConnection;
use crate::error::ConfigError;
use crate::window::WindowConfig;

use crate::constants::ZH_IN_FILE;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub connections: Vec<RedisConnection>,
    #[serde(default)]
    pub window: WindowConfig,
    #[serde(default)]
    pub language: String,
}

impl Config {
    pub fn config_path() -> Result<PathBuf, ConfigError> {
        let home = dirs::home_dir().ok_or(ConfigError::HomeDirMissing)?;
        Ok(home.join(".config/rudist"))
    }

    pub fn config_file_path() -> Result<PathBuf, ConfigError> {
        Ok(Self::config_path()?.join("config.toml"))
    }

    pub fn load() -> Result<Self, ConfigError> {
        let path = Self::config_file_path()?;

        if !path.exists() {
            // If the config file doesn't exist, create a default configuration
            let default_config = Config {
                connections: vec![],
                window: WindowConfig {
                    width: 1024.0,
                    height: 768.0,
                    x: 100.0,
                    y: 100.0,
                    maximized: false,
                },
                language: ZH_IN_FILE.to_string(),
            };
            default_config.save()?;
            return Ok(default_config);
        }

        let content =
            fs::read_to_string(&path).map_err(|e| ConfigError::ReadFailed(format!("{}", e)))?;

        toml::from_str(&content).map_err(|e| ConfigError::ParseFailed(format!("{}", e)))
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let config_path = Self::config_path()?;
        fs::create_dir_all(&config_path)
            .map_err(|e| ConfigError::CreateDirFailed(e.to_string()))?;

        let toml =
            toml::to_string_pretty(self).map_err(|e| ConfigError::WriteFailed(e.to_string()))?;
        fs::write(Self::config_file_path()?, toml)
            .map_err(|e| ConfigError::WriteFailed(e.to_string()))?;

        Ok(())
    }

    pub fn add_connection(&mut self, conn: RedisConnection) -> Result<(), ConfigError> {
        // Check if a connection with the same name already exists
        if self.connections.iter().any(|c| c.name == conn.name) {
            return Err(ConfigError::ConnectionNameExists);
        }

        self.connections.push(conn);
        self.save()?;
        Ok(())
    }

    pub fn remove_connection(&mut self, name: &str) -> Result<(), ConfigError> {
        self.connections.retain(|c| c.name != name);
        self.save()?;
        Ok(())
    }

    pub fn update_connection(
        &mut self,
        old_name: &str,
        new_name: String,
        new_url: String,
    ) -> Result<(), ConfigError> {
        // First check if the connection exists
        if !self.connections.iter().any(|c| c.name == old_name) {
            return Err(ConfigError::ConnectionNotFound);
        }

        // Then check for name conflicts if the name is being changed
        if old_name != new_name && self.connections.iter().any(|c| c.name == new_name) {
            return Err(ConfigError::ConnectionNameExists);
        }

        // Now we can safely update the connection
        if let Some(conn) = self.connections.iter_mut().find(|c| c.name == old_name) {
            conn.name = new_name;
            conn.url = new_url;
            self.save()?;
        }

        Ok(())
    }

    pub fn update_window_size(&mut self, width: f32, height: f32) -> Result<(), ConfigError> {
        self.window.width = width.max(100.0);
        self.window.height = height.max(100.0);
        self.save()
    }

    pub fn update_window_position(&mut self, x: f32, y: f32) -> Result<(), ConfigError> {
        self.window.x = x;
        self.window.y = y;
        self.save()
    }

    pub fn update_maximized(&mut self, maximized: bool) -> Result<(), ConfigError> {
        self.window.maximized = maximized;
        self.save()
    }

    pub fn update_language(&mut self, language: &str) -> Result<(), ConfigError> {
        self.language = language.to_string();
        self.save()
    }
}
