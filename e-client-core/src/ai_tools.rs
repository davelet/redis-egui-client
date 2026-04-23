use crate::redis_client::RedisClient;
use e_client_config::config::ai_config::BUILTIN_WHITELIST_COMMANDS;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use std::time::Instant;
use tracing::info;

/// Empty args struct that correctly deserializes from an empty JSON object `{}`
/// Using `()` as Args fails because serde cannot deserialize `{}` into unit type.
#[derive(Debug, Deserialize)]
pub struct EmptyArgs {}

/// Response type for tool calls
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>,
}

impl ToolResponse {
    pub fn ok(data: impl ToString) -> Self {
        Self {
            success: true,
            data: Some(data.to_string()),
            error: None,
        }
    }

    pub fn err(error: impl ToString) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error.to_string()),
        }
    }
}

/// Tool for filtering Redis keys by pattern
#[derive(Debug, Clone)]
pub struct FilterKeysTool {
    redis_client: Arc<RedisClient>,
}

impl FilterKeysTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct FilterKeysArgs {
    /// Glob pattern for key filtering (e.g., 'user:*' for all keys starting with 'user:')
    pattern: String,
    /// Maximum number of keys to return (default 100)
    #[serde(default = "default_count")]
    limit: usize,
}

fn default_count() -> usize {
    100
}

/// Error type for FilterKeysTool
#[derive(Debug, thiserror::Error)]
pub enum FilterKeysError {
    #[error("Failed to scan keys: {0}")]
    ScanError(String),
}

/// Error type for Redis tools
#[derive(Debug, thiserror::Error)]
pub enum RedisToolError {
    #[error("Redis error: {0}")]
    RedisError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("{CMD_NOT_WHITELISTED_PREFIX}{0}")]
    CommandNotWhitelisted(String),
}

/// Prefix for command not whitelisted error messages
pub const CMD_NOT_WHITELISTED_PREFIX: &str = "Command not in whitelist: ";

impl From<String> for RedisToolError {
    fn from(s: String) -> Self {
        RedisToolError::RedisError(s)
    }
}

impl From<redis::RedisError> for RedisToolError {
    fn from(e: redis::RedisError) -> Self {
        RedisToolError::RedisError(e.to_string())
    }
}

impl From<serde_json::Error> for RedisToolError {
    fn from(e: serde_json::Error) -> Self {
        RedisToolError::SerializationError(e.to_string())
    }
}

impl Tool for FilterKeysTool {
    const NAME: &'static str = "filter_keys";

    type Error = RedisToolError;
    type Args = FilterKeysArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Filter Redis keys by pattern. Returns a list of matching keys."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "pattern": {
                        "type": "string",
                        "description": "Glob pattern for key filtering (e.g., 'user:*' for all keys starting with 'user:')"
                    },
                    "limit": {
                        "type": "number",
                        "description": "Maximum number of keys to return",
                        "default": 100
                    }
                },
                "required": ["pattern"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!(
            "[ToolCall] {} started with pattern='{}', limit={}",
            Self::NAME,
            args.pattern,
            args.limit
        );
        let mut all_keys = Vec::new();
        let mut cursor = 0u64;

        loop {
            let (new_cursor, keys) = self
                .redis_client
                .scan_keys(cursor, &args.pattern, 1000)
                .await?;
            all_keys.extend(keys);
            cursor = new_cursor;
            if cursor == 0 || all_keys.len() >= args.limit {
                break;
            }
        }

        if all_keys.len() > args.limit {
            all_keys.truncate(args.limit);
        }

        let result = json!({
            "count": all_keys.len(),
            "keys": all_keys
        });

        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        Ok(result.to_string())
    }
}

/// Tool for getting detailed information about a specific key
#[derive(Debug, Clone)]
pub struct GetKeyInfoTool {
    redis_client: Arc<RedisClient>,
}

impl GetKeyInfoTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetKeyInfoArgs {
    /// The name of the Redis key to inspect
    key: String,
}

impl Tool for GetKeyInfoTool {
    const NAME: &'static str = "get_key_info";

