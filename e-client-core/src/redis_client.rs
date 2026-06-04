use e_client_basics::constants::{CONNECTION_RETRY_COUNT, DEFAULT_DATABASE_COUNT};
use e_client_config::connection::RedisConnectionConfig;
use redis::aio::ConnectionManagerConfig;
use redis::{AsyncCommands, Client, RedisError, aio::ConnectionManager};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, instrument, warn};

const HASH_SCAN_MAX_FIELDS: usize = 100_000;
const HASH_SCAN_COUNT_NOVALUES: usize = 500;
const HASH_VALUE_HEX_PREVIEW_BYTES: usize = 1024;

#[derive(Clone, Debug)]
pub struct RedisClient {
    manager: Arc<RwLock<Option<ConnectionManager>>>,
    server_version: Arc<RwLock<Option<(u32, u32, u32)>>>,
}

impl RedisClient {
    pub fn new() -> Self {
        Self {
            manager: Arc::new(RwLock::new(None)),
            server_version: Arc::new(RwLock::new(None)),
        }
    }

    #[instrument(skip(redis))]
    pub async fn connect(&self, redis: RedisConnectionConfig) -> Result<(), RedisError> {
        info!(url = %redis.url, port = %redis.port, "Connecting to Redis");
        let client = Client::open(redis)?;
        let config =
            ConnectionManagerConfig::default().set_number_of_retries(CONNECTION_RETRY_COUNT);
        match ConnectionManager::new_with_config(client, config).await {
            Ok(manager) => {
                *self.manager.write().await = Some(manager);
                info!("Redis connection established");
                Ok(())
            }
            Err(e) => {
                error!(error = %e, "Failed to connect to Redis");
                Err(e)
            }
        }
    }

    pub async fn is_connected(&self) -> bool {
        self.manager.read().await.is_some()
    }

    pub async fn disconnect(&self) {
        info!("Disconnecting from Redis");
        *self.manager.write().await = None;
        *self.server_version.write().await = None;
    }

