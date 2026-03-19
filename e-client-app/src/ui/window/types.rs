//! Window types - data structures for the main application window
use crate::core::AppState;
use e_client_basics::constants::DEFAULT_SIDE_PANEL_WIDTH;
use e_client_config::config::ai_config::AiModel;
use e_client_config::connection::RedisConnectionConfig;
use e_client_config::language::Language;
use e_client_config::translations::{keys, tr};

/// Represents a single Redis connection tab
pub struct RedisTab {
    pub id: usize,
    pub name: String,
    pub state: AppState,
    pub selected_connection: Option<usize>,
    pub connected_color: Option<String>, // Connection color (hex) after successful connection
    pub key_filter_input: String,
    pub side_panel_width: f32, // Current side panel width for this tab
    pub command_line_panel: CommandLinePanel, // Per-tab command line panel state
}

impl RedisTab {
    pub fn new(id: usize, language: Language) -> Self {
        let mut state = AppState::new();
        state.language = std::sync::Arc::new(tokio::sync::RwLock::new(language));

        Self {
            id,
            name: format!("{}{}", tr(keys::TAB, language), id),
            state,
            selected_connection: None,
            connected_color: None,
            key_filter_input: String::new(),
            side_panel_width: DEFAULT_SIDE_PANEL_WIDTH,
            command_line_panel: CommandLinePanel::default(),
        }
    }

    pub fn with_connection(
        id: usize,
        conn_idx: usize,
        conn: RedisConnectionConfig,
        language: Language,
    ) -> Self {
        let mut tab = Self::new(id, language);
        tab.selected_connection = Some(conn_idx);
        tab.name = conn.name.clone();
        tab.connected_color = conn.color.clone();
        *tab.state.connection_param.blocking_write() = Some(conn);
        tab
    }
}

/// Dialog for creating a new key
pub struct NewKeyDialog {
    pub show: bool,
    pub key_name: String,
    pub key_type: String,
    pub value: String,
    pub ttl: String,
    pub error_message: String,
}

impl NewKeyDialog {
    pub fn new() -> Self {
        Self {
            show: false,
            key_name: String::new(),
            key_type: "string".to_string(),
            value: String::new(),
            ttl: "-1".to_string(),
            error_message: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.key_name.clear();
        self.key_type = "string".to_string();
        self.value.clear();
        self.ttl = "-1".to_string();
        self.error_message.clear();
    }
}

impl Default for NewKeyDialog {
    fn default() -> Self {
        Self::new()
    }
}

/// Dialog for editing an element (hash field, list item, etc.)
pub struct ElementEditDialog {
    pub show: bool,
    pub key: String,
    pub field: String, // For hash: field name, for list/set/zset: index
    pub value: String,
    pub original_value: String,
    pub key_type: String,  // hash, list, set, zset
    pub just_opened: bool, // Track if dialog just opened for auto-formatting
}

impl Default for ElementEditDialog {
    fn default() -> Self {
        Self {
            show: false,
            key: String::new(),
            field: String::new(),
            value: String::new(),
            original_value: String::new(),
            key_type: String::new(),
            just_opened: false,
        }
    }
}

/// Command line panel state
pub struct CommandLinePanel {
    pub show: bool,
    pub input: String,
    pub history: Vec<(String, String)>, // (command, result)
    pub scroll_to_bottom: bool,
    pub history_index: Option<usize>,
    pub saved_input: String,
}

impl Default for CommandLinePanel {
    fn default() -> Self {
        Self {
            show: false,
            input: String::new(),
            history: Vec::new(),
            scroll_to_bottom: false,
            history_index: None,
            saved_input: String::new(),
        }
    }
}

/// AI Model Editor state - manages the UI for adding/editing AI models
pub struct AiModelEditor {
    pub show: bool,
    pub editing_model_id: Option<String>,
    // Editable fields
    pub name: String,
    pub api_key: String,
    pub url: String,
    pub model_id: String,
    pub temperature: f32,
    /// Whether the models list section is collapsed
    pub models_collapsed: bool,
}

impl Default for AiModelEditor {
    fn default() -> Self {
        Self {
            show: false,
            editing_model_id: None,
            name: String::new(),
            api_key: String::new(),
            url: String::new(),
            model_id: String::new(),
            temperature: 0.7,
            models_collapsed: true,
        }
    }
}

impl AiModelEditor {
    /// Open editor for creating a new model
    pub fn open_for_new(&mut self) {
        self.show = true;
        self.editing_model_id = None;
        self.name.clear();
        self.api_key.clear();
        self.url.clear();
        self.model_id.clear();
        self.temperature = 0.7;
    }

    /// Open editor for editing an existing model
    pub fn open_for_edit(&mut self, model: &AiModel) {
        self.show = true;
        self.editing_model_id = Some(model.id.clone());
        self.name = model.name.clone();
        self.api_key = model.api_key.clone().unwrap_or_default();
        self.url = model.url.clone();
        self.model_id = model.model_id.clone();
        self.temperature = model.temperature;
    }

    /// Close the editor
    pub fn close(&mut self) {
        self.show = false;
        self.editing_model_id = None;
    }

    /// Check if currently editing an existing model
    pub fn is_editing(&self) -> bool {
        self.editing_model_id.is_some()
    }
}