    type Error = RedisToolError;
    type Args = GetKeyInfoArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Get detailed information about a specific Redis key including its value, type, and TTL.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "key": {
                        "type": "string",
                        "description": "The name of the Redis key to inspect"
                    }
                },
                "required": ["key"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!("[ToolCall] {} started with key='{}'", Self::NAME, args.key);
        let key_type = self.redis_client.get_key_type(&args.key).await?;
        let ttl = self.redis_client.get_ttl(&args.key).await?;

        let value_preview = match self.redis_client.get_value(&args.key).await? {
            crate::redis_client::ValueData::String(s) => {
                if s.len() > 500 {
                    format!("{}...(truncated, total {} bytes)", &s[..500], s.len())
                } else {
                    s
                }
            }
            crate::redis_client::ValueData::List { len, .. } => format!("List with {} items", len),
            crate::redis_client::ValueData::Set { len, .. } => format!("Set with {} members", len),
            crate::redis_client::ValueData::ZSet { len, .. } => {
                format!("Sorted set with {} members", len)
            }
            crate::redis_client::ValueData::Hash { len, .. } => format!("Hash with {} fields", len),
            crate::redis_client::ValueData::None => "Key does not exist".to_string(),
        };

        let ttl_description = if ttl == -1 {
            "No expiration".to_string()
        } else if ttl == -2 {
            "Key does not exist or has no TTL".to_string()
        } else {
            format!("Expires in {} seconds", ttl)
        };

        let result = json!({
            "key": args.key,
            "type": key_type,
            "ttl": ttl,
            "ttl_description": ttl_description,
            "value_preview": value_preview
        });

        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        Ok(result.to_string())
    }
}

/// Tool for deleting one or more keys
#[derive(Debug, Clone)]
pub struct DeleteKeysTool {
    redis_client: Arc<RedisClient>,
}

impl DeleteKeysTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct DeleteKeysArgs {
    /// Array of key names to delete
    keys: Vec<String>,
}

impl Tool for DeleteKeysTool {
    const NAME: &'static str = "delete_keys";

    type Error = RedisToolError;
    type Args = DeleteKeysArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "Delete one or more keys from Redis. Returns the number of keys that were deleted."
                    .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "keys": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Array of key names to delete"
                    }
                },
                "required": ["keys"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!(
            "[ToolCall] {} started with {} keys",
            Self::NAME,
            args.keys.len()
        );

        let deleted_count = self.redis_client.del_keys(&args.keys).await?;

        let result = json!({
            "deleted_count": deleted_count
        });

        let elapsed = start.elapsed();
        info!(
            "[ToolCall] {} completed in {:?}, deleted {} keys",
            Self::NAME,
            elapsed,
            deleted_count
        );
        Ok(result.to_string())
    }
}

/// Tool for executing raw Redis commands
#[derive(Debug, Clone)]
pub struct ExecuteCommandTool {
    redis_client: Arc<RedisClient>,
    custom_whitelist: Vec<String>,
}

impl ExecuteCommandTool {
    pub fn new(redis_client: Arc<RedisClient>, custom_whitelist: Vec<String>) -> Self {
        Self {
            redis_client,
            custom_whitelist,
        }
    }

    /// Check if command is in whitelist
    pub fn is_command_allowed(&self, command: &str) -> bool {
        // Extract first token as command name, convert to uppercase
        let command_name = command
            .split_whitespace()
            .next()
            .map(|s| s.to_uppercase())
            .unwrap_or_default();

        // Validate command name is not empty
        if command_name.is_empty() {
            return false;
        }

        // Check built-in whitelist first
        if BUILTIN_WHITELIST_COMMANDS.contains(&command_name.as_str()) {
            return true;
        }

        // Check custom whitelist
        self.custom_whitelist
            .iter()
            .any(|c| c.eq_ignore_ascii_case(&command_name))
    }
}

#[derive(Debug, Deserialize)]
pub struct ExecuteCommandArgs {
    /// The complete Redis command to execute (e.g., 'GET mykey' or 'HGETALL myhash')
    command: String,
    /// Optional: Bypass whitelist check for this single execution (for user confirmed operations)
    #[serde(default)]
    allow_unsafe: bool,
}

impl Tool for ExecuteCommandTool {
    const NAME: &'static str = "execute_redis_command";

    type Error = RedisToolError;
    type Args = ExecuteCommandArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Execute a raw Redis command. Use this when the user asks for a specific Redis operation.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The complete Redis command to execute (e.g., 'GET mykey' or 'HGETALL myhash')"
                    }
                },
                "required": ["command"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!(
            "[ToolCall] {} started with command='{}'",
            Self::NAME,
            args.command
        );

        // Check whitelist before execution, skip if explicitly allowed by user
        if !args.allow_unsafe && !self.is_command_allowed(&args.command) {
            let command_name = args
                .command
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .to_uppercase();
            info!(
                "[ToolCall] {} blocked command '{}' - not in whitelist",
                Self::NAME,
                command_name
            );
            return Err(RedisToolError::CommandNotWhitelisted(command_name));
        }

        let result = self
            .redis_client
            .execute_command(&args.command)
            .await
            .map_err(|e| RedisToolError::RedisError(e.to_string()))?;
        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        Ok(result)
    }
}

