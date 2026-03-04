/// Formats JSON value for display/editing.
/// If the value is valid JSON, returns pretty-printed JSON.
/// Otherwise, returns the original value unchanged.
pub fn format_json_for_edit(value: &str) -> String {
    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(value) {
        if let Ok(formatted) = serde_json::to_string_pretty(&json_value) {
            formatted
        } else {
            value.to_string()
        }
    } else {
        value.to_string()
    }
}

/// Compacts JSON value if it was originally single-line (no newlines).
/// If the value is valid JSON and contains no newlines, returns compact JSON.
/// Otherwise, returns the original value unchanged.
pub fn compact_json_if_single_line(value: &str) -> String {
    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(value) {
        // Check if original string has newlines
        let has_newlines = value.contains('\n');
        if !has_newlines {
            // Original was compact, save as compact JSON
            serde_json::to_string(&json_value).unwrap_or(value.to_string())
        } else {
            // Original was formatted, save as is
            value.to_string()
        }
    } else {
        // Not JSON, save as is
        value.to_string()
    }
}

/// JSON value management with format preservation
#[derive(Debug, Clone)]
pub struct JsonValue {
    pub value: String,
    pub was_single_line: bool,
}

impl JsonValue {
    pub fn new(original: &str) -> Self {
        let was_single_line = !original.contains('\n');
        Self {
            value: format_json_for_edit(original),
            was_single_line,
        }
    }

    pub fn to_save(&self) -> String {
        if self.was_single_line {
            // Original was single-line, compact JSON
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&self.value) {
                serde_json::to_string(&json_value).unwrap_or(self.value.clone())
            } else {
                self.value.clone()
            }
        } else {
            // Original was multi-line, save as-is
            self.value.clone()
        }
    }
}
