use e_client_basics::constants::{CONNECTION_RETRY_COUNT, DEFAULT_DATABASE_COUNT};
use e_client_config::connection::RedisConnectionConfig;
use redis::aio::ConnectionManagerConfig;
use redis::{AsyncCommands, Client, RedisError, aio::ConnectionManager};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::instrument;

#[derive(Clone, Debug)] // with instrument
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
        let config =
            ConnectionManagerConfig::default().set_number_of_retries(CONNECTION_RETRY_COUNT);
        let manager = ConnectionManager::new_with_config(client, config).await?;
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
                        .unwrap_or(&DEFAULT_DATABASE_COUNT.to_string())
                        .parse()
                        .unwrap_or(DEFAULT_DATABASE_COUNT);
                    Ok((0..db_count).collect())
                }
                Err(_) => Ok((0..DEFAULT_DATABASE_COUNT).collect()),
                // Default to DEFAULT_DATABASE_COUNT databases if CONFIG fails
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
            // Use Vec<Vec<u8>> to handle both UTF-8 and binary keys
            let (new_cursor, keys_bytes): (u64, Vec<Vec<u8>>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(pattern)
                .arg("COUNT")
                .arg(count)
                .query_async(conn)
                .await?;

            // Convert bytes to strings, skipping any that can't be converted to UTF-8
            let mut string_keys = Vec::new();
            for key_bytes in keys_bytes {
                if let Ok(key_str) = String::from_utf8(key_bytes) {
                    string_keys.push(key_str);
                }
                // Skip keys that can't be converted to UTF-8
            }

            Ok((new_cursor, string_keys))
        } else {
            Ok((0, vec![]))
        }
    }

    pub async fn get_db_size(&self) -> Result<usize, RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            let size: usize = redis::cmd("DBSIZE").query_async(conn).await?;
            Ok(size)
        } else {
            Ok(0)
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

    pub async fn get_ttl(&self, key: &str) -> Result<i64, RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            redis::cmd("TTL").arg(key).query_async(conn).await
        } else {
            Ok(-2)
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
                        loaded_values: std::collections::HashMap::new(),
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
        _cursor: u64,
        _count: usize,
    ) -> Result<(u64, Vec<String>), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            let fields: Vec<String> = redis::cmd("HKEYS").arg(key).query_async(conn).await?;
            Ok((0, fields))
        } else {
            Ok((0, vec![]))
        }
    }

    pub async fn get_hash_field_value(
        &self,
        key: &str,
        field: &str,
    ) -> Result<Option<String>, RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            let value: Option<String> = conn.hget(key, field).await?;
            Ok(value)
        } else {
            Ok(None)
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

// Write operations
impl RedisClient {
    pub async fn rename_key_nx(&self, old_key: &str, new_key: &str) -> Result<bool, RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            let result: i32 = redis::cmd("RENAMENX")
                .arg(old_key)
                .arg(new_key)
                .query_async(conn)
                .await?;
            Ok(result == 1)
        } else {
            Ok(false)
        }
    }

    pub async fn set_ttl(&self, key: &str, ttl: i64) -> Result<(), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            if ttl < 0 {
                redis::cmd("PERSIST")
                    .arg(key)
                    .query_async::<String>(conn)
                    .await?;
            } else {
                redis::cmd("EXPIRE")
                    .arg(key)
                    .arg(ttl)
                    .query_async::<i32>(conn)
                    .await?;
            }
            Ok(())
        } else {
            Ok(())
        }
    }

    pub async fn del_key(&self, key: &str) -> Result<(), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            redis::cmd("DEL").arg(key).query_async::<i32>(conn).await?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub async fn set_string(&self, key: &str, value: &str) -> Result<(), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            redis::cmd("SET")
                .arg(key)
                .arg(value)
                .query_async::<String>(conn)
                .await?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub async fn hset(&self, key: &str, field: &str, value: &str) -> Result<(), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            redis::cmd("HSET")
                .arg(key)
                .arg(field)
                .arg(value)
                .query_async::<i32>(conn)
                .await?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub async fn hdel(&self, key: &str, field: &str) -> Result<(), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            redis::cmd("HDEL")
                .arg(key)
                .arg(field)
                .query_async::<i32>(conn)
                .await?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub async fn lset(&self, key: &str, index: i64, value: &str) -> Result<(), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            redis::cmd("LSET")
                .arg(key)
                .arg(index)
                .arg(value)
                .query_async::<String>(conn)
                .await?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub async fn sadd(&self, key: &str, member: &str) -> Result<(), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            redis::cmd("SADD")
                .arg(key)
                .arg(member)
                .query_async::<i32>(conn)
                .await?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub async fn srem(&self, key: &str, member: &str) -> Result<(), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            redis::cmd("SREM")
                .arg(key)
                .arg(member)
                .query_async::<i32>(conn)
                .await?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub async fn zadd(&self, key: &str, score: f64, member: &str) -> Result<(), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            redis::cmd("ZADD")
                .arg(key)
                .arg(score)
                .arg(member)
                .query_async::<i32>(conn)
                .await?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub async fn rpush(&self, key: &str, value: &str) -> Result<(), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            redis::cmd("RPUSH")
                .arg(key)
                .arg(value)
                .query_async::<i32>(conn)
                .await?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub async fn zrem(&self, key: &str, member: &str) -> Result<(), RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            redis::cmd("ZREM")
                .arg(key)
                .arg(member)
                .query_async::<i32>(conn)
                .await?;
            Ok(())
        } else {
            Ok(())
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
        fields: Vec<String>,
        loaded_values: std::collections::HashMap<String, String>,
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
