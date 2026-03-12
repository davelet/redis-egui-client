use crate::language::Language;
use std::collections::HashMap;

pub struct Translator {
    strings: HashMap<&'static str, HashMap<Language, &'static str>>,
}

impl Default for Translator {
    fn default() -> Self {
        let mut strings = HashMap::new();

        // Include all translation definitions
        Self::load_translations(&mut strings);

        Self { strings }
    }
}

impl Translator {
    pub fn get<'a>(&'a self, key: &'a str, lang: Language) -> &'a str {
        self.strings
            .get(key)
            .and_then(|m| m.get(&lang))
            .map(|s| s.as_ref())
            .unwrap_or_else(move || {
                eprintln!(
                    "Missing translation for key: {} in language: {:?}",
                    key, lang
                );
                key
            })
    }

    pub fn format(&self, key: &str, lang: Language, args: &[&str]) -> String {
        let template = self.get(key, lang);
        let mut result = template.to_string();

        for arg in args {
            result = result.replacen("{}", arg, 1);
        }

        result
    }

    /// Load all translation strings
    fn load_translations(strings: &mut HashMap<&'static str, HashMap<Language, &'static str>>) {
        use super::keys;
        use Language::*;

        // Connection translations
        strings.insert(keys::CONNECT, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connect");
            m.insert(Language::Chinese, "连接");
            m
        });

        strings.insert(keys::NEW_CONNECTION, {
            let mut m = HashMap::new();
            m.insert(Language::Chinese, "新建连接");
            m.insert(Language::English, "New Connection");
            m
        });

        strings.insert(keys::EDIT_CONNECTION, {
            let mut m = HashMap::new();
            m.insert(Language::Chinese, "编辑连接");
            m.insert(Language::English, "Edit Connection");
            m
        });

        strings.insert(keys::OPEN_IN_NEW_TAB, {
            let mut m = HashMap::new();
            m.insert(Language::Chinese, "在新标签页打开");
            m.insert(Language::English, "Open in New Tab");
            m
        });

