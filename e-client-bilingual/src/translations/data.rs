//! Translation data - all translations are defined here using macros for brevity
use crate::language::Language;
use crate::translations::keys::TranslationKey;
use std::collections::HashMap;

/// Load all translations into the provided HashMap
pub fn load_all_translations(
    strings: &mut HashMap<TranslationKey, HashMap<Language, &'static str>>,
) {
    // Connection related
    strings.insert(TranslationKey::Connect, tr_en_zh("Connect", "连接"));
    strings.insert(
        TranslationKey::NewConnection,
        tr_en_zh("New Connection", "新建连接"),
    );
    strings.insert(
        TranslationKey::EditConnection,
        tr_en_zh("Edit Connection", "编辑连接"),
    );
    strings.insert(
        TranslationKey::OpenInNewTab,
        tr_en_zh("Open in New Tab", "在新标签页打开"),
    );
    strings.insert(
        TranslationKey::Disconnect,
        tr_en_zh("Disconnect", "断开连接"),
    );
    strings.insert(
        TranslationKey::ConnectionUrl,
        tr_en_zh("Connection:", "连接:"),
    );
    strings.insert(TranslationKey::Database, tr_en_zh("Database:", "数据库:"));
    strings.insert(TranslationKey::Tab, tr_en_zh("Tab ", "标签页"));
    strings.insert(TranslationKey::Close, tr_en_zh("Close", "关闭"));
    strings.insert(
        TranslationKey::CloseOthers,
        tr_en_zh("Close Others", "关闭其他"),
    );
    strings.insert(TranslationKey::Duplicate, tr_en_zh("Duplicate", "复制"));

    // Keys
    strings.insert(TranslationKey::Keys, tr_en_zh("Keys", "键"));
    strings.insert(TranslationKey::Filter, tr_en_zh("Filter:", "过滤:"));

    // Command
    strings.insert(TranslationKey::Command, tr_en_zh("Command", "命令"));
    strings.insert(TranslationKey::Execute, tr_en_zh("Execute", "执行"));

    // Value
    strings.insert(TranslationKey::Value, tr_en_zh("Value", "值"));
    strings.insert(TranslationKey::Language, tr_en_zh("Language", "语言"));
    strings.insert(TranslationKey::Settings, tr_en_zh("Settings", "设置"));
    strings.insert(
        TranslationKey::AutoConnect,
        tr_en_zh("Auto Connect", "自动连接"),
    );
    strings.insert(TranslationKey::NewKey, tr_en_zh("New Key", "新建键"));
    strings.insert(
        TranslationKey::EditElement,
        tr_en_zh("Edit Element", "编辑元素"),
    );

    // Connection dialog
    strings.insert(
        TranslationKey::ConnectionFailed,
        tr_en_zh("Connection Failed", "连接失败"),
    );
    strings.insert(
        TranslationKey::ConnectionFailedMsg,
        tr_en_zh("Connection failed: {}", "连接失败: {}"),
    );
    strings.insert(
        TranslationKey::ConfigParseFailed,
        tr_en_zh("Config parse failed: {}", "配置解析失败: {}"),
    );
    strings.insert(
        TranslationKey::SelectConnection,
        tr_en_zh("Select Connection", "选择连接"),
    );
    strings.insert(
        TranslationKey::PleaseSelectConnection,
        tr_en_zh("Please select a connection", "请选择连接"),
    );
    strings.insert(TranslationKey::Save, tr_en_zh("Save", "保存"));
    strings.insert(TranslationKey::Cancel, tr_en_zh("Cancel", "取消"));
    strings.insert(TranslationKey::ConnectionName, tr_en_zh("Name:", "名称:"));
    strings.insert(
        TranslationKey::ConnectionAddress,
        tr_en_zh("Address:", "地址:"),
    );
    strings.insert(TranslationKey::ConnectionPort, tr_en_zh("Port:", "端口:"));
    strings.insert(
        TranslationKey::ConnectionUsername,
        tr_en_zh("Username:", "用户名:"),
    );
    strings.insert(
        TranslationKey::ConnectionPassword,
        tr_en_zh("Password:", "密码:"),
    );
    strings.insert(TranslationKey::ConnectionColor, tr_en_zh("Color:", "颜色:"));
    strings.insert(
        TranslationKey::PleaseEnterConnectionName,
        tr_en_zh("Please enter connection name", "请输入连接名称"),
    );
    strings.insert(
        TranslationKey::PleaseEnterConnectionAddress,
        tr_en_zh("Please enter connection address", "请输入连接地址"),
    );
    strings.insert(
        TranslationKey::NewConnectionDialog,
        tr_en_zh("New Connection", "新建连接"),
    );
    strings.insert(
        TranslationKey::EditConnectionDialog,
        tr_en_zh("Edit Connection", "编辑连接"),
    );

    // Command line
    strings.insert(TranslationKey::CommandLabel, tr_en_zh("Command:", "命令:"));
    strings.insert(TranslationKey::Output, tr_en_zh("Output:", "输出:"));

    // Key operations
    strings.insert(
        TranslationKey::SelectKeyPrompt,
        tr_en_zh("Select a key to view its value", "选择键以查看值"),
    );
    strings.insert(
        TranslationKey::KeyNotExist,
        tr_en_zh("Key does not exist", "键不存在"),
    );
    strings.insert(TranslationKey::KeyHeading, tr_en_zh("Key: {}", "键: {}"));
    strings.insert(
        TranslationKey::KeyExpired,
        tr_en_zh("Key has expired", "键已过期"),
    );
    strings.insert(TranslationKey::KeyNameLabel, tr_en_zh("Name:", "名称:"));
    strings.insert(TranslationKey::TypeLabel, tr_en_zh("Type:", "类型:"));
    strings.insert(TranslationKey::TtlLabel, tr_en_zh("TTL:", "过期时间:"));
    strings.insert(
        TranslationKey::TtlNoExpirationHint,
        tr_en_zh("No expiration", "永不过期"),
    );
    strings.insert(TranslationKey::ValueLabel, tr_en_zh("Value:", "值:"));
    strings.insert(
        TranslationKey::ValueHintString,
        tr_en_zh("Enter string value", "输入字符串值"),
    );
    strings.insert(
        TranslationKey::ValueHintList,
        tr_en_zh("Enter values, one per line", "输入值，每行一个"),
    );
    strings.insert(
        TranslationKey::ValueHintSet,
        tr_en_zh("Enter members, one per line", "输入成员，每行一个"),
    );
    strings.insert(
        TranslationKey::ValueHintHash,
        tr_en_zh(
            "Enter field=value pairs, one per line",
            "输入字段=值对，每行一个",
        ),
    );
    strings.insert(
        TranslationKey::ValueHintZset,
        tr_en_zh(
            "Enter score=member pairs, one per line",
            "输入分数=成员对，每行一个",
        ),
    );
    strings.insert(
        TranslationKey::KeyNameEmptyError,
        tr_en_zh("Key name cannot be empty", "键名不能为空"),
    );

    // Data types
    strings.insert(
        TranslationKey::TypeString,
        tr_en_zh("Type: String", "类型: 字符串"),
    );
    strings.insert(
        TranslationKey::TypeList,
        tr_en_zh("Type: List (length: {})", "类型: List (长度: {})"),
    );
    strings.insert(
        TranslationKey::TypeHash,
        tr_en_zh("Type: Hash (fields: {})", "类型: Hash (字段数: {})"),
    );
    strings.insert(
        TranslationKey::TypeSet,
        tr_en_zh("Type: Set (members: {})", "类型: Set (成员数: {})"),
    );
    strings.insert(
        TranslationKey::TypeZset,
        tr_en_zh("Type: ZSet (members: {})", "类型: ZSet (成员数: {})"),
    );

    // Loading
    strings.insert(
        TranslationKey::LoadFirst100,
        tr_en_zh("Load first 100", "加载前100个"),
    );
    strings.insert(
        TranslationKey::LoadFields,
        tr_en_zh("Load fields", "加载字段"),
    );
    strings.insert(
        TranslationKey::LoadMembers,
        tr_en_zh("Load members", "加载成员"),
    );
    strings.insert(
        TranslationKey::LoadMoreKeys,
        tr_en_zh("Load more keys", "加载更多键"),
    );
    strings.insert(
        TranslationKey::LoadAllKeys,
        tr_en_zh("Load all keys", "加载所有键"),
    );
    strings.insert(
        TranslationKey::TooManyKeys,
        tr_en_zh("Too many keys, please use filter", "键太多，请使用过滤器"),
    );

    // Copy operations
    strings.insert(TranslationKey::CopyKey, tr_en_zh("Copy Key", "复制键"));
    strings.insert(TranslationKey::CopyValue, tr_en_zh("Copy Value", "复制值"));
    strings.insert(TranslationKey::CopySuccess, tr_en_zh("Copied!", "已复制！"));
    strings.insert(
        TranslationKey::CopyFailed,
        tr_en_zh("Copy failed", "复制失败"),
    );

    // Edit operations
    strings.insert(TranslationKey::Edit, tr_en_zh("Edit", "编辑"));
    strings.insert(TranslationKey::Delete, tr_en_zh("Delete", "删除"));
    strings.insert(TranslationKey::Refresh, tr_en_zh("Refresh", "刷新"));
    strings.insert(
        TranslationKey::AddField,
        tr_en_zh("+ Add Field", "+ 添加字段"),
    );
    strings.insert(
        TranslationKey::AddItem,
        tr_en_zh("+ Add Item", "+ 添加项目"),
    );
    strings.insert(
        TranslationKey::EditTtl,
        tr_en_zh("Edit TTL", "编辑过期时间"),
    );

    // Hash operations
    strings.insert(TranslationKey::HashField, tr_en_zh("Field", "字段"));
    strings.insert(TranslationKey::HashValue, tr_en_zh("Value", "值"));
    strings.insert(
        TranslationKey::NoValueToEdit,
        tr_en_zh("No value to edit", "无值可编辑"),
    );
    strings.insert(TranslationKey::ValuePlaceholder, tr_en_zh("value", "值"));

    // ZSet operations
    strings.insert(
        TranslationKey::AddMember,
        tr_en_zh("+ Add Member", "+ 添加成员"),
    );
    strings.insert(TranslationKey::ScoreLabel, tr_en_zh("score:", "分数:"));
    strings.insert(TranslationKey::MemberLabel, tr_en_zh("member:", "成员:"));

    // Labels
    strings.insert(TranslationKey::KeyLabel, tr_en_zh("Key:", "键:"));
    strings.insert(TranslationKey::ColonSeparator, tr_both(":"));

    // Error handling
    strings.insert(
        TranslationKey::ResetConfigFile,
        tr_en_zh("click to reset problematic file", "点击重置有问题的文件"),
    );
    strings.insert(TranslationKey::WellDone, tr_en_zh("Well Done!", "完成！"));
    strings.insert(
        TranslationKey::RestartApp,
        tr_en_zh("Now restart your app.", "现在请重启您的应用。"),
    );
    strings.insert(TranslationKey::Ok, tr_en_zh("OK", "确定"));
    strings.insert(TranslationKey::Unknown, tr_en_zh("Unknown", "未知"));

    // Status bar
    strings.insert(TranslationKey::StatusBar, tr_en_zh("Status", "状态"));
    strings.insert(TranslationKey::TotalKeys, tr_en_zh("Total Keys", "总键数"));
    strings.insert(
        TranslationKey::LoadedKeys,
        tr_en_zh("Loaded Keys", "已加载键数"),
    );
    strings.insert(TranslationKey::Loading, tr_en_zh("Loading...", "加载中..."));
    strings.insert(TranslationKey::Ready, tr_en_zh("Ready", "就绪"));
    strings.insert(
        TranslationKey::Disconnected,
        tr_en_zh("Disconnected", "已断开"),
    );

    // Welcome page
    strings.insert(
        TranslationKey::WelcomeTitle,
        tr_en_zh("Welcome to {}", "欢迎使用{}"),
    );
    strings.insert(
        TranslationKey::WelcomeMessage,
        tr_en_zh(
            "Connect to a Redis server to get started",
            "连接 Redis 服务器以开始使用",
        ),
    );
    strings.insert(
        TranslationKey::WelcomeInstruction,
        tr_en_zh(
            "Select a connection from the list or create a new one",
            "从列表中选择连接或创建新连接",
        ),
    );
    strings.insert(
        TranslationKey::SavedConnections,
        tr_en_zh("Saved Connections", "已保存的连接"),
    );
    strings.insert(TranslationKey::Number, tr_en_zh("#", "序号"));
    strings.insert(TranslationKey::Action, tr_en_zh("Action", "操作"));
    strings.insert(
        TranslationKey::ConfirmDelete,
        tr_en_zh("Confirm Delete", "确认删除"),
    );
    strings.insert(
        TranslationKey::ShortcutNewConnection,
        tr_en_zh("New Connection", "新建连接"),
    );
    strings.insert(
        TranslationKey::ShortcutConfirmNewConnection,
        tr_en_zh("Confirm New Connection", "确认新建连接"),
    );
    strings.insert(
        TranslationKey::ShortcutCancelNewConnection,
        tr_en_zh("Cancel New Connection", "取消新建连接"),
    );
    strings.insert(
        TranslationKey::ShortcutConnectAllUnclosed,
        tr_en_zh("Connect All Unclosed", "连接所有未关闭"),
    );
    strings.insert(
        TranslationKey::ShortcutConnectConnection1,
        tr_en_zh("Connect Connection 1", "连接序号1"),
    );
    strings.insert(
        TranslationKey::ShortcutConnectConnection2,
        tr_en_zh("Connect Connection 2", "连接序号2"),
    );
    strings.insert(
        TranslationKey::ShortcutConnectConnection3,
        tr_en_zh("Connect Connection 3", "连接序号3"),
    );
    strings.insert(
        TranslationKey::ShortcutConnectConnection4,
        tr_en_zh("Connect Connection 4", "连接序号4"),
    );
    strings.insert(
        TranslationKey::ShortcutConnectConnection5,
        tr_en_zh("Connect Connection 5", "连接序号5"),
    );
    strings.insert(
        TranslationKey::ShortcutConnectConnection6,
        tr_en_zh("Connect Connection 6", "连接序号6"),
    );
    strings.insert(
        TranslationKey::ShortcutConnectConnection7,
        tr_en_zh("Connect Connection 7", "连接序号7"),
    );
    strings.insert(
        TranslationKey::ShortcutConnectConnection8,
        tr_en_zh("Connect Connection 8", "连接序号8"),
    );
    strings.insert(
        TranslationKey::ShortcutConnectConnection9,
        tr_en_zh("Connect Connection 9", "连接序号9"),
    );

    // Open connections prompt
    strings.insert(
        TranslationKey::OpenConnectionsPromptTitle,
        tr_en_zh("Open Previous Connections", "打开之前的连接"),
    );
    strings.insert(TranslationKey::OpenConnectionsPromptMessage, tr_en_zh(
        "The following coloured connections were not closed in the last session. Click 'Connect All' to reconnect",
        r##"以下有颜色的连接在上一次会话中未关闭。点击"全部连接"可重新连接"##
    ));
    strings.insert(
        TranslationKey::ConnectAll,
        tr_en_zh("Connect All", "连接全部"),
    );

    // Settings
    strings.insert(
        TranslationKey::ShowUnclosedConnections,
        tr_en_zh("Show Unclosed Connections", "显示未关闭的连接"),
    );
    strings.insert(
        TranslationKey::AllowDuplicateConnections,
        tr_en_zh("Allow Duplicate Connections", "允许重复连接"),
    );
    strings.insert(
        TranslationKey::GroupKeysByColon,
        tr_en_zh("Group Keys by Colon", "按冒号分组键"),
    );
    strings.insert(
        TranslationKey::AutoRefreshTtl,
        tr_en_zh("Auto Refresh TTL", "自动刷新过期时间"),
    );
    strings.insert(
        TranslationKey::AutoExpand,
        tr_en_zh("Auto Expand Composite Types", "自动展开复合类型"),
    );
    strings.insert(
        TranslationKey::AutoExpandThreshold,
        tr_en_zh(
            "Expand When Elements Not Exceeding",
            "元素不超过此值时自动展开",
        ),
    );

    // Keyboard shortcuts
    strings.insert(
        TranslationKey::KeyboardShortcuts,
        tr_en_zh("Keyboard Shortcuts", "键盘快捷键"),
    );
    strings.insert(
        TranslationKey::ShortcutNewTab,
        tr_en_zh("New Tab", "新建标签页"),
    );
    strings.insert(
        TranslationKey::ShortcutCloseTab,
        tr_en_zh("Close Tab", "关闭标签页"),
    );
    strings.insert(
        TranslationKey::ShortcutRefreshKey,
        tr_en_zh("Refresh Key", "刷新键"),
    );
    strings.insert(
        TranslationKey::ShortcutFocusFilter,
        tr_en_zh("Focus Filter", "过滤键"),
    );
    strings.insert(
        TranslationKey::ShortcutPressKeys,
        tr_en_zh("Press keys...", "按快捷键..."),
    );
    strings.insert(
        TranslationKey::ShortcutConflictsWith,
        tr_en_zh("Conflicts with", "与以下冲突"),
    );
    strings.insert(
        TranslationKey::ShortcutResetDefaults,
        tr_en_zh("Reset to Defaults", "重置为默认值"),
    );
    strings.insert(
        TranslationKey::ShortcutRestored,
        tr_en_zh("Restored", "重置成功"),
    );
    strings.insert(
        TranslationKey::ShortcutCloseSettings,
        tr_en_zh("Close Settings", "关闭设置"),
    );
    strings.insert(
        TranslationKey::ShortcutOpenSettings,
        tr_en_zh("Open Settings", "打开设置"),
    );
    strings.insert(
        TranslationKey::ShortcutToggleCommandLine,
        tr_en_zh("Toggle Command Line", "打开命令行"),
    );
    strings.insert(
        TranslationKey::ShortcutToggleLiveLogs,
        tr_en_zh("Toggle Live Logs", "切换日志面板"),
    );
    strings.insert(
        TranslationKey::ShortcutCloseCommandLine,
        tr_en_zh("Close Command Line", "关闭命令行"),
    );
    strings.insert(TranslationKey::ChatMode, tr_en_zh("AI Mode", "AI 模式"));
    // AI Settings
    strings.insert(
        TranslationKey::AiSettings,
        tr_en_zh("AI Settings", "AI 设置"),
    );
    strings.insert(TranslationKey::AiModels, tr_en_zh("AI Model List", "AI 模型清单"));
    strings.insert(
        TranslationKey::AiAddModel,
        tr_en_zh("Add Model", "添加模型"),
    );
    strings.insert(
        TranslationKey::AiEditModel,
        tr_en_zh("Edit Model", "编辑模型"),
    );
    strings.insert(
        TranslationKey::AiDeleteModel,
        tr_en_zh("Delete Model", "删除模型"),
    );
    strings.insert(
        TranslationKey::AiModelName,
        tr_en_zh("Model Name", "模型名称"),
    );
    strings.insert(TranslationKey::AiModelId, tr_en_zh("Model ID", "模型 ID"));
    strings.insert(TranslationKey::AiApiKey, tr_en_zh("API Key", "API 密钥"));
    strings.insert(TranslationKey::AiUrl, tr_en_zh("URL", "网址"));
    strings.insert(
        TranslationKey::AiTemperature,
        tr_en_zh("Temperature", "温度"),
    );
    strings.insert(
        TranslationKey::AiActiveModel,
        tr_en_zh("Active Model", "当前模型"),
    );
    strings.insert(
        TranslationKey::AiConfirmBeforeExecute,
        tr_en_zh(
            "Confirm Before Execute (Chat Mode)",
            "执行前确认（仅 Chat 模式）",
        ),
    );
    strings.insert(
        TranslationKey::AiShowThinking,
        tr_en_zh("Show Thinking", "显示思考过程"),
    );
    strings.insert(
        TranslationKey::AiThinking,
        tr_en_zh("Thinking...", "思考中..."),
    );
    strings.insert(
        TranslationKey::AiMaxTurns,
        tr_en_zh("Max Tool-Call Turns", "最大工具调用轮次"),
    );
    strings.insert(TranslationKey::AiMaxTurnsUnit, tr_en_zh(" turns", " 轮次"));
    strings.insert(
        TranslationKey::AiRenderMarkdown,
        tr_en_zh("Render Markdown in CLI", "CLI 渲染 Markdown"),
    );
    strings.insert(
        TranslationKey::AiEnable,
        tr_en_zh("Enable AI", "启用 AI 功能"),
    );
    strings.insert(
        TranslationKey::AiSelectModel,
        tr_en_zh("Select Model", "选择模型"),
    );
    strings.insert(
        TranslationKey::AiModelRequiredFields,
        tr_en_zh("Name and Model ID are required", "名称和模型 ID 为必填项"),
    );
    strings.insert(
        TranslationKey::AiConfirmDialogTitle,
        tr_en_zh("Confirm AI Execution", "确认AI执行"),
    );
    strings.insert(
        TranslationKey::AiConfirmDialogMessage,
        tr_en_zh(
            "Execute the following Redis command?",
            "执行以下Redis命令？",
        ),
    );
    strings.insert(TranslationKey::AiExecute, tr_en_zh("Execute", "执行"));
    strings.insert(
        TranslationKey::AiTestConnection,
        tr_en_zh("Test Connection", "测试连接"),
    );
    strings.insert(
        TranslationKey::AiTestingConnection,
        tr_en_zh("Testing...", "测试中..."),
    );
    strings.insert(
        TranslationKey::AiTestSuccess,
        tr_en_zh("Connection successful!", "连接成功！"),
    );
    // GIM Import
    strings.insert(
        TranslationKey::AiImportFromGim,
        tr_en_zh("Import from GIM", "从 GIM 导入"),
    );
    strings.insert(
        TranslationKey::AiImportFromGimTooltip,
        tr_en_zh("Import AI model configuration from Git Intelligence Message (~/.config/gim/config.toml)", "从 Git Intelligence Message (~/.config/gim/config.toml) 导入 AI 模型配置"),
    );
    strings.insert(
        TranslationKey::AiImportingTitle,
        tr_en_zh("Import from GIM", "从 GIM 导入"),
    );
    strings.insert(
        TranslationKey::AiImportModelName,
        tr_en_zh("Model", "模型"),
    );
    strings.insert(
        TranslationKey::AiImportProvider,
        tr_en_zh("Provider", "提供商"),
    );
    strings.insert(
        TranslationKey::AiImportUrl,
        tr_en_zh("URL", "地址"),
    );
    strings.insert(
        TranslationKey::AiImportApiKey,
        tr_en_zh("API Key", "API 密钥"),
    );
    strings.insert(
        TranslationKey::AiImportKeychainNote,
        tr_en_zh("Stored in system keychain", "将存储在系统密钥链中"),
    );
    strings.insert(
        TranslationKey::AiImportConfirm,
        tr_en_zh("Confirm Import", "确定导入"),
    );
    strings.insert(
        TranslationKey::AiImportSuccess,
        tr_en_zh("Import successful!", "导入成功！"),
    );
    strings.insert(
        TranslationKey::AiImportFailed,
        tr_en_zh("Import failed", "导入失败"),
    );
    strings.insert(
        TranslationKey::AiImportNotFound,
        tr_en_zh("GIM configuration not found at ~/.config/gim/config.toml", "未找到 GIM 配置文件 (~/.config/gim/config.toml)"),
    );
    strings.insert(
        TranslationKey::AiImportAlreadyExists,
        tr_en_zh("{} Model already exists", "{} 模型已存在"),
    );
    strings.insert(
        TranslationKey::AiImportOverride,
        tr_en_zh("Override existing", "覆盖现有"),
    );
    strings.insert(
        TranslationKey::AiImportSkip,
        tr_en_zh("Skip", "跳过"),
    );
    // JSON Import/Export
    strings.insert(
        TranslationKey::AiExportJson,
        tr_en_zh("Export", "导出"),
    );
    strings.insert(
        TranslationKey::AiExportJsonTooltip,
        tr_en_zh("Export AI config to JSON file", "导出 AI 配置到 JSON 文件"),
    );
    strings.insert(
        TranslationKey::AiExportSuccess,
        tr_en_zh("Export successful!", "导出成功！"),
    );
    strings.insert(
        TranslationKey::AiImportJson,
        tr_en_zh("Import", "导入"),
    );
    strings.insert(
        TranslationKey::AiImportJsonTooltip,
        tr_en_zh("Import AI config from JSON file", "从 JSON 文件导入 AI 配置"),
    );
    strings.insert(
        TranslationKey::AiImportJsonTitle,
        tr_en_zh("Import AI Configuration", "导入 AI 配置"),
    );
    strings.insert(
        TranslationKey::AiImportJsonWarning,
        tr_en_zh("{} API keys will NOT be imported. Please re-enter them after import.", "{}API 密钥不会被导入，请在导入后重新输入。"),
    );
    strings.insert(
        TranslationKey::AiImportJsonModelCount,
        tr_en_zh("Models to import", "将导入的模型"),
    );
    strings.insert(
        TranslationKey::AiImportJsonConfirm,
        tr_en_zh("Import", "导入"),
    );
    strings.insert(
        TranslationKey::AiImportJsonSuccess,
        tr_en_zh("Import successful!", "导入成功！"),
    );
    strings.insert(
        TranslationKey::AiImportSelectAll,
        tr_en_zh("Select All", "全选"),
    );
    strings.insert(
        TranslationKey::AiImportDeselectAll,
        tr_en_zh("Deselect All", "全不选"),
    );
    strings.insert(
        TranslationKey::AiImportInvertSelection,
        tr_en_zh("Invert Selection", "反选"),
    );
    strings.insert(
        TranslationKey::AiTestFailed,
        tr_en_zh("Connection failed", "连接失败"),
    );
    strings.insert(
        TranslationKey::AiErrorRateLimit,
        tr_en_zh("Rate limit exceeded", "请求频率超限"),
    );
    strings.insert(
        TranslationKey::AiErrorAuthFailed,
        tr_en_zh("Authentication failed", "认证失败"),
    );
    strings.insert(
        TranslationKey::AiErrorNetwork,
        tr_en_zh("Network error", "网络错误"),
    );
    strings.insert(
        TranslationKey::AiErrorTimeout,
        tr_en_zh("Request timeout", "请求超时"),
    );
    strings.insert(
        TranslationKey::AiSystemPrompt,
        tr_en_zh("Agent Mode Prompt", "Agent 模式提示词"),
    );
    strings.insert(
        TranslationKey::AiChatSystemPrompt,
        tr_en_zh("Chat Mode Prompt", "Chat 模式提示词"),
    );

    strings.insert(
        TranslationKey::AiNotConfigured,
        tr_en_zh("AI not configured", "AI 未配置"),
    );
    strings.insert(
        TranslationKey::AiPleaseConfigure,
        tr_en_zh(
            "Please configure an AI model in Settings > AI Settings",
            "请在 设置 > AI 设置 中配置 AI 模型",
        ),
    );
    strings.insert(
        TranslationKey::AiDisabled,
        tr_en_zh(
            "AI is disabled. Enable it in Settings.",
            "AI 已禁用。请在设置中启用。",
        ),
    );
    strings.insert(
        TranslationKey::AiErrorNoModelConfigured,
        tr_en_zh("No AI model configured", "未配置 AI 模型"),
    );
    strings.insert(
        TranslationKey::AiErrorMissingApiKey,
        tr_en_zh("API key is missing", "缺少 API 密钥"),
    );
    strings.insert(
        TranslationKey::AiErrorInvalidUrl,
        tr_en_zh("Invalid API URL", "无效的 API 地址"),
    );
    strings.insert(
        TranslationKey::AiErrorInvalidModel,
        tr_en_zh("Invalid model", "无效的模型"),
    );
    strings.insert(
        TranslationKey::AiErrorServerError,
        tr_en_zh("Server error: {}", "服务器错误: {}"),
    );
    strings.insert(
        TranslationKey::AiErrorModelNotFound,
        tr_en_zh("Model not found", "模型未找到"),
    );
    strings.insert(
        TranslationKey::AiErrorOther,
        tr_en_zh("AI error: {}", "AI 错误: {}"),
    );
    strings.insert(
        TranslationKey::AiErrorAgentCreationFailed,
        tr_en_zh("Failed to create AI agent: {}", "创建 AI 代理失败: {}"),
    );
    strings.insert(TranslationKey::AiProvider, tr_en_zh("Provider", "提供商"));
    strings.insert(
        TranslationKey::AiProviderCustom,
        tr_en_zh("Custom", "自定义"),
    );
    strings.insert(
        TranslationKey::AiProviderSearchHint,
        tr_en_zh("Type to filter providers", "输入以过滤提供商"),
    );
    strings.insert(
        TranslationKey::AiProvidersCount,
        tr_en_zh("({} providers)", "({} 个提供商)"),
    );
    strings.insert(
        TranslationKey::AiPleaseSelectProvider,
        tr_en_zh("Please select a provider", "请选择提供商"),
    );
    strings.insert(
        TranslationKey::AiProviderSearchPlaceholder,
        tr_en_zh("Search provider...", "搜索提供商..."),
    );
    strings.insert(
        TranslationKey::AiCopyPrompt,
        tr_en_zh("Copy Prompt", "复制提示词"),
    );
    strings.insert(
        TranslationKey::AiCopyChatPrompt,
        tr_en_zh("Copy Chat Prompt", "复制对话提示词"),
    );

    // Tab switching shortcuts
    strings.insert(
        TranslationKey::ShortcutSwitchToTab1,
        tr_en_zh("Switch to Tab 1", "切换到标签页 1"),
    );
    strings.insert(
        TranslationKey::ShortcutSwitchToTab2,
        tr_en_zh("Switch to Tab 2", "切换到标签页 2"),
    );
    strings.insert(
        TranslationKey::ShortcutSwitchToTab3,
        tr_en_zh("Switch to Tab 3", "切换到标签页 3"),
    );
    strings.insert(
        TranslationKey::ShortcutSwitchToTab4,
        tr_en_zh("Switch to Tab 4", "切换到标签页 4"),
    );
    strings.insert(
        TranslationKey::ShortcutSwitchToTab5,
        tr_en_zh("Switch to Tab 5", "切换到标签页 5"),
    );
    strings.insert(
        TranslationKey::ShortcutSwitchToTab6,
        tr_en_zh("Switch to Tab 6", "切换到标签页 6"),
    );
    strings.insert(
        TranslationKey::ShortcutSwitchToTab7,
        tr_en_zh("Switch to Tab 7", "切换到标签页 7"),
    );
    strings.insert(
        TranslationKey::ShortcutSwitchToTab8,
        tr_en_zh("Switch to Tab 8", "切换到标签页 8"),
    );
    strings.insert(
        TranslationKey::ShortcutSwitchToTab9,
        tr_en_zh("Switch to Tab 9", "切换到标签页 9"),
    );
    strings.insert(
        TranslationKey::ShortcutSwitchToLastTab,
        tr_en_zh("Switch to Last Tab", "切换到最后一个标签页"),
    );
    strings.insert(
        TranslationKey::ShortcutNonEditable,
        tr_en_zh("Non-editable", "不可编辑"),
    );
    strings.insert(
        TranslationKey::ShortcutUnderlineHint,
        tr_en_zh("Underlined items indicate customized shortcuts", "带下划线的项表示已自定义的快捷键"),
    );
    strings.insert(
        TranslationKey::SupportUs,
        tr_en_zh("☕ Support Us", "☕ 支持我们"),
    );
    strings.insert(
        TranslationKey::SupportUsDesc,
        tr_en_zh("Scan with WeChat to donate", "微信扫码打赏"),
    );
    strings.insert(
        TranslationKey::Recommended,
        tr_en_zh("🚀 Recommended", "🚀 推荐"),
    );
    strings.insert(
        TranslationKey::GitIntelligenceMessage,
        tr_en_zh("Git Intelligence Message", "Git Intelligence Message"),
    );
    strings.insert(
        TranslationKey::GitIntelligenceMessageDesc,
        tr_en_zh("AI-powered commit message generator", "AI 驱动的提交信息生成器"),
    );
    strings.insert(
        TranslationKey::ShortcutRemoveDuplicateAndInvalidTabs,
        tr_en_zh("Remove Duplicate and Invalid Tabs", "移除重复和无效标签页"),
    );
    strings.insert(TranslationKey::AllTabs, tr_en_zh("All Tabs", "所有标签页"));
    strings.insert(
        TranslationKey::RemoveDuplicateAndInvalidTabs,
        tr_en_zh("Remove Duplicate and Invalid Tabs", "移除重复和无效标签页"),
    );

    // Command line hint
    strings.insert(
        TranslationKey::CommandLineHint,
        tr_en_zh(
            "Enter Redis command or ask AI...",
            "输入 Redis 命令或询问 AI...",
        ),
    );

    // Refresh keys shortcut
    strings.insert(
        TranslationKey::ShortcutRefreshKeys,
        tr_en_zh("Refresh Keys (Side Panel)", "刷新键列表（侧边栏）"),
    );

    // AI command confirmation shortcuts
    strings.insert(
        TranslationKey::ShortcutExecuteAiCommand,
        tr_en_zh("Execute AI Command", "执行AI命令"),
    );
    strings.insert(
        TranslationKey::ShortcutCancelAiCommand,
        tr_en_zh("Cancel AI Command", "取消AI命令"),
    );
    // AI model editor shortcut
    strings.insert(
        TranslationKey::ShortcutCloseAiModelEditor,
        tr_en_zh("Close AI Model Editor", "关闭AI模型编辑器"),
    );
    strings.insert(TranslationKey::Help, tr_en_zh("Help", "帮助"));
    strings.insert(TranslationKey::HelpContents, tr_en_zh("Contents", "目录"));
    strings.insert(
        TranslationKey::HelpInThisSection,
        tr_en_zh("In This Section", "本节内容"),
    );
    strings.insert(
        TranslationKey::HelpOnlineDocs,
        tr_en_zh("Online Docs {}", "在线文档 {}"),
    );
    strings.insert(
        TranslationKey::HelpGithub,
        tr_en_zh("GitHub {}", "开源仓库 {}"),
    );

    // Theme settings
    strings.insert(TranslationKey::Theme, tr_en_zh("Theme", "主题"));
    strings.insert(TranslationKey::ThemeSystem, tr_en_zh("System", "跟随系统"));
    strings.insert(TranslationKey::ThemeLight, tr_en_zh("Light", "浅色"));
    strings.insert(TranslationKey::ThemeDark, tr_en_zh("Dark", "深色"));
    strings.insert(
        TranslationKey::DisplaySettings,
        tr_en_zh("Display Settings", "显示设置"),
    );

    // Quick Connect
    strings.insert(
        TranslationKey::QuickConnect,
        tr_en_zh("Quick Connect", "快速连接"),
    );
    strings.insert(
        TranslationKey::QuickConnectUrl,
        tr_en_zh("Connection URL:", "连接字符串:"),
    );
    strings.insert(
        TranslationKey::QuickConnectHint,
        tr_en_zh(
            "e.g. redis://user:pass@host:6379/0",
            "例如 redis://user:pass@host:6379/0",
        ),
    );
    strings.insert(TranslationKey::AdvancedMode, tr_en_zh("Advanced", "高级"));
    strings.insert(
        TranslationKey::InvalidConnectionString,
        tr_en_zh("Invalid connection string", "无效的连接字符串"),
    );
    strings.insert(
        TranslationKey::UnsupportedConnectionType,
        tr_en_zh("Unsupported connection type: {}", "不支持的连接类型: {}"),
    );
    strings.insert(
        TranslationKey::OpenConnectionsInNewTab,
        tr_en_zh("Open Connections in New Tab", "在新标签页打开连接"),
    );
    strings.insert(
        TranslationKey::UseTLS,
        tr_en_zh("Use TLS (rediss://)", "使用 TLS 加密连接"),
    );

    // CLI Mode labels
    strings.insert(
        TranslationKey::CliModeChat,
        tr_en_zh("Chat", "对话"),
    );
    strings.insert(
        TranslationKey::CliModeAgent,
        tr_en_zh("Agent", "代理"),
    );
    strings.insert(
        TranslationKey::CliModeChatHint,
        tr_en_zh("Stateless - translates natural language to Redis commands (no context, no tools)", "无状态 - 将自然语言转换为 Redis 命令（无上下文、无工具）"),
    );
    strings.insert(
        TranslationKey::CliModeAgentHint,
        tr_en_zh("Stateful - has access to Redis tools for direct operations (requires tool-calling capable models)", "有状态 - 可直接调用 Redis 工具执行操作（需要支持工具调用的模型）"),
    );
    strings.insert(
        TranslationKey::CliTurns,
        tr_en_zh("{} turns", "{} 轮次"),
    );

    // Live Logs
    strings.insert(
        TranslationKey::LiveLogs,
        tr_en_zh("Live Logs", "实时日志"),
    );
    strings.insert(
        TranslationKey::LiveLogsToggle,
        tr_en_zh("📋 Live Logs", "📋 实时日志"),
    );
    strings.insert(
        TranslationKey::LiveLogsShowHint,
        tr_en_zh("Show live logs during AI chat", "AI 对话期间显示实时日志"),
    );
    strings.insert(
        TranslationKey::LiveLogsClear,
        tr_en_zh("🗑 Clear", "🗑 清空"),
    );
    strings.insert(
        TranslationKey::LiveLogsClearHint,
        tr_en_zh("Clear log buffer", "清空日志缓冲区"),
    );
    strings.insert(
        TranslationKey::LiveLogsFollow,
        tr_en_zh("📜 Follow", "📜 跟随"),
    );
    strings.insert(
        TranslationKey::LiveLogsFollowActive,
        tr_en_zh("🔽 Follow", "🔽 跟随"),
    );
    strings.insert(
        TranslationKey::LiveLogsRecording,
        tr_en_zh("● Recording", "● 录制中"),
    );
    strings.insert(
        TranslationKey::LiveLogsIdle,
        tr_en_zh("○ Idle", "○ 空闲"),
    );
    strings.insert(
        TranslationKey::LiveLogsEmpty,
        tr_en_zh("No logs captured for this session", "本次会话尚未捕获日志"),
    );

    // CLI Status
    strings.insert(
        TranslationKey::Executing,
        tr_en_zh("Executing...", "执行中..."),
    );
    strings.insert(
        TranslationKey::AutoExecuteEnabled,
        tr_en_zh("(Auto-execute enabled)", "（已启用自动执行）"),
    );
    strings.insert(
        TranslationKey::ExecuteCommandHint,
        tr_en_zh("Execute the Redis command", "执行 Redis 命令"),
    );
}

/// Create a translation HashMap with English and Chinese variants
fn tr_en_zh(en: &'static str, zh: &'static str) -> HashMap<Language, &'static str> {
    let mut m = HashMap::new();
    m.insert(Language::English, en);
    m.insert(Language::Chinese, zh);
    m
}

/// Create a translation HashMap with the same text for both languages
fn tr_both(text: &'static str) -> HashMap<Language, &'static str> {
    let mut m = HashMap::new();
    m.insert(Language::English, text);
    m.insert(Language::Chinese, text);
    m
}
