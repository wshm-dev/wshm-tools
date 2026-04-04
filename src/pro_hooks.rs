//! Pro hooks registry — injected at startup by wshm-pro.
//! In the OSS build, all hooks are None (no-op).

use std::cell::RefCell;

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
