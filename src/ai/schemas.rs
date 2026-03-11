use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueClassification {
    pub category: String,
    pub confidence: f64,
    pub priority: Option<String>,
    pub summary: String,
    pub suggested_labels: Vec<String>,
    pub is_duplicate_of: Option<u64>,
    pub is_simple_fix: bool,
    pub relevant_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrAnalysis {
    pub summary: String,
    pub risk_level: String,
    pub pr_type: String,
    pub linked_issues: Vec<u64>,
    pub review_checklist: ReviewChecklist,
    pub suggested_labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewChecklist {
    pub tests_present: bool,
    pub breaking_change: bool,
    pub docs_updated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolution {
    pub resolvable: bool,
    pub confidence: f64,
    pub strategy: String,
    pub description: String,
}
