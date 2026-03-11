use anyhow::Result;
use rusqlite::params;

use crate::ai::schemas::IssueClassification;
use crate::db::Database;

impl Database {
    pub fn upsert_triage_result(
        &self,
        result: &IssueClassification,
        issue_number: u64,
    ) -> Result<()> {
        self.with_conn(|conn| {
            let suggested_labels = serde_json::to_string(&result.suggested_labels)?;
            let relevant_files = serde_json::to_string(&result.relevant_files)?;
            let now = chrono::Utc::now().to_rfc3339();

            conn.execute(
                "INSERT INTO triage_results (issue_number, category, confidence, priority, summary, suggested_labels, is_duplicate_of, is_simple_fix, relevant_files, acted_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                 ON CONFLICT(issue_number) DO UPDATE SET
                    category = excluded.category,
                    confidence = excluded.confidence,
                    priority = excluded.priority,
                    summary = excluded.summary,
                    suggested_labels = excluded.suggested_labels,
                    is_duplicate_of = excluded.is_duplicate_of,
                    is_simple_fix = excluded.is_simple_fix,
                    relevant_files = excluded.relevant_files,
                    acted_at = excluded.acted_at",
                params![
                    issue_number,
                    result.category,
                    result.confidence,
                    result.priority,
                    result.summary,
                    suggested_labels,
                    result.is_duplicate_of,
                    result.is_simple_fix,
                    relevant_files,
                    now,
                ],
            )?;
            Ok(())
        })
    }

    pub fn is_triaged(&self, issue_number: u64) -> Result<bool> {
        self.with_conn(|conn| {
            let count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM triage_results WHERE issue_number = ?1",
                params![issue_number],
                |row| row.get(0),
            )?;
            Ok(count > 0)
        })
    }
}
