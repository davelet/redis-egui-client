use std::collections::HashMap;
use crate::language::Language;

pub struct Translator {
    strings: HashMap<&'static str, HashMap<Language, &'static str>>,
}

impl Default for Translator {
    fn default() -> Self {
        let mut strings = HashMap::new();

        // Connection
        strings.insert("connect", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connect");
            m.insert(Language::Chinese, "连接");
            m
        });

        strings.insert("new_connection", {
            let mut m = HashMap::new();
            m.insert(Language::Chinese, "新建连接");
            m.insert(Language::English, "New Connection");
            m
        });

        strings.insert("disconnect", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Disconnect");
            m.insert(Language::Chinese, "断开连接");
            m
        });

        strings.insert("connection_url", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection:");
            m.insert(Language::Chinese, "连接:");
            m
        });

        strings.insert("database", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Database:");
            m.insert(Language::Chinese, "数据库:");
            m
        });

        // Keys
        strings.insert("keys", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Keys");
            m.insert(Language::Chinese, "键");
            m
        });

        strings.insert("filter", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Filter:");
            m.insert(Language::Chinese, "过滤:");
            m
        });

        // Command
        strings.insert("command", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Command");
            m.insert(Language::Chinese, "命令");
            m
        });

        strings.insert("execute", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Execute");
            m.insert(Language::Chinese, "执行");
            m
        });

        // Value
        strings.insert("value", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Value");
            m.insert(Language::Chinese, "值");
            m
        });

        // Language
        strings.insert("language", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Language");
            m.insert(Language::Chinese, "语言");
            m
        });

        strings.insert("english", {
            let mut m = HashMap::new();
            m.insert(Language::English, "English");
            m.insert(Language::Chinese, "English");
            m
        });

        strings.insert("chinese", {
            let mut m = HashMap::new();
            m.insert(Language::English, "中文");
            m.insert(Language::Chinese, "中文");
            m
        });

        // Errors
        strings.insert("connection_failed", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection failed: {}");
            m.insert(Language::Chinese, "连接失败: {}");
            m
        });

        // Other UI strings
        strings.insert("select_connection", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Select connection");
            m.insert(Language::Chinese, "选择连接");
            m
        });

        strings.insert("please_select_connection", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Please select a connection");
            m.insert(Language::Chinese, "请先选择一个连接");
            m
        });

        strings.insert("save", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Save");
            m.insert(Language::Chinese, "保存");
            m
        });

        strings.insert("cancel", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Cancel");
            m.insert(Language::Chinese, "取消");
            m
        });

        strings.insert("connection_name", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection name:");
            m.insert(Language::Chinese, "连接名称:");
            m
        });

        strings.insert("connection_address", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection address:");
            m.insert(Language::Chinese, "连接地址:");
            m
        });

        strings.insert("please_enter_connection_name", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Please enter connection name");
            m.insert(Language::Chinese, "请输入连接名称");
            m
        });

        strings.insert("please_enter_connection_address", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Please enter connection address");
            m.insert(Language::Chinese, "请输入连接地址");
            m
        });

        strings.insert("new_connection_dialog", {
            let mut m = HashMap::new();
            m.insert(Language::English, "New Redis Connection");
            m.insert(Language::Chinese, "新建 Redis 连接");
            m
        });

        strings.insert("command_label", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Command:");
            m.insert(Language::Chinese, "命令:");
            m
        });

        strings.insert("output", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Output:");
            m.insert(Language::Chinese, "输出:");
            m
        });

        strings.insert("select_key_prompt", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Select a key to view details");
            m.insert(Language::Chinese, "选择一个 key 查看详情");
            m
        });

        strings.insert("key_not_exist", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Key does not exist");
            m.insert(Language::Chinese, "Key 不存在");
            m
        });

        strings.insert("key_heading", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Key: {}");
            m.insert(Language::Chinese, "Key: {}");
            m
        });

        strings.insert("type_string", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Type: String");
            m.insert(Language::Chinese, "类型: String");
            m
        });

        strings.insert("type_list", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Type: List (length: {})");
            m.insert(Language::Chinese, "类型: List (长度: {})");
            m
        });

        strings.insert("type_hash", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Type: Hash (fields: {})");
            m.insert(Language::Chinese, "类型: Hash (字段数: {})");
            m
        });

        strings.insert("type_set", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Type: Set (members: {})");
            m.insert(Language::Chinese, "类型: Set (成员数: {})");
            m
        });

        strings.insert("type_zset", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Type: ZSet (members: {})");
            m.insert(Language::Chinese, "类型: ZSet (成员数: {})");
            m
        });

        strings.insert("load_first_100", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Load first 100 items");
            m.insert(Language::Chinese, "加载前100项");
            m
        });

        strings.insert("load_fields", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Load fields");
            m.insert(Language::Chinese, "加载字段");
            m
        });

        // Config error messages
        strings.insert("config_home_dir_missing", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Could not determine home directory");
            m.insert(Language::Chinese, "无法获取用户目录");
            m
        });

        strings.insert("config_read_failed", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Failed to read config file: {}");
            m.insert(Language::Chinese, "读取配置文件失败: {}");
            m
        });

        strings.insert("config_parse_failed", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Failed to parse config file: {}");
            m.insert(Language::Chinese, "解析配置文件失败: {}");
            m
        });

        strings.insert("config_create_dir_failed", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Failed to create config directory: {}");
            m.insert(Language::Chinese, "创建配置目录失败: {}");
            m
        });

        strings.insert("config_write_failed", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Failed to write config file: {}");
            m.insert(Language::Chinese, "写入配置文件失败: {}");
            m
        });

        strings.insert("connection_name_exists", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection name already exists");
            m.insert(Language::Chinese, "连接名称已存在");
            m
        });

        strings.insert("connection_not_found", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Connection not found");
            m.insert(Language::Chinese, "连接不存在");
            m
        });

        // Generic / command errors
        strings.insert("empty_command", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Empty command");
            m.insert(Language::Chinese, "命令为空");
            m
        });

        strings.insert("generic_error", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Error: {}");
            m.insert(Language::Chinese, "错误: {}");
            m
        });

        strings.insert("get_value_failed", {
            let mut m = HashMap::new();
            m.insert(Language::English, "Failed to get value: {}");
            m.insert(Language::Chinese, "获取值失败: {}");
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
