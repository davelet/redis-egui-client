use std::path::PathBuf;
use tracing::Level;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// Logging configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoggingConfig {
    /// Logging level (trace, debug, info, warn, error)
    pub level: String,
    /// Directory where log files will be stored
    pub log_dir: PathBuf,
    /// Prefix for log file names
    pub file_prefix: String,
    /// Whether to enable file logging
    pub enable_file: bool,
    /// Whether to enable console logging
    pub enable_console: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        let log_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("redis-egui-client")
            .join("logs");
        Self {
            level: "info".to_string(),
            log_dir,
            file_prefix: "app".to_string(),
            enable_file: true,
            enable_console: false,
        }
    }
}

/// Initialize logging with given configuration
pub fn init_logging(config: &LoggingConfig) -> Result<(), LoggingError> {
    // Parse log level
    let level: Level = config
        .level
        .parse()
        .map_err(|_| LoggingError::InvalidLevel(config.level.clone()))?;

    // Create env filter for additional filtering
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level.to_string()));

    // Console layer
    let console_layer = if config.enable_console {
        Some(
            fmt::layer()
                .with_target(false)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
    } else {
        None
    };

    // File layer
    let file_layer = if config.enable_file {
        // Ensure log directory exists
        std::fs::create_dir_all(&config.log_dir).map_err(LoggingError::IoError)?;

        // Create a rolling file appender that rotates daily and keeps 7 days of logs
        let file_appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix(&config.file_prefix)
            .filename_suffix("log")
            .max_log_files(7)
            .build(&config.log_dir)?;

        Some(
            fmt::layer()
                .with_writer(file_appender)
                .with_ansi(false)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
    } else {
        None
    };

    // Combine layers and set as global subscriber
    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .try_init()
        .map_err(|_| LoggingError::AlreadyInitialized)?;

    Ok(())
}

/// Initialize logging with default configuration
pub fn init_logging_default() -> Result<(), LoggingError> {
    init_logging(&LoggingConfig::default())
}

#[derive(Debug, thiserror::Error)]
pub enum LoggingError {
    #[error("Invalid log level: {0}")]
    InvalidLevel(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Log initialization error: {0}")]
    InitError(#[from] tracing_appender::rolling::InitError),
    #[error("Logging already initialized")]
    AlreadyInitialized,
}

/// Helper macro for logging at different levels
#[macro_export]
macro_rules! log_event {
    ($level:expr, $($arg:tt)*) => {
        tracing::event!($level, $($arg)*);
    };
}

/// Log an application start event
pub fn log_app_start() {
    tracing::info!("Application starting");
}

/// Log an application shutdown event
pub fn log_app_shutdown() {
    tracing::info!("Application shutting down");
}

/// Log a file operation event
pub fn log_file_operation(operation: &str, path: &std::path::Path, success: bool) {
    if success {
        tracing::info!(operation = %operation, path = %path.display(), "File operation succeeded");
    } else {
        tracing::error!(operation = %operation, path = %path.display(), "File operation failed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    use tempfile::tempdir;

    #[test]
    fn test_logging_initialization() {
        let dir = tempdir().unwrap();
        let config = LoggingConfig {
            level: "info".to_string(),
            log_dir: dir.path().to_path_buf(),
            file_prefix: "test".to_string(),
            enable_file: true,
            enable_console: false,
        };
        // Should not panic, use try_init to handle re-initialization
        // Ignore AlreadyInitialized error since tests may run in any order
        let _ = init_logging(&config);
        // Log something
        tracing::info!("Test log message");
        // Wait a bit for file write (though it's synchronous)
        thread::sleep(Duration::from_millis(100));
        // Check that log file exists
        let entries: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "log"))
            .collect();
        assert!(!entries.is_empty(), "No log files found");
        // Read the first log file
        let log_file = std::fs::read_to_string(entries[0].path()).unwrap();
        assert!(
            log_file.contains("Test log message"),
            "Log file does not contain expected message"
        );
        println!("Log file content:\n{}", log_file);
    }

    #[test]
    fn test_re_initialization_safe() {
        // First initialization
        let result1 = init_logging_default();
        // Second initialization should fail gracefully with AlreadyInitialized error
        let result2 = init_logging_default();
        assert!(result1.is_ok() || matches!(result1, Err(LoggingError::AlreadyInitialized)));
        assert!(matches!(result2, Err(LoggingError::AlreadyInitialized)));
    }
}
