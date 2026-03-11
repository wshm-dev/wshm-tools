use anyhow::Result;
use tracing::info;

use crate::ai::prompts::issue_classify;
use crate::ai::schemas::IssueClassification;
use crate::ai::AiClient;
use crate::cli::TriageArgs;
use crate::config::Config;
use crate::db::issues::Issue;
use crate::db::Database;
use crate::github::Client as GhClient;

pub async fn run(config: &Config, db: &Database, gh: &GhClient, args: &TriageArgs) -> Result<()> {
    let ai = AiClient::new(config)?;

    let issues = if let Some(number) = args.issue {
        match db.get_issue(number)? {
            Some(issue) => vec![issue],
            None => {
                println!("Issue #{number} not found in cache. Run `wshm sync` first.");
                return Ok(());
            }
        }
    } else {
        db.get_untriaged_issues()?
    };

    if issues.is_empty() {
        println!("No issues to triage.");
        return Ok(());
    }

    let existing_issues = db.get_open_issues()?;

    for issue in &issues {
        info!("Triaging issue #{}: {}", issue.number, issue.title);
        match triage_issue(config, &ai, db, gh, issue, &existing_issues, args.apply).await {
            Ok(classification) => {
                print_classification(issue, &classification, args.apply);
            }
            Err(e) => {
                tracing::error!("Failed to triage issue #{}: {e:#}", issue.number);
            }
        }
    }

    Ok(())
}

async fn triage_issue(
    config: &Config,
    ai: &AiClient,
    db: &Database,
    gh: &GhClient,
    issue: &Issue,
    existing_issues: &[Issue],
    apply: bool,
) -> Result<IssueClassification> {
    let user_prompt = issue_classify::build_user_prompt(issue, existing_issues);
    let classification: IssueClassification =
        ai.complete(issue_classify::SYSTEM, &user_prompt).await?;

    // Store result in DB
    db.upsert_triage_result(&classification, issue.number)?;

    if apply && classification.confidence >= config.triage.auto_fix_confidence {
        // Apply labels
        if !classification.suggested_labels.is_empty() {
            gh.label_issue(issue.number, &classification.suggested_labels)
                .await?;
            db.update_issue_labels(issue.number, &classification.suggested_labels)?;
        }

        // Post triage comment
        let comment = format_triage_comment(&classification);
        gh.comment_issue(issue.number, &comment).await?;

        // Handle special categories
        match classification.category.as_str() {
            "duplicate" => {
                if let Some(original) = classification.is_duplicate_of {
                    let close_msg = format!(
                        "Closing as duplicate of #{original}. See the original issue for updates."
                    );
                    gh.comment_issue(issue.number, &close_msg).await?;
                    gh.close_issue(issue.number).await?;
                }
            }
            "wontfix" => {
                gh.close_issue(issue.number).await?;
            }
            _ => {}
        }

        info!("Applied triage to issue #{}", issue.number);
    }

    Ok(classification)
}

fn format_triage_comment(c: &IssueClassification) -> String {
    let mut comment = format!(
        "## 🔍 Triage Summary\n\n\
         **Category:** {}\n\
         **Priority:** {}\n\
         **Confidence:** {:.0}%\n\n\
         {}\n",
        c.category,
        c.priority.as_deref().unwrap_or("unset"),
        c.confidence * 100.0,
        c.summary,
    );

    if c.is_simple_fix {
        comment.push_str("\n💡 This looks like a simple fix that could be auto-resolved.\n");
    }

    if !c.relevant_files.is_empty() {
        comment.push_str("\n**Relevant files:**\n");
        for f in &c.relevant_files {
            comment.push_str(&format!("- `{f}`\n"));
        }
    }

    comment.push_str("\n---\n*Triaged by [wshm](https://github.com/pszymkowiak/wshm)*");
    comment
}

fn print_classification(issue: &Issue, c: &IssueClassification, applied: bool) {
    let status = if applied { "APPLIED" } else { "DRY-RUN" };
    println!(
        "[{status}] #{} {} → {} (confidence: {:.0}%, priority: {})",
        issue.number,
        issue.title,
        c.category,
        c.confidence * 100.0,
        c.priority.as_deref().unwrap_or("unset"),
    );
}
