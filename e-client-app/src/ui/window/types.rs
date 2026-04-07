//! Window types - data structures for the main application window
use crate::core::AppState;
use e_client_basics::constants::DEFAULT_SIDE_PANEL_WIDTH;
use e_client_config::config::ai_config::{AiMode, AiModel};
use e_client_config::connection::RedisConnectionConfig;
use e_client_config::language::Language;
use e_client_config::translations::{tr, TranslationKey};
use e_client_core::AiResponseError;

/// A single entry in the CLI history.
#[derive(Clone)]
pub struct HistoryEntry {
    pub command: String,
    pub result: String,
}

impl HistoryEntry {
    pub fn new(command: impl Into<String>, result: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            result: result.into(),
        }
    }
}

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
            name: format!("{}{}", tr(TranslationKey::Tab, language), id),
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

/// AI chat async result
pub struct AiChatPending {
    pub thinking_idx: Option<usize>,
    pub user_input: String,
    #[allow(dead_code)]
    pub context: Option<String>,
    pub confirm_before_execute: bool,
    pub receiver: std::sync::mpsc::Receiver<Result<String, AiResponseError>>,
    /// Flag to indicate if this is using rig-agent (for multi-round conversation)
    #[allow(dead_code)]
    pub use_rig_agent: bool,
}

/// Command line panel state
pub struct CommandLinePanel {
    pub show: bool,
    pub input: String,
    pub history: Vec<HistoryEntry>,
    pub scroll_to_bottom: bool,
    pub history_index: Option<usize>,
    pub saved_input: String,
    /// Pending AI command awaiting confirmation
    pub pending_ai_command: Option<(String, String)>, // (user_input, suggested_redis_command)
    /// Pending AI chat async operation
    pub ai_chat_pending: Option<AiChatPending>,
    /// Pending Redis command execution
    pub redis_command_pending: Option<std::sync::mpsc::Receiver<Result<String, String>>>,
    /// Rig-based AI agent for multi-round conversation with tool calling
    /// Uses Mutex to allow mutable access from async context
    pub rig_agent: std::sync::Arc<tokio::sync::Mutex<Option<e_client_core::OpenAiRigAgent>>>,
    /// Current AI mode (Chat or Agent) - per-tab, not persisted
    pub current_mode: AiMode,
    /// Current model ID for this tab - per-tab, not persisted
    pub current_model_id: Option<String>,
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
            pending_ai_command: None,
            ai_chat_pending: None,
            redis_command_pending: None,
            rig_agent: std::sync::Arc::new(tokio::sync::Mutex::new(None)),
            current_mode: AiMode::Agent,
            current_model_id: None,
        }
    }
}

/// AI Model Editor state - manages the UI for adding/editing AI models
pub struct AiModelEditor {
    pub show: bool,
    pub editing_model_id: Option<String>,
    pub name: String,
    pub api_key: String,
    pub base_url: String,
    pub model_id: String,
    pub provider: String,
    pub provider_search: String,
    pub temperature: f32,
    pub models_collapsed: bool,
    pub testing_connection: bool,
    pub test_result: Option<Result<(), String>>,
    pub test_result_receiver: Option<std::sync::mpsc::Receiver<Result<(), String>>>,
}

impl Default for AiModelEditor {
    fn default() -> Self {
        Self {
            show: false,
            editing_model_id: None,
            name: String::new(),
            api_key: String::new(),
            base_url: String::new(),
            model_id: String::new(),
            provider: "Custom".to_string(),
            provider_search: String::new(),
            temperature: 0.7,
            models_collapsed: true,
            testing_connection: false,
            test_result: None,
            test_result_receiver: None,
        }
    }
}

impl AiModelEditor {
    pub fn open_for_new(&mut self) {
        self.show = true;
        self.editing_model_id = None;
        self.name.clear();
        self.api_key.clear();
        self.model_id.clear();
        self.provider = "Custom".to_string();
        self.provider_search.clear();
        // Custom provider - no default URL
        self.base_url.clear();
        self.temperature = 0.7;
        self.testing_connection = false;
        self.test_result = None;
    }

    pub fn open_for_edit(&mut self, model: &AiModel) {
        self.show = true;
        self.editing_model_id = Some(model.id.clone());
        self.name = model.name.clone();
        self.api_key = model.api_key.clone().unwrap_or_default();
        self.base_url = model.get_base_url();
        self.model_id = model.get_model_id();
        self.provider = model.provider.to_string();
        self.provider_search.clear();
        self.temperature = model.temperature;
        self.testing_connection = false;
        self.test_result = None;
    }

    /// Close the editor
    pub fn close(&mut self) {
        self.show = false;
        self.editing_model_id = None;
        self.testing_connection = false;
        self.test_result = None;
    }

    /// Check if currently editing an existing model
    pub fn is_editing(&self) -> bool {
        self.editing_model_id.is_some()
    }
}
