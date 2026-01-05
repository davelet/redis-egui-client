use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConnection {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub connections: Vec<RedisConnection>,
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
                connections: vec![
                    RedisConnection {
                        name: "本地 Redis".to_string(),
                        url: crate::constants::DEFAULT_REDIS_URL.to_string(),
                    }
                ],
            };
            default_config.save()?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| ConfigError::ReadFailed(format!("{}", e)))?;
        
        toml::from_str(&content)
            .map_err(|e| ConfigError::ParseFailed(format!("{}", e)))
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let config_dir = Self::config_path()?;
        
        // 确保配置目录存在
        fs::create_dir_all(&config_dir)
            .map_err(|e| ConfigError::CreateDirFailed(format!("{}", e)))?;

        let config_file = Self::config_file_path()?;
        let content = toml::to_string_pretty(self)
            .map_err(|e| ConfigError::WriteFailed(format!("{}", e)))?;

        fs::write(&config_file, content)
            .map_err(|e| ConfigError::WriteFailed(format!("{}", e)))?;

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

    pub fn update_connection(&mut self, old_name: &str, new_name: String, new_url: String) -> Result<(), ConfigError> {
        // 如果改名了,检查新名称是否已存在
        if old_name != new_name && self.connections.iter().any(|c| c.name == new_name) {
            return Err(ConfigError::ConnectionNameExists);
        }

        if let Some(conn) = self.connections.iter_mut().find(|c| c.name == old_name) {
            conn.name = new_name;
            conn.url = new_url;
            self.save()?;
            Ok(())
        } else {
            Err(ConfigError::ConnectionNotFound)
        }
    }
}
