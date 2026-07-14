//! Window types - data structures for the main application window
use crate::core::AppState;
use crate::ui::window::log_capture::{LogCaptureStop, start_log_capture};
use e_client_basics::constants::DEFAULT_SIDE_PANEL_WIDTH;
use e_client_config::config::ai_config::{AiMode, AiModel};
use e_client_config::connection::RedisConnectionConfig;
use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr};
use e_client_core::AiResponseError;
use std::sync::atomic;

/// Parse tool call information from log lines
pub fn parse_tool_calls_from_logs(log_lines: &[String]) -> Vec<ToolCallInfo> {
    let mut tool_calls = Vec::new();
    let mut running_tools: std::collections::HashMap<String, (String, String)> =
        std::collections::HashMap::new();

    for line in log_lines {
        if let Some(start_idx) = line.find("[ToolCall] ") {
            let after_marker = &line[start_idx + 11..];

            if let Some(space_idx) = after_marker.find(' ') {
                let tool_name = &after_marker[..space_idx];
                let rest = &after_marker[space_idx + 1..];

                if rest.starts_with("started") {
                    let args_summary = if let Some(with_idx) = rest.find(" with ") {
                        rest[with_idx + 6..].to_string()
                    } else {
                        String::new()
                    };
                    running_tools.insert(tool_name.to_string(), (args_summary, line.clone()));
                } else if rest.starts_with("completed in")
                    && let Some((args_summary, _)) = running_tools.remove(tool_name) {
                        let duration_str = &rest[13..];
                        let duration_ms = parse_duration_to_ms(duration_str);

                        tool_calls.push(ToolCallInfo {
                            name: tool_name.to_string(),
                            status: ToolCallStatus::Success,
                            duration_ms,
                            args_summary,
                        });
                    }
            }
        }
    }

    tool_calls
}

fn parse_duration_to_ms(duration_str: &str) -> Option<u64> {
    let duration_str = duration_str.trim();

    if let Some(ms_idx) = duration_str.find("ms") {
        let ms_part = &duration_str[..ms_idx].trim();
        if let Ok(ms) = ms_part.parse::<f64>() {
            return Some(ms as u64);
        }
    } else if let Some(s_idx) = duration_str.find('s') {
        let s_part = &duration_str[..s_idx].trim();
        if let Ok(s) = s_part.parse::<f64>() {
            return Some((s * 1000.0) as u64);
        }
    } else if let Some(min_idx) = duration_str.find("min") {
        let min_part = &duration_str[..min_idx].trim();
        if let Ok(min) = min_part.parse::<f64>() {
            return Some((min * 60000.0) as u64);
        }
    }

    None
}

/// A single entry in the CLI history.
#[derive(Clone)]
pub struct HistoryEntry {
    pub command: String,
    pub result: String,
    /// Optional translation key for consistent styling across language changes
    pub translation_key: Option<TranslationKey>,
    /// Tool call information if this entry represents a tool execution
    pub tool_calls: Vec<ToolCallInfo>,
}

/// Information about a single tool call
#[derive(Clone)]
pub struct ToolCallInfo {
    pub name: String,
    pub status: ToolCallStatus,
    pub duration_ms: Option<u64>,
    pub args_summary: String,
}

#[derive(Clone)]
pub enum ToolCallStatus {
    Running,
    Success,
    Error(String),
}