/// Tool for getting database statistics
#[derive(Debug, Clone)]
pub struct GetDbStatsTool {
    redis_client: Arc<RedisClient>,
}

impl GetDbStatsTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

impl Tool for GetDbStatsTool {
    const NAME: &'static str = "get_db_stats";

    type Error = RedisToolError;
    type Args = EmptyArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "Get statistics about the current Redis database including total key count."
                    .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {}
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!("[ToolCall] {} started", Self::NAME);
        let _ = args; // Consume the empty args struct
        let db_size = self.redis_client.get_db_size().await?;

        let result = json!({
            "total_keys": db_size,
            "description": format!("The current database contains {} keys", db_size)
        });

        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        Ok(result.to_string())
    }
}

/// Tool for setting a string value
#[derive(Debug, Clone)]
pub struct SetStringTool {
    redis_client: Arc<RedisClient>,
}

impl SetStringTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct SetStringArgs {
    key: String,
    value: String,
}

impl Tool for SetStringTool {
    const NAME: &'static str = "set_string";

    type Error = RedisToolError;
    type Args = SetStringArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Set a string value for a key. Creates the key if it doesn't exist, or updates it if it does.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "key": { "type": "string", "description": "The key to set" },
                    "value": { "type": "string", "description": "The string value to store" }
                },
                "required": ["key", "value"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!("[ToolCall] {} started with key='{}'", Self::NAME, args.key);
        self.redis_client.set_string(&args.key, &args.value).await?;
        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        Ok(
            json!({ "success": true, "message": format!("OK: Set key '{}' to value", args.key) })
                .to_string(),
        )
    }
}

/// Tool for setting TTL on a key
#[derive(Debug, Clone)]
pub struct SetTtlTool {
    redis_client: Arc<RedisClient>,
}

impl SetTtlTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct SetTtlArgs {
    key: String,
    ttl: i64,
}

impl Tool for SetTtlTool {
    const NAME: &'static str = "set_ttl";

    type Error = RedisToolError;
    type Args = SetTtlArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Set the expiration time (TTL) for a key. Use -1 to remove expiration (make it persistent). Use positive values for seconds.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "key": { "type": "string", "description": "The key to set TTL on" },
                    "ttl": { "type": "number", "description": "TTL in seconds. Use -1 to remove expiration." }
                },
                "required": ["key", "ttl"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!(
            "[ToolCall] {} started with key='{}', ttl={}",
            Self::NAME,
            args.key,
            args.ttl
        );
        self.redis_client.set_ttl(&args.key, args.ttl).await?;
        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        let message = if args.ttl < 0 {
            format!("OK: Removed expiration from key '{}'", args.key)
        } else {
            format!("OK: Set TTL {} seconds on key '{}'", args.ttl, args.key)
        };
        Ok(json!({ "success": true, "message": message }).to_string())
    }
}

/// Tool for renaming a key
#[derive(Debug, Clone)]
pub struct RenameKeyTool {
    redis_client: Arc<RedisClient>,
}

impl RenameKeyTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct RenameKeyArgs {
    old_key: String,
    new_key: String,
}

impl Tool for RenameKeyTool {
    const NAME: &'static str = "rename_key";

    type Error = RedisToolError;
    type Args = RenameKeyArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Rename a key to a new name. Only renames if the new key name doesn't already exist.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "old_key": { "type": "string", "description": "The current key name" },
                    "new_key": { "type": "string", "description": "The new key name" }
                },
                "required": ["old_key", "new_key"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!(
            "[ToolCall] {} started with old_key='{}', new_key='{}'",
            Self::NAME,
            args.old_key,
            args.new_key
        );
        let renamed = self
            .redis_client
            .rename_key_nx(&args.old_key, &args.new_key)
            .await?;
        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        if renamed {
            Ok(json!({ "success": true, "message": format!("OK: Renamed '{}' to '{}'", args.old_key, args.new_key) }).to_string())
        } else {
            Ok(json!({ "success": false, "message": format!("Key '{}' already exists or '{}' does not exist", args.new_key, args.old_key) }).to_string())
        }
    }
}

/// Tool for checking if a key exists
#[derive(Debug, Clone)]
pub struct KeyExistsTool {
    redis_client: Arc<RedisClient>,
}

impl KeyExistsTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct KeyExistsArgs {
    key: String,
}

