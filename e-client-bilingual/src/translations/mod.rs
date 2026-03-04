pub mod keys;
pub mod translator;

pub use translator::{tr, tr_fmt, Translator};

// Re-export keys for convenience
pub use keys::*;