    pub fn disconnect_sync(&self) {
        info!("Disconnecting from Redis (sync)");
        if let Ok(mut manager) = self.manager.try_write() {
            *manager = None;
        }
        if let Ok(mut v) = self.server_version.try_write() {
            *v = None;
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
                .query_async::<Vec<String>>(conn)
                .await
            {
                Ok(config) => {
                    let db_count: u32 = if config.len() >= 2 {
                        config[1].parse().unwrap_or(DEFAULT_DATABASE_COUNT)
                    } else {
                        config
                            .last()
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(DEFAULT_DATABASE_COUNT)
                    };
                    Ok((0..db_count).collect())
                }
                Err(e) => {
                    warn!(%e, "config get databases err：");
                    Ok((0..DEFAULT_DATABASE_COUNT).collect())
                }
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
            let (new_cursor, keys_bytes): (u64, Vec<Vec<u8>>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(pattern)
                .arg("COUNT")
                .arg(count)
                .query_async(conn)
                .await?;

            let mut string_keys = Vec::new();
            for key_bytes in keys_bytes {
                if let Ok(key_str) = String::from_utf8(key_bytes) {
                    string_keys.push(key_str);
                }
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

    /// Fetch and cache the connected server's major.minor.patch version.
    /// Returns None when the server is unreachable or INFO cannot be parsed.
    async fn server_version(&self) -> Option<(u32, u32, u32)> {
        if let Some(v) = *self.server_version.read().await {
            return Some(v);
        }
        let info: String = {
            let mut manager = self.manager.write().await;
            let conn = manager.as_mut()?;
            match redis::cmd("INFO")
                .arg("server")
                .query_async::<String>(conn)
                .await
            {
                Ok(s) => s,
                Err(e) => {
                    warn!(error = %e, "INFO server failed; skipping version detection");
                    return None;
                }
            }
        };
        let parsed = parse_redis_version(&info);
        if let Some(v) = parsed {
            *self.server_version.write().await = Some(v);
        }
        parsed
    }

    /// Redis 7.4 introduced `HSCAN ... NOVALUES`.
    async fn supports_hscan_novalues(&self) -> bool {
        match self.server_version().await {
            Some((maj, min, _)) => (maj, min) >= (7, 4),
            None => false,
        }
    }

    pub async fn get_hash_fields(
        &self,
        key: &str,
        match_pattern: &str,
    ) -> Result<Vec<String>, RedisError> {
        let pattern = match_pattern.trim();
        let want_filter = !pattern.is_empty() && pattern != "*";
        let supports_novalues = self.supports_hscan_novalues().await;

        let mut manager = self.manager.write().await;
        let Some(conn) = manager.as_mut() else {
            return Ok(vec![]);
        };

        // ------------------------------------------------------------------
        // Strategy for listing hash fields
        // ------------------------------------------------------------------
        //   * Redis >= 7.4 -> `HSCAN ... NOVALUES`
        //       Server filters by MATCH and returns only field names, so the
        //       payload stays small regardless of value size. This is the
        //       ideal path on a modern server.
        //
        //   * Redis <  7.4 -> `HKEYS` + optional client-side glob match.
        //       HKEYS also transfers field names only.
        //
        // Why we deliberately avoid plain `HSCAN` on pre-7.4 servers, even
        // with a tiny COUNT:
        //
        //   1. `HSCAN` (without NOVALUES) returns alternating
        //      field, value, field, value, ... pairs.
        //
        //   2. The `COUNT` argument is only a hint. For a `hashtable`-encoded
        //      hash, Redis walks a whole hash-table bucket per call and emits
        //      every entry it finds in that bucket. The actual number of
        //      elements returned per round trip is therefore bounded by the
        //      bucket population, not by COUNT - reducing COUNT does not
        //      reduce the per-call payload in the worst case. For a
        //      `ziplist`/`listpack`-encoded hash the entire hash is returned
        //      in a single call and COUNT is ignored outright.
        //
        //   3. The redis crate's `ConnectionManager` defaults to a 500 ms
        //      `response_timeout`. A hash with a handful of fields whose
        //      values are several hundred KB each (observed in the wild:
        //      20 fields x ~650 KB ~ 13 MB) cannot finish transferring inside
        //      that window over a typical WAN link, so the very first
        //      `HSCAN` call times out before any data is parsed - i.e. you
        //      cannot list the fields at all, even though the field *names*
        //      themselves are trivially small.
        //
        //   4. Raising `response_timeout` globally would mask other genuine
        //      stalls and still wastes bandwidth pulling values the caller
        //      did not ask for. Issuing `HGET` lazily per field (which the
        //      UI already does) is both cheaper and respects the existing
        //      timeout budget.
        //
        // `HKEYS` is O(N) on the server and briefly blocks the event loop,
        // but for hashes that fit comfortably in memory (which is the only
        // kind Redis supports anyway) N is small enough that the server-side
        // cost is dominated by network round-trip time, while the bandwidth
        // savings versus `HSCAN` are decisive. When a MATCH pattern is
        // supplied we replicate Redis' glob semantics client-side via
        // `glob_match`, so the external contract of this function is
        // unchanged across server versions.
        // ------------------------------------------------------------------
        if supports_novalues {
            let mut all_fields = Vec::new();
            let mut cursor: u64 = 0;
            loop {
                let mut cmd = redis::cmd("HSCAN");
                cmd.arg(key).arg(cursor);
                if want_filter {
                    cmd.arg("MATCH").arg(pattern);
                }
                cmd.arg("COUNT").arg(HASH_SCAN_COUNT_NOVALUES);
                cmd.arg("NOVALUES");

                let (new_cursor, items): (u64, Vec<redis::Value>) =
                    cmd.query_async(conn).await?;
                for item in &items {
                    if let Some(bytes) = redis_value_as_bytes(item) {
                        all_fields.push(bytes_to_display_string(bytes));
                    } else {
                        warn!(?item, "HSCAN NOVALUES returned unexpected variant");
                    }
                    if all_fields.len() >= HASH_SCAN_MAX_FIELDS {
                        warn!(key = %key, cap = HASH_SCAN_MAX_FIELDS, "Hash field scan truncated");
                        return Ok(all_fields);
                    }
                }
                cursor = new_cursor;
                if cursor == 0 {
                    break;
                }
            }
            Ok(all_fields)
        } else {
            // HKEYS only transfers field names, so even multi-MB values are safe.
            let raw: Vec<Vec<u8>> = conn.hkeys(key).await?;
            let mut fields: Vec<String> = Vec::with_capacity(raw.len());
            for bytes in raw {
                let s = bytes_to_display_string(bytes);
                if !want_filter || glob_match(pattern, &s) {
                    fields.push(s);
                    if fields.len() >= HASH_SCAN_MAX_FIELDS {
                        warn!(key = %key, cap = HASH_SCAN_MAX_FIELDS, "Hash field scan truncated");
                        break;
                    }
                }
            }
            Ok(fields)
        }
    }

    pub async fn get_hash_field_value(
        &self,
        key: &str,
        field: &str,
    ) -> Result<Option<String>, RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            // Read raw bytes rather than `Option<String>` so we don't lose
            // fields that are not valid UTF-8 (e.g. serialized protocol
            // buffers, gzipped blobs, image bytes). Redis treats hash field
            // values as opaque byte strings, so callers legitimately store
            // arbitrary content here.
            let value: Option<Vec<u8>> = conn.hget(key, field).await?;
            match value {
                Some(bytes) => match String::from_utf8(bytes) {
                    // Common case: value is text. Hand it back verbatim so
                    // downstream string operations (search, copy, edit) work
                    // exactly as they did before binary support existed.
                    Ok(s) => Ok(Some(s)),
                    Err(e) => {
                        // Binary case: return a *human-readable* representation
                        // (a hex preview capped at HASH_VALUE_HEX_PREVIEW_BYTES)
                        // instead of forcing the caller to handle Vec<u8>.
                        // The return type stays String to keep the AppState
                        // schema unchanged; the UI displays the placeholder
                        // header line so the user can still see the size.
                        let raw = e.into_bytes();
                        let total = raw.len();
                        let preview_len = total.min(HASH_VALUE_HEX_PREVIEW_BYTES);
                        let mut out = format!("[Binary data, {} bytes]\n", total);
                        out.push_str(&hex::encode(&raw[..preview_len]));
                        if total > preview_len {
                            out.push_str(&format!(
                                "\n... ({} more bytes truncated)",
                                total - preview_len
                            ));
                        }
                        Ok(Some(out))
                    }
                },
                None => Ok(None),
            }
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

    /// Batch delete multiple keys in chunks of 100 for optimal performance
    pub async fn del_keys(&self, keys: &[String]) -> Result<usize, RedisError> {
        if keys.is_empty() {
            return Ok(0);
        }
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            const BATCH_SIZE: usize = 100;
            let mut deleted_total = 0;

            for chunk in keys.chunks(BATCH_SIZE) {
                let mut cmd = redis::cmd("DEL");
                for key in chunk {
                    cmd.arg(key);
                }
                let deleted: i32 = cmd.query_async(conn).await?;
                deleted_total += deleted as usize;
            }

            Ok(deleted_total)
        } else {
            Ok(0)
        }
    }

    pub async fn key_exists(&self, key: &str) -> Result<bool, RedisError> {
        let mut manager = self.manager.write().await;
        if let Some(conn) = manager.as_mut() {
            let exists: i32 = redis::cmd("EXISTS").arg(key).query_async(conn).await?;
            Ok(exists > 0)
        } else {
            Ok(false)
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

/// Extract raw bytes from string-like redis::Value variants. Returns owned bytes
/// so callers do not need to keep the source value alive.
fn redis_value_as_bytes(value: &redis::Value) -> Option<Vec<u8>> {
    match value {
        redis::Value::BulkString(bytes) => Some(bytes.clone()),
        redis::Value::SimpleString(s) => Some(s.as_bytes().to_vec()),
        redis::Value::VerbatimString { text, .. } => Some(text.as_bytes().to_vec()),
        _ => None,
    }
}

/// Render bytes as a UTF-8 string when possible; otherwise emit a hex placeholder
/// so the caller can still distinguish/select the entry.
fn bytes_to_display_string(bytes: Vec<u8>) -> String {
    match String::from_utf8(bytes) {
        Ok(s) => s,
        Err(e) => {
            let raw = e.into_bytes();
            format!("[binary:{}]", hex::encode(&raw))
        }
    }
}

/// Parse the `redis_version:` line from `INFO server` output.
fn parse_redis_version(info: &str) -> Option<(u32, u32, u32)> {
    let line = info
        .lines()
        .find_map(|l| l.trim().strip_prefix("redis_version:"))?;
    let mut parts = line.trim().split('.');
    let major: u32 = parts.next()?.parse().ok()?;
    let minor: u32 = parts.next().unwrap_or("0").parse().ok()?;
    let patch: u32 = parts
        .next()
        .unwrap_or("0")
        .split(|c: char| !c.is_ascii_digit())
        .next()
        .unwrap_or("0")
        .parse()
        .ok()?;
    Some((major, minor, patch))
}

/// Minimal Redis-style glob matcher: `*` any-run, `?` single char, `[...]`
/// character class (with optional leading `^` for negation), `\\` escapes
/// the next char. Operates on bytes so non-UTF-8 input still works.
fn glob_match(pattern: &str, text: &str) -> bool {
    glob_match_bytes(pattern.as_bytes(), text.as_bytes())
}

fn glob_match_bytes(mut pat: &[u8], mut txt: &[u8]) -> bool {
    // Iterative backtracking on '*'.
    #[allow(clippy::collapsible_if, clippy::collapsible_match)]
    {
        let mut star_pat: Option<&[u8]> = None;
        let mut star_txt: &[u8] = &[];
        loop {
            if let Some((&p0, p_rest)) = pat.split_first() {
                match p0 {
                    b'*' => {
                        // Collapse consecutive '*'.
                        let mut rest = p_rest;
                        while let Some((&b'*', r)) = rest.split_first() {
                            rest = r;
                        }
                        if rest.is_empty() {
                            return true;
                        }
                        star_pat = Some(rest);
                        star_txt = txt;
                        pat = rest;
                        continue;
                    }
                    b'?' if !txt.is_empty() => {
                        pat = p_rest;
                        txt = &txt[1..];
                        continue;
                    }
                    b'[' if !txt.is_empty() => {
                        if let Some((matched, consumed)) = match_class(p_rest, txt[0])
                            && matched
                        {
                            pat = &p_rest[consumed..];
                            txt = &txt[1..];
                            continue;
                        }
                    }
                    b'\\' => {
                        if let Some((&esc, after)) = p_rest.split_first()
                            && !txt.is_empty()
                            && txt[0] == esc
                        {
                            pat = after;
                            txt = &txt[1..];
                            continue;
                        }
                    }
                    c if !txt.is_empty() && txt[0] == c => {
                        pat = p_rest;
                        txt = &txt[1..];
                        continue;
                    }
                    _ => {}
                }
            } else if txt.is_empty() {
                return true;
            }
            // Mismatch: backtrack to last '*' if any.
            if let Some(sp) = star_pat
                && !star_txt.is_empty()
            {
                star_txt = &star_txt[1..];
                pat = sp;
                txt = star_txt;
                continue;
            }
            return false;
        }
    }
}

/// Parse a `[...]` body starting just after the opening `[`. Returns
/// (matched, bytes_consumed_including_closing_bracket).
fn match_class(body: &[u8], ch: u8) -> Option<(bool, usize)> {
    let (negate, start) = match body.first() {
        Some(&b'^') => (true, 1),
        _ => (false, 0),
    };
    let mut i = start;
    let mut found = false;
    while i < body.len() {
        match body[i] {
            b']' if i > start => return Some((found ^ negate, i + 1)),
            b'\\' if i + 1 < body.len() => {
                if body[i + 1] == ch {
                    found = true;
                }
                i += 2;
            }
            c if i + 2 < body.len() && body[i + 1] == b'-' && body[i + 2] != b']' => {
                let lo = c;
                let hi = body[i + 2];
                let (lo, hi) = if lo <= hi { (lo, hi) } else { (hi, lo) };
                if ch >= lo && ch <= hi {
                    found = true;
                }
                i += 3;
            }
            c => {
                if c == ch {
                    found = true;
                }
                i += 1;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_standard_version() {
        assert_eq!(parse_redis_version("redis_version:7.4.1\r\n"), Some((7, 4, 1)));
    }

    #[test]
    fn parses_version_with_suffix() {
        let info = "# Server\r\nredis_version:8.0.0-rc1\r\nredis_git_sha1:0\r\n";
        assert_eq!(parse_redis_version(info), Some((8, 0, 0)));
    }

    #[test]
    fn parses_short_version() {
        assert_eq!(parse_redis_version("redis_version:6\r\n"), Some((6, 0, 0)));
    }

    #[test]
    fn returns_none_when_missing() {
        assert_eq!(parse_redis_version("# Server\r\nfoo:bar\r\n"), None);
    }

    #[test]
    fn utf8_bytes_round_trip() {
        assert_eq!(bytes_to_display_string(b"hello".to_vec()), "hello");
    }

    #[test]
    fn binary_bytes_become_hex_placeholder() {
        assert_eq!(
            bytes_to_display_string(vec![0xff, 0x00, 0x10]),
            "[binary:ff0010]"
        );
    }

    #[test]
    fn glob_exact() {
        assert!(glob_match("abc", "abc"));
        assert!(!glob_match("abc", "abcd"));
        assert!(!glob_match("abc", "ab"));
    }

    #[test]
    fn glob_star() {
        assert!(glob_match("*", "anything"));
        assert!(glob_match("*", ""));
        assert!(glob_match("prefix*", "prefix_xyz"));
        assert!(glob_match("*suffix", "abc_suffix"));
        assert!(glob_match("*mid*", "abcmidxyz"));
        assert!(!glob_match("prefix*", "pref"));
    }

    #[test]
    fn glob_question() {
        assert!(glob_match("a?c", "abc"));
        assert!(!glob_match("a?c", "ac"));
        assert!(!glob_match("a?c", "abbc"));
    }

    #[test]
    fn glob_class() {
        assert!(glob_match("[abc]", "a"));
        assert!(glob_match("[abc]", "c"));
        assert!(!glob_match("[abc]", "d"));
        assert!(glob_match("[a-z]*", "hello"));
        assert!(!glob_match("[a-z]*", "Hello"));
        assert!(glob_match("[^0-9]*", "abc"));
        assert!(!glob_match("[^0-9]*", "1abc"));
    }

    #[test]
    fn glob_escape() {
        assert!(glob_match(r"\*", "*"));
        assert!(!glob_match(r"\*", "a"));
    }

    #[test]
    fn glob_redis_date_pattern() {
        // Mirrors the user's real key namespace.
        assert!(glob_match("202605*", "20260527"));
        assert!(!glob_match("202605*", "20260601"));
        assert!(glob_match("2026????", "20260527"));
    }
}
