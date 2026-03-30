use crate::language::Language;
use crate::translations::keys::TranslationKey;
use std::collections::HashMap;

pub struct Translator {
    strings: HashMap<TranslationKey, HashMap<Language, &'static str>>,
}

impl Default for Translator {
    fn default() -> Self {
        let mut strings = HashMap::new();
        super::data::load_all_translations(&mut strings);
        Self { strings }
    }
}

impl Translator {
    pub fn get(&self, key: &TranslationKey, lang: Language) -> &str {
        self.strings
            .get(key)
            .and_then(|m| m.get(&lang))
            .map(|s| s.as_ref())
            .unwrap_or_else(|| {
                eprintln!(
                    "Missing translation for key: {:?} in language: {:?}",
                    key, lang
                );
                // Return a fallback - this is a temporary solution
                // In a real implementation, we'd need a mapping from enum to string
                "MISSING_TRANSLATION"
            })
    }

    pub fn format(&self, key: &TranslationKey, lang: Language, args: &[&str]) -> String {
        let template = self.get(key, lang);
        let mut result = template.to_string();

        for arg in args {
            result = result.replacen("{}", arg, 1);
        }

        result
    }
}

// Convenience function to get a translation
pub fn tr(key: TranslationKey, lang: Language) -> &'static str {
    static TRANSLATOR: std::sync::OnceLock<Translator> = std::sync::OnceLock::new();
    let translator = TRANSLATOR.get_or_init(Translator::default);
    translator.get(&key, lang)
}

// Convenience function to get a formatted translation
pub fn tr_fmt(key: TranslationKey, lang: Language, args: &[&str]) -> String {
    static TRANSLATOR: std::sync::OnceLock<Translator> = std::sync::OnceLock::new();
    let translator = TRANSLATOR.get_or_init(Translator::default);
    translator.format(&key, lang, args)
}
