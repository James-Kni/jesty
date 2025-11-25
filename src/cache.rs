use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub args: Vec<String>,
    pub paths: Vec<String>,
}

fn cache_file() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from(".cache"))
        .join("jesty/last_run.json")
}

pub fn load_cache() -> Option<Cache> {
    let p = cache_file();
    if !p.exists() {
        return None;
    }
    let s = fs::read_to_string(&p).ok()?;
    serde_json::from_str(&s).ok()
}

pub fn save_cache(args: Vec<String>, paths: Vec<String>) -> Result<()> {
    let c = Cache { args, paths };
    let p = cache_file();
    if let Some(d) = p.parent() {
        fs::create_dir_all(d)?;
    }
    fs::write(&p, serde_json::to_string(&c)?)?;
    Ok(())
}
