pub mod data;
pub mod keys;
pub mod translator;

pub use translator::{Translator, tr, tr_fmt};

// Re-export keys for convenience
pub use keys::*;
