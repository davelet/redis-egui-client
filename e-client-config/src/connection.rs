use crate::constants::{CONNECTION_AUTH_LIMIT, CONNECTION_NAME_LIMIT, CONNECTION_URL_LIMIT};
use crate::error::ConfigError;
use e_client_bilingual::language::Language;
use e_client_bilingual::translations::{TranslationKey, tr};

use redis::{
    ConnectionAddr, ConnectionInfo, IntoConnectionInfo, RedisConnectionInfo, RedisError,
    RedisResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisConnectionConfig {
    pub name: String,
    pub url: String,
    pub port: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: Option<i64>,
    pub color: Option<String>,
    #[serde(default)]
    pub use_tls: bool,
}

impl IntoConnectionInfo for RedisConnectionConfig {
    fn into_connection_info(self) -> RedisResult<ConnectionInfo> {
        let port: u16 = self.port.parse().map_err(|e| {
            RedisError::from((
                redis::ErrorKind::InvalidClientConfig,
                "Invalid redis port",
                format!("{e}"),
            ))
        })?;
        let addr = if self.use_tls {
            ConnectionAddr::TcpTls {
                host: self.url,
                port,
                insecure: false,
                tls_params: None,
            }
        } else {
            ConnectionAddr::Tcp(self.url, port)
        };
        let info = addr.into_connection_info()?;
        let mut rds = RedisConnectionInfo::default();
        if let Some(db) = self.database {
            rds = rds.set_db(db);
        }
        if let Some(u) = self.username {
            rds = rds.set_username(u);
        }
        if let Some(p) = self.password {
            rds = rds.set_password(p);
        }
        Ok(info.set_redis_settings(rds))
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
            database: None,
            color,
            use_tls: false,
        }
    }

    pub fn new_with_tls(
        name: String,
        url: String,
        port: String,
        username: Option<String>,
        password: Option<String>,
        color: Option<String>,
        use_tls: bool,
    ) -> Self {
        Self {
            name,
            url,
            port,
            username,
            password,
            database: None,
            color,
            use_tls,
        }
    }

    /// Parse a Redis connection URL and create a RedisConnectionConfig.
    /// Supported formats:
    /// - redis://[<username>][:<password>@]<hostname>[:port][/[<db>]]
    /// - rediss://... (TLS connection)
    /// - redis+unix:///<path>[?db=<db>][&pass=<password>][&user=<username>]
    pub fn from_url(
        name: String,
        connection_url: &str,
        color: Option<String>,
    ) -> Result<Self, ConfigError> {
        let info: ConnectionInfo = connection_url
            .into_connection_info()
            .map_err(|_| ConfigError::InvalidUrl)?;

        let (host, port, use_tls) = match info.addr() {
            ConnectionAddr::Tcp(host, port) => (host.clone(), port.to_string(), false),
            ConnectionAddr::TcpTls { host, port, .. } => (host.clone(), port.to_string(), true),
            ConnectionAddr::Unix(_) => {
                return Err(ConfigError::UnsupportedConnectionType(
                    "Unix socket".to_string(),
                ));
            }
            _ => {
                return Err(ConfigError::UnsupportedConnectionType(
                    "Unknown".to_string(),
                ));
            }
        };

        let redis_info = info.redis_settings();
        Ok(Self {
            name,
            url: host,
            port,
            username: redis_info.username().map(|s| s.to_string()),
            password: redis_info.password().map(|s| s.to_string()),
            database: Some(redis_info.db()),
            color,
            use_tls,
        })
    }

    /// Extract the host name from a Redis connection URL.
    /// Returns None if the URL is invalid.
    pub fn extract_host_from_url(connection_url: &str) -> Option<String> {
        let info: ConnectionInfo = connection_url.into_connection_info().ok()?;
        match info.addr() {
            ConnectionAddr::Tcp(host, _) => Some(host.clone()),
            ConnectionAddr::TcpTls { host, .. } => Some(host.clone()),
            _ => None,
        }
    }

    pub(crate) fn check_param(&self, lang: &str) -> Result<(), ConfigError> {
        let lang = Language::file_name_to_lang(lang);
        if self.name.len() > CONNECTION_NAME_LIMIT {
            return Err(ConfigError::HugeParam([
                self.name.len().to_string(),
                tr(TranslationKey::ConnectionName, lang).to_string(),
                CONNECTION_NAME_LIMIT.to_string(),
            ]));
        }
        if self.url.len() > CONNECTION_URL_LIMIT {
            return Err(ConfigError::HugeParam([
                self.url.len().to_string(),
                tr(TranslationKey::ConnectionUrl, lang).to_string(),
                CONNECTION_URL_LIMIT.to_string(),
            ]));
        }
        let port: Result<u16, _> = self.port.parse();
        if port.is_err() {
            return Err(ConfigError::HugeParam([
                self.port.to_string(),
                tr(TranslationKey::ConnectionPort, lang).to_string(),
                u16::MAX.to_string(),
            ]));
        }
        if let Some(user) = self.username.as_ref()
            && user.len() > CONNECTION_AUTH_LIMIT {
                return Err(ConfigError::HugeParam([
                    user.len().to_string(),
                    tr(TranslationKey::ConnectionUsername, lang).to_string(),
                    CONNECTION_AUTH_LIMIT.to_string(),
                ]));
            }
        if let Some(pass) = self.password.as_ref()
            && pass.len() > CONNECTION_AUTH_LIMIT {
                return Err(ConfigError::HugeParam([
                    pass.len().to_string(),
                    tr(TranslationKey::ConnectionPassword, lang).to_string(),
                    CONNECTION_AUTH_LIMIT.to_string(),
                ]));
            }
        Ok(())
    }
}
