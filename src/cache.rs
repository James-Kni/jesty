use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::OnceLock};

#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub args: Vec<String>,
    pub paths: Vec<String>,
}

fn cache_file() -> &'static PathBuf {
    static PATH: OnceLock<PathBuf> = OnceLock::new();
    PATH.get_or_init(|| {
        dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from(".cache"))
            .join("jesty/last_run.json")
    })
}

pub fn load_cache() -> Option<Cache> {
    let path = cache_file();
    if !path.exists() {
        return None;
    }
    let string = fs::read(&path).ok()?;
    serde_json::from_slice(&string).ok()
}

pub fn save_cache(args: Vec<String>, paths: Vec<String>) -> Result<()> {
    let cache = Cache { args, paths };
    let path = cache_file();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(&path, serde_json::to_vec(&cache)?)?;
    Ok(())
}
