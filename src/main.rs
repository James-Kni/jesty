mod cache;
mod picker;
mod runner;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use crate::{cache::load_cache, picker::pick_files, runner::run_jest};

#[derive(Parser)]
#[command(trailing_var_arg = true, allow_hyphen_values = true)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg()]
    args: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    Rerun,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Commands::Rerun) => {
            let cache = load_cache().context("no previous run")?;
            run_jest(cache.args, cache.paths)
        }
        None => {
            let paths = pick_files()?;
            run_jest(args.args, paths)
        }
    }
}
