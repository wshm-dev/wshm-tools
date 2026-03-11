use crate::db::issues::Issue;

pub const SYSTEM: &str = r#"You are a GitHub issue triage assistant. Classify the given issue and respond with a JSON object.

Categories:
- "bug" — something is broken
- "feature" — a request for new functionality
- "duplicate" — already reported (provide original issue number)
- "wontfix" — out of scope or by design
- "needs-info" — insufficient information to act on

Priority levels: "critical", "high", "medium", "low"

Response format (JSON only, no markdown):
{
  "category": "bug|feature|duplicate|wontfix|needs-info",
  "confidence": 0.0-1.0,
  "priority": "critical|high|medium|low",
  "summary": "One-line summary of the issue and recommended action",
  "suggested_labels": ["label1", "label2"],
  "is_duplicate_of": null or issue_number,
  "is_simple_fix": true/false,
  "relevant_files": ["path/to/file.rs"]
}

Be precise. If unsure, lower your confidence score. Only mark is_simple_fix=true for clear, localized bugs fixable in 1-3 files."#;

pub fn build_user_prompt(issue: &Issue, existing_issues: &[Issue]) -> String {
    let mut prompt = format!(
        "## Issue #{}: {}\n\n{}\n\n**Author:** {}\n**Labels:** {}\n**Created:** {}\n",
        issue.number,
        issue.title,
        issue.body.as_deref().unwrap_or("(no description)"),
        issue.author.as_deref().unwrap_or("unknown"),
        if issue.labels.is_empty() {
            "none".to_string()
        } else {
            issue.labels.join(", ")
        },
        issue.created_at,
    );

    if !existing_issues.is_empty() {
        prompt.push_str("\n## Existing open issues (for duplicate detection):\n");
        for existing in existing_issues.iter().take(50) {
            prompt.push_str(&format!("- #{}: {}\n", existing.number, existing.title));
        }
    }

    prompt
}
