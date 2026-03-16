use anyhow::Result;
use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use hmac::{Hmac, Mac};
use serde_json::{json, Value};
use sha2::Sha256;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, warn};

use super::processor::WebhookEvent;
use super::{DaemonState, MultiDaemonState};

struct ServerState {
    daemon: Arc<DaemonState>,
    tx: mpsc::Sender<WebhookEvent>,
    secret: Option<String>,
}

pub async fn run(
    daemon: Arc<DaemonState>,
    tx: mpsc::Sender<WebhookEvent>,
    bind: &str,
    secret: Option<&str>,
) -> Result<()> {
    let state = Arc::new(ServerState {
        daemon,
        tx,
        secret: secret.map(|s| s.to_string()),
    });

    let app = Router::new()
        .route("/webhook", post(handle_webhook))
        .route("/health", get(handle_health))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(bind).await?;
    info!("Webhook server listening on {bind}");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handle_health(State(state): State<Arc<ServerState>>) -> impl IntoResponse {
    let pending = state.daemon.db.pending_event_count().unwrap_or(0);
    Json(json!({
        "status": "ok",
        "apply": state.daemon.apply,
        "pending_events": pending,
        "repo": format!("{}/{}", state.daemon.config.repo_owner, state.daemon.config.repo_name),
    }))
}

async fn handle_webhook(
    State(state): State<Arc<ServerState>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    // Validate HMAC signature if secret is configured
    if let Some(ref secret) = state.secret {
        let sig_header = headers
            .get("x-hub-signature-256")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if !verify_signature(secret, &body, sig_header) {
            warn!("Invalid webhook signature");
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "invalid signature"})),
            )
                .into_response();
        }
    }

    // Parse event type from headers
    let event_type = match headers.get("x-github-event").and_then(|v| v.to_str().ok()) {
        Some(e) => e.to_string(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "missing x-github-event header"})),
            )
                .into_response();
        }
    };

    // Parse payload
    let payload: Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(e) => {
            warn!("Failed to parse webhook payload: {e}");
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "invalid JSON"})),
            )
                .into_response();
        }
    };

    let action = payload
        .get("action")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Filter: only process relevant events
    let should_process = match event_type.as_str() {
        "issues" => action == "opened",
        "pull_request" => action == "opened" || action == "synchronize",
        "issue_comment" => action == "created",
        _ => false,
    };

    if !should_process {
        info!("Ignoring event: {event_type}.{action}");
        return (
            StatusCode::OK,
            Json(json!({"status": "ignored", "event": event_type, "action": action})),
        )
            .into_response();
    }

    // Extract number
    let number = extract_number(&event_type, &payload);

    // Store in DB
    let payload_str = serde_json::to_string(&payload).unwrap_or_default();
    let event_id =
        match state
            .daemon
            .db
            .insert_webhook_event(&event_type, &action, number, &payload_str)
        {
            Ok(id) => id,
            Err(e) => {
                tracing::error!("Failed to store webhook event: {e}");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "failed to store event"})),
                )
                    .into_response();
            }
        };

    info!("Received webhook: {event_type}.{action} number={number:?} id={event_id}");

    // Enqueue for processing
    let event = WebhookEvent {
        id: event_id,
        event_type: event_type.clone(),
        action: action.clone(),
        number,
        payload: payload_str,
    };

    if let Err(e) = state.tx.send(event).await {
        tracing::error!("Failed to enqueue event: {e}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "queue full"})),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(json!({"status": "accepted", "event": event_type, "action": action, "id": event_id})),
    )
        .into_response()
}

// ── Multi-repo server ──────────────────────────────────────────

struct MultiServerState {
    multi: Arc<MultiDaemonState>,
    tx: mpsc::Sender<(String, WebhookEvent)>,
    secret: Option<String>,
}

