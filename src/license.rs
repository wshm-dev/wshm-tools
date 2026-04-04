use anyhow::{Context, Result};
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

/// Interactive login flow: ask for license key, activate, cache JWT.
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

    // Activate with API
    let resp = ureq::post(&format!("{LICENSE_API}/activate"))
        .set("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .send_string(
            &serde_json::json!({
                "license_key": key,
                "machine_id": machine_id,
                "app_version": env!("CARGO_PKG_VERSION"),
            })
            .to_string(),
        );

    match resp {
        Ok(r) => {
            let body: serde_json::Value = r.into_json().unwrap_or_default();
            if let Some(token) = body["token"].as_str() {
                cache_token(token)?;
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

fn cache_token(token: &str) -> Result<()> {
    let path = token_cache_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, token)?;
    // Restrict permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&path, fs::Permissions::from_mode(0o600));
    }
    Ok(())
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
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&path, fs::Permissions::from_mode(0o600));
    }
    Ok(())
}
