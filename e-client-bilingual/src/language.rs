use serde::{Deserialize, Serialize};
const EN_IN_FILE: &str = "en";
const ZH_IN_FILE: &str = "zh";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[derive(Default)]
pub enum Language {
    Chinese,
    #[default]
    English,
}


impl Language {
    pub fn to_file_string(&self) -> String {
        if let Language::English = self {
            EN_IN_FILE.to_string()
        } else {
            ZH_IN_FILE.to_string()
        }
    }

    pub fn file_name_to_lang(name: &str) -> Self {
        if name == EN_IN_FILE {
            return Language::English;
        }
        Language::Chinese
    }
}
