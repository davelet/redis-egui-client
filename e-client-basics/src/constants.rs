// Global non-localized constants

/// Application name (not localized)
pub const APP_NAME: &str = "Rudist";
pub const LOAD_ERROR_TITLE: &str = "Load Error:";

/// Default Redis URL used when no other is specified
pub const DEFAULT_REDIS_PORT: &str = "6379";

/// Default key filter pattern
pub const WILD_KEY_FILTER: char = '*';

/// Default main window size
pub const WINDOW_WIDTH: f32 = 1200.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

pub const ENGLISH: &str = "English";
pub const CHINESE: &str = "中文";

pub const CONNECTION_NAME_LIMIT: usize = 100;
pub const CONNECTION_URL_LIMIT: usize = 1000;
pub const CONNECTION_AUTH_LIMIT: usize = 1024;

/// AI model configuration limits
pub const AI_MODEL_NAME_LIMIT: usize = 50;
pub const AI_MODEL_URL_LIMIT: usize = 500;
pub const AI_MODEL_ID_LIMIT: usize = 100;
pub const AI_API_KEY_LIMIT: usize = 500;

/// Key scan configuration
pub const SCAN_COUNT: usize = 200;
pub const MAX_INITIAL_KEYS: usize = 2000;
pub const LOAD_MORE_BATCH_SIZE: usize = 2000;
pub const MAX_LOADED_KEYS: usize = 10000;
pub const CONNECTION_RETRY_COUNT: usize = 1;
pub const DEFAULT_DATABASE_COUNT: u32 = 16;
pub const ITEMS_PER_LOAD: usize = 100;
pub const HASH_FIELD_PAIR_STEP: usize = 2; // Step for iterating hash key-value pairs

/// UI size constants
pub const DEFAULT_SIDE_PANEL_WIDTH: f32 = 400.0;
pub const MIN_SIDE_PANEL_WIDTH: f32 = 250.0;
pub const MAX_SIDE_PANEL_WIDTH: f32 = 800.0;
pub const MIN_CENTRAL_PANEL_HEIGHT: f32 = 300.0;
pub const MAX_CENTRAL_PANEL_HEIGHT: f32 = 600.0;
pub const MAX_KEY_DISPLAY_LENGTH: usize = 60;

/// Batch processing
pub const SORT_INTERVAL_KEYS: usize = 200;
pub const UI_UPDATE_INTERVAL_BATCHES: usize = 2;

/// Time durations (in milliseconds)
pub const UI_REPAINT_INTERVAL_MS: u64 = 10;
pub const SCAN_SLEEP_INTERVAL_MS: u64 = 10;
pub const COPY_FEEDBACK_DURATION_MS: u64 = 500;

/// UI colors (RGB values)
pub const ACTIVE_TAB_BACKGROUND_COLOR: [u8; 3] = [200, 220, 240];