impl HistoryEntry {
    pub fn new(command: impl Into<String>, result: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            result: result.into(),
            translation_key: None,
            tool_calls: Vec::new(),
        }
    }

    pub fn with_translation_key(mut self, key: TranslationKey) -> Self {
        self.translation_key = Some(key);
        self
    }

    pub fn with_tool_calls(mut self, tool_calls: Vec<ToolCallInfo>) -> Self {
        self.tool_calls = tool_calls;
        self
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
    /// Timestamp of the last edit to `key_filter_input`. The actual SCAN is
    /// only triggered after this is at least `KEY_FILTER_DEBOUNCE_MS` ms in
    /// the past, so rapid typing does not fire one Redis round-trip per
    /// keystroke. `None` means there is no pending filter change.
    pub key_filter_pending_since: Option<std::time::Instant>,
    pub side_panel_width: f32, // Current side panel width for this tab
    pub command_line_panel: CommandLinePanel, // Per-tab command line panel state
    pub last_error_shown: Option<String>, // Prevent duplicate toasts
    pub was_loading: bool,     // Previous frame loading state, for transition detection
    pub pending_error_check: bool, // loading just finished; check next frames for error
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
            key_filter_pending_since: None,
            side_panel_width: DEFAULT_SIDE_PANEL_WIDTH,
            command_line_panel: CommandLinePanel::default(),
            last_error_shown: None,
            was_loading: false,
            pending_error_check: false,
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
#[derive(Default)]
pub struct ElementEditDialog {
    pub show: bool,
    pub key: String,
    pub field: String, // For hash: field name, for list/set/zset: index
    pub value: String,
    pub original_value: String,
    pub key_type: String,  // hash, list, set, zset
    pub just_opened: bool, // Track if dialog just opened for auto-formatting
}


/// AI chat async result
pub struct AiChatPending {
    pub thinking_idx: Option<usize>,
    pub user_input: String,
    #[allow(dead_code)]
    pub context: Option<String>,
    pub confirm_before_execute: bool,
    pub receiver: std::sync::mpsc::Receiver<Result<String, AiResponseError>>,
    /// Join handle for the async task, used for cancellation
    pub handle: tokio::task::JoinHandle<()>,
    /// Flag to indicate if this is using rig-agent (for multi-round conversation)
    #[allow(dead_code)]
    pub use_rig_agent: bool,
}

/// Log viewer state - UI display state only.
/// Capture logic is in the separate log_capture module.
pub struct LogViewer {
    /// Whether log viewer feature is enabled (can be toggled)
    pub enabled: bool,
    /// Buffered log lines (in display order)
    pub log_lines: std::collections::VecDeque<String>,
    /// Maximum number of lines to keep in buffer
    pub max_lines: usize,
    /// Auto-scroll to bottom when new lines arrive
    pub follow_tail: bool,
    /// Stop flag for the capture thread (None when not running)
    stop_flag: Option<LogCaptureStop>,
    /// Receiver for log lines from the capture thread
    log_receiver: Option<std::sync::mpsc::Receiver<String>>,
    /// Timestamp (Instant) when the user cancelled a pending AI command confirmation.
    /// Capture stops after a short delay to allow final log entries to be displayed.
    pub log_cancel_time: Option<std::time::Instant>,
}

const DEFAULT_MAX_LINES: usize = 300;

impl Default for LogViewer {
    fn default() -> Self {
        Self {
            enabled: false,
            log_lines: std::collections::VecDeque::new(),
            max_lines: DEFAULT_MAX_LINES,
            follow_tail: true,
            stop_flag: None,
            log_receiver: None,
            log_cancel_time: None,
        }
    }
}

impl LogViewer {
    /// Clear all buffered log lines
    pub fn clear(&mut self) {
        self.log_lines.clear();
    }

    /// Drain newly received log lines from the capture thread.
    /// Call this in the UI thread each frame.
    /// Safe: uses std::sync::mpsc so try_recv works synchronously.
    pub fn drain_received(&mut self) {
        let Some(ref receiver) = self.log_receiver else {
            return;
        };

        while let Ok(line) = receiver.try_recv() {
            self.log_lines.push_back(line);
        }

        // Trim excess from the front (O(1) with VecDeque)
        while self.log_lines.len() > self.max_lines {
            self.log_lines.pop_front();
        }
    }

    /// Check if capture is currently running
    pub fn is_capturing(&self) -> bool {
        self.stop_flag.is_some()
    }

    /// Start log capture using the app's log directory.
    /// Tails from the current end of file (only captures new lines).
    /// Safe to call from the UI thread.
    pub fn start_capture(&mut self) {
        use e_client_logging::LoggingConfig;
        let app_log_dir = LoggingConfig::default().log_dir;

        self.stop_capture();
        self.log_cancel_time = None;

        self.log_lines.clear();

        if let Some((receiver, stop_flag)) = start_log_capture(app_log_dir) {
            self.log_receiver = Some(receiver);
            self.stop_flag = Some(stop_flag);
        } else {
            self.log_lines
                .push_back("[LogCapture] No log file found".to_string());
        }
    }

    /// Stop the log capture thread gracefully.
    /// Safe to call from any thread.
    pub fn stop_capture(&mut self) {
        if let Some(stop_flag) = self.stop_flag.take() {
            stop_flag.store(true, atomic::Ordering::Relaxed);
            self.log_lines
                .push_back("--- Session ended ---".to_string());
        }
        self.log_receiver = None;
    }
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
    /// Cache of initialized AI agents, key: model_id + mode
    pub agent_cache: std::sync::Arc<
        tokio::sync::Mutex<std::collections::HashMap<String, e_client_core::OpenAiRigAgent>>,
    >,
    /// Current AI mode (Chat or Agent) - per-tab, not persisted
    pub current_mode: AiMode,
    /// Current model ID for this tab - per-tab, not persisted
    pub current_model_id: Option<String>,
    /// Log viewer for showing live logs during AI chat
    pub log_viewer: LogViewer,
    /// Flag to request aborting a pending AI chat (set by UI, consumed by process_ai_chat_results)
    pub abort_requested: bool,
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
            agent_cache: std::sync::Arc::new(tokio::sync::Mutex::new(
                std::collections::HashMap::new(),
            )),
            current_mode: AiMode::Agent,
            current_model_id: None,
            log_viewer: LogViewer::default(),
            abort_requested: false,
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

/// GIM configuration import dialog state
#[derive(Default)]
pub struct GimImportDialog {
    /// Whether to show the import dialog
    pub show: bool,
    /// The loaded GIM config (None if not loaded or not found)
    pub gim_config: Option<e_client_config::config::gim_importer::GimAiConfig>,
    /// The converted AI model (used for preview display)
    pub converted_model: Option<e_client_config::config::ai_config::AiModel>,
    /// Error message if import failed
    pub error_message: Option<String>,
    /// Whether a model with the same name already exists
    pub model_exists: bool,
}


impl GimImportDialog {
    /// Open the dialog and try to load GIM config
    pub fn open(&mut self) {
        self.show = true;
        self.error_message = None;
        self.model_exists = false;

        // Try to load GIM config
        match e_client_config::config::gim_importer::GimAiConfig::load_from_default() {
            Ok(Some(config)) => {
                // Convert to AiModel for preview display
                let converted = config.to_ai_model();
                self.converted_model = Some(converted);
                self.gim_config = Some(config);
            }
            Ok(None) => {
                // File not found
                self.gim_config = None;
                self.converted_model = None;
                self.error_message = Some("NOT_FOUND".to_string());
            }
            Err(e) => {
                self.gim_config = None;
                self.converted_model = None;
                self.error_message = Some(format!("{:?}", e));
            }
        }
    }

    /// Close the dialog and reset state
    pub fn close(&mut self) {
        self.show = false;
        self.gim_config = None;
        self.converted_model = None;
        self.error_message = None;
        self.model_exists = false;
    }
}

/// JSON import preview dialog state
pub struct JsonImportPreview {
    /// Path to the JSON file
    pub path: std::path::PathBuf,
    /// The loaded config (API keys will be None)
    pub config: e_client_config::config::ai_config::AiConfig,
    /// Selection state for each imported model
    pub selected: Vec<bool>,
}
