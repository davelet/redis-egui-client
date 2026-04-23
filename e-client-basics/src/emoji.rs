//! Unicode Emoji constants for consistent UI icons across the application.

/// Action icons - buttons and interactive elements
pub mod action {
    /// Add / Create new (➕)
    pub const ADD: &str = "\u{2795}";
    /// Refresh / Reload (🔄)
    pub const REFRESH: &str = "\u{1F504}";
    /// Edit / Pencil (✏)
    pub const EDIT: &str = "\u{270F}";
    /// Save / Floppy disk (💾)
    pub const SAVE: &str = "\u{1F4BE}";
    /// Cancel / Cross (❌)
    pub const CANCEL: &str = "\u{274C}";
    /// Delete / Trash (🗑)
    pub const DELETE: &str = "\u{1F5D1}";
    /// Close / Cross mark (×)
    pub const CLOSE: &str = "\u{00D7}";
    /// Settings / Gear (⚙)
    pub const SETTINGS: &str = "\u{2699}";
    /// Warning (⚠️)
    pub const WARNING: &str = "\u{26A0}";
    /// Import from file (📥)
    pub const IMPORT: &str = "\u{1F4E5}";
    /// Import folder / Browse (📂)
    pub const IMPORT_FOLDER: &str = "\u{1F4C2}";
    /// Export / Upload (📤)
    pub const EXPORT: &str = "\u{1F4E4}";
}

/// Navigation icons
pub mod navigation {
    /// Open in new tab / Clipboard (📑)
    pub const NEW_TAB: &str = "\u{1F4C1}";
}

/// Status icons
pub mod status {
    /// Connected / Check (✓)
    pub const CONNECTED: &str = "\u{2713}";
    /// Disconnected / Cross (✗)
    pub const DISCONNECTED: &str = "\u{2717}";
    /// Active / Dot (●)
    pub const DOT: &str = "\u{25CF}";
}

/// Web/Link icons
pub mod web {
    /// Globe / Web (🌐)
    pub const WEB: &str = "\u{1F310}";
}
