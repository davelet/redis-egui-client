mod github;
mod version;

pub use e_client_basics::constants::{
    GITHUB_ACCEPT_HEADER, GITHUB_RELEASES_LATEST_URL, GITHUB_USER_AGENT,
};
use e_client_config::config::UpdateConfig;
pub use github::{GitHubRelease, fetch_latest_tag};
pub use version::{is_update_available, normalize_version, should_skip_version};

/// Result of an update check
#[derive(Debug, Clone)]
pub enum UpdateCheckResult {
    /// A newer version is available
    UpdateAvailable { latest_version: String },
    /// The current version is the latest
    UpToDate,
    /// The available update should be skipped (user choice)
    Skipped { latest_version: String },
    /// Check failed (network error, etc.) — silently ignored
    CheckFailed,
}

/// Check for updates against GitHub.
/// This function handles:
/// - Fetching the latest release tag
/// - Comparing with the current version
/// - Respecting the skip_version setting
///
/// Note: interval checking ("is it time to check?") is handled by the caller (UI layer)
/// to keep this module focused on pure version logic.
///
/// Returns `UpdateCheckResult` with the outcome.
pub async fn check_for_updates(current_version: &str, config: &UpdateConfig) -> UpdateCheckResult {
    tracing::info!(
        current_version = %current_version,
        skip_version = ?config.skip_version,
        "starting update check",
    );

    // Fetch the latest release
    let latest_tag = match fetch_latest_tag().await {
        Some(tag) => {
            tracing::info!(latest_version = %tag, "received latest version from GitHub");
            tag
        }
        None => {
            tracing::warn!("failed to fetch latest release — network error or API rate limit");
            return UpdateCheckResult::CheckFailed;
        }
    };

    // Check if this version should be skipped
    if should_skip_version(config.skip_version.as_deref(), &latest_tag) {
        tracing::info!(
            latest_version = %latest_tag,
            skip_version = ?config.skip_version,
            "update available but skipped by user preference",
        );
        return UpdateCheckResult::Skipped {
            latest_version: latest_tag,
        };
    }

    // Check if there's actually a newer version
    if is_update_available(current_version, &latest_tag) {
        tracing::info!(
            current_version = %current_version,
            latest_version = %latest_tag,
            "newer version available",
        );
        UpdateCheckResult::UpdateAvailable {
            latest_version: latest_tag,
        }
    } else {
        tracing::info!(
            current_version = %current_version,
            latest_version = %latest_tag,
            "already on the latest version",
        );
        UpdateCheckResult::UpToDate
    }
}
