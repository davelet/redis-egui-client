use crate::language::Language;
use std::collections::HashMap;

pub struct Translator {
    strings: HashMap<&'static str, HashMap<Language, &'static str>>,
}

pub mod keys {
    pub const CONNECT: &str = "connect";
    pub const NEW_CONNECTION: &str = "new_connection";
    pub const EDIT_CONNECTION: &str = "edit_connection";
    pub const OPEN_IN_NEW_TAB: &str = "open_in_new_tab";
    pub const DISCONNECT: &str = "disconnect";
    pub const CONNECTION_URL: &str = "connection_url";
    pub const DATABASE: &str = "database";
    pub const TAB: &str = "tab";
    pub const KEYS: &str = "keys";
    pub const FILTER: &str = "filter";
    pub const COMMAND: &str = "command";
    pub const EXECUTE: &str = "execute";
    pub const VALUE: &str = "value";
    pub const LANGUAGE: &str = "language";
    pub const CLOSE: &str = "close";
    pub const CLOSE_OTHERS: &str = "close_others";
    pub const DUPLICATE: &str = "duplicate";
    pub const CONNECTION_FAILED: &str = "connection_failed";
    pub const SELECT_CONNECTION: &str = "select_connection";
    pub const PLEASE_SELECT_CONNECTION: &str = "please_select_connection";
    pub const SAVE: &str = "save";
    pub const CANCEL: &str = "cancel";
    pub const CONNECTION_NAME: &str = "connection_name";
    pub const CONNECTION_ADDRESS: &str = "connection_address";
    pub const CONNECTION_PORT: &str = "connection_port";
    pub const CONNECTION_USERNAME: &str = "connection_username";
    pub const CONNECTION_PASSWORD: &str = "connection_password";
    pub const CONNECTION_COLOR: &str = "connection_color";
    pub const PLEASE_ENTER_CONNECTION_NAME: &str = "please_enter_connection_name";
    pub const PLEASE_ENTER_CONNECTION_ADDRESS: &str = "please_enter_connection_address";
    pub const NEW_CONNECTION_DIALOG: &str = "new_connection_dialog";
    pub const EDIT_CONNECTION_DIALOG: &str = "edit_connection_dialog";
    pub const COMMAND_LABEL: &str = "command_label";
    pub const OUTPUT: &str = "output";
    pub const SELECT_KEY_PROMPT: &str = "select_key_prompt";
    pub const KEY_NOT_EXIST: &str = "key_not_exist";
    pub const KEY_HEADING: &str = "key_heading";
    pub const TYPE_STRING: &str = "type_string";
    pub const TYPE_LIST: &str = "type_list";
    pub const TYPE_HASH: &str = "type_hash";
    pub const TYPE_SET: &str = "type_set";
    pub const TYPE_ZSET: &str = "type_zset";
    pub const LOAD_FIRST_100: &str = "load_first_100";
    pub const LOAD_FIELDS: &str = "load_fields";
    pub const LOAD_MEMBERS: &str = "load_members";
    pub const LOAD_MORE_KEYS: &str = "load_more_keys";
    pub const LOAD_ALL_KEYS: &str = "load_all_keys";
    pub const CONFIG_PARAM_TOO_BIG: &str = "config_param_too_huge";
    pub const CONFIG_HOME_DIR_MISSING: &str = "config_home_dir_missing";
    pub const CONFIG_READ_FAILED: &str = "config_read_failed";
    pub const CONFIG_PARSE_FAILED: &str = "config_parse_failed";
    pub const CONFIG_CREATE_DIR_FAILED: &str = "config_create_dir_failed";
    pub const CONFIG_WRITE_FAILED: &str = "config_write_failed";
    pub const CONNECTION_NAME_EXISTS: &str = "connection_name_exists";
    pub const CONNECTION_NOT_FOUND: &str = "connection_not_found";
    pub const EMPTY_COMMAND: &str = "empty_command";
    pub const GENERIC_ERROR: &str = "generic_error";
    pub const GET_VALUE_FAILED: &str = "get_value_failed";
    pub const STATUS_BAR: &str = "status_bar";
    pub const UNKNOWN: &str = "unknown";
    pub const TOTAL_KEYS: &str = "total_keys";
    pub const LOADED_KEYS: &str = "loaded_keys";
    pub const LOADING: &str = "loading";
    pub const READY: &str = "ready";
    pub const DISCONNECTED: &str = "disconnected";
}

impl Default for Translator {
    fn default() -> Self {
        let mut strings = HashMap::new();

        // Connection
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
            m.insert(Language::English, "Load 5000 more keys");
            m.insert(Language::Chinese, "再加载5000个");
            m
        });

        strings.insert(keys::LOAD_ALL_KEYS, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Load all remaining keys");
            m.insert(Language::Chinese, "加载所有剩余key");
            m
        });

        strings.insert(keys::CONFIG_PARAM_TOO_BIG, {
            let mut m = HashMap::new();
            m.insert(
                Language::English,
                "Config param is too long: {}, the length of '{}' must be less than {}",
            );
            m.insert(Language::Chinese, "参数超长：{} “{}” 长度不能超过{}");
            m
        });

        // Config error messages
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

        // Generic / command errors
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

        strings.insert(keys::UNKNOWN, {
            let mut m = HashMap::new();
            m.insert(Language::English, "Unknown");
            m.insert(Language::Chinese, "未知");
            m
        });

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
