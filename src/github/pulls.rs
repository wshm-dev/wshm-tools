use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::db::pulls::PullRequest;
use crate::github::Client;

/// A merged pull request with its merge date
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergedPullRequest {
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub author: Option<String>,
    pub labels: Vec<String>,
    pub merged_at: String,
    pub created_at: String,
}

/// Build a [`PullRequest`] from a GitHub API pull request JSON object.
fn parse_pull(pr: &serde_json::Value) -> PullRequest {
    let state = pr.get("state").and_then(|v| v.as_str()).unwrap_or("open");
    PullRequest {
        number: pr["number"].as_u64().unwrap_or(0),
        title: pr["title"].as_str().unwrap_or("").to_string(),
        body: pr.get("body").and_then(|v| v.as_str()).map(String::from),
        state: state.to_string(),
        labels: super::extract_labels(pr),
        author: super::extract_author(pr),
        head_sha: pr
            .get("head")
            .and_then(|h| h.get("sha"))
            .and_then(|v| v.as_str())
            .map(String::from),
        base_sha: pr
            .get("base")
            .and_then(|h| h.get("sha"))
            .and_then(|v| v.as_str())
            .map(String::from),
        head_ref: pr
            .get("head")
            .and_then(|h| h.get("ref"))
            .and_then(|v| v.as_str())
            .map(String::from),
        base_ref: pr
            .get("base")
            .and_then(|h| h.get("ref"))
            .and_then(|v| v.as_str())
            .map(String::from),
        mergeable: pr.get("mergeable").and_then(|v| v.as_bool()),
        ci_status: None,
        created_at: pr["created_at"].as_str().unwrap_or("").to_string(),
        updated_at: pr["updated_at"].as_str().unwrap_or("").to_string(),
    }
}

impl Client {
    /// Fetch merged pull requests, optionally filtering to those merged since a given ISO date.
    pub async fn fetch_merged_pulls(&self, since: Option<&str>) -> Result<Vec<MergedPullRequest>> {
        let mut all = Vec::with_capacity(128);
        let mut page = 1u32;

        loop {
            let url = format!(
                "https://api.github.com/repos/{}/{}/pulls?state=closed&sort=updated&direction=desc&per_page={pp}&page={page}",
                self.owner, self.repo, pp = super::GITHUB_PER_PAGE
            );

            let body = crate::retry::with_retry("github: list closed PRs", || async {
                let response = self
                    .octocrab
                    ._get(&url)
                    .await
                    .context("Failed to fetch closed pull requests")?;
                self.octocrab
                    .body_to_string(response)
                    .await
                    .context("Failed to read closed pulls response body")
            })
            .await?;

            let items = super::parse_json_array(&body, "closed pulls")?;

            if items.is_empty() {
                break;
            }

            let mut stop = false;
            for pr in &items {
                // Only include actually merged PRs
                let merged_at = match pr.get("merged_at").and_then(|v| v.as_str()) {
                    Some(date) => date.to_string(),
                    None => continue,
                };

                // If we have a since cutoff, skip PRs merged before it
                if let Some(cutoff) = since {
                    if merged_at.as_str() < cutoff {
                        stop = true;
                        break;
                    }
                }

                all.push(MergedPullRequest {
                    number: pr["number"].as_u64().unwrap_or(0),
                    title: pr["title"].as_str().unwrap_or("").to_string(),
                    body: pr.get("body").and_then(|v| v.as_str()).map(String::from),
                    author: super::extract_author(pr),
                    labels: super::extract_labels(pr),
                    merged_at,
                    created_at: pr["created_at"].as_str().unwrap_or("").to_string(),
                });
            }

            if stop || items.len() < 100 {
                break;
            }
            page += 1;
        }

        Ok(all)
    }

    pub async fn fetch_pulls(&self) -> Result<Vec<PullRequest>> {
        self.fetch_pulls_by_state("all").await
    }

    /// Fetch pull requests filtered by state ("open", "closed", or "all").
    /// Used by incremental sync to fetch only open PRs (saves bandwidth).
    pub async fn fetch_pulls_by_state(&self, state: &str) -> Result<Vec<PullRequest>> {
        self.fetch_pulls_incremental(state, None).await
    }

