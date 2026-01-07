use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisConnection {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    #[serde(default = "default_maximized")]
    pub maximized: bool,
}

fn default_maximized() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub connections: Vec<RedisConnection>,
    #[serde(default)]
    pub window: WindowConfig,
    #[serde(default)]
    pub language: String,
}

#[derive(Debug)]
pub enum ConfigError {
    HomeDirMissing,
    ReadFailed(String),
    ParseFailed(String),
    CreateDirFailed(String),
    WriteFailed(String),
    ConnectionNameExists,
    ConnectionNotFound,
}

impl ConfigError {
    pub fn to_message(&self, lang: crate::app_state::Language) -> String {
        use crate::translations::{tr, tr_fmt};
        match self {
            ConfigError::HomeDirMissing => tr("config_home_dir_missing", lang).to_string(),
            ConfigError::ReadFailed(e) => tr_fmt("config_read_failed", lang, &[e]),
            ConfigError::ParseFailed(e) => tr_fmt("config_parse_failed", lang, &[e]),
            ConfigError::CreateDirFailed(e) => tr_fmt("config_create_dir_failed", lang, &[e]),
            ConfigError::WriteFailed(e) => tr_fmt("config_write_failed", lang, &[e]),
            ConfigError::ConnectionNameExists => tr("connection_name_exists", lang).to_string(),
            ConfigError::ConnectionNotFound => tr("connection_not_found", lang).to_string(),
        }
    }
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
            // 如果配置文件不存在,创建默认配置
            let default_config = Config {
                connections: vec![RedisConnection {
                    name: "本地 Redis".to_string(),
                    url: crate::constants::DEFAULT_REDIS_URL.to_string(),
                }],
                window: WindowConfig {
                    width: 1024.0,
                    height: 768.0,
                    x: 100.0,
                    y: 100.0,
                    maximized: false,
                },
                language: "zh".to_string(),
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
        std::fs::create_dir_all(&config_path)
            .map_err(|e| ConfigError::CreateDirFailed(e.to_string()))?;

        let toml =
            toml::to_string_pretty(self).map_err(|e| ConfigError::WriteFailed(e.to_string()))?;
        std::fs::write(Self::config_file_path()?, toml)
            .map_err(|e| ConfigError::WriteFailed(e.to_string()))?;

        Ok(())
    }

    pub fn add_connection(&mut self, name: String, url: String) -> Result<(), ConfigError> {
        // 检查是否已存在同名连接
        if self.connections.iter().any(|c| c.name == name) {
            return Err(ConfigError::ConnectionNameExists);
        }

        self.connections.push(RedisConnection { name, url });
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
