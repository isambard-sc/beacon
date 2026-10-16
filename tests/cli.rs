// Integration tests only use a few of the package's dependencies
#![allow(unused_crate_dependencies)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

fn jwt(exp: i64) -> String {
    let body = URL_SAFE_NO_PAD.encode(format!(r#"{{"exp":{exp}}}"#));
    format!("header.{body}.sig")
}

/// A fresh HOME for one test, so the cache lives in a temp directory
fn home(test: &str) -> PathBuf {
    std::env::temp_dir().join(format!("beacon-{}-{test}", std::process::id()))
}

fn run_token(home: &Path) -> Result<Output> {
    Ok(Command::new(env!("CARGO_BIN_EXE_beacon"))
        .arg("token")
        .env("HOME", home)
        .env("XDG_CACHE_HOME", home)
        .output()?)
}

/// Write a cache holding an access token that expires at `exp`
fn write_cache(home: &Path, token: &str, exp: i64) -> Result<()> {
    // Where `dirs::cache_dir()` points given HOME and XDG_CACHE_HOME
    let cache_dir = if cfg!(target_os = "macos") {
        home.join("Library/Caches")
    } else {
        home.to_path_buf()
    }
    .join("beacon");
    std::fs::create_dir_all(&cache_dir)?;
    let cache = serde_json::json!({ "access_token": token, "expires_at": exp });
    std::fs::write(cache_dir.join("token.json"), cache.to_string())?;
    Ok(())
}

#[test]
fn token_prints_valid_cached_token() -> Result<()> {
    let home = home("valid");
    let exp = chrono::Utc::now().timestamp() + 300;
    let token = jwt(exp);
    write_cache(&home, &token, exp)?;

    let output = run_token(&home)?;

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout)?, format!("{token}\n"));
    Ok(())
}

#[test]
fn token_fails_near_expiry() -> Result<()> {
    let home = home("expiring");
    let exp = chrono::Utc::now().timestamp() + 30;
    write_cache(&home, &jwt(exp), exp)?;

    let output = run_token(&home)?;

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8(output.stderr)?.contains("Run `beacon auth`"));
    Ok(())
}

#[test]
fn token_fails_when_not_logged_in() -> Result<()> {
    let output = run_token(&home("none"))?;

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8(output.stderr)?,
        "You are not logged in. Run `beacon auth` to obtain an access token.\n"
    );
    Ok(())
}
