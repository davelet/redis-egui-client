use crate::constants::{CONNECTION_AUTH_LIMIT, CONNECTION_NAME_LIMIT, CONNECTION_URL_LIMIT};
use crate::error::ConfigError;
use e_client_bilingual::language::Language;
use e_client_bilingual::translations::keys::{
    CONNECTION_NAME, CONNECTION_PASSWORD, CONNECTION_PORT, CONNECTION_URL, CONNECTION_USERNAME,
};
use e_client_bilingual::translations::tr;
use redis::{ConnectionInfo, IntoConnectionInfo, RedisResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisConnectionConfig {
    pub name: String,
    pub url: String,
    pub port: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub color: Option<String>,
}

impl IntoConnectionInfo for RedisConnectionConfig {
    fn into_connection_info(self) -> RedisResult<ConnectionInfo> {
        todo!()
    }
}

impl RedisConnectionConfig {
    pub fn new(
        name: String,
        url: String,
        port: String,
        username: Option<String>,
        password: Option<String>,
        color: Option<String>,
    ) -> Self {
        Self {
            name,
            url,
            port,
            username,
            password,
            color,
        }
    }

    pub(crate) fn check_param(&self, lang: &str) -> Result<(), ConfigError> {
        let lang = Language::file_name_to_lang(lang);
        if self.name.len() > CONNECTION_NAME_LIMIT {
            return Err(ConfigError::HugeParam([
                self.name.len().to_string(),
                tr(CONNECTION_NAME, lang).to_string(),
                CONNECTION_NAME_LIMIT.to_string(),
            ]));
        }
        if self.url.len() > CONNECTION_URL_LIMIT {
            return Err(ConfigError::HugeParam([
                self.url.len().to_string(),
                tr(CONNECTION_URL, lang).to_string(),
                CONNECTION_URL_LIMIT.to_string(),
            ]));
        }
        let port: Result<u16, _> = self.port.parse();
        if port.is_err() {
            return Err(ConfigError::HugeParam([
                self.port.to_string(),
                tr(CONNECTION_PORT, lang).to_string(),
                u16::MAX.to_string(),
            ]));
        }
        if let Some(user) = self.username.as_ref() {
            if user.len() > CONNECTION_AUTH_LIMIT {
                return Err(ConfigError::HugeParam([
                    user.len().to_string(),
                    tr(CONNECTION_USERNAME, lang).to_string(),
                    CONNECTION_AUTH_LIMIT.to_string(),
                ]));
            }
        }
        if let Some(pass) = self.password.as_ref() {
            if pass.len() > CONNECTION_AUTH_LIMIT {
                return Err(ConfigError::HugeParam([
                    pass.len().to_string(),
                    tr(CONNECTION_PASSWORD, lang).to_string(),
                    CONNECTION_AUTH_LIMIT.to_string(),
                ]));
            }
        }
        Ok(())
    }
}
