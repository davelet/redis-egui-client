//! JSON import/export for AI configuration
//! API keys are automatically excluded from export for security

use std::io::{Read, Write};
use std::path::Path;

use crate::config::ai_config::{AiConfig, AiConfigError};

/// Export AI configuration to JSON file (API keys are excluded)
pub fn export_to_json(config: &AiConfig, path: &Path) -> Result<(), AiConfigError> {
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| AiConfigError::SerializationError(e.to_string()))?;

    let mut file =
        std::fs::File::create(path).map_err(|e| AiConfigError::IoError(e.to_string()))?;

    file.write_all(json.as_bytes())
        .map_err(|e| AiConfigError::IoError(e.to_string()))?;

    Ok(())
}

/// Import AI configuration from JSON file
/// Note: API keys will be None (empty) after import, user needs to re-enter them
pub fn import_from_json(path: &Path) -> Result<AiConfig, AiConfigError> {
    let mut file = std::fs::File::open(path).map_err(|e| AiConfigError::IoError(e.to_string()))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| AiConfigError::IoError(e.to_string()))?;

    let config: AiConfig = serde_json::from_str(&contents)
        .map_err(|e| AiConfigError::DeserializationError(e.to_string()))?;

    // Note: API keys are not imported (they're skipped during serialization)
    // All imported models will have empty API keys

    Ok(config)
}

/// Preview import - returns the number of models that would be imported
/// without actually importing them
pub fn preview_import(path: &Path) -> Result<ImportPreview, AiConfigError> {
    let config = import_from_json(path)?;

    Ok(ImportPreview {
        model_count: config.models.len(),
        models: config
            .models
            .iter()
            .map(|m| ModelPreview {
                name: m.name.clone(),
                provider: m.provider.to_string(),
                model_id: m.model_id.clone(),
                has_api_key: false, // Always false after import
            })
            .collect(),
    })
}

/// Preview information for import confirmation
#[derive(Debug, Clone)]
pub struct ImportPreview {
    pub model_count: usize,
    pub models: Vec<ModelPreview>,
}

/// Individual model preview info
#[derive(Debug, Clone)]
pub struct ModelPreview {
    pub name: String,
    pub provider: String,
    pub model_id: String,
    pub has_api_key: bool, // Always false after import, needs re-entry
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ai_config::{AiModel, AiProviderType};
    use std::fs;

    #[test]
    fn test_export_import_roundtrip() {
        let mut config = AiConfig::default();

        let mut model1 = AiModel::default();
        model1.name = "Test Model".to_string();
        model1.provider = AiProviderType::OpenAi;
        model1.model_id = "gpt-4".to_string();
        model1.temperature = 0.8;
        model1.api_key = Some("secret-key".to_string()); // Should be skipped

        config.add_model(model1);
        config.enabled = true;

        let dir = std::env::temp_dir();
        let path = dir.join(format!("ai_config_test_{}.json", std::process::id()));

        // Export
        export_to_json(&config, &path).unwrap();

        // Import
        let imported = import_from_json(&path).unwrap();

        assert_eq!(imported.models.len(), 1);
        assert_eq!(imported.models[0].name, "Test Model");
        assert_eq!(imported.models[0].model_id, "gpt-4");
        assert_eq!(imported.models[0].temperature, 0.8);
        assert!(imported.models[0].api_key.is_none()); // Key should be None after import
        assert!(imported.enabled);

        // Verify file doesn't contain the API key
        let file_content = fs::read_to_string(&path).unwrap();
        assert!(!file_content.contains("secret-key"));

        // Clean up temp file
        let _ = std::fs::remove_file(&path);
    }
}
