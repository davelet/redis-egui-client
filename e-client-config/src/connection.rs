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
