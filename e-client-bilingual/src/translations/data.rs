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
        tr_en_zh("Welcome to Redis Client", "欢迎使用 Redis 客户端"),
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
        tr_en_zh("Focus Filter", "聚焦过滤器"),
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
        tr_en_zh("Toggle Command Line", "切换命令行"),
    );
    strings.insert(
        TranslationKey::ShortcutCloseCommandLine,
        tr_en_zh("Close Command Line", "关闭命令行"),
    );

    // AI Settings
    strings.insert(
        TranslationKey::AiSettings,
        tr_en_zh("AI Settings", "AI 设置"),
    );
    strings.insert(TranslationKey::AiModels, tr_en_zh("AI Models", "AI 模型"));
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
        tr_en_zh("Confirm Before Execute", "执行前确认"),
    );
    strings.insert(
        TranslationKey::AiShowThinking,
        tr_en_zh("Show Thinking", "显示思考过程"),
    );
    strings.insert(TranslationKey::AiDisable, tr_en_zh("Disable", "禁用"));
    strings.insert(TranslationKey::AiEnable, tr_en_zh("Enable", "启用"));
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
        tr_en_zh("System Prompt", "系统提示词"),
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
