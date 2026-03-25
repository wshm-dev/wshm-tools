use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const LICENSE_API: &str = "https://api.wshm.dev/api/v1/license";

fn credentials_path() -> PathBuf {
    PathBuf::from(".wshm").join("credentials")
}

fn token_cache_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".wshm")
        .join("license.jwt")
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    email: String,
    plan: String,
    machine_id: String,
    exp: i64,
}

#[derive(Debug, Clone)]
pub struct LicenseStatus {
    pub valid: bool,
    pub plan: String,
    pub max_repos: Option<usize>,
    pub message: String,
}

impl LicenseStatus {
    pub fn free() -> Self {
        Self {
            valid: false,
            plan: "free".into(),
            max_repos: Some(1),
            message: "No license — limited to 1 repo. Run: wshm login --license".into(),
        }
    }

    pub fn repo_limit(&self) -> Option<usize> {
        if self.valid {
            None // unlimited
        } else {
            Some(1)
        }
    }
}

/// Check license at startup. Non-blocking, never fails.
pub fn check() -> LicenseStatus {
    // 1. Try cached JWT (offline)
    if let Some(status) = verify_cached() {
        return status;
    }

    // 2. Try to validate with API
    if let Some(status) = validate_remote() {
        return status;
    }

    LicenseStatus::free()
}

/// Login flow: ask for license key and validate
pub fn login() -> Result<()> {
    println!("\n── License ──");
    println!("Enter your license key (get one at https://wshm.dev):\n");

    print!("License key: ");
    std::io::Write::flush(&mut std::io::stdout())?;
    let mut key = String::new();
    std::io::stdin().read_line(&mut key)?;
    let key = key.trim();

    if key.is_empty() {
        println!("Skipped.");
        return Ok(());
    }

    // Save to credentials
    save_credential("LICENSE_KEY", key)?;

    // Generate machine ID
    let machine_id = generate_machine_id();

    // Validate
    let client = ureq::agent();
    let resp = client
        .post(&format!("{LICENSE_API}/activate"))
        .set("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .send_string(&serde_json::json!({
            "license_key": key,
            "machine_id": machine_id,
            "app_version": env!("CARGO_PKG_VERSION"),
        }).to_string());

    match resp {
        Ok(r) => {
            let body: serde_json::Value = r.into_json().unwrap_or_default();
            if let Some(token) = body["token"].as_str() {
                cache_token(token);
                let plan = body["license"]["type"].as_str().unwrap_or("pro");
                println!("✓ License activated — plan: {plan}");
            } else {
                println!("✗ Unexpected response: {body}");
            }
        }
        Err(ureq::Error::Status(code, _)) => {
            println!("✗ Validation failed (HTTP {code})");
        }
        Err(e) => {
            println!("✗ Cannot reach license server: {e}");
        }
    }

    Ok(())
}

fn verify_cached() -> Option<LicenseStatus> {
    let path = token_cache_path();
    let token = fs::read_to_string(&path).ok()?;
    let token = token.trim();
    if token.is_empty() {
        return None;
    }

    // Decode JWT payload (no signature verification — offline)
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return None;
    }

    use base64::Engine;
    let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(parts[1])
        .ok()?;
    let claims: JwtClaims = serde_json::from_slice(&payload).ok()?;

    // Check expiry
    let now = chrono::Utc::now().timestamp();
    if claims.exp < now {
        return None; // expired, need to re-validate
    }

    Some(LicenseStatus {
        valid: true,
        plan: claims.plan,
        max_repos: None,
        message: format!("Licensed ({})", claims.email),
    })
}

fn validate_remote() -> Option<LicenseStatus> {
    let key = get_credential("LICENSE_KEY")?;
    let machine_id = generate_machine_id();

    let resp = ureq::post(&format!("{LICENSE_API}/activate"))
        .set("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(5))
        .send_string(&serde_json::json!({
            "license_key": key,
            "machine_id": machine_id,
            "app_version": env!("CARGO_PKG_VERSION"),
        }).to_string())
        .ok()?;

    let body: serde_json::Value = resp.into_json().ok()?;
    let token = body["token"].as_str()?;
    cache_token(token);

    let plan = body["license"]["type"].as_str().unwrap_or("pro").to_string();
    Some(LicenseStatus {
        valid: true,
        plan,
        max_repos: None,
        message: "License valid".into(),
    })
}

fn cache_token(token: &str) {
    let path = token_cache_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(&path, token);
}

fn generate_machine_id() -> String {
    use sha2::{Digest, Sha256};
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_default();
    let username = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(format!("{hostname}:{username}"));
    format!("{:x}", hasher.finalize())
}

fn get_credential(key: &str) -> Option<String> {
    let path = credentials_path();
    let content = fs::read_to_string(&path).ok()?;
    for line in content.lines() {
        let line = line.trim();
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn save_credential(key: &str, value: &str) -> Result<()> {
    let path = credentials_path();
    let mut creds: Vec<(String, String)> = Vec::new();

    if let Ok(content) = fs::read_to_string(&path) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                if k.trim() != key {
                    creds.push((k.trim().to_string(), v.trim().to_string()));
                }
            }
        }
    }
    creds.push((key.to_string(), value.to_string()));

    let content: String = creds
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, format!("# wshm credentials\n{content}\n"))
        .context("Failed to save credentials")?;
    Ok(())
}
