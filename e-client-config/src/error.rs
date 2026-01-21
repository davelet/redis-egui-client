use e_client_bilingual::language::Language;
use e_client_bilingual::translations::keys::*;

#[derive(Debug)]
pub enum ConfigError {
    HugeParam([String; 3]), // [field, value, limit]
    HomeDirMissing,
    ReadFailed(String),
    ParseFailed(String),
    CreateDirFailed(String),
    WriteFailed(String),
    ConnectionNameExists,
    ConnectionNotFound,
}

impl ConfigError {
    pub fn to_message(&self, lang: Language) -> String {
        use e_client_bilingual::translations::{tr, tr_fmt};
        match self {
            ConfigError::HugeParam(e) => tr_fmt(
                CONFIG_PARAM_TOO_BIG,
                lang,
                &(e.iter().map(|s| s.as_str()).collect::<Vec<_>>()[..]),
            ),
            ConfigError::HomeDirMissing => tr(CONFIG_HOME_DIR_MISSING, lang).to_string(),
            ConfigError::ReadFailed(e) => tr_fmt(CONFIG_READ_FAILED, lang, &[e]),
            ConfigError::ParseFailed(e) => tr_fmt(CONFIG_PARSE_FAILED, lang, &[e]),
            ConfigError::CreateDirFailed(e) => tr_fmt(CONFIG_CREATE_DIR_FAILED, lang, &[e]),
            ConfigError::WriteFailed(e) => tr_fmt(CONFIG_WRITE_FAILED, lang, &[e]),
            ConfigError::ConnectionNameExists => tr(CONNECTION_NAME_EXISTS, lang).to_string(),
            ConfigError::ConnectionNotFound => tr(CONNECTION_NOT_FOUND, lang).to_string(),
        }
    }
}
