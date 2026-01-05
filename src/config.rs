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

impl Config {
    pub fn config_path() -> Result<PathBuf, String> {
        let home = dirs::home_dir().ok_or("无法获取用户目录")?;
        Ok(home.join(".config/rudist"))
    }

    pub fn config_file_path() -> Result<PathBuf, String> {
        Ok(Self::config_path()?.join("config.toml"))
    }

    pub fn load() -> Result<Self, String> {
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
            .map_err(|e| format!("读取配置文件失败: {}", e))?;
        
        toml::from_str(&content)
            .map_err(|e| format!("解析配置文件失败: {}", e))
    }

    pub fn save(&self) -> Result<(), String> {
        let config_dir = Self::config_path()?;
        
        // 确保配置目录存在
        fs::create_dir_all(&config_dir)
            .map_err(|e| format!("创建配置目录失败: {}", e))?;

        let config_file = Self::config_file_path()?;
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("序列化配置失败: {}", e))?;

        fs::write(&config_file, content)
            .map_err(|e| format!("写入配置文件失败: {}", e))?;

        Ok(())
    }

    pub fn add_connection(&mut self, name: String, url: String) -> Result<(), String> {
        // 检查是否已存在同名连接
        if self.connections.iter().any(|c| c.name == name) {
            return Err("连接名称已存在".to_string());
        }

        self.connections.push(RedisConnection { name, url });
        self.save()?;
        Ok(())
    }

    pub fn remove_connection(&mut self, name: &str) -> Result<(), String> {
        self.connections.retain(|c| c.name != name);
        self.save()?;
        Ok(())
    }

    pub fn update_connection(&mut self, old_name: &str, new_name: String, new_url: String) -> Result<(), String> {
        // 如果改名了,检查新名称是否已存在
        if old_name != new_name && self.connections.iter().any(|c| c.name == new_name) {
            return Err("连接名称已存在".to_string());
        }

        if let Some(conn) = self.connections.iter_mut().find(|c| c.name == old_name) {
            conn.name = new_name;
            conn.url = new_url;
            self.save()?;
            Ok(())
        } else {
            Err("连接不存在".to_string())
        }
    }
}
