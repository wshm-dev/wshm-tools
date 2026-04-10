use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::PathBuf;
use std::sync::OnceLock;

const TELEMETRY_URL: Option<&str> = option_env!("WSHM_TELEMETRY_URL");
// Default: https://telemetry.wshm.dev/api/v1/telemetry
const PING_INTERVAL_SECS: u64 = 23 * 3600; // 23 hours

static CACHED_SALT: OnceLock<String> = OnceLock::new();

/// Send a telemetry ping if enabled and not already sent today.
/// Fire-and-forget: errors are silently ignored.
pub fn maybe_ping() {
    if TELEMETRY_URL.is_none() {
        return;
    }

    // Opt-out: env var
    if std::env::var("WSHM_TELEMETRY_DISABLED").unwrap_or_default() == "1" {
        return;
    }

    // Check last ping time
    let marker = telemetry_marker_path();
    if let Ok(metadata) = std::fs::metadata(&marker) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(elapsed) = modified.elapsed() {
                if elapsed.as_secs() < PING_INTERVAL_SECS {
                    return;
                }
            }
        }
    }

    touch_marker(&marker);

    std::thread::spawn(|| {
        let _ = send_ping();
    });
}

fn send_ping() -> Result<(), Box<dyn std::error::Error>> {
    let url = TELEMETRY_URL.ok_or("no telemetry URL")?;
    let device_hash = generate_device_hash();
    let version = env!("CARGO_PKG_VERSION").to_string();
    let os = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();

    // Get basic stats
    let repos = count_configured_repos();

    let payload = serde_json::json!({
        "machine_id": device_hash,
        "event": "ping",
        "data": {
            "version": version,
            "os": os,
            "arch": arch,
            "repos": repos,
            "install_method": detect_install_method(),
        },
        "app_version": version,
    });

    ureq::post(url)
        .set("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(2))
        .send_string(&payload.to_string())?;

    Ok(())
}

fn generate_device_hash() -> String {
    let salt = get_or_create_salt();
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_default();
    let username = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_default();

    let mut hasher = Sha256::new();
    hasher.update(salt.as_bytes());
    hasher.update(b":");
    hasher.update(hostname.as_bytes());
    hasher.update(b":");
    hasher.update(username.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn get_or_create_salt() -> String {
    CACHED_SALT
        .get_or_init(|| {
            let salt_path = salt_file_path();

            // Try to read existing salt
            if let Ok(contents) = std::fs::read_to_string(&salt_path) {
                let trimmed = contents.trim().to_string();
                if trimmed.len() == 64 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
                    return trimmed;
                }
            }

            // Generate new random salt
            let salt = random_salt();
            if let Some(parent) = salt_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(mut f) = std::fs::File::create(&salt_path) {
                let _ = f.write_all(salt.as_bytes());
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let _ = std::fs::set_permissions(
                        &salt_path,
                        std::fs::Permissions::from_mode(0o600),
                    );
                }
            }
            salt
        })
        .clone()
}

fn random_salt() -> String {
    let mut buf = [0u8; 32];
    if getrandom::getrandom(&mut buf).is_err() {
        // Fallback: use time + pid as entropy source
        let fallback = format!("{:?}:{}", std::time::SystemTime::now(), std::process::id());
        let mut hasher = Sha256::new();
        hasher.update(fallback.as_bytes());
        return format!("{:x}", hasher.finalize());
    }
    buf.iter().map(|b| format!("{:02x}", b)).collect()
}

fn salt_file_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("wshm")
        .join(".device_salt")
}

fn count_configured_repos() -> usize {
    let config_path = dirs::home_dir()
        .map(|h| h.join(".wshm").join("config.toml"))
        .unwrap_or_default();
    let content = std::fs::read_to_string(&config_path).unwrap_or_default();
    content.matches("[[repos]]").count()
}

fn detect_install_method() -> &'static str {
    let exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => return "unknown",
    };
    let path = std::fs::canonicalize(&exe)
        .unwrap_or(exe)
        .to_string_lossy()
        .to_string();

    if path.contains("homebrew") || path.contains("Cellar") { "homebrew" }
    else if path.contains("cargo") { "cargo" }
    else if path.contains(".local/bin") { "manual" }
    else { "unknown" }
}

fn telemetry_marker_path() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("wshm-telemetry-ping")
}

fn touch_marker(path: &PathBuf) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(path, "");
}