        strings.insert(keys::DISCONNECT, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Disconnect");
            m.insert(Language::Chinese, "断开连接");
            m
        });

        strings.insert(keys::CONNECTION_URL, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection:");
            m.insert(Language::Chinese, "连接:");
            m
        });

        strings.insert(keys::DATABASE, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Database:");
            m.insert(Language::Chinese, "数据库:");
            m
        });

        strings.insert(keys::TAB, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Tab ");
            m.insert(Language::Chinese, "标签页");
            m
        });

        strings.insert(keys::CLOSE, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Close");
            m.insert(Language::Chinese, "关闭");
            m
        });

        strings.insert(keys::CLOSE_OTHERS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Close Others");
            m.insert(Language::Chinese, "关闭其他");
            m
        });

        strings.insert(keys::DUPLICATE, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Duplicate");
            m.insert(Language::Chinese, "复制");
            m
        });

        // Keys
        strings.insert(keys::KEYS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Keys");
            m.insert(Language::Chinese, "键");
            m
        });

        strings.insert(keys::FILTER, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Filter:");
            m.insert(Language::Chinese, "过滤:");
            m
        });

        // Command
        strings.insert(keys::COMMAND, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Command");
            m.insert(Language::Chinese, "命令");
            m
        });

        strings.insert(keys::EXECUTE, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Execute");
            m.insert(Language::Chinese, "执行");
            m
        });

        // Value
        strings.insert(keys::VALUE, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Value");
            m.insert(Language::Chinese, "值");
            m
        });

        // Language
        strings.insert(keys::LANGUAGE, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Language");
            m.insert(Language::Chinese, "语言");
            m
        });

        strings.insert(keys::SETTINGS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Settings");
            m.insert(Language::Chinese, "设置");
            m
        });

        strings.insert(keys::AUTO_CONNECT, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Auto Connect");
            m.insert(Language::Chinese, "自动连接");
            m
        });

        strings.insert(keys::NEW_KEY, {
            let mut m = HashMap::new();
            m.insert(Language::English, "New Key");
            m.insert(Language::Chinese, "新增键");
            m
        });

        strings.insert(keys::EDIT_ELEMENT, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Edit Element");
            m.insert(Language::Chinese, "编辑元素");
            m
        });

        // Errors
        strings.insert(keys::CONNECTION_FAILED, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection failed: {}");
            m.insert(Language::Chinese, "连接失败: {}");
            m
        });

        // Other UI strings
        strings.insert(keys::SELECT_CONNECTION, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Select connection");
            m.insert(Language::Chinese, "选择连接");
            m
        });

        strings.insert(keys::PLEASE_SELECT_CONNECTION, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Please select a connection");
            m.insert(Language::Chinese, "请先选择一个连接");
            m
        });

        strings.insert(keys::SAVE, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Save");
            m.insert(Language::Chinese, "保存");
            m
        });

        strings.insert(keys::CANCEL, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Cancel");
            m.insert(Language::Chinese, "取消");
            m
        });

        strings.insert(keys::CONNECTION_NAME, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection name:");
            m.insert(Language::Chinese, "连接名称:");
            m
        });

        strings.insert(keys::CONNECTION_ADDRESS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection address:");
            m.insert(Language::Chinese, "连接地址:");
            m
        });

        strings.insert(keys::CONNECTION_PORT, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection port:");
            m.insert(Language::Chinese, "连接端口:");
            m
        });

        strings.insert(keys::CONNECTION_USERNAME, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection username:");
            m.insert(Language::Chinese, "连接用户名:");
            m
        });

        strings.insert(keys::CONNECTION_PASSWORD, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection password:");
            m.insert(Language::Chinese, "连接密码:");
            m
        });

        strings.insert(keys::CONNECTION_COLOR, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection color:");
            m.insert(Language::Chinese, "连接颜色:");
            m
        });

        strings.insert(keys::PLEASE_ENTER_CONNECTION_NAME, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Please enter connection name");
            m.insert(Language::Chinese, "请输入连接名称");
            m
        });

        strings.insert(keys::PLEASE_ENTER_CONNECTION_ADDRESS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Please enter connection address");
            m.insert(Language::Chinese, "请输入连接地址");
            m
        });

        strings.insert(keys::NEW_CONNECTION_DIALOG, {
            let mut m = HashMap::new();
            m.insert(Language::English, "New Redis Connection");
            m.insert(Language::Chinese, "新建 Redis 连接");
            m
        });

        strings.insert(keys::EDIT_CONNECTION_DIALOG, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Modifying Redis Connection");
            m.insert(Language::Chinese, "修改 Redis 连接");
            m
        });

        strings.insert(keys::COMMAND_LABEL, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Command:");
            m.insert(Language::Chinese, "命令:");
            m
        });

        strings.insert(keys::OUTPUT, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Output:");
            m.insert(Language::Chinese, "输出:");
            m
        });

        strings.insert(keys::SELECT_KEY_PROMPT, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Select a key to view details");
            m.insert(Language::Chinese, "选择一个 key 查看详情");
            m
        });

        strings.insert(keys::KEY_NOT_EXIST, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Key does not exist");
            m.insert(Language::Chinese, "Key 不存在");
            m
        });

        strings.insert(keys::KEY_HEADING, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Key: {}");
            m.insert(Language::Chinese, "Key: {}");
            m
        });

        strings.insert(keys::TYPE_STRING, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Type: String");
            m.insert(Language::Chinese, "类型: String");
            m
        });

        strings.insert(keys::TYPE_LIST, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Type: List (length: {})");
            m.insert(Language::Chinese, "类型: List (长度: {})");
            m
        });

        strings.insert(keys::TYPE_HASH, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Type: Hash (fields: {})");
            m.insert(Language::Chinese, "类型: Hash (字段数: {})");
            m
        });

        strings.insert(keys::TYPE_SET, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Type: Set (members: {})");
            m.insert(Language::Chinese, "类型: Set (成员数: {})");
            m
        });

        strings.insert(keys::TYPE_ZSET, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Type: ZSet (members: {})");
            m.insert(Language::Chinese, "类型: ZSet (成员数: {})");
            m
        });

        strings.insert(keys::LOAD_FIRST_100, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Load first 100 items");
            m.insert(Language::Chinese, "加载前100项");
            m
        });

        strings.insert(keys::LOAD_FIELDS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Load fields");
            m.insert(Language::Chinese, "加载字段");
            m
        });

        strings.insert(keys::LOAD_MEMBERS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Load members");
            m.insert(Language::Chinese, "加载成员");
            m
        });

        strings.insert(keys::LOAD_MORE_KEYS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Load more keys");
            m.insert(Language::Chinese, "再加载一些");
            m
        });

        strings.insert(keys::LOAD_ALL_KEYS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Load all remaining keys");
            m.insert(Language::Chinese, "加载剩余所有");
            m
        });

        strings.insert(keys::TOO_MANY_KEYS, {
            let mut m = HashMap::new();
            m.insert(
                Language::English,
                "Too many keys loaded, please enter proper filter keyword",
            );
            m.insert(Language::Chinese, "加载的列表过长,请输入合适关键字过滤");
            m
        });

        strings.insert(keys::CONFIG_PARAM_TOO_BIG, {
            let mut m = HashMap::new();
            m.insert(
                Language::English,
                "Config param is too long: {}, the length of '{}' must be less than {}",
            );
            m.insert(Language::Chinese, "参数超长：{} '{}' 长度不能超过{}");
            m
        });

        strings.insert(keys::CONFIG_HOME_DIR_MISSING, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Could not determine home directory");
            m.insert(Language::Chinese, "无法获取用户目录");
            m
        });

        strings.insert(keys::CONFIG_READ_FAILED, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Failed to read config file: {}");
            m.insert(Language::Chinese, "读取配置文件失败: {}");
            m
        });

        strings.insert(keys::CONFIG_PARSE_FAILED, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Failed to parse config file: {}");
            m.insert(Language::Chinese, "解析配置文件失败: {}");
            m
        });

        strings.insert(keys::CONFIG_CREATE_DIR_FAILED, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Failed to create config directory: {}");
            m.insert(Language::Chinese, "创建配置目录失败: {}");
            m
        });

        strings.insert(keys::CONFIG_WRITE_FAILED, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Failed to write config file: {}");
            m.insert(Language::Chinese, "写入配置文件失败: {}");
            m
        });

        strings.insert(keys::CONNECTION_NAME_EXISTS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection name already exists");
            m.insert(Language::Chinese, "连接名称已存在");
            m
        });

        strings.insert(keys::CONNECTION_NOT_FOUND, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection not found");
            m.insert(Language::Chinese, "连接不存在");
            m
        });

        strings.insert(keys::EMPTY_COMMAND, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Empty command");
            m.insert(Language::Chinese, "命令为空");
            m
        });

        strings.insert(keys::GENERIC_ERROR, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Error: {}");
            m.insert(Language::Chinese, "错误: {}");
            m
        });

        strings.insert(keys::GET_VALUE_FAILED, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Failed to get value: {}");
            m.insert(Language::Chinese, "获取值失败: {}");
            m
        });

        strings.insert(keys::STATUS_BAR, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Status");
            m.insert(Language::Chinese, "状态");
            m
        });

        strings.insert(keys::TOTAL_KEYS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Total:");
            m.insert(Language::Chinese, "总数:");
            m
        });

        strings.insert(keys::LOADED_KEYS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Loaded:");
            m.insert(Language::Chinese, "已加载:");
            m
        });

        strings.insert(keys::LOADING, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Loading...");
            m.insert(Language::Chinese, "加载中...");
            m
        });

        strings.insert(keys::READY, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Ready");
            m.insert(Language::Chinese, "就绪");
            m
        });

        strings.insert(keys::DISCONNECTED, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Disconnected");
            m.insert(Language::Chinese, "未连接");
            m
        });

        strings.insert(keys::WELCOME_TITLE, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Welcome to Redis Client");
            m.insert(Language::Chinese, "欢迎使用 Redis 客户端");
            m
        });

        strings.insert(keys::WELCOME_MESSAGE, {
            let mut m = HashMap::new();
            m.insert(
                Language::English,
                "Connect to a Redis server to start managing your keys and data.",
            );
            m.insert(
                Language::Chinese,
                "连接到 Redis 服务器以开始管理您的键和数据。",
            );
            m
        });

        strings.insert(keys::WELCOME_INSTRUCTION, {
            let mut m = HashMap::new();
            m.insert(
                Language::English,
                "Click the button above to create a new Redis server.",
            );
            m.insert(Language::Chinese, "点击上方按钮以创建新的 Redis 服务器。");
            m
        });

        strings.insert(keys::OPEN_CONNECTIONS_PROMPT_TITLE, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Open Connections");
            m.insert(Language::Chinese, "未关闭的连接");
            m
        });

        strings.insert(keys::OPEN_CONNECTIONS_PROMPT_MESSAGE, {
            let mut m = HashMap::new();
            m.insert(
                Language::English,
                "The following coloured connections were not closed in the last session. Click 'Connect All' to reconnect",
            );
            m.insert(
                Language::Chinese,
                "这种颜色的连接在上一次会话中未关闭。点击'全部连接'可重新连接",
            );
            m
        });

        strings.insert(keys::CONNECT_ALL, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connect All");
            m.insert(Language::Chinese, "全部连接");
            m
        });

        strings.insert(keys::SAVED_CONNECTIONS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Saved Connections");
            m.insert(Language::Chinese, "已保存的连接");
            m
        });

        strings.insert(keys::ACTION, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Action");
            m.insert(Language::Chinese, "操作");
            m
        });

        strings.insert(keys::KEYBOARD_SHORTCUTS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Keyboard Shortcuts");
            m.insert(Language::Chinese, "键盘快捷键");
            m
        });

        strings.insert(keys::SHORTCUT_NEW_TAB, {
            let mut m = HashMap::new();
            m.insert(Language::English, "New Tab");
            m.insert(Language::Chinese, "新建标签页");
            m
        });

        strings.insert(keys::SHORTCUT_CLOSE_TAB, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Close Tab");
            m.insert(Language::Chinese, "关闭标签页");
            m
        });

        strings.insert(keys::SHORTCUT_REFRESH_KEY, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Refresh Key");
            m.insert(Language::Chinese, "刷新键");
            m
        });

        strings.insert(keys::SHORTCUT_FOCUS_FILTER, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Focus Filter");
            m.insert(Language::Chinese, "聚焦过滤器");
            m
        });

        strings.insert(keys::SHORTCUT_PRESS_KEYS, {
            let mut m = HashMap::new();
            m.insert(
                Language::English,
                "Press the key combination you want to set...",
            );
            m.insert(Language::Chinese, "按下想要设置的快捷键组合...");
            m
        });

        strings.insert(keys::SHORTCUT_CONFLICTS_WITH, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Conflicts with '{}'");
            m.insert(Language::Chinese, "与'{}'冲突");
            m
        });

        strings.insert(keys::SHORTCUT_RESET_DEFAULTS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Reset to Defaults");
            m.insert(Language::Chinese, "恢复默认设置");
            m
        });

        strings.insert(keys::SHORTCUT_CLOSE_SETTINGS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Close Settings");
            m.insert(Language::Chinese, "关闭设置");
            m
        });

        strings.insert(keys::SHORTCUT_OPEN_SETTINGS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Open Settings");
            m.insert(Language::Chinese, "打开设置");
            m
        });

        strings.insert(keys::SHORTCUT_TOGGLE_COMMAND_LINE, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Toggle Command Line");
            m.insert(Language::Chinese, "切出命令行");
            m
        });

        strings.insert(keys::SHORTCUT_CLOSE_COMMAND_LINE, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Close Command Line");
            m.insert(Language::Chinese, "关闭命令行");
            m
        });

        // tab shortcuts
        strings.insert(keys::SHORTCUT_SWITCH_TO_TAB_1, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Tab 1");
            m.insert(Language::Chinese, "标签页 1");
            m
        });

        strings.insert(keys::SHORTCUT_SWITCH_TO_TAB_2, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Tab 2");
            m.insert(Language::Chinese, "标签页 2");
            m
        });

        strings.insert(keys::SHORTCUT_SWITCH_TO_TAB_3, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Tab 3");
            m.insert(Language::Chinese, "标签页 3");
            m
        });

        strings.insert(keys::SHORTCUT_SWITCH_TO_TAB_4, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Tab 4");
            m.insert(Language::Chinese, "标签页 4");
            m
        });

        strings.insert(keys::SHORTCUT_SWITCH_TO_TAB_5, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Tab 5");
            m.insert(Language::Chinese, "标签页 5");
            m
        });

        strings.insert(keys::SHORTCUT_SWITCH_TO_TAB_6, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Tab 6");
            m.insert(Language::Chinese, "标签页 6");
            m
        });

        strings.insert(keys::SHORTCUT_SWITCH_TO_TAB_7, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Tab 7");
            m.insert(Language::Chinese, "标签页 7");
            m
        });

        strings.insert(keys::SHORTCUT_SWITCH_TO_TAB_8, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Tab 8");
            m.insert(Language::Chinese, "标签页 8");
            m
        });

        strings.insert(keys::SHORTCUT_SWITCH_TO_TAB_9, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Tab 9");
            m.insert(Language::Chinese, "标签页 9");
            m
        });

        strings.insert("shortcut_switch_to_last_tab", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Last Tab");
            m.insert(Language::Chinese, "最右边标签页");
            m
        });

        strings.insert(keys::SHORTCUT_NON_EDITABLE, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Not editable");
            m.insert(Language::Chinese, "不可自定义");
            m
        });

        strings.insert(keys::COPY_KEY, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Copy Key");
            m.insert(Language::Chinese, "复制键");
            m
        });

        strings.insert(keys::COPY_VALUE, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Copy Value");
            m.insert(Language::Chinese, "复制值");
            m
        });

        strings.insert(keys::COPY_SUCCESS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Copied");
            m.insert(Language::Chinese, "已复制");
            m
        });

        strings.insert(keys::COPY_FAILED, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Copy Failed");
            m.insert(Language::Chinese, "复制失败");
            m
        });

        strings.insert(keys::EDIT, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Edit");
            m.insert(Language::Chinese, "编辑");
            m
        });

        strings.insert(keys::DELETE, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Delete");
            m.insert(Language::Chinese, "删除");
            m
        });

        strings.insert(keys::REFRESH, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Refresh");
            m.insert(Language::Chinese, "刷新");
            m
        });

        strings.insert(keys::ADD_FIELD, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Add Field");
            m.insert(Language::Chinese, "添加字段");
            m
        });

        strings.insert(keys::ADD_ITEM, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Add Item");
            m.insert(Language::Chinese, "添加项");
            m
        });

        strings.insert(keys::EDIT_TTL, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Edit TTL");
            m.insert(Language::Chinese, "编辑TTL");
            m
        });

        strings.insert(keys::UNKNOWN, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Unknown");
            m.insert(Language::Chinese, "未知");
            m
        });
    }
}

// Convenience function to get a translation
pub fn tr(key: &'static str, lang: Language) -> &'static str {
    static TRANSLATOR: std::sync::OnceLock<Translator> = std::sync::OnceLock::new();
    let translator = TRANSLATOR.get_or_init(Translator::default);
    translator.get(key, lang)
}

// Convenience function to get a formatted translation
pub fn tr_fmt(key: &'static str, lang: Language, args: &[&str]) -> String {
    static TRANSLATOR: std::sync::OnceLock<Translator> = std::sync::OnceLock::new();
    let translator = TRANSLATOR.get_or_init(Translator::default);
    translator.format(key, lang, args)
}
