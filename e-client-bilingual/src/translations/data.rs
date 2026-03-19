//! Translation data - all translations are defined here using macros for brevity
use crate::language::Language;
use std::collections::HashMap;

/// Load all translations into the provided HashMap
pub fn load_all_translations(strings: &mut HashMap<&'static str, HashMap<Language, &'static str>>) {
    use super::keys;

    // Connection related
    strings.insert(keys::CONNECT, tr_en_zh("Connect", "连接"));
    strings.insert(keys::NEW_CONNECTION, tr_en_zh("New Connection", "新建连接"));
    strings.insert(
        keys::EDIT_CONNECTION,
        tr_en_zh("Edit Connection", "编辑连接"),
    );
    strings.insert(
        keys::OPEN_IN_NEW_TAB,
        tr_en_zh("Open in New Tab", "在新标签页打开"),
    );
    strings.insert(keys::DISCONNECT, tr_en_zh("Disconnect", "断开连接"));
    strings.insert(keys::CONNECTION_URL, tr_en_zh("Connection:", "连接:"));
    strings.insert(keys::DATABASE, tr_en_zh("Database:", "数据库:"));
    strings.insert(keys::TAB, tr_en_zh("Tab ", "标签页"));
    strings.insert(keys::CLOSE, tr_en_zh("Close", "关闭"));
    strings.insert(keys::CLOSE_OTHERS, tr_en_zh("Close Others", "关闭其他"));
    strings.insert(keys::DUPLICATE, tr_en_zh("Duplicate", "复制"));

    // Keys
    strings.insert(keys::KEYS, tr_en_zh("Keys", "键"));
    strings.insert(keys::FILTER, tr_en_zh("Filter:", "过滤:"));

    // Command
    strings.insert(keys::COMMAND, tr_en_zh("Command", "命令"));
    strings.insert(keys::EXECUTE, tr_en_zh("Execute", "执行"));

    // Value
    strings.insert(keys::VALUE, tr_en_zh("Value", "值"));
    strings.insert(keys::LANGUAGE, tr_en_zh("Language", "语言"));
    strings.insert(keys::SETTINGS, tr_en_zh("Settings", "设置"));
    strings.insert(keys::AUTO_CONNECT, tr_en_zh("Auto Connect", "自动连接"));
    strings.insert(keys::NEW_KEY, tr_en_zh("New Key", "新建键"));
    strings.insert(keys::EDIT_ELEMENT, tr_en_zh("Edit Element", "编辑元素"));

    // Connection dialog
    strings.insert(
        keys::CONNECTION_FAILED,
        tr_en_zh("Connection Failed", "连接失败"),
    );
    strings.insert(
        keys::CONNECTION_FAILED_MSG,
        tr_en_zh("Connection failed: {}", "连接失败: {}"),
    );
    strings.insert(
        keys::SELECT_CONNECTION,
        tr_en_zh("Select Connection", "选择连接"),
    );
    strings.insert(
        keys::PLEASE_SELECT_CONNECTION,
        tr_en_zh("Please select a connection", "请选择连接"),
    );
    strings.insert(keys::SAVE, tr_en_zh("Save", "保存"));
    strings.insert(keys::CANCEL, tr_en_zh("Cancel", "取消"));
    strings.insert(keys::CONNECTION_NAME, tr_en_zh("Name:", "名称:"));
    strings.insert(keys::CONNECTION_ADDRESS, tr_en_zh("Address:", "地址:"));
    strings.insert(keys::CONNECTION_PORT, tr_en_zh("Port:", "端口:"));
    strings.insert(keys::CONNECTION_USERNAME, tr_en_zh("Username:", "用户名:"));
    strings.insert(keys::CONNECTION_PASSWORD, tr_en_zh("Password:", "密码:"));
    strings.insert(keys::CONNECTION_COLOR, tr_en_zh("Color:", "颜色:"));
    strings.insert(
        keys::PLEASE_ENTER_CONNECTION_NAME,
        tr_en_zh("Please enter connection name", "请输入连接名称"),
    );
    strings.insert(
        keys::PLEASE_ENTER_CONNECTION_ADDRESS,
        tr_en_zh("Please enter connection address", "请输入连接地址"),
    );
    strings.insert(
        keys::NEW_CONNECTION_DIALOG,
        tr_en_zh("New Connection", "新建连接"),
    );
    strings.insert(
        keys::EDIT_CONNECTION_DIALOG,
        tr_en_zh("Edit Connection", "编辑连接"),
    );

    // Command line
    strings.insert(keys::COMMAND_LABEL, tr_en_zh("Command:", "命令:"));
    strings.insert(keys::OUTPUT, tr_en_zh("Output:", "输出:"));

    // Key operations
    strings.insert(
        keys::SELECT_KEY_PROMPT,
        tr_en_zh("Select a key to view its value", "选择键以查看值"),
    );
    strings.insert(
        keys::KEY_NOT_EXIST,
        tr_en_zh("Key does not exist", "键不存在"),
    );
    strings.insert(keys::KEY_HEADING, tr_en_zh("Key: {}", "键: {}"));
    strings.insert(keys::KEY_EXPIRED, tr_en_zh("Key has expired", "键已过期"));
    strings.insert(keys::KEY_NAME_LABEL, tr_en_zh("Name:", "名称:"));
    strings.insert(keys::TYPE_LABEL, tr_en_zh("Type:", "类型:"));
    strings.insert(keys::TTL_LABEL, tr_en_zh("TTL:", "过期时间:"));
    strings.insert(
        keys::TTL_NO_EXPIRATION_HINT,
        tr_en_zh("No expiration", "永不过期"),
    );
    strings.insert(keys::VALUE_LABEL, tr_en_zh("Value:", "值:"));
    strings.insert(
        keys::VALUE_HINT_STRING,
        tr_en_zh("Enter string value", "输入字符串值"),
    );
    strings.insert(
        keys::VALUE_HINT_LIST,
        tr_en_zh("Enter values, one per line", "输入值，每行一个"),
    );
    strings.insert(
        keys::VALUE_HINT_SET,
        tr_en_zh("Enter members, one per line", "输入成员，每行一个"),
    );
    strings.insert(
        keys::VALUE_HINT_HASH,
        tr_en_zh(
            "Enter field=value pairs, one per line",
            "输入字段=值对，每行一个",
        ),
    );
    strings.insert(
        keys::VALUE_HINT_ZSET,
        tr_en_zh(
            "Enter score=member pairs, one per line",
            "输入分数=成员对，每行一个",
        ),
    );
    strings.insert(
        keys::KEY_NAME_EMPTY_ERROR,
        tr_en_zh("Key name cannot be empty", "键名不能为空"),
    );

    // Data types
    strings.insert(keys::TYPE_STRING, tr_en_zh("Type: String", "类型: 字符串"));
    strings.insert(
        keys::TYPE_LIST,
        tr_en_zh("Type: List (length: {})", "类型: List (长度: {})"),
    );
    strings.insert(
        keys::TYPE_HASH,
        tr_en_zh("Type: Hash (fields: {})", "类型: Hash (字段数: {})"),
    );
    strings.insert(
        keys::TYPE_SET,
        tr_en_zh("Type: Set (members: {})", "类型: Set (成员数: {})"),
    );
    strings.insert(
        keys::TYPE_ZSET,
        tr_en_zh("Type: ZSet (members: {})", "类型: ZSet (成员数: {})"),
    );

    // Loading
    strings.insert(
        keys::LOAD_FIRST_100,
        tr_en_zh("Load first 100", "加载前100个"),
    );
    strings.insert(keys::LOAD_FIELDS, tr_en_zh("Load fields", "加载字段"));
    strings.insert(keys::LOAD_MEMBERS, tr_en_zh("Load members", "加载成员"));
    strings.insert(
        keys::LOAD_MORE_KEYS,
        tr_en_zh("Load more keys", "加载更多键"),
    );
    strings.insert(keys::LOAD_ALL_KEYS, tr_en_zh("Load all keys", "加载所有键"));
    strings.insert(
        keys::TOO_MANY_KEYS,
        tr_en_zh("Too many keys, please use filter", "键太多，请使用过滤器"),
    );

    // Copy operations
    strings.insert(keys::COPY_KEY, tr_en_zh("Copy Key", "复制键"));
    strings.insert(keys::COPY_VALUE, tr_en_zh("Copy Value", "复制值"));
    strings.insert(keys::COPY_SUCCESS, tr_en_zh("Copied!", "已复制！"));
    strings.insert(keys::COPY_FAILED, tr_en_zh("Copy failed", "复制失败"));

    // Edit operations
    strings.insert(keys::EDIT, tr_en_zh("Edit", "编辑"));
    strings.insert(keys::DELETE, tr_en_zh("Delete", "删除"));
    strings.insert(keys::REFRESH, tr_en_zh("Refresh", "刷新"));
    strings.insert(keys::ADD_FIELD, tr_en_zh("+ Add Field", "+ 添加字段"));
    strings.insert(keys::ADD_ITEM, tr_en_zh("+ Add Item", "+ 添加项目"));
    strings.insert(keys::EDIT_TTL, tr_en_zh("Edit TTL", "编辑过期时间"));

    // Hash operations
    strings.insert(keys::HASH_FIELD, tr_en_zh("Field", "字段"));
    strings.insert(keys::HASH_VALUE, tr_en_zh("Value", "值"));
    strings.insert(
        keys::NO_VALUE_TO_EDIT,
        tr_en_zh("No value to edit", "无值可编辑"),
    );
    strings.insert(keys::VALUE_PLACEHOLDER, tr_en_zh("value", "值"));

    // ZSet operations
    strings.insert(keys::ADD_MEMBER, tr_en_zh("+ Add Member", "+ 添加成员"));
    strings.insert(keys::SCORE_LABEL, tr_en_zh("score:", "分数:"));
    strings.insert(keys::MEMBER_LABEL, tr_en_zh("member:", "成员:"));

    // Labels
    strings.insert(keys::KEY_LABEL, tr_en_zh("Key:", "键:"));
    strings.insert(keys::COLON_SEPARATOR, tr_both(":"));

    // Error handling
    strings.insert(
        keys::RESET_CONFIG_FILE,
        tr_en_zh("click to reset problematic file", "点击重置有问题的文件"),
    );
    strings.insert(keys::WELL_DONE, tr_en_zh("Well Done!", "完成！"));
    strings.insert(
        keys::RESTART_APP,
        tr_en_zh("Now restart your app.", "现在请重启您的应用。"),
    );
    strings.insert(keys::OK, tr_en_zh("OK", "确定"));
    strings.insert(keys::UNKNOWN, tr_en_zh("Unknown", "未知"));

    // Status bar
    strings.insert(keys::STATUS_BAR, tr_en_zh("Status", "状态"));
    strings.insert(keys::TOTAL_KEYS, tr_en_zh("Total Keys", "总键数"));
    strings.insert(keys::LOADED_KEYS, tr_en_zh("Loaded Keys", "已加载键数"));
    strings.insert(keys::LOADING, tr_en_zh("Loading...", "加载中..."));
    strings.insert(keys::READY, tr_en_zh("Ready", "就绪"));
    strings.insert(keys::DISCONNECTED, tr_en_zh("Disconnected", "已断开"));

    // Welcome page
    strings.insert(
        keys::WELCOME_TITLE,
        tr_en_zh("Welcome to Redis Client", "欢迎使用 Redis 客户端"),
    );
    strings.insert(
        keys::WELCOME_MESSAGE,
        tr_en_zh(
            "Connect to a Redis server to get started",
            "连接 Redis 服务器以开始使用",
        ),
    );
    strings.insert(
        keys::WELCOME_INSTRUCTION,
        tr_en_zh(
            "Select a connection from the list or create a new one",
            "从列表中选择连接或创建新连接",
        ),
    );
    strings.insert(
        keys::SAVED_CONNECTIONS,
        tr_en_zh("Saved Connections", "已保存的连接"),
    );
    strings.insert(keys::ACTION, tr_en_zh("Action", "操作"));
    strings.insert(keys::CONFIRM_DELETE, tr_en_zh("Confirm Delete", "确认删除"));

    // Open connections prompt
    strings.insert(
        keys::OPEN_CONNECTIONS_PROMPT_TITLE,
        tr_en_zh("Open Previous Connections", "打开之前的连接"),
    );
    strings.insert(keys::OPEN_CONNECTIONS_PROMPT_MESSAGE, tr_en_zh(
        "The following coloured connections were not closed in the last session. Click 'Connect All' to reconnect",
        r##"以下有颜色的连接在上一次会话中未关闭。点击"全部连接"可重新连接"##
    ));
    strings.insert(keys::CONNECT_ALL, tr_en_zh("Connect All", "连接全部"));

    // Settings
    strings.insert(
        keys::SHOW_UNCLOSED_CONNECTIONS,
        tr_en_zh("Show Unclosed Connections", "显示未关闭的连接"),
    );
    strings.insert(
        keys::ALLOW_DUPLICATE_CONNECTIONS,
        tr_en_zh("Allow Duplicate Connections", "允许重复连接"),
    );
    strings.insert(
        keys::GROUP_KEYS_BY_COLON,
        tr_en_zh("Group Keys by Colon", "按冒号分组键"),
    );

    // Keyboard shortcuts
    strings.insert(
        keys::KEYBOARD_SHORTCUTS,
        tr_en_zh("Keyboard Shortcuts", "键盘快捷键"),
    );
    strings.insert(keys::SHORTCUT_NEW_TAB, tr_en_zh("New Tab", "新建标签页"));
    strings.insert(
        keys::SHORTCUT_CLOSE_TAB,
        tr_en_zh("Close Tab", "关闭标签页"),
    );
    strings.insert(
        keys::SHORTCUT_REFRESH_KEY,
        tr_en_zh("Refresh Key", "刷新键"),
    );
    strings.insert(
        keys::SHORTCUT_FOCUS_FILTER,
        tr_en_zh("Focus Filter", "聚焦过滤器"),
    );
    strings.insert(
        keys::SHORTCUT_PRESS_KEYS,
        tr_en_zh("Press keys...", "按快捷键..."),
    );
    strings.insert(
        keys::SHORTCUT_CONFLICTS_WITH,
        tr_en_zh("Conflicts with", "与以下冲突"),
    );
    strings.insert(
        keys::SHORTCUT_RESET_DEFAULTS,
        tr_en_zh("Reset to Defaults", "重置为默认值"),
    );
    strings.insert(
        keys::SHORTCUT_CLOSE_SETTINGS,
        tr_en_zh("Close Settings", "关闭设置"),
    );
    strings.insert(
        keys::SHORTCUT_OPEN_SETTINGS,
        tr_en_zh("Open Settings", "打开设置"),
    );
    strings.insert(
        keys::SHORTCUT_TOGGLE_COMMAND_LINE,
        tr_en_zh("Toggle Command Line", "切换命令行"),
    );
    strings.insert(
        keys::SHORTCUT_CLOSE_COMMAND_LINE,
        tr_en_zh("Close Command Line", "关闭命令行"),
    );

    // AI Settings
    strings.insert(keys::AI_SETTINGS, tr_en_zh("AI Settings", "AI 设置"));
    strings.insert(keys::AI_MODELS, tr_en_zh("AI Models", "AI 模型"));
    strings.insert(keys::AI_ADD_MODEL, tr_en_zh("Add Model", "添加模型"));
    strings.insert(keys::AI_EDIT_MODEL, tr_en_zh("Edit Model", "编辑模型"));
    strings.insert(keys::AI_DELETE_MODEL, tr_en_zh("Delete Model", "删除模型"));
    strings.insert(keys::AI_MODEL_NAME, tr_en_zh("Model Name", "模型名称"));
    strings.insert(keys::AI_MODEL_ID, tr_en_zh("Model ID", "模型 ID"));
    strings.insert(keys::AI_API_KEY, tr_en_zh("API Key", "API 密钥"));
    strings.insert(keys::AI_URL, tr_en_zh("URL", "网址"));
    strings.insert(keys::AI_TEMPERATURE, tr_en_zh("Temperature", "温度"));
    strings.insert(keys::AI_ACTIVE_MODEL, tr_en_zh("Active Model", "当前模型"));
    strings.insert(
        keys::AI_CONFIRM_BEFORE_EXECUTE,
        tr_en_zh("Confirm Before Execute", "执行前确认"),
    );
    strings.insert(
        keys::AI_SHOW_THINKING,
        tr_en_zh("Show Thinking", "显示思考过程"),
    );
    strings.insert(keys::AI_DISABLE, tr_en_zh("Disable", "禁用"));
    strings.insert(keys::AI_ENABLE, tr_en_zh("Enable", "启用"));
    strings.insert(keys::AI_SELECT_MODEL, tr_en_zh("Select Model", "选择模型"));
    strings.insert(
        keys::AI_MODEL_REQUIRED_FIELDS,
        tr_en_zh("Name and Model ID are required", "名称和模型 ID 为必填项"),
    );

    // Tab switching shortcuts
    strings.insert(
        keys::SHORTCUT_SWITCH_TO_TAB_1,
        tr_en_zh("Switch to Tab 1", "切换到标签页 1"),
    );
    strings.insert(
        keys::SHORTCUT_SWITCH_TO_TAB_2,
        tr_en_zh("Switch to Tab 2", "切换到标签页 2"),
    );
    strings.insert(
        keys::SHORTCUT_SWITCH_TO_TAB_3,
        tr_en_zh("Switch to Tab 3", "切换到标签页 3"),
    );
    strings.insert(
        keys::SHORTCUT_SWITCH_TO_TAB_4,
        tr_en_zh("Switch to Tab 4", "切换到标签页 4"),
    );
    strings.insert(
        keys::SHORTCUT_SWITCH_TO_TAB_5,
        tr_en_zh("Switch to Tab 5", "切换到标签页 5"),
    );
    strings.insert(
        keys::SHORTCUT_SWITCH_TO_TAB_6,
        tr_en_zh("Switch to Tab 6", "切换到标签页 6"),
    );
    strings.insert(
        keys::SHORTCUT_SWITCH_TO_TAB_7,
        tr_en_zh("Switch to Tab 7", "切换到标签页 7"),
    );
    strings.insert(
        keys::SHORTCUT_SWITCH_TO_TAB_8,
        tr_en_zh("Switch to Tab 8", "切换到标签页 8"),
    );
    strings.insert(
        keys::SHORTCUT_SWITCH_TO_TAB_9,
        tr_en_zh("Switch to Tab 9", "切换到标签页 9"),
    );
    strings.insert(
        keys::SHORTCUT_SWITCH_TO_LAST_TAB,
        tr_en_zh("Switch to Last Tab", "切换到最后一个标签页"),
    );
    strings.insert(
        keys::SHORTCUT_NON_EDITABLE,
        tr_en_zh("Non-editable", "不可编辑"),
    );
    strings.insert(
        keys::SHORTCUT_REMOVE_DUPLICATE_AND_INVALID_TABS,
        tr_en_zh("Remove Duplicate and Invalid Tabs", "移除重复和无效标签页"),
    );
    strings.insert(keys::ALL_TABS, tr_en_zh("All Tabs", "所有标签页"));
    strings.insert(
        keys::REMOVE_DUPLICATE_AND_INVALID_TABS,
        tr_en_zh("Remove Duplicate and Invalid Tabs", "移除重复和无效标签页"),
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
