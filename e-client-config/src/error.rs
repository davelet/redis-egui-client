use e_client_bilingual::language::Language;

#[derive(Debug)]
pub enum ConfigError {
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
            ConfigError::HomeDirMissing => tr(
                e_client_bilingual::translations::keys::CONFIG_HOME_DIR_MISSING,
                lang,
            )
            .to_string(),
            ConfigError::ReadFailed(e) => tr_fmt(
                e_client_bilingual::translations::keys::CONFIG_READ_FAILED,
                lang,
                &[e],
            ),
            ConfigError::ParseFailed(e) => tr_fmt(
                e_client_bilingual::translations::keys::CONFIG_PARSE_FAILED,
                lang,
                &[e],
            ),
            ConfigError::CreateDirFailed(e) => tr_fmt(
                e_client_bilingual::translations::keys::CONFIG_CREATE_DIR_FAILED,
                lang,
                &[e],
            ),
            ConfigError::WriteFailed(e) => tr_fmt(
                e_client_bilingual::translations::keys::CONFIG_WRITE_FAILED,
                lang,
                &[e],
            ),
            ConfigError::ConnectionNameExists => tr(
                e_client_bilingual::translations::keys::CONNECTION_NAME_EXISTS,
                lang,
            )
            .to_string(),
            ConfigError::ConnectionNotFound => tr(
                e_client_bilingual::translations::keys::CONNECTION_NOT_FOUND,
                lang,
            )
            .to_string(),
        }
    }
}
