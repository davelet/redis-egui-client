use super::app_state::Language;
use std::collections::HashMap;

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
