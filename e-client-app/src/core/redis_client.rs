use e_client_config::connection::RedisConnectionConfig;
use redis::{AsyncCommands, Client, RedisError, aio::ConnectionManager};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::instrument;

#[derive(Clone)]
#[derive(Debug)] // with instrument
pub struct RedisClient {
    manager: Arc<RwLock<Option<ConnectionManager>>>,
}

impl RedisClient {
    pub fn new() -> Self {
        Self {
            manager: Arc::new(RwLock::new(None)),
        }
    }

    #[instrument]
    pub async fn connect(&self, redis: RedisConnectionConfig) -> Result<(), RedisError> {
        let client = Client::open(redis)?;
        let manager = ConnectionManager::new(client).await?;
        *self.manager.write().await = Some(manager);
        Ok(())
    }

    pub async fn is_connected(&self) -> bool {
        self.manager.read().await.is_some()
    }

    pub async fn disconnect(&self) {
        *self.manager.write().await = None;
    }

    pub fn disconnect_sync(&self) {
        if let Ok(mut manager) = self.manager.try_write() {
            *manager = None;
        }
    }

    pub async fn execute_command(&self, cmd: &str) -> Result<String, RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            if parts.is_empty() {
                return Ok("Empty command".to_string());
            }

            let result: redis::Value = redis::cmd(parts[0])
                .arg(&parts[1..])
                .query_async(conn)
                .await?;

            Ok(format_redis_value(&result))
        } else {
            Err(RedisError::from((redis::ErrorKind::Io, "Not connected")))
        }
    }

    pub async fn get_databases(&self) -> Result<Vec<u32>, RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            match redis::cmd("CONFIG")
                .arg("GET")
                .arg("databases")
                .query_async::<String>(conn)
                .await
            {
                Ok(config) => {
                    let db_count: u32 = config
                        .split_whitespace()
                        .last()
                        .unwrap_or("16")
                        .parse()
                        .unwrap_or(16);
                    Ok((0..db_count).collect())
                }
                Err(_) => Ok((0..16).collect()), // Default to 16 databases if CONFIG fails
            }
        } else {
            Ok(vec![])
        }
    }

    pub async fn select_db(&self, db: u32) -> Result<(), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            redis::cmd("SELECT").arg(db).query_async(conn).await
        } else {
            Err(RedisError::from((redis::ErrorKind::Io, "Not connected")))
        }
    }

    pub async fn scan_keys(
        &self,
        cursor: u64,
        pattern: &str,
        count: usize,
    ) -> Result<(u64, Vec<String>), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            let (new_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(pattern)
                .arg("COUNT")
                .arg(count)
                .query_async(conn)
                .await?;
            Ok((new_cursor, keys))
        } else {
            Ok((0, vec![]))
        }
    }

    pub async fn get_key_type(&self, key: &str) -> Result<String, RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            redis::cmd("TYPE").arg(key).query_async(conn).await
        } else {
            Ok("none".to_string())
        }
    }

    pub async fn get_value(&self, key: &str) -> Result<ValueData, RedisError> {
        let key_type = self.get_key_type(key).await?;
        let mut manager = self.manager.write().await;

        if let Some(conn) = manager.as_mut() {
            match key_type.as_str() {
                "string" => {
                    let val: String = conn.get(key).await?;
                    Ok(ValueData::String(val))
                }
                "list" => {
                    let len: usize = conn.llen(key).await?;
                    Ok(ValueData::List { len, items: vec![] })
                }
                "set" => {
                    let len: usize = conn.scard(key).await?;
                    Ok(ValueData::Set { len, items: vec![] })
                }
                "zset" => {
                    let len: usize = conn.zcard(key).await?;
                    Ok(ValueData::ZSet { len, items: vec![] })
                }
                "hash" => {
                    let len: usize = conn.hlen(key).await?;
                    Ok(ValueData::Hash {
                        len,
                        fields: vec![],
                    })
                }
                _ => Ok(ValueData::None),
            }
        } else {
            Ok(ValueData::None)
        }
    }

    pub async fn get_list_range(
        &self,
        key: &str,
        start: isize,
        stop: isize,
    ) -> Result<Vec<String>, RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            conn.lrange(key, start, stop).await
        } else {
            Ok(vec![])
        }
    }

    pub async fn get_hash_fields(
        &self,
        key: &str,
        cursor: u64,
        count: usize,
    ) -> Result<(u64, Vec<(String, String)>), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            let (new_cursor, items): (u64, Vec<String>) = redis::cmd("HSCAN")
                .arg(key)
                .arg(cursor)
                .arg("COUNT")
                .arg(count)
                .query_async(conn)
                .await?;

            let mut pairs = vec![];
            for i in (0..items.len()).step_by(2) {
                if i + 1 < items.len() {
                    pairs.push((items[i].clone(), items[i + 1].clone()));
                }
            }
            Ok((new_cursor, pairs))
        } else {
            Ok((0, vec![]))
        }
    }

    pub async fn get_set_members(
        &self,
        key: &str,
        cursor: u64,
        count: usize,
    ) -> Result<(u64, Vec<String>), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            let (new_cursor, items): (u64, Vec<String>) = redis::cmd("SSCAN")
                .arg(key)
                .arg(cursor)
                .arg("COUNT")
                .arg(count)
                .query_async(conn)
                .await?;
            Ok((new_cursor, items))
        } else {
            Ok((0, vec![]))
        }
    }

    pub async fn get_zset_range(
        &self,
        key: &str,
        start: isize,
        stop: isize,
    ) -> Result<Vec<(String, f64)>, RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            let items: Vec<(String, f64)> = redis::cmd("ZRANGE")
                .arg(key)
                .arg(start)
                .arg(stop)
                .arg("WITHSCORES")
                .query_async(conn)
                .await?;
            Ok(items)
        } else {
            Ok(vec![])
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValueData {
    String(String),
    List {
        len: usize,
        items: Vec<String>,
    },
    Set {
        len: usize,
        items: Vec<String>,
    },
    ZSet {
        len: usize,
        items: Vec<(String, f64)>,
    },
    Hash {
        len: usize,
        fields: Vec<(String, String)>,
    },
    None,
}

fn format_redis_value(value: &redis::Value) -> String {
    match value {
        redis::Value::Nil => "(nil)".to_string(),
        redis::Value::Int(i) => i.to_string(),
        redis::Value::BulkString(bytes) => String::from_utf8_lossy(bytes).to_string(),
        redis::Value::Array(values) => {
            let formatted: Vec<String> = values.iter().map(format_redis_value).collect();
            formatted.join("\n")
        }
        redis::Value::SimpleString(s) => s.clone(),
        redis::Value::Okay => "OK".to_string(),
        _ => format!("{:?}", value),
    }
}
