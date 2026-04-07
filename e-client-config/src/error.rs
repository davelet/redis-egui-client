use e_client_bilingual::language::Language;
use e_client_bilingual::translations::TranslationKey;

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
    InvalidUrl,
    UnsupportedConnectionType(String),
}

impl ConfigError {
    pub fn to_message(&self, lang: Language) -> String {
        use e_client_bilingual::translations::{tr, tr_fmt};
        match self {
            ConfigError::HugeParam(e) => tr_fmt(
                TranslationKey::ConfigParamTooBig,
                lang,
                &(e.iter().map(|s| s.as_str()).collect::<Vec<_>>()[..]),
            ),
            ConfigError::HomeDirMissing => {
                tr(TranslationKey::ConfigHomeDirMissing, lang).to_string()
            }
            ConfigError::ReadFailed(e) => tr_fmt(TranslationKey::ConfigReadFailed, lang, &[e]),
            ConfigError::ParseFailed(e) => tr_fmt(TranslationKey::ConfigParseFailed, lang, &[e]),
            ConfigError::CreateDirFailed(e) => {
                tr_fmt(TranslationKey::ConfigCreateDirFailed, lang, &[e])
            }
            ConfigError::WriteFailed(e) => tr_fmt(TranslationKey::ConfigWriteFailed, lang, &[e]),
            ConfigError::ConnectionNameExists => {
                tr(TranslationKey::ConnectionNameExists, lang).to_string()
            }
            ConfigError::ConnectionNotFound => {
                tr(TranslationKey::ConnectionNotFound, lang).to_string()
            }
            ConfigError::InvalidUrl => {
                tr(TranslationKey::InvalidConnectionString, lang).to_string()
            }
            ConfigError::UnsupportedConnectionType(t) => {
                tr_fmt(TranslationKey::UnsupportedConnectionType, lang, &[t])
            }
        }
    }
}
