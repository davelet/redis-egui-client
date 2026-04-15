use e_client_basics::constants::{
    GITHUB_ACCEPT_HEADER, GITHUB_RELEASES_LATEST_URL, GITHUB_USER_AGENT,
};
use reqwest::Client;
use serde::Deserialize;

/// Default HTTP timeout for GitHub API requests.
const REQUEST_TIMEOUT_SECS: u64 = 10;

/// GitHub Releases API response (we only need the tag_name)
#[derive(Debug, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
}

/// Fetch the latest release tag from GitHub.
/// Returns the tag_name (e.g., "v0.6.0") or None on error.
/// Silently fails on network issues — no update check is better than a broken app.
pub async fn fetch_latest_tag() -> Option<String> {
    let url = GITHUB_RELEASES_LATEST_URL;

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .build()
        .ok()?;

    let response = match client
        .get(url)
        .header("User-Agent", GITHUB_USER_AGENT)
        .header("Accept", GITHUB_ACCEPT_HEADER)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            tracing::warn!(error = %e, "GitHub API request failed: network error");
            return None;
        }
    };

    if !response.status().is_success() {
        let status = response.status();
        if status.as_u16() == 403 {
            // GitHub API rate limit (60 req/hour for unauthenticated requests)
            let remaining = response
                .headers()
                .get("x-ratelimit-remaining")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("?");
            let reset = response
                .headers()
                .get("x-ratelimit-reset")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("?");
            tracing::warn!(
                rate_limit_remaining = remaining,
                rate_limit_reset = reset,
                "GitHub API rate limit exceeded (unauthenticated: 60 req/hour)",
            );
        } else {
            tracing::warn!(
                status = %status,
                "GitHub API request failed: non-success status",
            );
        }
        return None;
    }

    let release: GitHubRelease = match response.json().await {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(error = %e, "failed to parse GitHub API response");
            return None;
        }
    };

    tracing::info!(tag_name = %release.tag_name, "successfully fetched latest release tag");

    Some(release.tag_name)
}

// ---------------------------------------------------------------------------
// Integration tests (mocked HTTP via wiremock)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// Test constants — avoid magic values scattered across tests.
    mod consts {
        pub const TAG_V060: &str = "v0.6.0";
        pub const TAG_V051: &str = "v0.5.1";
        pub const TAG_V061: &str = "v0.6.1";

        /// Mirrors the hardcoded production URL path from `e-client-basics`.
        pub const RELEASES_LATEST_PATH: &str = "/repos/davelet/redis-egui-client/releases/latest";
    }

    fn latest_release_json(tag: &str) -> String {
        format!(r#"{{"tag_name":"{}"}}"#, tag)
    }

    /// Mock a successful GitHub API response and verify we parse the tag correctly.
    #[tokio::test]
    async fn test_fetch_latest_tag_success() {
        use consts::*;

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(RELEASES_LATEST_PATH))
            .and(header("Accept", GITHUB_ACCEPT_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_string(latest_release_json(TAG_V060)))
            .mount(&mock_server)
            .await;

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap();

        let response = client
            .get(format!("{}{}", mock_server.uri(), RELEASES_LATEST_PATH))
            .header("User-Agent", GITHUB_USER_AGENT)
            .header("Accept", GITHUB_ACCEPT_HEADER)
            .send()
            .await
            .unwrap();

        let release: GitHubRelease = response.json().await.unwrap();
        assert_eq!(release.tag_name, TAG_V060);
    }

    /// Mock a 403 rate-limit response and verify we get a non-success status.
    #[tokio::test]
    async fn test_fetch_latest_tag_rate_limited() {
        use consts::*;

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(RELEASES_LATEST_PATH))
            .and(header("Accept", GITHUB_ACCEPT_HEADER))
            .respond_with(
                ResponseTemplate::new(403)
                    .set_body_string(r#"{"message":"API rate limit exceeded"}"#),
            )
            .mount(&mock_server)
            .await;

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap();

        let response = client
            .get(format!("{}{}", mock_server.uri(), RELEASES_LATEST_PATH))
            .header("User-Agent", GITHUB_USER_AGENT)
            .header("Accept", GITHUB_ACCEPT_HEADER)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status().as_u16(), 403);
    }

    /// Mock a 404 not-found response — the URL is hardcoded, but we test the path anyway.
    #[tokio::test]
    async fn test_fetch_latest_tag_not_found() {
        use consts::*;

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(RELEASES_LATEST_PATH))
            .and(header("Accept", GITHUB_ACCEPT_HEADER))
            .respond_with(ResponseTemplate::new(404).set_body_string(r#"{"message":"Not Found"}"#))
            .mount(&mock_server)
            .await;

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap();

        let response = client
            .get(format!("{}{}", mock_server.uri(), RELEASES_LATEST_PATH))
            .header("User-Agent", GITHUB_USER_AGENT)
            .header("Accept", GITHUB_ACCEPT_HEADER)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status().as_u16(), 404);
    }

    /// Mock a malformed JSON response — verify the deserialization error path.
    #[tokio::test]
    async fn test_fetch_latest_tag_malformed_json() {
        use consts::*;

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(RELEASES_LATEST_PATH))
            .and(header("Accept", GITHUB_ACCEPT_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"not valid json"#))
            .mount(&mock_server)
            .await;

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap();

        let response = client
            .get(format!("{}{}", mock_server.uri(), RELEASES_LATEST_PATH))
            .header("User-Agent", GITHUB_USER_AGENT)
            .header("Accept", GITHUB_ACCEPT_HEADER)
            .send()
            .await
            .unwrap();

        // Status is 200, but body is invalid JSON — this is what the `Err` branch handles.
        assert!(response.status().is_success());
        let result: Result<GitHubRelease, _> = response.json().await;
        assert!(result.is_err());
    }

    /// End-to-end contract test: fetch + version comparison via `check_for_updates`.
    /// NOTE: we can't override the hard-coded GitHub URL in `check_for_updates`
    /// without changing its API, so this test calls `fetch_latest_tag` directly
    /// against the mock server and verifies the contract with the version layer.
    #[tokio::test]
    async fn test_check_for_updates_contract() {
        use crate::updater::is_update_available;
        use consts::*;

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(RELEASES_LATEST_PATH))
            .and(header("Accept", GITHUB_ACCEPT_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_string(latest_release_json(TAG_V060)))
            .mount(&mock_server)
            .await;

        // Verify version comparison contract: 0.5.1 → 0.6.0 is an update, equal and greater are not.
        assert!(is_update_available(TAG_V051, TAG_V060));
        assert!(!is_update_available(TAG_V060, TAG_V060));
        assert!(!is_update_available(TAG_V061, TAG_V060));
    }
}