    /// Incremental fetch: stops paginating once we hit PRs older than `since`.
    /// Combined with sort=updated direction=desc, this means we only download
    /// PRs that changed since the last sync. Forever incremental — no full re-fetch.
    ///
    /// `since` is an RFC3339 timestamp (e.g. "2026-04-01T00:00:00Z").
    /// If None, fetches all PRs (used for first-time sync).
    pub async fn fetch_pulls_incremental(
        &self,
        state: &str,
        since: Option<&str>,
    ) -> Result<Vec<PullRequest>> {
        let mut all_pulls = Vec::with_capacity(64);
        let mut page = 1u32;

        loop {
            // sort=updated direction=desc → newest first → we can break early
            let url = format!(
                "https://api.github.com/repos/{}/{}/pulls?state={state}&sort=updated&direction=desc&per_page={pp}&page={page}",
                self.owner, self.repo, pp = super::GITHUB_PER_PAGE
            );

            let body = crate::retry::with_retry("github: list PRs", || async {
                let response = self
                    .octocrab
                    ._get(&url)
                    .await
                    .context("Failed to fetch pull requests")?;
                self.octocrab
                    .body_to_string(response)
                    .await
                    .context("Failed to read pulls response body")
            })
            .await?;

            let items = super::parse_json_array(&body, "pulls")?;

            if items.is_empty() {
                break;
            }

            let mut should_stop = false;

            for pr in &items {
                // Forever-incremental: stop if this PR is older than `since`
                if let Some(since_ts) = since {
                    let updated_at = pr["updated_at"].as_str().unwrap_or("");
                    if updated_at < since_ts {
                        should_stop = true;
                        break;
                    }
                }

                all_pulls.push(parse_pull(pr));
            }

            if should_stop || items.len() < 100 || page >= 100 {
                break; // Hit a PR older than since, last page, or safety cap
            }
            page += 1;
        }

        Ok(all_pulls)
    }

