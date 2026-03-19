//! State fragments - helper types for Arc<RwLock<T>> patterns
pub use e_client_config::language::Language;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Helper type alias for shared state pattern
pub type Shared<T> = Arc<RwLock<T>>;

/// Create a new shared state with initial value
pub fn shared<T>(value: T) -> Shared<T> {
    Arc::new(RwLock::new(value))
}

/// Extension trait for easier access to shared state
pub trait SharedExt<T> {
    /// Quick read access (blocking)
    fn get(&self) -> T
    where
        T: Clone;
    /// Quick write access
    fn set(&self, value: T);
}

impl<T: Clone + Send + Sync + 'static> SharedExt<T> for Shared<T> {
    fn get(&self) -> T {
        self.blocking_read().clone()
    }

    fn set(&self, value: T) {
        *self.blocking_write() = value;
    }
}