/// Redis commands list (used to detect if input is a Redis command)
pub const REDIS_COMMANDS: &[&str] = &[
    // Connection
    "AUTH",
    "CLIENT",
    "ECHO",
    "PING",
    "QUIT",
    "SELECT",
    "SWAPDB",
    // Server
    "ACL",
    "BGREWRITEAOF",
    "BGSAVE",
    "COMMAND",
    "CONFIG",
    "DBSIZE",
    "DEBUG",
    "FLUSHALL",
    "FLUSHDB",
    "INFO",
    "LASTSAVE",
    "LATENCY",
    "MEMORY",
    "MODULE",
    "MONITOR",
    "PSYNC",
    "REPLICAOF",
    "RESTORE",
    "SAVE",
    "SHUTDOWN",
    "SLAVEOF",
    "SLOWLOG",
    "SYNC",
    "TIME",
    // Keys
    "COPY",
    "DEL",
    "DUMP",
    "EXISTS",
    "EXPIRE",
    "EXPIREAT",
    "KEYS",
    "MIGRATE",
    "MOVE",
    "OBJECT",
    "PERSIST",
    "PEXPIRE",
    "PEXPIREAT",
    "PTTL",
    "RANDOMKEY",
    "RENAME",
    "RENAMENX",
    "RESTORE",
    "SCAN",
    "SORT",
    "TOUCH",
    "TTL",
    "TYPE",
    "UNLINK",
    "WAIT",
    // Strings
    "APPEND",
    "BITCOUNT",
    "BITFIELD",
    "BITOP",
    "BITPOS",
    "DECR",
    "DECRBY",
    "GET",
    "GETBIT",
    "GETDEL",
    "GETEX",
    "GETRANGE",
    "GETSET",
    "INCR",
    "INCRBY",
    "INCRBYFLOAT",
    "MGET",
    "MSET",
    "MSETNX",
    "PSETEX",
    "SET",
    "SETBIT",
    "SETEX",
    "SETNX",
    "SETRANGE",
    "STRALGO",
    "STRLEN",
    "SUBSTR",
    // Lists
    "BLMOVE",
    "BLMPOP",
    "BLPOP",
    "BRPOP",
    "BRPOPLPUSH",
    "LINDEX",
    "LINSERT",
    "LLEN",
    "LMOVE",
    "LMPOP",
    "LPOP",
    "LPOS",
    "LPUSH",
    "LPUSHX",
    "LRANGE",
    "LREM",
    "LSET",
    "LTRIM",
    "RPOP",
    "RPOPLPUSH",
    "RPUSH",
    "RPUSHX",
    // Sets
    "SADD",
    "SCARD",
    "SDIFF",
    "SDIFFSTORE",
    "SINTER",
    "SINTERCARD",
    "SINTERSTORE",
    "SISMEMBER",
    "SMEMBERS",
    "SMISMEMBER",
    "SMOVE",
    "SPOP",
    "SRANDMEMBER",
    "SREM",
    "SSCAN",
    "SUNION",
    "SUNIONSTORE",
    // Sorted Sets
    "ZADD",
    "ZCARD",
    "ZCOUNT",
    "ZDIFF",
    "ZDIFFSTORE",
    "ZINCRBY",
    "ZINTER",
    "ZINTERCARD",
    "ZINTERSTORE",
    "ZLEXCOUNT",
    "ZMPOP",
    "ZMSCORE",
    "ZPOPMAX",
    "ZPOPMIN",
    "ZRANDMEMBER",
    "ZRANGE",
    "ZRANGEBYLEX",
    "ZRANGEBYSCORE",
    "ZRANGESTORE",
    "ZRANK",
    "ZREM",
    "ZREMRANGEBYLEX",
    "ZREMRANGEBYSCORE",
    "ZREVRANGE",
    "ZREVRANGEBYLEX",
    "ZREVRANGEBYSCORE",
    "ZREVRANK",
    "ZSCAN",
    "ZSCORE",
    "ZUNION",
    "ZUNIONSTORE",
    // Hashes
    "HDEL",
    "HEXISTS",
    "HGET",
    "HGETALL",
    "HINCRBY",
    "HINCRBYFLOAT",
    "HKEYS",
    "HLEN",
    "HMGET",
    "HMSET",
    "HRANDFIELD",
    "HSCAN",
    "HSET",
    "HSETNX",
    "HSTRLEN",
    "HVALS",
    // Pub/Sub
    "PSUBSCRIBE",
    "PUBLISH",
    "PUNSUBSCRIBE",
    "SUBSCRIBE",
    "UNSUBSCRIBE",
    // Transactions
    "DISCARD",
    "EXEC",
    "MULTI",
    "UNWATCH",
    "WATCH",
    // Streams
    "XACK",
    "XADD",
    "XAUTOCLAIM",
    "XCLAIM",
    "XDEL",
    "XGROUP",
    "XINFO",
    "XLEN",
    "XPENDING",
    "XRANGE",
    "XREAD",
    "XREADGROUP",
    "XREVRANGE",
    "XTRIM",
    // Geospatial
    "GEOADD",
    "GEODIST",
    "GEOHASH",
    "GEOPOS",
    "GEORADIUS",
    "GEORADIUSBYMEMBER",
    "GEOSEARCH",
    // HyperLogLog
    "PFADD",
    "PFCOUNT",
    "PFMERGE",
    // Scripting
    "EVAL",
    "EVALSHA",
    "SCRIPT",
    // Cluster
    "ASKING",
    "CLUSTER",
    "READONLY",
    "READWRITE",
];