impl Tool for KeyExistsTool {
    const NAME: &'static str = "key_exists";

    type Error = RedisToolError;
    type Args = KeyExistsArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Check if a key exists in the database.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "key": { "type": "string", "description": "The key to check" }
                },
                "required": ["key"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!("[ToolCall] {} started with key='{}'", Self::NAME, args.key);
        let exists = self.redis_client.key_exists(&args.key).await?;
        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        Ok(json!({ "exists": exists }).to_string())
    }
}

/// Tool for setting a hash field
#[derive(Debug, Clone)]
pub struct HsetTool {
    redis_client: Arc<RedisClient>,
}

impl HsetTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct HsetArgs {
    key: String,
    field: String,
    value: String,
}

impl Tool for HsetTool {
    const NAME: &'static str = "hset";

    type Error = RedisToolError;
    type Args = HsetArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Set a field-value pair in a Redis Hash. Creates the hash and field if they don't exist.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "key": { "type": "string", "description": "The hash key" },
                    "field": { "type": "string", "description": "The field name" },
                    "value": { "type": "string", "description": "The value to store" }
                },
                "required": ["key", "field", "value"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!(
            "[ToolCall] {} started with key='{}', field='{}'",
            Self::NAME,
            args.key,
            args.field
        );
        self.redis_client
            .hset(&args.key, &args.field, &args.value)
            .await?;
        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        Ok(json!({ "success": true, "message": format!("OK: Set field '{}' in hash '{}'", args.field, args.key) }).to_string())
    }
}

/// Tool for deleting a hash field
#[derive(Debug, Clone)]
pub struct HdelTool {
    redis_client: Arc<RedisClient>,
}

impl HdelTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct HdelArgs {
    key: String,
    field: String,
}

impl Tool for HdelTool {
    const NAME: &'static str = "hdel";

    type Error = RedisToolError;
    type Args = HdelArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Delete a field from a Redis Hash.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "key": { "type": "string", "description": "The hash key" },
                    "field": { "type": "string", "description": "The field to delete" }
                },
                "required": ["key", "field"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!(
            "[ToolCall] {} started with key='{}', field='{}'",
            Self::NAME,
            args.key,
            args.field
        );
        self.redis_client.hdel(&args.key, &args.field).await?;
        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        Ok(json!({ "success": true, "message": format!("OK: Deleted field '{}' from hash '{}'", args.field, args.key) }).to_string())
    }
}

/// Tool for setting a list element
#[derive(Debug, Clone)]
pub struct LsetTool {
    redis_client: Arc<RedisClient>,
}

impl LsetTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct LsetArgs {
    key: String,
    index: i64,
    value: String,
}

impl Tool for LsetTool {
    const NAME: &'static str = "lset";

    type Error = RedisToolError;
    type Args = LsetArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Set the value of an element in a Redis List by its index.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "key": { "type": "string", "description": "The list key" },
                    "index": { "type": "number", "description": "The index of the element (0-based, negative for from end)" },
                    "value": { "type": "string", "description": "The value to set" }
                },
                "required": ["key", "index", "value"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!(
            "[ToolCall] {} started with key='{}', index={}",
            Self::NAME,
            args.key,
            args.index
        );
        self.redis_client
            .lset(&args.key, args.index, &args.value)
            .await?;
        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        Ok(json!({ "success": true, "message": format!("OK: Set element at index {} in list '{}'", args.index, args.key) }).to_string())
    }
}

/// Tool for adding a member to a Set
#[derive(Debug, Clone)]
pub struct SaddTool {
    redis_client: Arc<RedisClient>,
}

impl SaddTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct SaddArgs {
    key: String,
    member: String,
}

impl Tool for SaddTool {
    const NAME: &'static str = "sadd";

    type Error = RedisToolError;
    type Args = SaddArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Add a member to a Redis Set. Duplicate members are ignored.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "key": { "type": "string", "description": "The set key" },
                    "member": { "type": "string", "description": "The member to add" }
                },
                "required": ["key", "member"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!(
            "[ToolCall] {} started with key='{}', member='{}'",
            Self::NAME,
            args.key,
            args.member
        );
        self.redis_client.sadd(&args.key, &args.member).await?;
        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        Ok(json!({ "success": true, "message": format!("OK: Added member '{}' to set '{}'", args.member, args.key) }).to_string())
    }
}

/// Tool for removing a member from a Set
#[derive(Debug, Clone)]
pub struct SremTool {
    redis_client: Arc<RedisClient>,
}

impl SremTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct SremArgs {
    key: String,
    member: String,
}

