pub mod client;
pub mod git;
pub mod issues;
pub mod pulls;
pub mod sync;

pub use client::Client;

/// Maximum items per page for GitHub API pagination.
pub const GITHUB_PER_PAGE: u32 = 100;

/// Extract label names from a GitHub API JSON object.
pub fn extract_labels(json: &serde_json::Value) -> Vec<String> {
    json.get("labels")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|l| l.get("name").and_then(|n| n.as_str()))
                .map(String::from)
                .collect()
        })
        .unwrap_or_default()
}

/// Extract author login from a GitHub API JSON object.
pub fn extract_author(json: &serde_json::Value) -> Option<String> {
    json.get("user")
        .and_then(|u| u.get("login"))
        .and_then(|v| v.as_str())
        .map(String::from)
}

/// Parse a GitHub API response body that's expected to be a JSON array.
///
/// On failure, GitHub typically returns an *object* with a `message`
/// field (rate limit, 401, 404, …). serde then complains about
/// "invalid type: map, expected a sequence", which is unhelpful for
/// operators reading logs. This helper detects that case and surfaces
/// the GitHub-provided message verbatim.
pub fn parse_json_array(body: &str, what: &str) -> anyhow::Result<Vec<serde_json::Value>> {
    match serde_json::from_str::<Vec<serde_json::Value>>(body) {
        Ok(v) => Ok(v),
        Err(e) => {
            if let Ok(obj) = serde_json::from_str::<serde_json::Value>(body) {
                if let Some(msg) = obj.get("message").and_then(|v| v.as_str()) {
                    let lower = msg.to_ascii_lowercase();
                    if lower.contains("rate limit") {
                        anyhow::bail!(
                            "GitHub rate limit exceeded while fetching {what} \
                             — anonymous mode (60 req/h). Add a github_token \
                             in Settings → Secrets for 5000 req/h."
                        );
                    }
                    if lower.contains("not found") {
                        anyhow::bail!(
                            "GitHub returned 'Not Found' while fetching {what} \
                             — repo private or token lacks access?"
                        );
                    }
                    if lower.contains("bad credentials") {
                        anyhow::bail!(
                            "GitHub rejected the credentials while fetching {what} \
                             — token expired or revoked?"
                        );
                    }
                    anyhow::bail!("GitHub error while fetching {what}: {msg}");
                }
            }
            Err(anyhow::Error::from(e).context(format!("Failed to parse {what} JSON")))
        }
    }
}
