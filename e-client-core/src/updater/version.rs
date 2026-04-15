use semver::Version;

/// Compare the current version with the latest version from GitHub.
/// Returns `Some(latest_version)` if a newer version is available, `None` otherwise.
pub fn is_update_available(current_version: &str, latest_version: &str) -> bool {
    let current = normalize_version(current_version);
    let latest = normalize_version(latest_version);

    match (current, latest) {
        (Ok(current), Ok(latest)) => latest > current,
        _ => false, // If parsing fails, don't report an update
    }
}

/// Check if a version should be skipped
pub fn should_skip_version(skip_version: Option<&str>, latest_version: &str) -> bool {
    if let Some(skip) = skip_version {
        let skip_norm = normalize_version(skip).ok();
        let latest_norm = normalize_version(latest_version).ok();
        if let (Some(skip), Some(latest)) = (skip_norm, latest_norm) {
            return skip == latest;
        }
    }
    false
}

/// Normalize a version string by stripping leading 'v' or 'V'
pub fn normalize_version(version: &str) -> Result<Version, semver::Error> {
    let stripped = version
        .strip_prefix(|c| c == 'v' || c == 'V')
        .unwrap_or(version);
    Version::parse(stripped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_version() {
        assert_eq!(normalize_version("1.2.3").unwrap(), Version::new(1, 2, 3));
        assert_eq!(normalize_version("v1.2.3").unwrap(), Version::new(1, 2, 3));
        assert_eq!(normalize_version("V0.5.1").unwrap(), Version::new(0, 5, 1));
    }

    #[test]
    fn test_is_update_available() {
        assert!(is_update_available("0.5.1", "0.6.0"));
        assert!(is_update_available("1.0.0", "1.0.1"));
        assert!(!is_update_available("1.0.0", "1.0.0"));
        assert!(!is_update_available("1.1.0", "1.0.0"));
        // With 'v' prefix
        assert!(is_update_available("v0.5.1", "v0.6.0"));
        // Invalid versions
        assert!(!is_update_available("invalid", "1.0.0"));
    }

    #[test]
    fn test_should_skip_version() {
        assert!(should_skip_version(Some("0.6.0"), "0.6.0"));
        assert!(should_skip_version(Some("v0.6.0"), "0.6.0"));
        assert!(!should_skip_version(Some("0.5.0"), "0.6.0"));
        assert!(!should_skip_version(None, "0.6.0"));
    }
}
