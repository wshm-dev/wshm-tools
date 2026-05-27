//! DatabaseBackend trait — abstraction over SQLite and PostgreSQL backends.

use anyhow::Result;

use crate::ai::schemas::IssueClassification;
use crate::db::events::WebhookEventRow;
use crate::db::issues::Issue;
use crate::db::pulls::{PrAnalysisRow, PullRequest};
use crate::db::sync::SyncEntry;
use crate::db::triage::TriageResultRow;

/// Unified interface for both SQLite and PostgreSQL database backends.
///
/// All methods mirror the existing `Database` (SQLite) implementation.
/// Implementations must be Send + Sync for use across async contexts.
pub trait DatabaseBackend: Send + Sync {
    // ── Issues ──────────────────────────────────────────────────

    fn upsert_issue(&self, issue: &Issue) -> Result<()>;
    fn batch_upsert_issues(&self, issues: &[Issue]) -> Result<()>;
    fn get_issue(&self, number: u64) -> Result<Option<Issue>>;
    fn get_open_issues(&self) -> Result<Vec<Issue>>;
    fn get_untriaged_issues(&self) -> Result<Vec<Issue>>;
    fn get_issues_needing_triage(
        &self,
        limit: usize,
        relabel_labels: &[String],
        no_labels_min_age_hours: u32,
    ) -> Result<Vec<Issue>>;
    fn merge_issue_labels(&self, number: u64, add: &[String], remove: &[String]) -> Result<()>;

    // ── Pull Requests ───────────────────────────────────────────

    fn upsert_pull(&self, pr: &PullRequest) -> Result<()>;
    fn batch_upsert_pulls(&self, pulls: &[PullRequest]) -> Result<()>;
    fn get_pull(&self, number: u64) -> Result<Option<PullRequest>>;
    fn get_open_pulls(&self) -> Result<Vec<PullRequest>>;
    fn get_unanalyzed_pulls(&self) -> Result<Vec<PullRequest>>;
    fn get_pr_analysis(&self, pr_number: u64) -> Result<Option<PrAnalysisRow>>;
    /// Recently-closed pull requests (highest `updated_at` first), capped
    /// at `limit`. Backs the changelog view in TUI and the /api/v1/changelog
    /// endpoint.
    fn get_closed_pulls(&self, limit: usize) -> Result<Vec<PullRequest>>;

    // ── Triage ──────────────────────────────────────────────────

    fn upsert_triage_result(&self, result: &IssueClassification, issue_number: u64) -> Result<()>;
    /// Same as `upsert_triage_result` but also persists the content hash so
    /// the next batch can detect whether the issue changed and skip re-spending
    /// AI credits.
    fn upsert_triage_result_with_hash(
        &self,
        result: &IssueClassification,
        issue_number: u64,
        content_hash: Option<&str>,
    ) -> Result<()>;
    fn get_triage_result(&self, issue_number: u64) -> Result<Option<TriageResultRow>>;
    fn get_stale_triage_results(&self, max_age_hours: u32) -> Result<Vec<TriageResultRow>>;
    fn get_wshm_applied_labels(&self, issue_number: u64) -> Result<Vec<String>>;
    fn recent_activity(&self, limit: usize) -> Result<Vec<TriageResultRow>>;
    fn is_triaged(&self, issue_number: u64) -> Result<bool>;

    // ── PR analysis ─────────────────────────────────────────────

    /// Open PRs that need (re)analysis: never analyzed OR content_hash changed.
    fn get_pulls_needing_analysis(&self) -> Result<Vec<PullRequest>>;
    /// Upsert one row into `pr_analyses`. Replaces the previous inline
    /// `with_conn(|conn| INSERT ... ON CONFLICT ...)` so callers can run
    /// against any backend that implements this trait.
    fn upsert_pr_analysis(&self, row: &PrAnalysisRow) -> Result<()>;

    // ── Admin / maintenance ─────────────────────────────────────

    /// Wipe every triage result and PR analysis. Used by the `revert` flow
    /// to undo all wshm-applied state before re-syncing from the forge.
    fn clear_triage_and_analyses(&self) -> Result<()>;

    // ── Sync Log ────────────────────────────────────────────────

    fn get_sync_entry(&self, table_name: &str) -> Result<Option<SyncEntry>>;
    fn update_sync_entry(
        &self,
        table_name: &str,
        last_synced_at: &str,
        etag: Option<&str>,
    ) -> Result<()>;

    // ── Webhook Events ──────────────────────────────────────────

    fn insert_webhook_event(
        &self,
        event_type: &str,
        action: &str,
        number: Option<u64>,
        payload: &str,
    ) -> Result<i64>;
    fn update_event_status(&self, id: i64, status: &str, error: Option<&str>) -> Result<()>;
    fn pending_event_count(&self) -> Result<u64>;
    fn cleanup_old_events(&self, days: u32) -> Result<u64>;
    fn get_pending_events(&self) -> Result<Vec<WebhookEventRow>>;
}

