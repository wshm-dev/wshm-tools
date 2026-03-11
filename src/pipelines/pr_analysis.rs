use anyhow::Result;
use tracing::info;

use crate::ai::prompts::pr_analyze;
use crate::ai::schemas::PrAnalysis;
use crate::ai::AiClient;
use crate::cli::PrArgs;
use crate::config::Config;
use crate::db::pulls::PullRequest;
use crate::db::Database;
use crate::github::Client as GhClient;

pub async fn run(config: &Config, db: &Database, gh: &GhClient, args: &PrArgs) -> Result<()> {
    let ai = AiClient::new(config)?;

    let pulls = if let Some(number) = args.pr {
        match db.get_pull(number)? {
            Some(pr) => vec![pr],
            None => {
                println!("PR #{number} not found in cache. Run `wshm sync` first.");
                return Ok(());
            }
        }
    } else {
        db.get_unanalyzed_pulls()?
    };

    if pulls.is_empty() {
        println!("No PRs to analyze.");
        return Ok(());
    }

    for pr in &pulls {
        info!("Analyzing PR #{}: {}", pr.number, pr.title);
        match analyze_pr(config, &ai, db, gh, pr, args.apply).await {
            Ok(analysis) => {
                print_analysis(pr, &analysis, args.apply);
            }
            Err(e) => {
                tracing::error!("Failed to analyze PR #{}: {e:#}", pr.number);
            }
        }
    }

    Ok(())
}

async fn analyze_pr(
    _config: &Config,
    ai: &AiClient,
    db: &Database,
    gh: &GhClient,
    pr: &PullRequest,
    apply: bool,
) -> Result<PrAnalysis> {
    // Try to fetch diff (best-effort)
    let diff = match gh.fetch_pr_diff(pr.number).await {
        Ok(d) => Some(d),
        Err(e) => {
            tracing::warn!("Could not fetch diff for PR #{}: {e}", pr.number);
            None
        }
    };

    let user_prompt = pr_analyze::build_user_prompt(pr, diff.as_deref());
    let analysis: PrAnalysis = ai.complete(pr_analyze::SYSTEM, &user_prompt).await?;

    // Store in DB
    let now = chrono::Utc::now().to_rfc3339();
    db.with_conn(|conn| {
        conn.execute(
            "INSERT INTO pr_analyses (pr_number, summary, risk_level, pr_type, review_notes, analyzed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(pr_number) DO UPDATE SET
                summary = excluded.summary,
                risk_level = excluded.risk_level,
                pr_type = excluded.pr_type,
                review_notes = excluded.review_notes,
                analyzed_at = excluded.analyzed_at",
            rusqlite::params![
                pr.number,
                analysis.summary,
                analysis.risk_level,
                analysis.pr_type,
                serde_json::to_string(&analysis.review_checklist)?,
                now,
            ],
        )?;
        Ok(())
    })?;

    if apply {
        if !analysis.suggested_labels.is_empty() {
            gh.label_pr(pr.number, &analysis.suggested_labels).await?;
        }

        let comment = format_analysis_comment(&analysis);
        gh.comment_pr(pr.number, &comment).await?;

        info!("Applied analysis to PR #{}", pr.number);
    }

    Ok(analysis)
}

fn format_analysis_comment(a: &PrAnalysis) -> String {
    let mut comment = format!(
        "## 📊 PR Analysis\n\n\
         **Type:** {}\n\
         **Risk:** {}\n\n\
         {}\n\n\
         ### Review Checklist\n\
         - [{}] Tests present\n\
         - [{}] Breaking change\n\
         - [{}] Docs updated\n",
        a.pr_type,
        a.risk_level,
        a.summary,
        if a.review_checklist.tests_present {
            "x"
        } else {
            " "
        },
        if a.review_checklist.breaking_change {
            "x"
        } else {
            " "
        },
        if a.review_checklist.docs_updated {
            "x"
        } else {
            " "
        },
    );

    if !a.linked_issues.is_empty() {
        comment.push_str("\n**Linked issues:** ");
        let links: Vec<String> = a.linked_issues.iter().map(|n| format!("#{n}")).collect();
        comment.push_str(&links.join(", "));
        comment.push('\n');
    }

    comment.push_str("\n---\n*Analyzed by [wshm](https://github.com/pszymkowiak/wshm)*");
    comment
}

fn print_analysis(pr: &PullRequest, a: &PrAnalysis, applied: bool) {
    let status = if applied { "APPLIED" } else { "DRY-RUN" };
    println!(
        "[{status}] #{} {} → {} (risk: {}, type: {})",
        pr.number, pr.title, a.summary, a.risk_level, a.pr_type,
    );
}
