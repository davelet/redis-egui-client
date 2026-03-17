use super::JsonValue;

#[derive(Debug, Clone)]
pub struct EditState {
    pub editing: bool,
    pub edited_key: String,
    pub edited_ttl: String,
    pub original_ttl: i64, // TTL at edit start time
    pub edited_value: super::EditedValue,
    pub save_message: String,
    pub saving: bool,
}

impl Default for EditState {
    fn default() -> Self {
        Self {
            editing: false,
            edited_key: String::new(),
            edited_ttl: String::new(),
            original_ttl: -1,
            edited_value: super::EditedValue::None,
            save_message: String::new(),
            saving: false,
        }
    }
}

impl EditState {
    pub fn enter_edit(
        &mut self,
        key: &str,
        ttl: i64,
        value: &Option<crate::redis_client::ValueData>,
    ) {
        use super::EditedValue;
        use super::JsonValue;
        use crate::redis_client::ValueData;

        self.editing = true;
        self.edited_key = key.to_string();
        self.original_ttl = ttl; // Store original TTL at edit start
        self.edited_ttl = if ttl == -1 {
            "-1".to_string()
        } else if ttl >= 0 {
            ttl.to_string()
        } else {
            String::new()
        };
        self.edited_value = match value {
            Some(ValueData::String(s)) => {
                // Auto-format JSON if valid, track original format
                EditedValue::String(JsonValue::new(s))
            }
            Some(ValueData::List { items, .. }) => {
                // Auto-format JSON for each item, track original format
                EditedValue::List(items.iter().map(|s| JsonValue::new(s)).collect())
            }
            Some(ValueData::Hash {
                fields,
                loaded_values,
                ..
            }) => {
                let hash_fields: Vec<(String, JsonValue)> = fields
                    .iter()
                    .map(|f| {
                        let value = loaded_values.get(f).cloned().unwrap_or_default();
                        (f.clone(), JsonValue::new(&value))
                    })
                    .collect();
                EditedValue::Hash(hash_fields)
            }
            Some(ValueData::Set { items, .. }) => {
                // Auto-format JSON for each item, track original format
                EditedValue::Set(items.iter().map(|s| JsonValue::new(s)).collect())
            }
            Some(ValueData::ZSet { items, .. }) => EditedValue::ZSet(
                items
                    .iter()
                    .map(|(m, s)| (JsonValue::new(m), s.to_string()))
                    .collect(),
            ),
            _ => EditedValue::None,
        };
        self.save_message.clear();
        self.saving = false;
    }

    pub fn cancel_edit(&mut self) {
        self.editing = false;
        self.save_message.clear();
        self.saving = false;
    }
}

/// Edited value representation
#[derive(Debug, Clone)]
pub enum EditedValue {
    String(JsonValue),
    List(Vec<JsonValue>),
    Hash(Vec<(String, JsonValue)>),
    Set(Vec<JsonValue>),
    ZSet(Vec<(JsonValue, String)>),
    None,
}
