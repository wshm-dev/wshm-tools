//! Pro hooks registry — injected at startup by wshm-pro.
//! In the OSS build, all hooks are None (no-op).

use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::OnceLock;

use crate::config::Config;
use crate::daemon::web::WebState;
use crate::daemon::MultiDaemonState;
use crate::db::Database;
use crate::github::Client as GhClient;

// --- Feature gate hook ---
// Checks if a pro feature is available (license validation).
type FeatureGateFn = fn(&str) -> bool;

thread_local! {
    static FEATURE_GATE: RefCell<Option<FeatureGateFn>> = const { RefCell::new(None) };
}

pub fn set_feature_gate(f: FeatureGateFn) {
    FEATURE_GATE.with(|cell| {
        *cell.borrow_mut() = Some(f);
    });
}

/// Check if a pro feature is available.
/// Returns false in OSS build (no hook registered).
pub fn has_feature(feature: &str) -> bool {
    FEATURE_GATE.with(|cell| {
        cell.borrow()
            .map(|f| f(feature))
            .unwrap_or(false)
    })
}

/// Returns true if running as pro edition.
pub fn is_pro() -> bool {
    FEATURE_GATE.with(|cell| cell.borrow().is_some())
}

// --- Cloud sync hook ---
// Called after telemetry/events to sync with cloud.
type SyncHookFn = fn(&str, &serde_json::Value);

thread_local! {
    static SYNC_HOOK: RefCell<Option<SyncHookFn>> = const { RefCell::new(None) };
}

pub fn set_sync_hook(f: SyncHookFn) {
    SYNC_HOOK.with(|cell| {
        *cell.borrow_mut() = Some(f);
    });
}

pub fn maybe_sync(event: &str, data: &serde_json::Value) {
    SYNC_HOOK.with(|cell| {
        if let Some(f) = *cell.borrow() {
            f(event, data);
        }
    });
}

// --- Output filter hook ---
// Intercepts output for redaction (e.g., secret shielding).
type OutputHookFn = fn(&str) -> String;

thread_local! {
    static OUTPUT_HOOK: RefCell<Option<OutputHookFn>> = const { RefCell::new(None) };
}

pub fn set_output_hook(f: OutputHookFn) {
    OUTPUT_HOOK.with(|cell| {
        *cell.borrow_mut() = Some(f);
    });
}

pub fn apply_output_hook(text: &str) -> String {
    OUTPUT_HOOK.with(|cell| {
        match *cell.borrow() {
            Some(f) => f(text),
            None => text.to_string(),
        }
    })
}

// --- Async Pro pipeline hooks ---
// These allow wshm-core code paths (triage auto-fix, daemon slash commands)
// to invoke Pro-only pipelines (autogen, review) without depending on Pro
// source code at compile time.
//
// Each hook takes a borrowed context (Config, Database, Client, plus a
// minimal set of primitive args) and returns a boxed future resolving to
// `anyhow::Result<()>`. In the OSS build the hook is `None` and calls are
// skipped with a warning.
//
// Note: we use `OnceLock` + `&'static dyn` rather than `thread_local!` to
// ensure the hook survives tokio task migrations across worker threads.

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub type AutoFixHook = for<'a> fn(
    &'a Config,
    &'a Database,
    &'a GhClient,
    u64,
) -> BoxFuture<'a, anyhow::Result<()>>;

pub type ReviewHook = for<'a> fn(
    &'a Config,
    &'a Database,
    &'a GhClient,
    u64,
    bool,
) -> BoxFuture<'a, anyhow::Result<()>>;

static AUTO_FIX_HOOK: OnceLock<AutoFixHook> = OnceLock::new();
static REVIEW_HOOK: OnceLock<ReviewHook> = OnceLock::new();

pub fn set_auto_fix_hook(f: AutoFixHook) {
    let _ = AUTO_FIX_HOOK.set(f);
}

pub fn set_review_hook(f: ReviewHook) {
    let _ = REVIEW_HOOK.set(f);
}

/// Invoke the Pro auto-fix pipeline for a given issue number.
/// Returns `Ok(false)` if no Pro hook is registered (OSS build).
pub async fn run_auto_fix(
    config: &Config,
    db: &Database,
    gh: &GhClient,
    issue_number: u64,
) -> anyhow::Result<bool> {
    match AUTO_FIX_HOOK.get() {
        Some(f) => {
            f(config, db, gh, issue_number).await?;
            Ok(true)
        }
        None => Ok(false),
    }
}

/// Invoke the Pro review pipeline for a given PR number.
/// Returns `Ok(false)` if no Pro hook is registered (OSS build).
pub async fn run_review(
    config: &Config,
    db: &Database,
    gh: &GhClient,
    pr_number: u64,
    apply: bool,
) -> anyhow::Result<bool> {
    match REVIEW_HOOK.get() {
        Some(f) => {
            f(config, db, gh, pr_number, apply).await?;
            Ok(true)
        }
        None => Ok(false),
    }
}

// --- Web extensions hook ---
// Lets wshm-pro inject Pro API endpoints and/or override the embedded SPA
// serving at daemon startup. Returns `(extra_api, spa_override)`.
//
// `MultiDaemonState` is passed in so Pro routes can capture state if they
// need it (today they don't — they go through `WebState` — but the hook
// signature accepts it for future extensibility).

/// A pair of `(extra_api, spa_override)` routers returned by the web
/// extensions hook. Both sub-routers share `Arc<WebState>`.
pub type WebExtensions = (
    Option<axum::Router<Arc<WebState>>>,
    Option<axum::Router<Arc<WebState>>>,
);

pub type WebExtensionsFn = fn(&Arc<MultiDaemonState>) -> WebExtensions;

static WEB_EXTENSIONS_HOOK: OnceLock<WebExtensionsFn> = OnceLock::new();

pub fn set_web_extensions_hook(f: WebExtensionsFn) {
    let _ = WEB_EXTENSIONS_HOOK.set(f);
}

/// Invoke the web extensions hook if registered. Returns the pair of
/// optional routers `(extra_api, spa_override)`. In OSS build both are
/// `None`.
pub fn get_web_extensions(multi: &Arc<MultiDaemonState>) -> WebExtensions {
    match WEB_EXTENSIONS_HOOK.get() {
        Some(f) => f(multi),
        None => (None, None),
    }
}
