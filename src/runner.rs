use std::process::Command;
use anyhow::{Context, Result};

use crate::cache::save_cache;

pub fn run_jest(args: Vec<String>, paths: Vec<String>) -> Result<()> {
    save_cache(args.clone(), paths.clone())?;

    let mut cmd = Command::new("npx");
    cmd.arg("jest");
    cmd.args(&args);
    cmd.args(&paths);

    let status = cmd.status().context("failed to launch jest")?;
    std::process::exit(status.code().unwrap_or(1));
}
