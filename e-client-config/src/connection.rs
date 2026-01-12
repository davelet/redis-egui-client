use crate::constants::{DEFAULT_REDIS_NAME, DEFAULT_REDIS_PORT, DEFAULT_REDIS_URL};
use redis::{ConnectionInfo, IntoConnectionInfo, RedisResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisConnection {
    pub name: String,
    pub url: String,
    pub port: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl IntoConnectionInfo for RedisConnection {
    fn into_connection_info(self) -> RedisResult<ConnectionInfo> {
        todo!()
    }
}

impl Default for RedisConnection {
    fn default() -> Self {
        Self {
            name: DEFAULT_REDIS_NAME.to_string(),
            url: DEFAULT_REDIS_URL.to_string(),
            port: DEFAULT_REDIS_PORT.to_string(),
            username: None,
            password: None,
        }
    }
}

impl RedisConnection {
    pub fn new(
        name: String,
        url: String,
        port: String,
        username: Option<String>,
        password: Option<String>,
    ) -> Self {
        Self {
            name,
            url,
            port,
            username,
            password,
        }
    }
}
