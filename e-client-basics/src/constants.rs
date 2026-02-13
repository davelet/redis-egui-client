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
