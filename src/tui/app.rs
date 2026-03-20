use anyhow::Result;
use std::collections::HashMap;

use crate::config::Config;
use crate::db::issues::Issue;
use crate::db::pulls::PullRequest;
use crate::db::triage::TriageResultRow;
use crate::db::Database;

#[derive(Clone, Copy, PartialEq)]
pub enum Tab {
    Issues,
    PullRequests,
    Queue,
    Stats,
    Activity,
}

impl Tab {
    pub fn title(&self) -> &'static str {
        match self {
            Tab::Issues => "Issues",
            Tab::PullRequests => "Pull Requests",
            Tab::Queue => "Merge Queue",
            Tab::Stats => "Stats",
            Tab::Activity => "Activity",
        }
    }

    pub fn all() -> &'static [Tab] {
        &[Tab::Issues, Tab::PullRequests, Tab::Queue, Tab::Stats, Tab::Activity]
    }
}

pub struct IssueRow {
    pub issue: Issue,
    pub triage: Option<TriageResultRow>,
}

pub struct Stats {
    pub by_category: Vec<(String, usize)>,
    pub by_priority: Vec<(String, usize)>,
    pub avg_confidence: f64,
    pub total_triaged: usize,
    pub recent_triages: Vec<TriageResultRow>,
}

pub struct App {
    pub repo_slug: String,
    pub active_tab: Tab,
    pub scroll_offset: usize,

    pub issues: Vec<IssueRow>,
    pub pulls: Vec<PullRequest>,
    pub triaged_count: usize,
    pub open_issue_count: usize,
    pub open_pr_count: usize,
    pub conflict_count: usize,
    pub stats: Stats,
}

impl App {
    pub fn new(config: &Config, db: &Database) -> Result<Self> {
        let mut app = Self {
            repo_slug: config.repo_slug(),
            active_tab: Tab::Issues,
            scroll_offset: 0,
            issues: Vec::new(),
            pulls: Vec::new(),
            triaged_count: 0,
            open_issue_count: 0,
            open_pr_count: 0,
            conflict_count: 0,
            stats: Stats {
                by_category: Vec::new(),
                by_priority: Vec::new(),
                avg_confidence: 0.0,
                total_triaged: 0,
                recent_triages: Vec::new(),
            },
        };
        app.refresh(db)?;
        Ok(app)
    }

    pub fn refresh(&mut self, db: &Database) -> Result<()> {
        let open_issues = db.get_open_issues()?;
        self.open_issue_count = open_issues.len();

        // Build issue rows with triage results
        self.issues = open_issues
            .into_iter()
            .map(|issue| {
                let triage = db.get_triage_result(issue.number).ok().flatten();
                IssueRow { issue, triage }
            })
            .collect();

        self.triaged_count = self.issues.iter().filter(|r| r.triage.is_some()).count();

        let pulls = db.get_open_pulls()?;
        self.open_pr_count = pulls.len();
        self.conflict_count = pulls.iter().filter(|p| p.mergeable == Some(false)).count();
        self.pulls = pulls;

        // Build stats from triage results
        self.build_stats();

        self.scroll_offset = 0;
        Ok(())
    }

    fn build_stats(&mut self) {
        let triaged: Vec<&TriageResultRow> = self.issues.iter()
            .filter_map(|r| r.triage.as_ref())
            .collect();

        self.stats.total_triaged = triaged.len();

        // Category breakdown
        let mut cat_map: HashMap<String, usize> = HashMap::new();
        for t in &triaged {
            *cat_map.entry(t.category.clone()).or_default() += 1;
        }
        self.stats.by_category = cat_map.into_iter().collect();
        self.stats.by_category.sort_by(|a, b| b.1.cmp(&a.1));

        // Priority breakdown
        let mut pri_map: HashMap<String, usize> = HashMap::new();
        for t in &triaged {
            let pri = t.priority.clone().unwrap_or_else(|| "unset".to_string());
            *pri_map.entry(pri).or_default() += 1;
        }
        self.stats.by_priority = pri_map.into_iter().collect();
        self.stats.by_priority.sort_by(|a, b| b.1.cmp(&a.1));

        // Average confidence
        if !triaged.is_empty() {
            let sum: f64 = triaged.iter().map(|t| t.confidence).sum();
            self.stats.avg_confidence = sum / triaged.len() as f64;
        }

        // Recent triages (last 20, by acted_at descending)
        let mut recent: Vec<TriageResultRow> = self.issues.iter()
            .filter_map(|r| r.triage.clone())
            .collect();
        recent.sort_by(|a, b| b.acted_at.cmp(&a.acted_at));
        recent.truncate(20);
        self.stats.recent_triages = recent;
    }

    pub fn next_tab(&mut self) {
        let tabs = Tab::all();
        let idx = tabs.iter().position(|t| *t == self.active_tab).unwrap_or(0);
        self.active_tab = tabs[(idx + 1) % tabs.len()];
        self.scroll_offset = 0;
    }

    pub fn prev_tab(&mut self) {
        let tabs = Tab::all();
        let idx = tabs.iter().position(|t| *t == self.active_tab).unwrap_or(0);
        self.active_tab = tabs[(idx + tabs.len() - 1) % tabs.len()];
        self.scroll_offset = 0;
    }

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        let max = self.current_list_len().saturating_sub(1);
        if self.scroll_offset < max {
            self.scroll_offset += 1;
        }
    }

    fn current_list_len(&self) -> usize {
        match self.active_tab {
            Tab::Issues => self.issues.len(),
            Tab::PullRequests => self.pulls.len(),
            Tab::Queue => self.pulls.len(),
            Tab::Stats => self.stats.recent_triages.len(),
            Tab::Activity => 0,
        }
    }
}