    /// Fetch mergeable status for a single PR (requires individual API call)
    pub async fn fetch_pr_mergeable(&self, number: u64) -> Result<Option<bool>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls/{number}",
            self.owner, self.repo
        );
        let body = crate::retry::with_retry("github: fetch PR mergeable", || async {
            let response = self
                .octocrab
                ._get(&url)
                .await
                .with_context(|| format!("Failed to fetch PR #{number} details"))?;
            self.octocrab
                .body_to_string(response)
                .await
                .with_context(|| format!("Failed to read PR #{number} body"))
        })
        .await?;

        let pr_json: serde_json::Value =
            serde_json::from_str(&body).context("Failed to parse PR JSON")?;

        Ok(pr_json.get("mergeable").and_then(|v| v.as_bool()))
    }

    /// Fetch a single pull request by number from the canonical (strongly
    /// consistent) `/pulls/{number}` endpoint.
    ///
    /// Mirrors [`Client::fetch_issue`]: the list endpoint is served from a
    /// replicated index that lags PR creation by a few seconds, so a sync
    /// triggered right after a `pull_request.opened` webhook can miss the
    /// brand-new PR and surface "PR #N not found in cache" at analysis time.
    /// The single-PR endpoint always reflects the latest state.
    ///
    /// Returns `Ok(None)` when the PR was deleted/transferred (404).
    pub async fn fetch_pull(&self, number: u64) -> Result<Option<PullRequest>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls/{number}",
            self.owner, self.repo
        );

        let body = crate::retry::with_retry("github: fetch PR", || async {
            let response = self
                .octocrab
                ._get(&url)
                .await
                .with_context(|| format!("Failed to fetch PR #{number}"))?;
            self.octocrab
                .body_to_string(response)
                .await
                .with_context(|| format!("Failed to read PR #{number} body"))
        })
        .await?;

        let item: serde_json::Value =
            serde_json::from_str(&body).context("Failed to parse PR JSON")?;

        // A 404 (deleted/transferred PR) comes back as an object with a
        // `message` field rather than a PR payload.
        if item.get("number").is_none() {
            if let Some(msg) = item.get("message").and_then(|v| v.as_str()) {
                if msg.eq_ignore_ascii_case("Not Found") {
                    return Ok(None);
                }
                anyhow::bail!("GitHub error while fetching PR #{number}: {msg}");
            }
            return Ok(None);
        }

        Ok(Some(parse_pull(&item)))
    }

    pub async fn fetch_pr_diff(&self, number: u64) -> Result<String> {
        // Use the raw diff endpoint for actual unified diff content
        self.fetch_pr_diff_raw(number).await
    }

    /// Fetch the raw unified diff for a PR
    pub async fn fetch_pr_diff_raw(&self, number: u64) -> Result<String> {
        // Use the .diff URL which returns raw unified diff
        let url = format!(
            "https://github.com/{}/{}/pull/{number}.diff",
            self.owner, self.repo
        );

        crate::retry::with_retry("github: fetch PR diff", || async {
            let response = self
                .http
                .get(&url)
                .send()
                .await
                .with_context(|| format!("Failed to fetch raw diff for PR #{number}"))?;

            let status = response.status();
            let text = response
                .text()
                .await
                .with_context(|| format!("Failed to read raw diff for PR #{number}"))?;

            if !status.is_success() {
                anyhow::bail!("Failed to fetch diff for PR #{number}: HTTP {status}");
            }
            Ok(text)
        })
        .await
    }

    /// Submit a review with inline comments on a PR
    pub async fn submit_review(
        &self,
        number: u64,
        body: &str,
        comments: &[(String, u64, String)], // (path, line, body)
    ) -> Result<()> {
        let review_comments: Vec<serde_json::Value> = comments
            .iter()
            .map(|(path, line, comment_body)| {
                serde_json::json!({
                    "path": path,
                    "line": line,
                    "body": comment_body,
                })
            })
            .collect();

        let event = "COMMENT";

        let review_body = serde_json::json!({
            "body": body,
            "event": event,
            "comments": review_comments,
        });

        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls/{number}/reviews",
            self.owner, self.repo
        );

        crate::retry::with_retry("github: submit review", || async {
            let response = self
                .octocrab
                ._post(&url, Some(&review_body))
                .await
                .with_context(|| format!("Failed to submit review on PR #{number}"))?;

            let status = response.status();
            if !status.is_success() {
                let body = self.octocrab.body_to_string(response).await?;
                anyhow::bail!("Failed to submit review on PR #{number}: {status} {body}");
            }
            Ok::<_, anyhow::Error>(())
        })
        .await
    }

    /// Create a new pull request, returns the PR number
    pub async fn create_pr(&self, title: &str, body: &str, head: &str, base: &str) -> Result<u64> {
        let pr_body = serde_json::json!({
            "title": title,
            "body": body,
            "head": head,
            "base": base,
        });

        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls",
            self.owner, self.repo
        );

        let response_body = crate::retry::with_retry("github: create PR", || async {
            let response = self
                .octocrab
                ._post(&url, Some(&pr_body))
                .await
                .context("Failed to create pull request")?;
            self.octocrab
                .body_to_string(response)
                .await
                .context("Failed to read create PR response")
        })
        .await?;

        let pr_json: serde_json::Value =
            serde_json::from_str(&response_body).context("Failed to parse create PR response")?;

        let number = pr_json["number"]
            .as_u64()
            .context("Missing PR number in response")?;

        Ok(number)
    }

    pub async fn label_pr(&self, number: u64, labels: &[String]) -> Result<()> {
        crate::retry::with_retry("github: label PR", || async {
            self.octocrab
                .issues(&self.owner, &self.repo)
                .add_labels(number, labels)
                .await
                .with_context(|| format!("Failed to label PR #{number}"))?;
            Ok::<_, anyhow::Error>(())
        })
        .await
    }

    /// Post or update a wshm comment on a PR.
    /// Delegates to comment_issue since GitHub uses the same API for both.
    pub async fn comment_pr(&self, number: u64, body: &str) -> Result<()> {
        self.comment_issue(number, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pull_builds_pull_request() {
        let item = serde_json::json!({
            "number": 42,
            "title": "Add feature",
            "body": "description",
            "state": "open",
            "labels": [{ "name": "enhancement" }],
            "user": { "login": "octocat" },
            "head": { "sha": "abc123", "ref": "feature" },
            "base": { "sha": "def456", "ref": "main" },
            "mergeable": true,
            "created_at": "2026-05-01T00:00:00Z",
            "updated_at": "2026-05-02T00:00:00Z",
        });
        let pr = parse_pull(&item);
        assert_eq!(pr.number, 42);
        assert_eq!(pr.title, "Add feature");
        assert_eq!(pr.state, "open");
        assert_eq!(pr.labels, vec!["enhancement".to_string()]);
        assert_eq!(pr.author.as_deref(), Some("octocat"));
        assert_eq!(pr.head_sha.as_deref(), Some("abc123"));
        assert_eq!(pr.base_ref.as_deref(), Some("main"));
        assert_eq!(pr.mergeable, Some(true));
    }

    #[test]
    fn test_parse_pull_handles_missing_optionals() {
        // A freshly opened PR may have null mergeable and no body.
        let item = serde_json::json!({
            "number": 43,
            "title": "Draft",
            "state": "open",
            "head": { "sha": "aaa", "ref": "wip" },
            "base": { "sha": "bbb", "ref": "develop" },
            "created_at": "2026-05-01T00:00:00Z",
            "updated_at": "2026-05-01T00:00:00Z",
        });
        let pr = parse_pull(&item);
        assert_eq!(pr.number, 43);
        assert_eq!(pr.body, None);
        assert_eq!(pr.mergeable, None);
        assert!(pr.labels.is_empty());
    }
}
