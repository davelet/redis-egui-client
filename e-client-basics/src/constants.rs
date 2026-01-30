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

/// Key scan configuration
pub const SCAN_COUNT: usize = 200;
pub const MAX_INITIAL_KEYS: usize = 2000;
pub const LOAD_MORE_BATCH_SIZE: usize = 2000;