/// Implement DatabaseBackend for the existing SQLite Database.
impl DatabaseBackend for super::Database {
    fn upsert_issue(&self, issue: &Issue) -> Result<()> {
        self.upsert_issue(issue)
    }

    fn batch_upsert_issues(&self, issues: &[Issue]) -> Result<()> {
        self.batch_upsert_issues(issues)
    }

    fn get_issue(&self, number: u64) -> Result<Option<Issue>> {
        self.get_issue(number)
    }

    fn get_open_issues(&self) -> Result<Vec<Issue>> {
        self.get_open_issues()
    }

    fn get_untriaged_issues(&self) -> Result<Vec<Issue>> {
        self.get_untriaged_issues()
    }

    fn get_issues_needing_triage(
        &self,
        limit: usize,
        relabel_labels: &[String],
        no_labels_min_age_hours: u32,
    ) -> Result<Vec<Issue>> {
        self.get_issues_needing_triage(limit, relabel_labels, no_labels_min_age_hours)
    }

    fn merge_issue_labels(&self, number: u64, add: &[String], remove: &[String]) -> Result<()> {
        self.merge_issue_labels(number, add, remove)
    }

    fn upsert_pull(&self, pr: &PullRequest) -> Result<()> {
        self.upsert_pull(pr)
    }

    fn batch_upsert_pulls(&self, pulls: &[PullRequest]) -> Result<()> {
        self.batch_upsert_pulls(pulls)
    }

    fn get_pull(&self, number: u64) -> Result<Option<PullRequest>> {
        self.get_pull(number)
    }

    fn get_open_pulls(&self) -> Result<Vec<PullRequest>> {
        self.get_open_pulls()
    }

    fn get_unanalyzed_pulls(&self) -> Result<Vec<PullRequest>> {
        self.get_unanalyzed_pulls()
    }

    fn get_pr_analysis(&self, pr_number: u64) -> Result<Option<PrAnalysisRow>> {
        self.get_pr_analysis(pr_number)
    }

    fn get_closed_pulls(&self, limit: usize) -> Result<Vec<PullRequest>> {
        self.get_closed_pulls(limit)
    }

    fn upsert_triage_result(&self, result: &IssueClassification, issue_number: u64) -> Result<()> {
        self.upsert_triage_result(result, issue_number)
    }

    fn upsert_triage_result_with_hash(
        &self,
        result: &IssueClassification,
        issue_number: u64,
        content_hash: Option<&str>,
    ) -> Result<()> {
        self.upsert_triage_result_with_hash(result, issue_number, content_hash)
    }

    fn get_triage_result(&self, issue_number: u64) -> Result<Option<TriageResultRow>> {
        self.get_triage_result(issue_number)
    }

    fn get_stale_triage_results(&self, max_age_hours: u32) -> Result<Vec<TriageResultRow>> {
        self.get_stale_triage_results(max_age_hours)
    }

    fn get_wshm_applied_labels(&self, issue_number: u64) -> Result<Vec<String>> {
        self.get_wshm_applied_labels(issue_number)
    }

    fn recent_activity(&self, limit: usize) -> Result<Vec<TriageResultRow>> {
        self.recent_activity(limit)
    }

    fn is_triaged(&self, issue_number: u64) -> Result<bool> {
        self.is_triaged(issue_number)
    }

    fn get_sync_entry(&self, table_name: &str) -> Result<Option<SyncEntry>> {
        self.get_sync_entry(table_name)
    }

    fn update_sync_entry(
        &self,
        table_name: &str,
        last_synced_at: &str,
        etag: Option<&str>,
    ) -> Result<()> {
        self.update_sync_entry(table_name, last_synced_at, etag)
    }

    fn insert_webhook_event(
        &self,
        event_type: &str,
        action: &str,
        number: Option<u64>,
        payload: &str,
    ) -> Result<i64> {
        self.insert_webhook_event(event_type, action, number, payload)
    }

    fn update_event_status(&self, id: i64, status: &str, error: Option<&str>) -> Result<()> {
        self.update_event_status(id, status, error)
    }

    fn pending_event_count(&self) -> Result<u64> {
        self.pending_event_count()
    }

    fn cleanup_old_events(&self, days: u32) -> Result<u64> {
        self.cleanup_old_events(days)
    }

    fn get_pending_events(&self) -> Result<Vec<WebhookEventRow>> {
        self.get_pending_events()
    }

    fn get_pulls_needing_analysis(&self) -> Result<Vec<PullRequest>> {
        self.get_pulls_needing_analysis()
    }

    fn upsert_pr_analysis(&self, row: &PrAnalysisRow) -> Result<()> {
        self.upsert_pr_analysis(row)
    }

    fn clear_triage_and_analyses(&self) -> Result<()> {
        self.clear_triage_and_analyses()
    }
}