pub async fn run_multi(
    multi: Arc<MultiDaemonState>,
    tx: mpsc::Sender<(String, WebhookEvent)>,
    bind: &str,
    secret: Option<&str>,
) -> Result<()> {
    let state = Arc::new(MultiServerState {
        multi,
        tx,
        secret: secret.map(|s| s.to_string()),
    });

    let app = Router::new()
        .route("/webhook", post(handle_webhook_multi))
        .route("/health", get(handle_health_multi))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(bind).await?;
    info!("Multi-repo webhook server listening on {bind}");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handle_health_multi(State(state): State<Arc<MultiServerState>>) -> impl IntoResponse {
    let repos: Vec<Value> = state
        .multi
        .repos
        .iter()
        .map(|(slug, ds)| {
            let pending = ds.db.pending_event_count().unwrap_or(0);
            json!({
                "repo": slug,
                "apply": ds.apply,
                "pending_events": pending,
            })
        })
        .collect();

    Json(json!({
        "status": "ok",
        "repos": repos,
    }))
}

async fn handle_webhook_multi(
    State(state): State<Arc<MultiServerState>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    // Validate HMAC signature if global secret is configured
    if let Some(ref secret) = state.secret {
        let sig_header = headers
            .get("x-hub-signature-256")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if !verify_signature(secret, &body, sig_header) {
            warn!("Invalid webhook signature");
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "invalid signature"})),
            )
                .into_response();
        }
    }

    // Parse event type
    let event_type = match headers.get("x-github-event").and_then(|v| v.to_str().ok()) {
        Some(e) => e.to_string(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "missing x-github-event header"})),
            )
                .into_response();
        }
    };

    // Parse payload
    let payload: Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(e) => {
            warn!("Failed to parse webhook payload: {e}");
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "invalid JSON"})),
            )
                .into_response();
        }
    };

    // Route by repository.full_name
    let repo_slug = match payload
        .get("repository")
        .and_then(|r| r.get("full_name"))
        .and_then(|n| n.as_str())
    {
        Some(s) => s.to_string(),
        None => {
            warn!("Webhook payload missing repository.full_name");
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "missing repository.full_name"})),
            )
                .into_response();
        }
    };

    let daemon = match state.multi.repos.get(&repo_slug) {
        Some(d) => d,
        None => {
            info!("Ignoring webhook for unconfigured repo: {repo_slug}");
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "repo not configured", "repo": repo_slug})),
            )
                .into_response();
        }
    };

    let action = payload
        .get("action")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Filter relevant events
    let should_process = match event_type.as_str() {
        "issues" => action == "opened",
        "pull_request" => action == "opened" || action == "synchronize",
        "issue_comment" => action == "created",
        _ => false,
    };

    if !should_process {
        return (
            StatusCode::OK,
            Json(json!({"status": "ignored", "repo": repo_slug, "event": event_type})),
        )
            .into_response();
    }

    let number = extract_number(&event_type, &payload);

    // Store in the correct repo's DB
    let payload_str = serde_json::to_string(&payload).unwrap_or_default();
    let event_id = match daemon
        .db
        .insert_webhook_event(&event_type, &action, number, &payload_str)
    {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("[{repo_slug}] Failed to store webhook event: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "failed to store event"})),
            )
                .into_response();
        }
    };

    info!("[{repo_slug}] Received webhook: {event_type}.{action} number={number:?} id={event_id}");

    let event = WebhookEvent {
        id: event_id,
        event_type: event_type.clone(),
        action: action.clone(),
        number,
        payload: payload_str,
    };

    if let Err(e) = state.tx.send((repo_slug.clone(), event)).await {
        tracing::error!("[{repo_slug}] Failed to enqueue event: {e}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "queue full"})),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(json!({"status": "accepted", "repo": repo_slug, "event": event_type, "id": event_id})),
    )
        .into_response()
}

// ── Shared helpers ─────────────────────────────────────────────

fn extract_number(event_type: &str, payload: &Value) -> Option<u64> {
    match event_type {
        "issues" => payload
            .get("issue")
            .and_then(|i| i.get("number"))
            .and_then(|n| n.as_u64()),
        "pull_request" => payload
            .get("pull_request")
            .and_then(|p| p.get("number"))
            .and_then(|n| n.as_u64()),
        "issue_comment" => payload
            .get("issue")
            .and_then(|i| i.get("number"))
            .and_then(|n| n.as_u64()),
        _ => None,
    }
}

fn verify_signature(secret: &str, body: &[u8], sig_header: &str) -> bool {
    let expected_hex = match sig_header.strip_prefix("sha256=") {
        Some(h) => h,
        None => return false,
    };
    let expected_bytes = match hex::decode(expected_hex) {
        Ok(b) => b,
        Err(_) => return false,
    };
    let mut mac = match Hmac::<Sha256>::new_from_slice(secret.as_bytes()) {
        Ok(m) => m,
        Err(_) => return false,
    };
    mac.update(body);
    // Constant-time comparison via hmac::Mac::verify_slice
    mac.verify_slice(&expected_bytes).is_ok()
}
