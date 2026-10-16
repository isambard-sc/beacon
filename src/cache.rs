use std::io::Write as _;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt as _;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

/// The cached access token. Times are unix seconds.
#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub access_token: String,
    pub expires_at: i64,
}

fn path() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("beacon")
        .join("token.json")
}

pub fn load() -> Result<Cache> {
    let path = path();
    let contents = match std::fs::read_to_string(&path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            bail!("You are not logged in. Run `beacon auth` to obtain an access token.")
        }
        r => r.with_context(|| format!("Could not read token cache `{}`.", path.display()))?,
    };
    serde_json::from_str(&contents).context("Could not parse token cache.")
}

pub fn save(cache: &Cache) -> Result<()> {
    let path = path();
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)
            .with_context(|| format!("Could not create cache directory `{}`.", dir.display()))?;
    }
    let mut f = std::fs::OpenOptions::new();
    #[cfg(unix)]
    f.mode(0o600); // u=rw,g=,o=
    f.write(true)
        .truncate(true)
        .create(true)
        .open(&path)
        .with_context(|| format!("Could not open cache file `{}`.", path.display()))?
        .write_all(&serde_json::to_vec_pretty(cache)?)
        .context("Could not write to cache.")
}

pub fn delete() -> Result<()> {
    match std::fs::remove_file(path()) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        r => r.context("Could not delete cache file."),
    }
}
