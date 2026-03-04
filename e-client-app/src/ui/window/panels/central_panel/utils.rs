use e_client_basics::constants::MAX_KEY_DISPLAY_LENGTH;

/// Truncate key if it's too long
pub fn truncate_key(key: &str) -> String {
    if key.len() > MAX_KEY_DISPLAY_LENGTH {
        format!("{}...", &key[..MAX_KEY_DISPLAY_LENGTH])
    } else {
        key.to_string()
    }
}

/// Truncate text with ellipsis if it exceeds max_chars
pub fn truncate_with_ellipsis(text: &str, max_chars: usize) -> String {
    let char_count = text.chars().count();
    if char_count > max_chars {
        let truncated: String = text.chars().take(max_chars).collect();
        format!("{}...", truncated)
    } else {
        text.to_string()
    }
}

/// Format TTL value to human-readable string
pub fn format_ttl(ttl: i64) -> String {
    match ttl {
        -1 => "ttl: -1".to_string(),
        -2 => String::new(),
        t if t >= 100 => {
            let days = t / 86400;
            let hours = (t % 86400) / 3600;
            let minutes = (t % 3600) / 60;
            let seconds = t % 60;
            if days > 0 {
                format!("ttl: {}d {}h {}m {}s", days, hours, minutes, seconds)
            } else if hours > 0 {
                format!("ttl: {}h {}m {}s", hours, minutes, seconds)
            } else {
                format!("ttl: {}m {}s", minutes, seconds)
            }
        }
        t if t >= 0 => format!("ttl: {}s", t),
        t => format!("ttl: {}", t),
    }
}

/// Convert ValueData to copy-friendly text format
pub fn value_to_copy_text(val: &crate::core::ValueData) -> String {
    use crate::core::ValueData;

    match val {
        ValueData::String(s) => s.clone(),
        ValueData::List { items, .. } => items.join("\n"),
        ValueData::Hash {
            fields,
            loaded_values,
            ..
        } => fields
            .iter()
            .map(|k| {
                format!(
                    "{}: {}",
                    k,
                    loaded_values.get(k).cloned().unwrap_or_default()
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        ValueData::Set { items, .. } => items.join("\n"),
        ValueData::ZSet { items, .. } => items
            .iter()
            .map(|(m, s)| format!("{} ({})", m, s))
            .collect::<Vec<_>>()
            .join("\n"),
        ValueData::None => String::new(),
    }
}

/// Parse hex color string (#RRGGBB) to egui Color32
pub fn parse_color_hex(hex: &str) -> Option<egui::Color32> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(egui::Color32::from_rgb(r, g, b))
}