impl Tool for SremTool {
    const NAME: &'static str = "srem";

    type Error = RedisToolError;
    type Args = SremArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Remove a member from a Redis Set.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "key": { "type": "string", "description": "The set key" },
                    "member": { "type": "string", "description": "The member to remove" }
                },
                "required": ["key", "member"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!(
            "[ToolCall] {} started with key='{}', member='{}'",
            Self::NAME,
            args.key,
            args.member
        );
        self.redis_client.srem(&args.key, &args.member).await?;
        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        Ok(json!({ "success": true, "message": format!("OK: Removed member '{}' from set '{}'", args.member, args.key) }).to_string())
    }
}

/// Tool for adding a member to a Sorted Set
#[derive(Debug, Clone)]
pub struct ZaddTool {
    redis_client: Arc<RedisClient>,
}

impl ZaddTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct ZaddArgs {
    key: String,
    score: f64,
    member: String,
}

impl Tool for ZaddTool {
    const NAME: &'static str = "zadd";

    type Error = RedisToolError;
    type Args = ZaddArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Add a member with a score to a Redis Sorted Set (ZSet).".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "key": { "type": "string", "description": "The sorted set key" },
                    "score": { "type": "number", "description": "The score for sorting" },
                    "member": { "type": "string", "description": "The member to add" }
                },
                "required": ["key", "score", "member"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!(
            "[ToolCall] {} started with key='{}', member='{}', score={}",
            Self::NAME,
            args.key,
            args.member,
            args.score
        );
        self.redis_client
            .zadd(&args.key, args.score, &args.member)
            .await?;
        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        Ok(json!({ "success": true, "message": format!("OK: Added member '{}' with score {} to zset '{}'", args.member, args.score, args.key) }).to_string())
    }
}

/// Tool for removing a member from a Sorted Set
#[derive(Debug, Clone)]
pub struct ZremTool {
    redis_client: Arc<RedisClient>,
}

impl ZremTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct ZremArgs {
    key: String,
    member: String,
}

impl Tool for ZremTool {
    const NAME: &'static str = "zrem";

    type Error = RedisToolError;
    type Args = ZremArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Remove a member from a Redis Sorted Set (ZSet).".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "key": { "type": "string", "description": "The sorted set key" },
                    "member": { "type": "string", "description": "The member to remove" }
                },
                "required": ["key", "member"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!(
            "[ToolCall] {} started with key='{}', member='{}'",
            Self::NAME,
            args.key,
            args.member
        );
        self.redis_client.zrem(&args.key, &args.member).await?;
        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        Ok(json!({ "success": true, "message": format!("OK: Removed member '{}' from zset '{}'", args.member, args.key) }).to_string())
    }
}

/// Tool for pushing to a List
#[derive(Debug, Clone)]
pub struct RpushTool {
    redis_client: Arc<RedisClient>,
}

impl RpushTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct RpushArgs {
    key: String,
    value: String,
}

impl Tool for RpushTool {
    const NAME: &'static str = "rpush";

    type Error = RedisToolError;
    type Args = RpushArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Push a value to the right end of a Redis List.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "key": { "type": "string", "description": "The list key" },
                    "value": { "type": "string", "description": "The value to push" }
                },
                "required": ["key", "value"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!("[ToolCall] {} started with key='{}'", Self::NAME, args.key);
        self.redis_client.rpush(&args.key, &args.value).await?;
        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        Ok(json!({ "success": true, "message": format!("OK: Pushed value to list '{}'", args.key) }).to_string())
    }
}

/// Tool for switching database
#[derive(Debug, Clone)]
pub struct SelectDbTool {
    redis_client: Arc<RedisClient>,
}

impl SelectDbTool {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[derive(Debug, Deserialize)]
pub struct SelectDbArgs {
    db: u32,
}

impl Tool for SelectDbTool {
    const NAME: &'static str = "select_db";

    type Error = RedisToolError;
    type Args = SelectDbArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Switch to a different Redis database (0-15 typically). Use with caution as it changes the current database context.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "db": { "type": "number", "description": "The database number (0-15)" }
                },
                "required": ["db"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let start = Instant::now();
        info!("[ToolCall] {} started with db={}", Self::NAME, args.db);
        self.redis_client.select_db(args.db).await?;
        let elapsed = start.elapsed();
        info!("[ToolCall] {} completed in {:?}", Self::NAME, elapsed);
        Ok(
            json!({ "success": true, "message": format!("OK: Switched to database {}", args.db) })
                .to_string(),
        )
    }
}
